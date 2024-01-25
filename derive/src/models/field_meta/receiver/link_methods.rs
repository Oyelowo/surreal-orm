use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{
        derive_attributes::TableDeriveAttributes, variables::VariablesModelMacro,
        FieldIdentSerialized, LinkManyAttrType, LinkOneAttrType, LinkSelfAttrType, ListSimple,
        MyFieldReceiver, NestArrayAttrType, NestObjectAttrType, NormalisedFieldMeta,
    },
};

use super::{field_receiver, RelationType};

struct LinkerMethodToken(TokenStream);
impl RelationType {
    pub fn get_field_link_method(&self) -> LinkerMethodToken {
        match self {
            RelationType::LinkSelf(link_self) => {
                let method = link_self.get_field_link_method();
                LinkerMethodToken(method)
            }
            RelationType::LinkOne(link_one) => {
                let method = link_one.get_field_link_method();
                LinkerMethodToken(method)
            }
            RelationType::LinkMany(link_many) => {
                let method = link_many.get_field_link_method();
                LinkerMethodToken(method)
            }
            RelationType::NestArray(nest_array) => {
                let method = nest_array.get_field_link_method();
                LinkerMethodToken(method)
            }
            RelationType::NestObject(nest_object) => {
                let method = nest_object.get_field_link_method();
                LinkerMethodToken(method)
            }
            RelationType::List(list) => {
                let method = list.get_field_link_method();
                LinkerMethodToken(method)
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
}

struct ForeignNodeSchemaImport(TokenStream);
struct ForeignNodeTypeValidator(TokenStream);
struct RecordLinkDefaultAliasAsMethod(TokenStream);

struct LinkMethodMeta {
    foreign_node_schema_import: ForeignNodeSchemaImport,
    foreign_node_type_validator: ForeignNodeTypeValidator,
    record_link_default_alias_as_method: RecordLinkDefaultAliasAsMethod,
}

impl ListSimple {
    pub fn get_field_link_method(
        &self,
        // db_field_serialized_name: &NormalisedFieldMeta,
        field_receiver: &MyFieldReceiver,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<TokenStream> {
        let NormalisedFieldMeta {
            field_ident_raw_to_underscore_suffix: normalized_field_name,
            field_ident_serialized_fmt: normalized_field_name_str,
        } = field_receiver.normalize_ident(table_derive_attrs.struct_level_casing()?);
        let crate_name = get_crate_name(false);

        let record_link_default_alias_as_method = quote!(
            pub fn #normalized_field_name(&self, clause: impl Into<#crate_name::NodeAliasClause>) -> #crate_name::Field {
                let clause: #crate_name::NodeAliasClause = clause.into();
                let clause: #crate_name::NodeClause = clause.into_inner();

                let normalized_field_name_str = if self.build().is_empty(){
                    #normalized_field_name_str.to_string()
                }else {
                    format!(".{}", #normalized_field_name_str)
                };

                let clause: #crate_name::NodeClause = clause.into();
                let bindings = self.get_bindings().into_iter().chain(clause.get_bindings().into_iter()).collect::<Vec<_>>();

                let errors = self.get_errors().into_iter().chain(clause.get_errors().into_iter()).collect::<Vec<_>>();

                let field = #crate_name::Field::new(format!("{normalized_field_name_str}{}", clause.build()))
                            .with_bindings(bindings)
                            .with_errors(errors);
                field

            }
        );

        // Self {
        //     foreign_node_schema_import: quote!(),
        //
        //     foreign_node_type_validator: quote!(),
        //
        //     record_link_default_alias_as_method,
        //     foreign_node_type: quote!(schema_type_ident),
        //     field_definition: quote!(),
        //     field_type_validation_asserts: vec![],
        // }
    }
}

impl LinkOneAttrType {
    pub(crate) fn from_record_link(
        &self,
        field_receiver: &MyFieldReceiver,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> Self {
        let linked_node_type = &self.replace_self_with_current_struct_ident(table_def);
        let normalized_field_name =
            &field_receiver.normalize_ident(table_derive_attrs.struct_level_casing()?);
        let struct_name_ident = &table_derive_attrs.ident;
        let normalized_field_name_str = normalized_field_name.field_ident_serialized_fmt;
        let normalized_field_name = normalized_field_name.field_ident_raw_to_underscore_suffix;
        let VariablesModelMacro {
            ___________graph_traversal_string,
            __________connect_node_to_graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let schema_type = linked_node_type;
        let crate_name = get_crate_name(false);

        let foreign_node_schema_import = if *struct_name_ident == linked_node_type.to_string() {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #schema_type = <super::#schema_type as #crate_name::SchemaGetter>::Schema;)
        };

        let record_link_default_alias_as_method = quote!(
            pub fn #normalized_field_name(&self) -> #schema_type {
                let clause = #crate_name::Clause::from(#crate_name::Empty);

                let normalized_field_name_str = if self.build().is_empty(){
                    #normalized_field_name_str.to_string()
                }else {
                    format!(".{}", #normalized_field_name_str)
                };

                #schema_type::#__________connect_node_to_graph_traversal_string(
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
                #crate_name::validators::assert_impl_one!(#schema_type: #crate_name::Node);
            ),

            record_link_default_alias_as_method,
            foreign_node_type: quote!(schema_type_ident),
            field_definition: quote!(),
            field_type_validation_asserts: vec![],
        }
    }
}

impl LinkSelfAttrType {
    pub(crate) fn to_linkone_attr_type(self) -> LinkOneAttrType {
        LinkOneAttrType(self.0)
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

impl LinkManyAttrType {
    pub(crate) fn from_record_link(
        linked_node_type: &DestinationNodeTypeOriginal,
        normalized_field_name: &NormalisedFieldMeta,
        // normalized_field_name: &::syn::Ident,
        struct_name_ident: &::syn::Ident,
        is_list: bool,
    ) -> Self {
        let normalized_field_name_str = normalized_field_name.field_ident_serialized_fmt;
        let normalized_field_name = normalized_field_name.field_ident_raw_to_underscore_suffix;
        let VariablesModelMacro {
            ___________graph_traversal_string,
            __________connect_node_to_graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let schema_type = linked_node_type;
        let crate_name = get_crate_name(false);

        let foreign_node_schema_import = if *struct_name_ident == linked_node_type.to_string() {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #schema_type = <super::#schema_type as #crate_name::SchemaGetter>::Schema;)
        };

        let record_link_default_alias_as_method = quote!(
            pub fn #normalized_field_name(&self, clause: impl Into<#crate_name::NodeAliasClause>) -> #schema_type {
            let clause: #crate_name::NodeAliasClause = clause.into();
            let clause: #crate_name::NodeClause = clause.into_inner();

            let normalized_field_name_str = if self.build().is_empty(){
            #normalized_field_name_str.to_string()
            }else {
            format!(".{}", #normalized_field_name_str)
            };


            #schema_type::#__________connect_node_to_graph_traversal_string(
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
                #crate_name::validators::assert_impl_one!(#schema_type: #crate_name::Node);
            ),

            record_link_default_alias_as_method,
            foreign_node_type: quote!(schema_type_ident),
            field_definition: quote!(),
            field_type_validation_asserts: vec![],
        }
    }
}

impl NestObjectAttrType {
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
            pub fn #normalized_field_name(&self) -> #schema_type_ident {
                let clause = #crate_name::Clause::from(#crate_name::Empty);

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
