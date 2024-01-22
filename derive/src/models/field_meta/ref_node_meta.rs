/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::quote;
use syn::Ident;

use super::*;
use crate::{
    errors::ExtractorResult,
    models::{
        derive_attributes::TableDeriveAttributes, variables::VariablesModelMacro, DataType,
        LinkRustFieldType,
    },
};

#[derive(Default, Clone)]
pub(crate) struct ReferencedNodeMeta {
    pub(crate) foreign_node_type: TokenStream,
    pub(crate) foreign_node_schema_import: TokenStream,
    pub(crate) record_link_default_alias_as_method: TokenStream,
    pub(crate) foreign_node_type_validator: TokenStream,
    pub(crate) field_definition: TokenStream,
    pub(crate) field_type_validation_asserts: Vec<TokenStream>,
}

impl ReferencedNodeMeta {
    pub fn with_field_definition(
        mut self,
        field_receiver: &MyFieldReceiver,
        _struct_name_ident: &Ident,
        field_name_normalized: &String,
        data_type: &DataType,
        table: &Ident,
    ) -> ExtractorResult<Self> {
        let crate_name = get_crate_name(false);
        let mut define_field: Option<TokenStream> = None;
        let mut define_field_methods = vec![];
        let mut define_array_field_item_methods = vec![];
        let mut static_assertions = vec![];
        let DbFieldTypeAstMeta {
                db_field_type,
                static_assertion,
            } = field_receiver.get_db_type_with_assertion(field_name, model_type, table)?;

        if let Some(value) = validate_field_attributes(field_receiver, field_name_normalized) {
            return value;
        }

        if let Some(define) = field_receiver.define {
            quote!(#define.to_raw());
        }

if let Some(assert) = field_receiver.assert {
            define_field_methods.push(quote!(.assert(#assert)));
        }

        if let Some(item_assert) = field_receiver.item_assert {
            define_array_field_item_methods.push(quote!(.assert(#item_assert)));  {
        }

        if let Some(value) = field_receiver.value {
            define_array_field_item_methods.push(quote!(.value(#value)));
            // TODO: Continue tomorrow or at night
            static_assertions.push(value.get_static_assrtion(db_field_type.into_inner()));
        }

        if let Some(permissions) = field_receiver.permissions {
            define_field_methods.push(permissions);  {
        }

        // Helps to define the schema definition of the content
        let array_field_item_str = format!("{field_name_normalized}.*");
        // Im putting coma before this to separate from the top field array type definition in case
        // it is present
        let array_item_definition = if define_array_field_item_methods.is_empty() {
            quote!()
        } else {
            quote!(
                    ,
                #crate_name::statements::define_field(#crate_name::Field::new(#array_field_item_str))
                                        .on_table(#crate_name::Table::from(Self::table_name()))
                                        #( # define_array_field_item_methods) *
                                        .to_raw()

            )
        };

        self.field_definition = define_field.unwrap_or_else(||quote!(
                    #crate_name::statements::define_field(#crate_name::Field::new(#field_name_normalized))
                                            .on_table(#crate_name::Table::from(Self::table_name()))
                                            #( # define_field_methods) *
                                            .to_raw()
                    #array_item_definition
            ));

        self.field_type_validation_asserts.extend(static_assertions);

        Ok(self)
    }

    fn validate_field_attributes(field_receiver: &MyFieldReceiver, field_name_normalized: &String) -> Option<Result<ReferencedNodeMeta, crate::errors::ExtractorError>> {
    match field_receiver {
        MyFieldReceiver {
            define,
            assert,
            value,
            permissions,
            item_assert,
            ..
        } if (define_fn.is_some() || define.is_some())
            && (assert.is_some()
                || assert_fn.is_some()
                || value.is_some()
                || value_fn.is_some()
                || permissions.is_some()
                || permissions_fn.is_some()
                || item_assert.is_some()
                || item_assert_fn.is_some()) =>
        {
            return Some(Err(
            syn::Error::new_spanned(
                field_name_normalized,
                r#"Invalid combination. When `define`, the following attributes cannot be use in combination to prevent confusion:
    assert,
    value,
    permissions,
    item_assert"#).into()));
        }
        MyFieldReceiver {
            define,
            assert,
            value,
            permissions,
            item_assert,
            relate,
            ..
        } if (relate.is_some())
            && (define.is_some()
                || assert.is_some()
                || value.is_some()
                || permissions.is_some()
                || item_assert.is_some()) =>
        {
            return Some(Err(
            syn::Error::new_spanned(
                field_name_normalized,
                r#"This is a read-only relation field and does not allow the following attributes:
    define,
    assert,
    value,
    permissions,
    item_assert"#).into()));
        }
        _ => {}
    };
    None
}


    pub(crate) fn from_simple_array(normalized_field_name: &::syn::Ident) -> Self {
        let normalized_field_name_str = normalized_field_name.to_string();
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

        Self {
            foreign_node_schema_import: quote!(),

            foreign_node_type_validator: quote!(),

            record_link_default_alias_as_method,
            foreign_node_type: quote!(schema_type_ident),
            field_definition: quote!(),
            field_type_validation_asserts: vec![],
        }
    }

    pub(crate) fn from_record_link(
        linked_node_type: &LinkRustFieldType,
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

        let record_link_default_alias_as_method = if is_list {
            quote!(
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
            )
        } else {
            quote!(
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
            )
        };

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

    pub(crate) fn from_nested(
        node_type: &LinkRustFieldType,
        normalized_field_name: &::syn::Ident,
        struct_name_ident: &::syn::Ident,
        is_list: bool,
    ) -> ExtractorResult<Self> {
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

        let record_link_default_alias_as_method = if is_list {
            quote!(
                pub fn #normalized_field_name(&self, clause: impl Into<#crate_name::ObjectClause>) -> #schema_type_ident {
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
            )
        } else {
            quote!(
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
            )
        };

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

