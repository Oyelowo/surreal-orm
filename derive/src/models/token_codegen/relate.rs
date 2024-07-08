/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    ops::{Deref, DerefMut},
};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};
use surreal_query_builder::EdgeDirection;

use crate::models::*;

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn create_relation_connection_tokenstream(&mut self) -> ExtractorResult<()> {
        let crate_name = get_crate_name(false);
        let table_derive_attributes = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let db_field_name = field_receiver.db_field_name(&table_derive_attributes.casing()?)?;

        if let RelationType::Relate(ref relate) =
            field_receiver.to_relation_type(table_derive_attributes)
        {
            self.static_assertions
                .push(self.create_static_assertions(relate)?);
            self.relate(relate)?;
            let connection = &relate.connection;
            self.fields_relations_aliased.push(quote!(#crate_name::Field::new(#connection).__as__(#crate_name::AliasName::new(#db_field_name))).into());
        }
        Ok(())
    }

    fn relate(&mut self, relate: &Relate) -> ExtractorResult<()> {
        let crate_name = get_crate_name(false);
        let table_derive_attributes = &self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let VariablesModelMacro {
            __________connect_node_to_graph_traversal_string,
            __________connect_edge_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();
        let field_type = &field_receiver.ty();
        let edge_type = &relate.edge_type;
        let RelateAttribute {
            edge_direction,
            edge_table,
            foreign_node_table,
        } = &RelateAttribute::try_from(relate)?;
        let arrow = &ArrowTokenStream::from(edge_direction);
        let destination_node_table_str = &foreign_node_table.to_string();
        let binding = field_type.get_generics_from_current_struct(table_derive_attributes)?;
        let (_edge_impl_generics, edge_ty_generics, _edge_where_clause) = binding.split_for_impl();

        let edge_name_with_direction_as_method_ident =
            &(|| Self::add_direction_indication_to_ident(edge_table, edge_direction));

        let foreign_node_schema_ident = ForeignNodeSchemaIdent::from_node_table(foreign_node_table);
        let foreign_node_schema_type_alias_with_generics =
            ForeignNodeTypeAliasWithGenerics(quote!(#foreign_node_schema_ident #edge_ty_generics));

        let foreign_node_schema_type_alias_with_generics_turbofishized =
            if edge_ty_generics.is_empty() {
                quote!(#foreign_node_schema_ident)
            } else {
                // NOTE: This may  not really be needed as the return type
                // should help to auto-infer the type or generics as we call the
                // connect method on the struct.
                quote!(#foreign_node_schema_ident :: #edge_ty_generics)
            };
        let foreign_node_schema_type_alias_with_generics_turbofishized =
            ForeignNodeTypeAliasWithGenerics(
                foreign_node_schema_type_alias_with_generics_turbofishized,
            );

        // Meant to represent the variable of struct model(node) itself.
        // Within edge generics, there is usually In and Out associated types, this is used to
        // access those
        let foreign_node_in_or_out = ForeignNodeAssociatedTypeInOrOut(match edge_direction {
            EdgeDirection::Out => format_ident!("Out"),
            EdgeDirection::In => format_ident!("In"),
        });
        // We use super twice because we're trying to access the relation model struct name from
        // the outer outer module because all edge related functionalities are nested
        let foreign_node_schema_one = || {
            ForeignNodeSchema(quote!(
                type #foreign_node_schema_type_alias_with_generics =
                            <<super::super::#edge_type as #crate_name::Edge>::#foreign_node_in_or_out
                        as #crate_name::SchemaGetter>::Schema;
            ))
        };

        // i.e Edge to foreign Node
        let edge_to_foreign_node_connection_method = || {
            EdgeToForeignNodeMethod(quote!(
                pub fn #foreign_node_table(self, clause: impl ::std::convert::Into<#crate_name::NodeClause>) -> #foreign_node_schema_type_alias_with_generics {
                    let clause: #crate_name::NodeClause = clause.into();
                    let clause = clause.with_arrow(#arrow).with_table(#destination_node_table_str);

                    #foreign_node_schema_type_alias_with_generics_turbofishized::#__________connect_node_to_graph_traversal_string(
                                self,
                                clause,
                    )
                }
            ))
        };

        // let imports =|| quote!(use super::StudentWritesBook;);
        let edge_type_name = edge_type.type_name()?;
        let import = || EdgeImport(quote!(use super::#edge_type_name;));

        let node_edge_meta = NodeEdgeMetadata {
            edge_table: edge_table.to_owned(),
            // We want to take all edge concrete types even for each edge type(with direction)
            // because we want to allow the use of generics in edge.
            edge_types: vec![edge_type.to_owned()],
            direction: *edge_direction,
            foreign_node_schema: vec![foreign_node_schema_one()],
            edge_to_foreign_node_connection_method: vec![edge_to_foreign_node_connection_method()],
            table_derive_attributes,
            edge_name_as_method_ident: format_ident!(
                "{}",
                edge_name_with_direction_as_method_ident().to_string()
            )
            .into(),
            imports: vec![import()],
            edge_relation_model_selected_ident: edge_type.type_name()?.into(),
            foreign_node_table: foreign_node_table.clone().into(),
        };

        match self
            .node_edge_metadata
            .deref_mut()
            .entry(edge_name_with_direction_as_method_ident())
        {
            Entry::Occupied(o) => {
                let node_edge_meta = o.into_mut();
                // Get all the Edge types(with direction) aliased types, so
                // we can find a common denomator types between them all.
                node_edge_meta.edge_types.push(edge_type.to_owned());
                node_edge_meta
                    .foreign_node_schema
                    .push(foreign_node_schema_one());
                node_edge_meta
                    .edge_to_foreign_node_connection_method
                    .push(edge_to_foreign_node_connection_method());
                node_edge_meta.imports.push(import());
            }
            Entry::Vacant(v) => {
                v.insert(node_edge_meta);
            }
        };
        Ok(())
    }

    /// add direction indication to the edge name. This is non-functional
    /// semantics, it's just a way to differentiate same edge but with different
    /// direction
    /// e.g for ->writes->book, gives writes__. <-writes<-book, gives __writes
    pub fn add_direction_indication_to_ident(
        edge_table: &EdgeTableName,
        edge_direction: &EdgeDirection,
    ) -> EdgeWithDunderDirectionIndicator {
        let edge_table = edge_table.to_string();
        let edge = match edge_direction {
            EdgeDirection::Out => format_ident!("{edge_table}__"),
            EdgeDirection::In => format_ident!("__{edge_table}"),
        };
        edge.into()
    }

    fn create_static_assertions(&self, relate: &Relate) -> ExtractorResult<StaticAssertionToken> {
        let crate_name = get_crate_name(false);
        let model_attributes = &self.table_derive_attributes();
        let current_struct_ident = &model_attributes.ident();
        let current_struct_generics = model_attributes.generics();
        let (_struct_impl_generics, struct_ty_generics, _struct_where_clause) =
            current_struct_generics.split_for_impl();
        let field_receiver = self.field_receiver();
        let field_type = &field_receiver.ty();
        let edge_type = &relate.edge_type;
        let RelateAttribute {
            edge_table,
            foreign_node_table: destination_node_table,
            edge_direction,
        } = RelateAttribute::try_from(relate)?;
        let (home_node_associated_type_ident, foreign_node_associated_type_ident) =
            match &edge_direction {
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
                     // #crate_name::validators::assert_fields!(<#home_node_type as #crate_name::Node>::TableNameChecker: #origin_node_table);
                     #crate_name::validators::assert_type_eq_all!(#home_node_type, #current_struct_ident #struct_ty_generics);
                     #crate_name::validators::assert_impl_one!(<#edge_type as #crate_name::Edge>::#home_node_associated_type_ident: #crate_name::Node);
                }
            ),
            quote!(
                 #crate_name::validators::assert_fields!(<#foreign_node_type as #crate_name::Node>::TableNameChecker: #destination_node_table);
                 #crate_name::validators::assert_impl_one!(#foreign_node_type: #crate_name::Node);
            ),
            quote!(
                 #crate_name::validators::assert_fields!(<#edge_type as #crate_name::Edge>::TableNameChecker: #edge_table);
            ),
            // assert field type and attribute reference match
            // e.g Relate<Book> should match from attribute link = "->Writes->Book"
            quote!(
                 #crate_name::validators::assert_impl_one!(#edge_type: #crate_name::Edge);
                 #crate_name::validators::assert_type_eq_all!(#field_type,  #crate_name::Relate<#foreign_node_type>);
            ),
        ];
        Ok(StaticAssertionToken(quote!(
                #( #static_assertions) *
        )))
    }
}

impl EdgeTableName {
    pub fn to_str_typed(&self) -> EdgeTableNameStr {
        EdgeTableNameStr(self.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct EdgeTableNameStr(String);

impl EdgeTableNameStr {
    pub fn to_pascal_case_ident(&self) -> EdgeNameAsStructOriginalIdent {
        EdgeNameAsStructOriginalIdent(format_ident!("{}", self.as_pascal_case().to_string()))
    }

    fn as_pascal_case(&self) -> EdgeTableNameStr {
        EdgeTableNameStr(self.to_string().to_case(Case::Pascal))
    }
}

impl Display for EdgeTableNameStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToTokens for EdgeTableNameStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = &self.0;
        tokens.extend(quote!(#value));
    }
}

create_ident_wrapper!(EdgeNameAsMethodIdent);
create_ident_wrapper!(EdgeRelationModelSelectedIdent);

create_tokenstream_wrapper!(=>ForeignNodeSchema);
create_tokenstream_wrapper!(=>EdgeImport);
create_tokenstream_wrapper!(=>ForeignNodeTypeAliasWithGenerics);
create_tokenstream_wrapper!(=>EdgeToForeignNodeMethod);
create_tokenstream_wrapper!(=>DestinationNodeSchemaOne);
create_ident_wrapper!(ForeignNodeAssociatedTypeInOrOut);
create_ident_wrapper!(EdgeNameAsStructOriginalIdent);
create_ident_wrapper!(EdgeNameAsStructWithDirectionIdent);
create_ident_wrapper!(EdgeInnerModuleName);
create_ident_wrapper!(ForeignNodeSchemaIdent);

impl ForeignNodeSchemaIdent {
    /// represents the schema but aliased as the pascal case of the destination table name
    pub fn from_node_table(node_table: &NodeTableName) -> Self {
        let node_table = node_table.to_string();
        let node_table = format_ident!("{}", node_table.to_case(Case::Pascal));
        Self(node_table)
    }
}

#[derive(Clone, Debug)]
struct DestinationNodeTableName(String);

impl From<NodeTableName> for DestinationNodeTableName {
    fn from(name: NodeTableName) -> Self {
        Self(name.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct NodeEdgeMetadata<'a> {
    /// Example value: writes
    edge_table: EdgeTableName,
    // edge_type: EdgeType,
    pub edge_types: Vec<EdgeType>,
    // /// The current struct name ident.
    // /// e.g given: struct Student {  }, value = Student
    // current_struct_ident: StructIdent,
    table_derive_attributes: &'a ModelAttributes<'a>,
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
    foreign_node_schema: Vec<ForeignNodeSchema>,
    #[allow(dead_code)]
    foreign_node_table: DestinationNodeTableName,
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
    edge_to_foreign_node_connection_method: Vec<EdgeToForeignNodeMethod>,
    // static_assertions: Vec<StaticAssertionToken>,
    imports: Vec<EdgeImport>,
    edge_name_as_method_ident: EdgeNameAsMethodIdent,
}

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// struct EdgeNameWithDirectionIndicator(String);
create_ident_wrapper!(EdgeWithDunderDirectionIndicator);

#[derive(Default, Clone)]
pub struct NodeEdgeMetadataLookupTable<'a>(
    HashMap<EdgeWithDunderDirectionIndicator, NodeEdgeMetadata<'a>>,
);
impl<'a> Deref for NodeEdgeMetadataLookupTable<'a> {
    type Target = HashMap<EdgeWithDunderDirectionIndicator, NodeEdgeMetadata<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for NodeEdgeMetadataLookupTable<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> NodeEdgeMetadataLookupTable<'a> {}

create_tokenstream_wrapper!(=>ArrowTokenStream);

impl From<&EdgeDirection> for ArrowTokenStream {
    fn from(value: &EdgeDirection) -> Self {
        let crate_name = get_crate_name(false);

        let arrow = match value {
            EdgeDirection::Out => quote!(#crate_name::Arrow::Right),
            EdgeDirection::In => quote!(#crate_name::Arrow::Left),
        };
        Self(arrow)
    }
}

impl<'a> ToTokens for NodeEdgeMetadata<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_name = get_crate_name(false);
        let VariablesModelMacro {
            __________connect_edge_to_graph_traversal_string,
            ___________bindings,
            ___________errors,
            ___________graph_traversal_string,
            ____________update_many_bindings,
            ___________model,
            ___________in_marker,
            ___________out_marker,
            _____field_names,
            ..
        } = VariablesModelMacro::new();
        let value = self;
        let Self {
            // current_struct_ident,
            direction,
            edge_relation_model_selected_ident,
            foreign_node_schema,
            edge_to_foreign_node_connection_method,
            imports,
            edge_name_as_method_ident,
            edge_table,
            edge_types,
            table_derive_attributes,
            ..
        }: &NodeEdgeMetadata = value;
        let current_struct_ident = value.table_derive_attributes.ident();
        let (struct_impl_generics, struct_ty_generics, struct_where_clause) =
            value.table_derive_attributes.generics().split_for_impl();
        let arrow = ArrowTokenStream::from(direction);
        let edge_table_str = edge_table.to_str_typed();
        let edge_name_as_struct_original_ident = edge_table_str.to_pascal_case_ident();
        let aggregated_edge_type =
            EdgeType::aggregate_lifetime_and_generics_from_variations_of_a_type(
                edge_name_as_struct_original_ident.deref(),
                edge_types,
            );

        let binding =
            match aggregated_edge_type.get_generics_from_current_struct(table_derive_attributes) {
                Ok(binding) => binding,
                Err(err) => return tokens.extend(err.write_errors()),
            };

        let (edge_type_impl_generics, edge_type_ty_generics, edge_type_where_clause) =
            binding.split_for_impl();

        let edge_name_as_struct_with_direction_ident =
            EdgeNameAsStructWithDirectionIdent(format_ident!(
                "{}",
                Codegen::add_direction_indication_to_ident(
                    // edge_name_as_struct_original_ident.deref(),
                    edge_table, direction,
                )
                .to_string()
            ));
        // TODO: Remove if no good purpose served
        // let edge_type_with_direction = CustomType::from(syn::parse2::<Type>(
        //     quote!(#edge_name_as_struct_with_direction_ident #edge_type_ty_generics),
        // ));
        // .expect("Failed to parse edge type");
        let edge_inner_module_name = EdgeInnerModuleName(format_ident!(
            "{}_schema________________",
            edge_name_as_struct_with_direction_ident
                .to_string()
                .to_lowercase()
        ));

        tokens.extend(quote!(
            #( #imports) *

            // Edge to Node
            impl #struct_impl_generics #current_struct_ident #struct_ty_generics #struct_where_clause {
                pub fn #edge_name_as_method_ident(
                    &self,
                    clause: impl ::std::convert::Into<#crate_name::EdgeClause>,
                ) -> #edge_inner_module_name::#edge_name_as_struct_with_direction_ident {
                    let clause: #crate_name::EdgeClause = clause.into();
                    let clause = clause.with_arrow(#arrow).with_table(#edge_table_str);

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

                #( #foreign_node_schema) *

         // Add generics here and use in the ___connect edge method above
                pub type #edge_name_as_struct_original_ident #edge_type_ty_generics = <super::super::#edge_relation_model_selected_ident #edge_type_ty_generics as #crate_name::SchemaGetter>::Schema;

                pub struct #edge_name_as_struct_with_direction_ident #edge_type_ty_generics(#edge_name_as_struct_original_ident #edge_type_ty_generics);


                impl #edge_type_impl_generics ::std::convert::From<#edge_name_as_struct_original_ident #edge_type_ty_generics> for #edge_name_as_struct_with_direction_ident #edge_type_ty_generics #edge_type_where_clause {
                    fn from(value: #edge_name_as_struct_original_ident #edge_type_ty_generics) -> Self {
                        Self(value)
                    }
                }

                impl #edge_type_impl_generics #crate_name::Buildable for #edge_name_as_struct_with_direction_ident #edge_type_ty_generics #edge_type_where_clause {
                    fn build(&self) -> ::std::string::String {
                        self.0.build()
                    }
                }

                impl #edge_type_impl_generics #crate_name::Parametric for #edge_name_as_struct_with_direction_ident #edge_type_ty_generics #edge_type_where_clause {
                    fn get_bindings(&self) -> #crate_name::BindingsList {
                        self.0.get_bindings()
                    }
                }

                impl #edge_type_impl_generics #crate_name::Erroneous for #edge_name_as_struct_with_direction_ident #edge_type_ty_generics #edge_type_where_clause {
                    fn get_errors(&self) -> Vec<::std::string::String> {
                        self.0.get_errors()
                    }
                }

                impl #edge_type_impl_generics #crate_name::Buildable for &#edge_name_as_struct_with_direction_ident #edge_type_ty_generics #edge_type_where_clause {
                    fn build(&self) -> ::std::string::String {
                        self.0.build()
                    }
                }

                impl #edge_type_impl_generics #crate_name::Parametric for &#edge_name_as_struct_with_direction_ident  #edge_type_ty_generics #edge_type_where_clause {
                    fn get_bindings(&self) -> #crate_name::BindingsList {
                        self.0.get_bindings()
                    }
                }

                impl #edge_type_impl_generics #crate_name::Erroneous for &#edge_name_as_struct_with_direction_ident #edge_type_ty_generics #edge_type_where_clause{
                    fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
                        self.0.get_errors()
                    }
                }

                impl #edge_type_impl_generics ::std::ops::Deref for #edge_name_as_struct_with_direction_ident #edge_type_ty_generics #edge_type_where_clause {
                    type Target = #edge_name_as_struct_original_ident #edge_type_ty_generics;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                impl #edge_type_impl_generics #edge_name_as_struct_with_direction_ident #edge_type_ty_generics #edge_type_where_clause {
                    #( #edge_to_foreign_node_connection_method) *

                     // This is for recurive edge traversal which is supported by surrealdb: e.g ->knows(..)->knows(..)->knows(..)
                    // -- Select all 1st, 2nd, and 3rd level people who this specific person record knows, or likes, as separate outputs
                    // SELECT ->knows->(? AS f1)->knows->(? AS f2)->(knows, likes AS e3 WHERE influencer = true)->(? AS f3) FROM person:tobie;
                    pub fn #edge_name_as_method_ident(
                        &self,
                        clause: impl ::std::convert::Into<#crate_name::EdgeClause>,
                    ) -> #edge_name_as_struct_with_direction_ident #edge_type_ty_generics {
                        let clause: #crate_name::EdgeClause = clause.into();
                        let clause = clause.with_arrow(#arrow).with_table(#edge_table_str);

                        // i.e Edge to Edge
                        #edge_name_as_struct_original_ident::#__________connect_edge_to_graph_traversal_string(
                            self,
                            clause,
                        ).into()
                    }
                }
            }

        ))
    }
}

impl<'a> ToTokens for NodeEdgeMetadataLookupTable<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let node_edge_token_streams = self
            .deref()
            .values()
            .map(|value| value.to_token_stream())
            .collect::<Vec<_>>();

        let token = quote!(#( #node_edge_token_streams) *);

        tokens.extend(token);
    }
}
