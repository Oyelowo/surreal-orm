use std::{collections::HashMap, fmt::Display};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote};

use crate::{
    errors::ExtractorResult,
    models::{
        create_ident_wrapper, create_tokenstream_wrapper, derive_attributes::TableDeriveAttributes,
        variables::VariablesModelMacro, RelateAttribute, StaticAssertionToken,
    },
};

use super::{
    variables::VariablesModelMacro, EdgeDirection, FieldGenericsMeta, MyFieldReceiver, Relate,
};

impl MyFieldReceiver {}

type EdgeNameWithDirectionIndicator = String;

// struct EdgeTableName(syn::Ident);
create_ident_wrapper!(EdgeTableName);
create_ident_wrapper!(OriginNodeStructIdent);
create_ident_wrapper!(EdgeNameAsMethodIdent);
create_ident_wrapper!(EdgeRelationModelSelectedIdent);

create_tokenstream_wrapper!(=>DestinationNodeSchema);
create_tokenstream_wrapper!(=>ForeignNodeConnectionMethod);
create_tokenstream_wrapper!(=>RelationImports);
create_tokenstream_wrapper!(=>DestinationNodeTypeAlias);
create_tokenstream_wrapper!(=>EdgeToDestinationNodeMethod);
create_tokenstream_wrapper!(=>DestinationNodeSchemaOne);
create_ident_wrapper!(ForeignNodeAssociatedTypeInOrOut);

struct DestinationNodeName(String);

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct NodeEdgeMetadata {
    /// Example value: writes
    edge_table_name: EdgeTableName,
    /// The current struct name ident.
    /// e.g given: struct Student {  }, value = Student
    origin_struct_ident: OriginNodeStructIdent,
    /// The database table name of the edge. Used for generating other tokens
    /// e.g "writes"
    direction: EdgeDirection,
    /// Example of value: `StudentWritesBook`
    ///
    /// For each edge table e.g writes, we usually can have many aliases reusing thesame edge
    /// e.g for Writes<In, Out>, we could have  StudentWritesBook, StudentWritesBlog, for each direction(e.g ->),
    /// we want to select one of these to use its schema which is aliased as the Cased table name
    /// in the calling location e.g
    /// for a model field annotation e.g relate(edge="StudentWritesBook", link="->writes->book")
    /// So we can do
    /// type Writes = <StudentWritesBook as Edge>::Schema;
    edge_relation_model_selected_ident: EdgeRelationModelSelectedIdent,
    /// Example Generated:
    /// ```rust, ignore
    ///   type BookModel = <StudentWritesBook as surreal_macros::Edge>::Out;
    ///   type Book = <BookModel as surreal_macros::Node>::Schema;
    ///
    ///   type BlogModel = <StudentWritesBlog as surreal_macros::Edge>::Out;
    ///   type Blog = <BlogModel as surreal_macros::Node>::Schema;
    /// ```
    ///
    /// Example Value:
    /// ```rust, ignore
    /// vec![
    ///    quote!(
    ///       type BookModel = <StudentWritesBook as surreal_macros::Edge>::Out;
    ///       type Book = <BookModel as surreal_macros::Node>::Schema;
    ///     ),
    ///     quote!(
    ///       type BlogModel = <StudentWritesBlog as surreal_macros::Edge>::Out;
    ///       type Blog = <BlogModel as surreal_macros::Node>::Schema;
    ///     ),
    /// ],
    /// ```
    destination_node_schema: Vec<DestinationNodeSchema>,
    destination_node_name: DestinationNodeSchema,
    /// Example Generated:
    ///
    /// ```rust, ignore
    /// impl Writes__ {
    ///     fn book(&self, filter: Filter) -> Book {
    ///         Book::__________connect_to_graph_traversal_string(
    ///             &self.___________graph_traversal_string,
    ///             filter,
    ///         )
    ///     }
    ///
    ///     fn blog(&self, filter: Filter) -> Blog {
    ///         Blog::__________connect_to_graph_traversal_string(
    ///             &self.___________graph_traversal_string,
    ///             filter,
    ///         )
    ///     }
    /// }
    /// ```
    ///
    /// Example Value:
    /// ```rust, ignore
    /// vec![
    ///     quote!(
    ///        fn book(&self, filter: Filter) -> Book {
    ///            Book::__________connect_to_graph_traversal_string(
    ///                &self.___________graph_traversal_string,
    ///                filter,
    ///            )
    ///        }
    ///     ),
    ///     quote!(
    ///        fn blog(&self, filter: Filter) -> Blog {
    ///            Blog::__________connect_to_graph_traversal_string(
    ///                &self.___________graph_traversal_string,
    ///                filter,
    ///            )
    ///        }
    ///     ),
    ///    ]
    /// ```
    foreign_node_connection_method: Vec<ForeignNodeConnectionMethod>,
    static_assertions: Vec<StaticAssertionToken>,
    imports: Vec<RelationImports>,
    edge_name_as_method_ident: EdgeNameAsMethodIdent,
}

#[derive(Default, Clone)]
pub struct NodeEdgeMetadataStore(HashMap<EdgeNameWithDirectionIndicator, NodeEdgeMetadata>);
create_ident_wrapper!(EdgeWithDunderDirectionIndicator);

impl NodeEdgeMetadataStore {
    /// e.g for ->writes->book, gives writes__. <-writes<-book, gives __writes
    fn add_direction_indication_to_ident(
        &self,
        ident: &impl Display,
        edge_direction: &EdgeDirection,
    ) -> EdgeWithDunderDirectionIndicator {
        let edge = match edge_direction {
            EdgeDirection::OutArrowRight => format_ident!("{ident}__"),
            EdgeDirection::InArrowLeft => format_ident!("__{ident}"),
        };
        edge.into()
    }

    fn create_static_assertions(
        relation: &Relate,
        origin_struct_ident: &syn::Ident,
        field_type: &syn::Type,
    ) -> StaticAssertionToken {
        let crate_name = get_crate_name(false);
        let relation_model = relation.edge_type;
        let relation_attributes = RelateAttribute::from(relation);
        let RelateAttribute {
            edge_table_name,
            node_table_name: foreign_node_table_name,
            edge_direction,
        } = relation_attributes;

        let (home_node_associated_type_ident, foreign_node_associated_type_ident) =
            match &relation_attributes.edge_direction {
                EdgeDirection::OutArrowRight => (format_ident!("In"), format_ident!("Out")),
                EdgeDirection::InArrowLeft => (format_ident!("Out"), format_ident!("In")),
            };

        // e.g for struct Student {
        //                   #[surreal_orm(relate(mode="StudentWritesBook", connection="->writes->book"))]
        //                   fav_books: Relate<Book>
        //              }
        let static_assertions = &[
            // type HomeIdent = <StudentWritesBook  as surreal_macros::Edge>::In;
            // type HomeNodeTableChecker = <HomeIdent as
            // surreal_macros::Node>::TableNameChecker;
            // #crate_name::validators::assert_type_eq_all!(HomeIdent, Student);
            // #crate_name::validators::assert_impl_one!(HomeIdent, surreal_macros::Node);
            quote!(
            {
            type #home_node_ident = <#relation_model as #crate_name::Edge>::#home_node_associated_type_ident;
             type #home_node_table_name_checker_ident = <#home_node_ident as #crate_name::Node>::TableNameChecker;
             #crate_name::validators::assert_type_eq_all!(#home_node_ident, #origin_struct_ident);
             #crate_name::validators::assert_impl_one!(#home_node_ident: #crate_name::Node);

            }
            ),
            quote!(
             type #foreign_node_ident = <#relation_model as #crate_name::Edge>::#foreign_node_associated_type_ident;
             type #foreign_node_table_name_checker_ident = <#foreign_node_ident as #crate_name::Node>::TableNameChecker;
             #crate_name::validators::assert_fields!(#foreign_node_table_name_checker_ident: #foreign_node_table_name);
             #crate_name::validators::assert_impl_one!(#foreign_node_ident: #crate_name::Node);
            ),
            quote!(
             type #edge_table_name_checker_ident = <#relation_model as #crate_name::Edge>::TableNameChecker;
             #crate_name::validators::assert_fields!(#edge_table_name_checker_ident: #edge_table_name);
            ),
            // assert field type and attribute reference match
            // e.g Relate<Book> should match from attribute link = "->Writes->Book"
            quote!(
             #crate_name::validators::assert_impl_one!(#relation_model: #crate_name::Edge);
             #crate_name::validators::assert_type_eq_all!(#field_type,  #crate_name::Relate<#foreign_node_ident>);
            ),
        ];
        quote!(
                #( #static_assertions) *
        )
    }

    fn update(
        &mut self,
        relation: &Relate,
        field_receriver: &MyFieldReceiver,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let crate_name = get_crate_name(false);
        let origin_struct_ident = &table_derive_attrs.ident;
        let field_type = &field_receriver.ty;
        let edge_type = relation.edge_type;
        let RelateAttribute {
            edge_direction,
            edge_table_name,
            node_table_name: destination_node_table_name,
        } = &RelateAttribute::from(relation);
        let arrow = format!("{}", &edge_direction);
        let destination_node_table_name_str = &destination_node_table_name.to_string();
        let VariablesModelMacro {
            __________connect_node_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();
        let edge_name_as_method_ident =
            &(|| self.add_direction_indication_to_ident(edge_table_name, edge_direction));
        let FieldGenericsMeta {
            field_impl_generics,
            field_ty_generics,
            field_where_clause,
        } = &edge_type.get_generics_meta(table_derive_attrs);
        // represents the schema but aliased as the pascal case of the destination table name
        let destination_node_schema_ident = format_ident!(
            "{}",
            destination_node_table_name
                .to_string()
                .to_case(Case::Pascal)
        );
        let destination_node_schema_type_alias =
            DestinationNodeTypeAlias(quote!(#destination_node_schema_ident #field_ty_generics));
        // let dest_node_as_type =
        //     syn::parse_str::<syn::Type>(&destination_node_schema_type_alias.to_string())?;

        // Meant to represent the variable of struct model(node) itself.
        // Within edge generics, there is usually In and Out associated types, this is used to
        // access those
        let foreign_node_in_or_out = ForeignNodeAssociatedTypeInOrOut(match edge_direction {
            EdgeDirection::OutArrowRight => format_ident!("Out"),
            EdgeDirection::InArrowLeft => format_ident!("In"),
        });
        // We use super twice because we're trying to access the relation model struct name from
        // the outer outer module because all edge related functionalities are nested
        let destination_node_schema_one = || {
            DestinationNodeSchema(quote!(
                type #destination_node_schema_type_alias #field_ty_generics =
                            <<super::super::#edge_type as #crate_name::Edge>::#foreign_node_in_or_out
                        as #crate_name::SchemaGetter>::Schema;
            ))
        };

        // i.e Edge to destination Node
        let foreign_node_connection_method = || {
            EdgeToDestinationNodeMethod(quote!(
                pub fn #destination_node_table_name(self, clause: impl ::std::convert::Into<#crate_name::NodeClause>) -> #destination_node_schema_type_alias {
                    let clause: #crate_name::NodeClause = clause.into();
                    let clause = clause.with_arrow(#arrow).with_table(#destination_node_table_name_str);

                    #destination_node_schema_type_alias::#__________connect_node_to_graph_traversal_string(
                                self,
                                clause,
                    )
                }
            ))
        };
        let static_assertions =
            || Self::create_static_assertions(relation, origin_struct_ident, field_type);

        // let imports =|| quote!(use super::StudentWritesBook;);
        let imports = || quote!(use super::#edge_type;);

        let node_edge_meta = NodeEdgeMetadata {
            edge_table_name: format_ident!(
                "{}",
                &relation_attributes.edge_table_name.clone().to_string()
            ),
            direction: *edge_direction,
            destination_node_schema: vec![destination_node_schema_one()],
            foreign_node_connection_method: vec![foreign_node_connection_method()],
            origin_struct_ident: origin_struct_ident.to_owned(),
            static_assertions: vec![static_assertions()],
            edge_name_as_method_ident: format_ident!("{}", edge_name_as_method_ident()),
            imports: vec![imports()],
            edge_relation_model_selected_ident: relation.edge_type.type_name()?,
            destination_node_name: destination_node_table_name.to_string(),
        };

        match self.0.entry(edge_name_as_method_ident()) {
            Entry::Occupied(o) => {
                let node_edge_meta = o.into_mut();
                node_edge_meta
                    .destination_node_schema
                    .push(destination_node_schema_one());
                node_edge_meta
                    .foreign_node_connection_method
                    .push(ForeignNodeConnectionMethod());
                node_edge_meta.static_assertions.push(static_assertions());
                node_edge_meta.imports.push(imports());
            }
            Entry::Vacant(v) => {
                v.insert(node_edge_meta);
            }
        };
        Ok(())
    }

    pub(crate) fn generate_static_assertions(&self) -> TokenStream {
        let static_assertions = self
            .0
            .values()
            .map(|value| {
                let static_assertions = &value.static_assertions;

                quote!(
                    #( #static_assertions) *
                )
            })
            .collect::<Vec<_>>();

        quote!(#( #static_assertions) *)
    }

    pub(crate) fn generate_token_stream(&self) -> TokenStream {
        let node_edge_token_streams = self.0.values().map(|value| {
            let NodeEdgeMetadata {
                    origin_struct_ident,
                    direction,
                    EdgeRelationModelSelectedIdent: edge_relation_model_selected_ident,
                    destination_node_schema,
                    ForeignNodeConnectionMethod: foreign_node_connection_method,
                    imports,
                    edge_name_as_method_ident,
                    edge_table_name,
                    ..
            }: &NodeEdgeMetadata = value;

            let crate_name = get_crate_name(false);
            let arrow = format!("{}", direction);
            let edge_table_name_str = format!("{}", &edge_table_name);
            let  edge_name_as_struct_original_ident = format_ident!("{}", &edge_table_name.to_string().to_case(Case::Pascal));
            let  edge_name_as_struct_with_direction_ident = format_ident!("{}",
                                                                          self.add_direction_indication_to_ident(
                                                                                  &edge_table_name
                                                                                  .to_string()
                                                                                  .to_case(Case::Pascal),
                                                                              direction,
                                                                              )
                                                                          );
            let edge_inner_module_name = format_ident!("{}_schema________________", edge_name_as_struct_with_direction_ident.to_string().to_lowercase());

            let VariablesModelMacro {
                __________connect_edge_to_graph_traversal_string,
                ___________graph_traversal_string,
                ..
            } = VariablesModelMacro::new();


             quote!(
                #( #imports) *

                // Edge to Node
                impl #origin_struct_ident {
                    pub fn #edge_name_as_method_ident(
                        &self,
                        clause: impl Into<#crate_name::EdgeClause>,
                    ) -> #edge_inner_module_name::#edge_name_as_struct_with_direction_ident {
                        let clause: #crate_name::EdgeClause = clause.into();
                        let clause = clause.with_arrow(#arrow).with_table(#edge_table_name_str);

                        // i.e Edge to Node
                        #edge_inner_module_name::#edge_name_as_struct_original_ident::#__________connect_edge_to_graph_traversal_string(
                            self,
                            clause,
                        ).into()
                    }
                }

                mod #edge_inner_module_name {
                    #( #imports) *
                    use #crate_name::Parametric as _;
                    use #crate_name::Buildable as _;
                    use #crate_name::Erroneous as _;

                    #( #destination_node_schema) *

                    pub type #edge_name_as_struct_original_ident = <super::super::#edge_relation_model_selected_ident as #crate_name::SchemaGetter>::Schema;

                    pub struct #edge_name_as_struct_with_direction_ident(#edge_name_as_struct_original_ident);


                    impl ::std::convert::From<#edge_name_as_struct_original_ident> for #edge_name_as_struct_with_direction_ident {
                        fn from(value: #edge_name_as_struct_original_ident) -> Self {
                            Self(value)
                        }
                    }

                    impl #crate_name::Buildable for #edge_name_as_struct_with_direction_ident {
                        fn build(&self) -> ::std::string::String {
                            self.0.build()
                        }
                    }

                    impl #crate_name::Parametric for #edge_name_as_struct_with_direction_ident {
                        fn get_bindings(&self) -> #crate_name::BindingsList {
                            self.0.get_bindings()
                        }
                    }

                    impl #crate_name::Erroneous for #edge_name_as_struct_with_direction_ident {
                        fn get_errors(&self) -> Vec<::std::string::String> {
                            self.0.get_errors()
                        }
                    }

                    impl #crate_name::Buildable for &#edge_name_as_struct_with_direction_ident {
                        fn build(&self) -> ::std::string::String {
                            self.0.build()
                        }
                    }

                    impl #crate_name::Parametric for &#edge_name_as_struct_with_direction_ident {
                        fn get_bindings(&self) -> #crate_name::BindingsList {
                            self.0.get_bindings()
                        }
                    }

                    impl #crate_name::Erroneous for &#edge_name_as_struct_with_direction_ident {
                        fn get_errors(&self) -> Vec<::std::string::String> {
                            self.0.get_errors()
                        }
                    }

                    impl ::std::ops::Deref for #edge_name_as_struct_with_direction_ident {
                        type Target = #edge_name_as_struct_original_ident;

                        fn deref(&self) -> &Self::Target {
                            &self.0
                        }
                    }

                    impl #edge_name_as_struct_with_direction_ident {
                        #( #foreign_node_connection_method) *

                         // This is for recurive edge traversal which is supported by surrealdb: e.g ->knows(..)->knows(..)->knows(..)
                        // -- Select all 1st, 2nd, and 3rd level people who this specific person record knows, or likes, as separate outputs
                        // SELECT ->knows->(? AS f1)->knows->(? AS f2)->(knows, likes AS e3 WHERE influencer = true)->(? AS f3) FROM person:tobie;
                        pub fn #edge_name_as_method_ident(
                            &self,
                            clause: impl Into<#crate_name::EdgeClause>,
                        ) -> #edge_name_as_struct_with_direction_ident {
                            let clause: #crate_name::EdgeClause = clause.into();
                            let clause = clause.with_arrow(#arrow).with_table(#edge_table_name_str);

                            // i.e Edge to Edge
                            #edge_name_as_struct_original_ident::#__________connect_edge_to_graph_traversal_string(
                                self,
                                clause,
                            ).into()
                        }
                    }
                }

            )
        }).collect::<Vec<_>>();

        quote!(#( #node_edge_token_streams) *)
    }
}
