use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{
        create_tokenstream_wrapper, derive_attributes::TableDeriveAttributes,
        variables::VariablesModelMacro, FieldIdentSerialized, FieldsMeta, LinkManyAttrType,
        LinkOneAttrType, LinkSelfAttrType, ListSimple, MyFieldReceiver, NestArrayAttrType,
        NestObjectAttrType, NormalisedFieldMeta, StaticAssertionToken,
    },
};

use super::{field_receiver, RelationType};

create_tokenstream_wrapper!(
/// Genera
=>
ListSimpleTraversalMethod
);

impl MyFieldReceiver {
    pub fn create_link_methods(
        &self,
        store: &mut FieldsMeta,
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
                let meta = self.link_one(link_one, table_derive_attrs);
                push_to_link(meta);
            }
            RelationType::LinkOne(link_one) => {
                let meta = self.link_one(link_one, table_derive_attrs);
                push_to_link(meta);
            }
            RelationType::LinkMany(link_many) => {
                let meta = self.link_many(link_many, table_derive_attrs);
                push_to_link(meta);
            }
            RelationType::NestArray(nest_array) => {
                let method = nest_array.get_field_link_method();
                LinkerMethodToken(method)
            }
            RelationType::NestObject(nest_object) => {
                let meta = self.nest_object(nest_object, table_derive_attrs)?;
                push_to_link(meta);
            }
            RelationType::List(list) => {
                let method_token = self.list_simple(list, table_derive_attrs)?;
                store.record_link_fields_methods.push(method_token);
            }
            RelationType::Relate(relate) => {
                let method = relate.get_field_link_method();
                LinkerMethodToken(method)
            }
            RelationType::None => {
                let method = quote!();
                LinkerMethodToken(method)
            }
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
        let field_name_serialized = self.field_name_serialized(&struct_casing);

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
    ) -> LinkMethodMeta {
        let crate_name = get_crate_name(false);
        let linked_node_type = &link_one.replace_self_with_current_struct_ident(table_def);
        let crate_name = get_crate_name(false);
        let struct_name_ident = table_derive_attrs.ident;
        let struct_casing = table_derive_attrs.casing()?;
        let field_ident_normalized = self.field_ident_normalized(&struct_casing);
        let field_name_serialized = self.field_name_serialized(&struct_casing);
        let VariablesModelMacro {
            ___________graph_traversal_string,
            __________connect_node_to_graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let foreign_node_schema_import = if *struct_name_ident.is_same(linked_node_type) {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #linked_node_type = <super::#linked_node_type as #crate_name::SchemaGetter>::Schema;)
        };

        let record_link_default_alias_as_method = quote!(
            pub fn #normalized_field_name(&self) -> #linked_node_type {
            let clause = #crate_name::Clause::from(#crate_name::Empty);

            let normalized_field_name_str = if self.build().is_empty(){
                #normalized_field_name_str.to_string()
            }else {
                format!(".{}", #normalized_field_name_str)
            };

            #linked_node_type::#__________connect_node_to_graph_traversal_string(
                self,
                clause.with_field(normalized_field_name_str)
                )
            }
        );

        LinkMethodMeta {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SchemaGetter>::Schema;
            foreign_node_schema_import: foreign_node_schema_import.into(),

            foreign_node_type_validator: quote!(
                #crate_name::validators::assert_impl_one!(#linked_node_type: #crate_name::Node);
            ),

            link_field_method: record_link_default_alias_as_method.into(),
        }
    }

    fn link_many(
        &self,
        link_many_node_type: &LinkManyAttrType,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> LinkMethodMeta {
        let normalized_field_name_str = normalized_field_name.field_ident_serialized_fmt;
        let normalized_field_name = normalized_field_name.field_ident_raw_to_underscore_suffix;
        let VariablesModelMacro {
            ___________graph_traversal_string,
            __________connect_node_to_graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let crate_name = get_crate_name(false);

        let foreign_node_schema_import = if *struct_name_ident == link_many_node_type.to_string() {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #link_many_node_type = <super::#link_many_node_type as #crate_name::SchemaGetter>::Schema;)
        };

        let link_field_method = quote!(
            pub fn #normalized_field_name(&self, clause: impl ::std::convert::Into<#crate_name::NodeAliasClause>) -> #link_many_node_type {
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

        LinkMethodMeta {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SchemaGetter>::Schema;
            foreign_node_schema_import: foreign_node_schema_import.into(),
            link_field_method: link_field_method.into(),
            foreign_node_type_validator: quote!(
                #crate_name::validators::assert_impl_one!(#link_many_node_type: #crate_name::Node);
            )
            .into(),
        }
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
        let field_name_serialized = self.field_name_serialized(&struct_casing)?;
        let node_type_alias_with_trait_bounds = embedded_object;
        let VariablesModelMacro {
            __________connect_object_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let foreign_node_schema_import = if *current_struct_ident
            .is_same(embedded_object.type_name()?)
        {
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
}

struct LinkerMethodToken(TokenStream);

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

impl LinkOneAttrType {}

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

    pub(crate) fn from_record_link(
        &self,
        field_receiver: &MyFieldReceiver,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> Self {
        let link_self_type =
            LinkOneAttrType(self.replace_self_with_current_struct_ident(table_derive_attrs));
        LinkOneAttrType::from_record_link(&link_self_type, field_receiver, table_derive_attrs)
    }
}

impl LinkManyAttrType {}

impl NestObjectAttrType {}

impl NestArrayAttrType {
    pub(crate) fn from_nested(
        &self,
        field_receiver: &MyFieldReceiver,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<Self> {
        let node_type = &self;
        let normalized_field_name = &field_receiver
            .normalize_ident(table_derive_attrs.struct_level_casing()?)
            .field_ident_raw_to_underscore_suffix;
        let struct_name_ident = &table_derive_attrs.ident;

        let VariablesModelMacro {
            __________connect_object_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let normalized_field_name_str = normalized_field_name.to_string();
        let crate_name = get_crate_name(false);
        let node_type_alias_with_trait_bounds = node_type;

        let foreign_node_schema_import = if *struct_name_ident == node_type.struct_type_name()? {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            // e.g type Book = <super::Book as SchemaGetter>::Schema;
            // type Book<'a, 'b, T, U: Clone + Default, V: Node> = <super::Book<'a, 'b, T, U, V> as SchemaGetter>::Schema;
            quote!(type #node_type = <super::#node_type as #crate_name::SchemaGetter>::Schema;)
        };

        let record_link_default_alias_as_method = quote!(
            pub fn #normalized_field_name(
                &self,
                clause: impl Into<#crate_name::ObjectClause>
            ) -> #schema_type_ident {
                let clause: #crate_name::ObjectClause = clause.into();
                let normalized_field_name_str = if self.build().is_empty(){
                    #normalized_field_name_str.to_string()
                }else {
                    format!(".{}", #normalized_field_name_str)
                };

                #schema_type_ident::#__________connect_object_to_graph_traversal_string(
                    self,
                    clause.with_field(normalized_field_name_str)
                )

            }
        );

        Self {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SchemaGetter>::Schema;
            foreign_node_schema_import,

            foreign_node_type_validator: quote!(
                #crate_name::validators::assert_impl_one!(#schema_type_ident: #crate_name::Object);
            ),

            record_link_default_alias_as_method,
            foreign_node_type: quote!(schema_type_ident),
            field_definition: quote!(),
            field_type_validation_asserts: vec![],
        }
    }
}
