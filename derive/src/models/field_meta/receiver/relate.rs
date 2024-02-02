use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    ops::Deref,
};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};
use surreal_query_builder::{Arrow, EdgeDirection, NodeAliasClause};

use crate::{
    errors::ExtractorResult,
    models::{
        create_ident_wrapper, create_tokenstream_wrapper,
        derive_attributes::{StructIdent, TableDeriveAttributes},
        variables::VariablesModelMacro,
        EdgeTableName, FieldsMeta, NodeTableName, RelateAttribute, StaticAssertionToken,
    },
};

use super::{
    variables::VariablesModelMacro, EdgeDirection, FieldGenericsMeta, MyFieldReceiver, Relate,
};

impl MyFieldReceiver {}

// struct EdgeTableName(syn::Ident);
create_ident_wrapper!(EdgeTableName);

create_ident_wrapper!(EdgeNameAsMethodIdent);
create_ident_wrapper!(EdgeRelationModelSelectedIdent);

create_tokenstream_wrapper!(=>DestinationNodeSchema);
create_tokenstream_wrapper!(=>EdgeImport);
create_tokenstream_wrapper!(=>DestinationNodeTypeAlias);
create_tokenstream_wrapper!(=>EdgeToDestinationNodeMethod);
create_tokenstream_wrapper!(=>DestinationNodeSchemaOne);
create_ident_wrapper!(ForeignNodeAssociatedTypeInOrOut);
create_ident_wrapper!(EdgeNameAsStructOriginalIdent);
create_ident_wrapper!(EdgeNameAsStructWithDirectionIdent);
create_ident_wrapper!(EdgeInnerModuleName);

struct DestinationNodeName(String);

impl From<NodeTableName> for DestinationNodeName {
    fn from(name: NodeTableName) -> Self {
        Self(name.to_string())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct NodeEdgeMetadata<'a> {
    /// Example value: writes
    edge_table_name: EdgeTableName,
    // /// The current struct name ident.
    // /// e.g given: struct Student {  }, value = Student
    // current_struct_ident: StructIdent,
    table_derive_attributes: &'a TableDeriveAttributes,
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
    destination_node_name: DestinationNodeName,
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
    edge_to_destination_node_connection_method: Vec<EdgeToDestinationNodeMethod>,
    // static_assertions: Vec<StaticAssertionToken>,
    imports: Vec<EdgeImport>,
    edge_name_as_method_ident: EdgeNameAsMethodIdent,
}

type EdgeNameWithDirectionIndicator = String;
#[derive(Default, Clone)]
pub struct NodeEdgeMetadataStore(HashMap<EdgeNameWithDirectionIndicator, NodeEdgeMetadata>);
impl Deref for NodeEdgeMetadataStore {
    type Target = HashMap<EdgeNameAsStructWithDirectionIdent, NodeEdgeMetadata>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl NodeEdgeMetadataStore {
    pub fn into_inner(self) -> HashMap<EdgeNameWithDirectionIndicator, NodeEdgeMetadata> {
        self.0
    }
}
create_ident_wrapper!(EdgeWithDunderDirectionIndicator);

create_tokenstream_wrapper!(=>ArrowTokenStream);

impl From<EdgeDirection> for ArrowTokenStream {
    fn from(value: EdgeDirection) -> Self {
        let crate_name = get_crate_name(false);

        let arrow = match value {
            EdgeDirection::Outgoing => quote!(#crate_name::Arrow::Right),
            EdgeDirection::Incoming => quote!(#crate_name::Arrow::Left),
        };
        Self(arrow)
    }
}

impl MyFieldReceiver {
    fn create_relation_connection_tokenstream(
        &self,
        store: &mut FieldsMeta,
        relation: &Relate,
        table_derive_attributes: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let crate_name = get_crate_name(false);
        let current_struct_ident = table_derive_attributes.ident;
        let field_type = &self.ty;
        let edge_type = relation.edge_type;
        let RelateAttribute {
            edge_direction,
            edge_table_name,
            node_table_name: destination_node_table_name,
        } = &RelateAttribute::from(relation);
        let arrow = &ArrowTokenStream::from(edge_direction);
        let destination_node_table_name_str = &destination_node_table_name.to_string();
        let VariablesModelMacro {
            __________connect_node_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();
        let edge_name_as_method_ident =
            &(|| Self::add_direction_indication_to_ident(edge_table_name, edge_direction));
        let FieldGenericsMeta {
            field_impl_generics,
            field_ty_generics,
            field_where_clause,
        } = &edge_type.get_generics_meta(table_derive_attributes);
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
            EdgeDirection::Out => format_ident!("Out"),
            EdgeDirection::In => format_ident!("In"),
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
        let edge_to_destination_node_connection_method = || {
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

        store
            .static_assertions
            .push(self.create_static_assertions(relation, &table_derive_attributes.ident));

        // let imports =|| quote!(use super::StudentWritesBook;);
        let import = || EdgeImport(quote!(use super::#edge_type;));

        let node_edge_meta = NodeEdgeMetadata {
            edge_table_name: edge_table_name.to_owned(),
            direction: *edge_direction,
            destination_node_schema: vec![destination_node_schema_one()],
            edge_to_destination_node_connection_method: vec![
                edge_to_destination_node_connection_method(),
            ],
            // current_struct_ident: current_struct_ident.to_owned(),
            table_derive_attributes,
            // static_assertions: vec![static_assertions()],
            edge_name_as_method_ident: format_ident!("{}", edge_name_as_method_ident()),
            imports: vec![import()],
            edge_relation_model_selected_ident: relation.edge_type.type_name()?,
            destination_node_name: destination_node_table_name.into(),
        };

        match store
            .node_edge_metadata
            .into_inner()
            .entry(edge_name_as_method_ident())
        {
            Entry::Occupied(o) => {
                let node_edge_meta = o.into_mut();
                node_edge_meta
                    .destination_node_schema
                    .push(destination_node_schema_one());
                node_edge_meta
                    .edge_to_destination_node_connection_method
                    .push(edge_to_destination_node_connection_method());
                node_edge_meta.static_assertions.push(static_assertions());
                node_edge_meta.imports.push(import());
            }
            Entry::Vacant(v) => {
                v.insert(node_edge_meta);
            }
        };
        Ok(())
    }

    /// e.g for ->writes->book, gives writes__. <-writes<-book, gives __writes
    fn add_direction_indication_to_ident(
        edge_table_name: &EdgeTableName,
        edge_direction: &EdgeDirection,
    ) -> EdgeWithDunderDirectionIndicator {
        let edge_table_name = edge_table_name.to_string();
        let edge = match edge_direction {
            EdgeDirection::Out => format_ident!("{edge_table_name}__"),
            EdgeDirection::In => format_ident!("__{edge_table_name}"),
        };
        edge.into()
    }

    fn create_static_assertions(
        &self,
        relation: &Relate,
        current_struct: &StructIdent,
    ) -> StaticAssertionToken {
        let field_type = &self.ty;
        let crate_name = get_crate_name(false);
        let edge_type = relation.edge_type;
        let RelateAttribute {
            edge_table_name,
            node_table_name: destination_node_table_name,
            edge_direction,
        } = RelateAttribute::from(relation);
        let (home_node_associated_type_ident, foreign_node_associated_type_ident) =
            match &relation_attributes.edge_direction {
                EdgeDirection::Out => (format_ident!("In"), format_ident!("Out")),
                EdgeDirection::In => (format_ident!("Out"), format_ident!("In")),
            };

        // e.g for struct Student {
        //                   #[surreal_orm(relate(mode="StudentWritesBook", connection="->writes->book"))]
        //                   fav_books: Relate<Book>
        //              }
        let home_node_type =
            quote!(<#edge_type as #crate_name::Edge>::#home_node_associated_type_ident);

        let foreign_node_type =
            quote!(<#edge_type as #crate_name::Edge>::#foreign_node_associated_type_ident);

        let static_assertions = &[
            // type HomeIdent = <StudentWritesBook  as surreal_macros::Edge>::In;
            // type HomeNodeTableChecker = <HomeIdent as
            // surreal_macros::Node>::TableNameChecker;
            // #crate_name::validators::assert_type_eq_all!(HomeIdent, Student);
            // #crate_name::validators::assert_impl_one!(HomeIdent, surreal_macros::Node);
            quote!(
            {
            type #home_node_ident = <#edge_type as #crate_name::Edge>::#home_node_associated_type_ident;
             // #crate_name::validators::assert_fields!(<#home_node_type as #crate_name::Node>::TableNameChecker: #origin_node_table_name);
             #crate_name::validators::assert_type_eq_all!(#home_node_type, #current_struct);
             #crate_name::validators::assert_impl_one!(#home_node_ident: #crate_name::Node);

            }
            ),
            quote!(
             #crate_name::validators::assert_fields!(<#foreign_node_type as #crate_name::Node>::TableNameChecker: #destination_node_table_name);
             #crate_name::validators::assert_impl_one!(#foreign_node_type: #crate_name::Node);
            ),
            quote!(
             #crate_name::validators::assert_fields!(<#edge_type as #crate_name::Edge>::TableNameChecker: #edge_table_name);
            ),
            // assert field type and attribute reference match
            // e.g Relate<Book> should match from attribute link = "->Writes->Book"
            quote!(
             #crate_name::validators::assert_impl_one!(#edge_type: #crate_name::Edge);
             #crate_name::validators::assert_type_eq_all!(#field_type,  #crate_name::Relate<#foreign_node_type>);
            ),
        ];
        StaticAssertionToken(quote!(
                #( #static_assertions) *
        ))
    }
}

impl ToTokens for NodeEdgeMetadataStore {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let node_edge_token_streams = self.into_inner().values().map(|value| {
            let NodeEdgeMetadata {
                    // current_struct_ident,
                    direction,
                    edge_relation_model_selected_ident,
                    destination_node_schema,
                    edge_to_destination_node_connection_method: foreign_node_connection_method,
                    imports,
                    edge_name_as_method_ident,
                    edge_table_name,
                    ..
            }: &NodeEdgeMetadata = value;
            let current_struct_ident = value.table_derive_attributes.ident;
            let (struct_impl_generics, struct_ty_generics, struct_where_clause) = value.table_derive_attributes.generics.split_for_impl();

            let crate_name = get_crate_name(false);
            let arrow = ArrowTokenStream::from(direction);
            let edge_table_name_str = edge_table_name.to_string();
            let  edge_name_as_struct_original_ident = EdgeNameAsStructOriginalIdent(format_ident!("{}", &edge_table_name_str.to_case(Case::Pascal)));
            let  edge_name_as_struct_with_direction_ident = EdgeNameAsStructWithDirectionIdent(format_ident!("{}",
                                                                          MyFieldReceiver::add_direction_indication_to_ident(
                                                                                  &edge_name_as_struct_original_ident
                                                                                  .into(),
                                                                              direction,
                                                                              )
                                                                          ));
            let edge_inner_module_name = EdgeInnerModuleName(format_ident!("{}_schema________________", edge_name_as_struct_with_direction_ident.to_string().to_lowercase()));

            let VariablesModelMacro {
                __________connect_edge_to_graph_traversal_string,
                ___________graph_traversal_string,
                ..
            } = VariablesModelMacro::new();

             quote!(
                #( #imports) *

                // Edge to Node
                impl #struct_impl_generics #current_struct_ident #struct_ty_generics #struct_where_clause {
                    pub fn #edge_name_as_method_ident(
                        &self,
                        clause: impl ::std::convert::Into<#crate_name::EdgeClause>,
                    ) -> #edge_inner_module_name::#edge_name_as_struct_with_direction_ident {
                        let clause: #crate_name::EdgeClause = clause.into();
                        let clause = clause.with_arrow(#arrow).with_table(#edge_table_name_str);

                        // i.e Edge to Node
                         // TODO: Use type over mere ident. include potential generics
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
                            clause: impl ::std::convert::Into<#crate_name::EdgeClause>,
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

        token = quote!(#( #node_edge_token_streams) *);

        tokens.extend(self.generate_token_stream());
    }
}
