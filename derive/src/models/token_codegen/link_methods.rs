use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{variables::VariablesModelMacro, *},
};
use table_meta::derive_attributes::TableDeriveAttributes;

create_tokenstream_wrapper!(
/// Genera
=>
ListSimpleTraversalMethod
);

use super::{Codegen, RelationType};

impl Codegen {
    pub fn create_link_methods(
        &self,
        store: &mut Codegen,
        table_derive_attrs: &TableDeriveAttributes,
    ) {
        let relation_type = self.to_relation_type();
        let push_to_link = |link_method_meta: LinkMethodMeta| {
            store
                .imports_referenced_node_schema
                .push(meta.foreign_node_schema_import);

            store
                .record_link_fields_methods
                .push(meta.link_field_method);

            store
                .static_assertions
                .push(meta.foreign_node_type_validator.to_static_assertion());
        };

        match self {
            RelationType::LinkSelf(link_self) => {
                let link_one = link_self.to_linkone_attr_type(table_derive_attrs);
                let meta = self.link_one(link_one, table_derive_attrs)?;
                push_to_link(meta);
            }
            RelationType::LinkOne(link_one) => {
                let meta = self.link_one(link_one, table_derive_attrs)?;
                push_to_link(meta);
            }
            RelationType::LinkMany(link_many) => {
                let meta = self.link_many(link_many, table_derive_attrs)?;
                push_to_link(meta);
            }
            RelationType::NestArray(nest_array) => {
                let meta = self.nest_array(nest_array, table_derive_attrs)?;
                push_to_link(meta);
            }
            RelationType::NestObject(nest_object) => {
                let meta = self.nest_object(nest_object, table_derive_attrs)?;
                push_to_link(meta);
            }
            RelationType::List(list) => {
                let method_token = self.list_simple(list, table_derive_attrs)?;
                store.record_link_fields_methods.push(method_token);
            }
            RelationType::Relate(relate) => {}
            RelationType::None => {}
        }
    }

    pub fn list_simple(
        &self,
        list_simple: &ListSimple,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<LinkFieldTraversalMethodToken> {
        let crate_name = get_crate_name(false);
        let struct_casing = table_derive_attrs.casing()?;
        let field_ident_normalized = self.field_ident_normalized(&struct_casing);
        let field_name_serialized = self.db_field_name(&struct_casing);

        let record_link_default_alias_as_method = quote!(
            pub fn #field_ident_normalized(&self, clause: impl Into<#crate_name::NodeAliasClause>) -> #crate_name::Field {
                let clause: #crate_name::NodeAliasClause = clause.into();
                let clause: #crate_name::NodeClause = clause.into_inner();

                let normalized_field_name_str = if self.build().is_empty(){
                    #field_name_serialized.to_string()
                }else {
                    format!(".{}", #field_name_serialized)
                };

                let clause: #crate_name::NodeClause = clause.into();
                let bindings = self.get_bindings().into_iter().chain(clause.get_bindings().into_iter()).collect::<Vec<_>>();

                let errors = self.get_errors().into_iter().chain(clause.get_errors().into_iter()).collect::<Vec<_>>();

                let field = #crate_name::Field::new(format!("{field_name_serialized}{}", clause.build()))
                            .with_bindings(bindings)
                            .with_errors(errors);
                field

            }
        );

        Ok(record_link_default_alias_as_method.into())
    }

    fn link_one(
        &self,
        link_one: LinkOneAttrType,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<LinkMethodMeta> {
        let crate_name = get_crate_name(false);
        // TODO: Cross-check if not replacing self here is more ergonomic/correct
        // let link_one = &link_one.replace_self_with_current_struct_ident(table_def);
        let struct_name_ident = table_derive_attrs.ident;
        let struct_casing = table_derive_attrs.casing()?;
        let field_ident_normalized = self.field_ident_normalized(&struct_casing);
        let field_name_serialized = self.db_field_name(&struct_casing);
        let VariablesModelMacro {
            ___________graph_traversal_string,
            __________connect_node_to_graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let foreign_node_schema_import = if *struct_name_ident.is_same_name(link_one)? {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #link_one = <super::#link_one as #crate_name::SchemaGetter>::Schema;)
        };

        let record_link_default_alias_as_method = quote!(
            pub fn #normalized_field_name(&self) -> #link_one {
            let clause = #crate_name::Clause::from(#crate_name::Empty);

            let normalized_field_name_str = if self.build().is_empty(){
                #normalized_field_name_str.to_string()
            }else {
                format!(".{}", #normalized_field_name_str)
            };

            #link_one::#__________connect_node_to_graph_traversal_string(
                self,
                clause.with_field(normalized_field_name_str)
                )
            }
        );

        Ok(LinkMethodMeta {
            foreign_node_schema_import: foreign_node_schema_import.into(),
            foreign_node_type_validator: quote!(
                #crate_name::validators::assert_impl_one!(#link_one: #crate_name::Node);
            )
            .into(),
            link_field_method: record_link_default_alias_as_method.into(),
        })
    }

    fn link_many(
        &self,
        link_many_node_type: &LinkManyAttrType,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<LinkMethodMeta> {
        let crate_name = get_crate_name(false);
        let current_struct_ident = &table_derive_attrs.ident;
        let struct_casing = table_derive_attrs.casing()?;
        let field_ident_normalized = self.field_ident_normalized(&struct_casing)?;
        let field_name_serialized = self.db_field_name(&struct_casing)?;
        let VariablesModelMacro {
            ___________graph_traversal_string,
            __________connect_node_to_graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let foreign_node_schema_import = if *current_struct_ident
            .is_same_name(link_many_node_type)?
        {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #link_many_node_type = <super::#link_many_node_type as #crate_name::SchemaGetter>::Schema;)
        };

        let link_field_method = quote!(
            pub fn #field_ident_normalized(&self, clause: impl ::std::convert::Into<#crate_name::NodeAliasClause>) -> #link_many_node_type {
            let clause: #crate_name::NodeAliasClause = clause.into();
            let clause: #crate_name::NodeClause = clause.into_inner();

            let normalized_field_name_str = if self.build().is_empty(){
                #normalized_field_name_str.to_string()
            }else {
                format!(".{}", #normalized_field_name_str)
            };

            #link_many_node_type::#__________connect_node_to_graph_traversal_string(
                    self,
                    clause.with_field(normalized_field_name_str)
                )
            }
        );

        Ok(LinkMethodMeta {
            foreign_node_schema_import: foreign_node_schema_import.into(),
            link_field_method: link_field_method.into(),
            foreign_node_type_validator: quote!(
                #crate_name::validators::assert_impl_one!(#link_many_node_type: #crate_name::Node);
            )
            .into(),
        })
    }

    pub(crate) fn nest_object(
        &self,
        embedded_object: &NestObjectAttrType,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<LinkMethodMeta> {
        let crate_name = get_crate_name(false);
        let current_struct_ident = &table_derive_attrs.ident;
        let struct_casing = table_derive_attrs.casing()?;
        let field_ident_normalized = self.field_ident_normalized(&struct_casing)?;
        let field_name_serialized = self.db_field_name(&struct_casing)?;
        let node_type_alias_with_trait_bounds = embedded_object;
        let VariablesModelMacro {
            __________connect_object_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let foreign_node_schema_import = if *current_struct_ident.is_same_name(embedded_object)? {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            // e.g type Book = <super::Book as SchemaGetter>::Schema;
            // type Book<'a, 'b, T, U: Clone + Default, V: Node> = <super::Book<'a, 'b, T, U, V> as SchemaGetter>::Schema;
            quote!(type #embedded_object = <super::#embedded_object as #crate_name::SchemaGetter>::Schema;)
        };

        let connect_object_field_method = quote!(
            pub fn #field_ident_normalized(&self) -> #embedded_object {
                let clause = #crate_name::Clause::from(#crate_name::Empty);

                let normalized_field_name_str = if self.build().is_empty(){
                    #field_name_serialized.to_string()
                }else {
                    format!(".{}", #field_name_serialized)
                };

                #embedded_object::#__________connect_object_to_graph_traversal_string(
                    self,
                    clause.with_field(normalized_field_name_str)
                )

            }
        );

        Ok(LinkMethodMeta {
            foreign_node_schema_import: foreign_node_schema_import.into(),
            foreign_node_type_validator: quote!(
                #crate_name::validators::assert_impl_one!(#schema_type_ident: #crate_name::Object);
            )
            .into(),
            link_field_method: connect_object_field_method.into(),
        })
    }

    pub(crate) fn nest_array(
        &self,
        nested_array: &NestArrayAttrType,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<LinkMethodMeta> {
        let crate_name = get_crate_name(false);
        let current_struct_ident = &table_derive_attrs.ident;
        let struct_casing = table_derive_attrs.casing()?;
        let field_ident_normalized = self.field_ident_normalized(&struct_casing)?;
        let field_name_serialized = self.db_field_name(&struct_casing)?;
        let VariablesModelMacro {
            __________connect_object_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let foreign_node_schema_import = if *current_struct_ident.is_same_name(nested_array)? {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            // e.g type Book = <super::Book as SchemaGetter>::Schema;
            // type Book<'a, 'b, T, U: Clone + Default, V: Node> = <super::Book<'a, 'b, T, U, V> as SchemaGetter>::Schema;
            quote!(type #nested_array = <super::#nested_array as #crate_name::SchemaGetter>::Schema;)
        };

        let record_link_default_alias_as_method = quote!(
            pub fn #field_ident_normalized(
                &self,
                clause: impl ::std::convert::Into<#crate_name::ObjectClause>
            ) -> #nested_array {
                let clause: #crate_name::ObjectClause = clause.into();
                let normalized_field_name_str = if self.build().is_empty(){
                    #field_name_serialized.to_string()
                }else {
                    format!(".{}", #field_name_serialized)
                };

                #nested_array::#__________connect_object_to_graph_traversal_string(
                    self,
                    clause.with_field(normalized_field_name_str)
                )

            }
        );

        Ok(LinkMethodMeta {
            foreign_node_schema_import: foreign_node_schema_import.into(),
            foreign_node_type_validator: quote!(
                #crate_name::validators::assert_impl_one!(#schema_type_ident: #crate_name::Object);
            )
            .into(),
            link_field_method: record_link_default_alias_as_method.into(),
        })
    }
}

create_tokenstream_wrapper!(
    /// imports for specific schema from the trait Generic Associated types e.g
    /// Example:
    ///
    /// ```rust, ignore
    /// Generated example:
    ///
    /// type Book = <super::Book as Node>::Schema;
    /// We need imports to be unique, hence the hashset
    /// Used when you use a Node in field e.g: favourite_book: LinkOne<Book>,
    /// e.g: type Book = <super::Book as Node>::Schema;
    /// e.g type Book = <super::Book as SchemaGetter>::Schema;
    /// type Book<'a, 'b, T, U: Clone + Default, V: Node> = <super::Book<'a, 'b, T, U, V> as SchemaGetter>::Schema;
    /// ```
    =>
    ForeignNodeSchemaImport
);
create_tokenstream_wrapper!(
/// Contains static assertions for foreign node type
=>
ForeignNodeTypeValidator
);

impl ForeignNodeTypeValidator {
    pub fn to_static_assertion(self) -> StaticAssertionToken {
        self.0.into()
    }
}

create_tokenstream_wrapper!(
/// TODO: Complete later if you decide to document the token generated on the type or on
/// the struct gatherer FieldsMeta fields
/// Contains the default alias method for the foreign node type
/// Example:
/// ```rust, ignore
///
=>
LinkFieldTraversalMethodToken
);

struct LinkMethodMeta {
    foreign_node_schema_import: ForeignNodeSchemaImport,
    foreign_node_type_validator: ForeignNodeTypeValidator,
    link_field_method: LinkFieldTraversalMethodToken,
}

impl LinkSelfAttrType {
    pub(crate) fn to_linkone_attr_type(
        self,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> LinkOneAttrType {
        LinkOneAttrType(
            self.0
                .replace_self_with_current_struct_ident(table_derive_attrs),
        )
    }
}
