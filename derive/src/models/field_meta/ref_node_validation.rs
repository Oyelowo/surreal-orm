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
    models::{variables::VariablesModelMacro, DataType, LinkRustFieldType},
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
        println!(
            "with_field_definition1 fieldname {}; ",
            field_name_normalized,
        );
        let crate_name = get_crate_name(false);
        let mut define_field: Option<TokenStream> = None;
        let mut define_field_methods = vec![];
        let mut define_array_field_item_methods = vec![];
        let mut static_assertions = vec![];

        println!(
            "with_field_definition2 fieldname {}; ",
            field_name_normalized,
        );
        let type_inf = field_receiver.get_db_type(field_name_normalized, data_type, table)?;
        // println!("type_inf {}", type_inf.unwrap_or_default());

        println!(
            "with_field_definition3 fieldname {}; ",
            field_name_normalized,
        );
        let field_type_resolved = if let Some(type_data) = type_inf {
            let DbFieldTypeMeta {
                db_field_type: field_type,
                static_assertion,
            } = type_data;

            define_field_methods.push(quote!(.type_(#field_type)));
            static_assertions.push(static_assertion);

            // TODO: Check if this would be needed.
            // if let Some(field_item_type) = field_item_type {
            //     define_array_field_item_methods.push(quote!(.type_(#field_item_type)));
            // }
            // Return field_type for later overriding type information in define_fn/define
            // attributes in case user uses either of those attributes. This is cause the type
            // attribute should supersede as it is what is used to validate field data at compile
            // time. Doing that with the `define` function attributes at compile-time may be tricky/impossible.
            field_type
        } else {
            return Err(
                syn::Error::new_spanned(field_name_normalized, "Invalid type provided").into(),
            );
        };

        match field_receiver {
            MyFieldReceiver {
                define,
                define_fn,
                // type_,
                assert,
                assert_fn,
                value,
                value_fn,
                permissions,
                permissions_fn,
                // item_type,
                item_assert,
                item_assert_fn,
                ..
            } if (define_fn.is_some() || define.is_some())
                && (
                    // I think type should be allowed in addition to define or define_fn but will
                    // override whatever is defined in define or define_fn, so we can use it for
                    // code inference and generation.
                    // type_.is_some()
                    assert.is_some()
                        || assert_fn.is_some()
                        || value.is_some()
                        || value_fn.is_some()
                        || permissions.is_some()
                        || permissions_fn.is_some()
                        || item_assert.is_some()
                        || item_assert_fn.is_some()
                ) =>
            {
                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    r#"Invalid combination. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:
    assert,
    assert_fn,
    value,
    value_fn,
    permissions,
    permissions_fn,
    item_assert,
    item_assert_fn"#).into());
            }
            MyFieldReceiver {
                define,
                define_fn,
                // type_,
                assert,
                assert_fn,
                value,
                value_fn,
                permissions,
                permissions_fn,
                item_assert,
                item_assert_fn,
                relate,
                ..
            } if (relate.is_some())
                && (
                    // type_.is_some()
                    define.is_some()
                        || define_fn.is_some()
                        || assert.is_some()
                        || assert_fn.is_some()
                        || value.is_some()
                        || value_fn.is_some()
                        || permissions.is_some()
                        || permissions_fn.is_some()
                        || item_assert.is_some()
                        || item_assert_fn.is_some()
                ) =>
            {
                //             return Err("Invalid combination. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:
                // define,
                // define_fn,
                // assert,
                // assert_fn,
                // value,
                // value_fn,
                // permissions,
                // permissions_fn,
                // item_assert,
                // item_assert_fn");

                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    r#"Invalid combination. When `relate`, the following attributes cannot be use in combination to prevent confusion:
    define,
    define_fn,
    assert,
    assert_fn,
    value,
    value_fn,
    permissions,
    permissions_fn,
    item_assert,
    item_assert_fn"#).into());
            }
            MyFieldReceiver {
                define: Some(_),
                define_fn: Some(_),
                ..
            } => {
                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    "define and define_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.").into());
            }
            MyFieldReceiver {
                define: Some(define),
                ..
            } => {
                let define = parse_lit_to_tokenstream(define).expect("Unable to parse define");
                if define.to_token_stream().to_string().chars().count() < 3 {
                    // If empty, we get only the `()` of the function, so we can assume that it is empty
                    // if there are less than 3 characters.
                    // return Err("define attribute is empty. Please provide a define_fn attribute.");
                    return Err(syn::Error::new_spanned(
                        field_name_normalized,
                        "define attribute is empty. Please provide a define_fn attribute.",
                    )
                    .into());
                }
                define_field = Some(
                    quote!(#define.on_table(Self::table_name()).type_(#field_type_resolved).to_raw()),
                );
            }
            MyFieldReceiver {
                define_fn: Some(define_fn),
                ..
            } => {
                if define_fn.to_token_stream().to_string().is_empty() {
                    return Err(syn::Error::new_spanned(
                        field_name_normalized,
                        "define_fn attribute is empty. Please provide a define_fn attribute.",
                    )
                    .into());
                }

                define_field = Some(quote!(#define_fn().type_(#field_type_resolved).to_raw()));
            }
            _ => {}
        };

        match field_receiver {
            MyFieldReceiver {
                item_assert: Some(_),
                item_assert_fn: Some(_),
                ..
            } => {
                return Err(syn::Error::new_spanned(
                    field_name_normalized,
                    "item_assert and item_assert_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into());
            }
            MyFieldReceiver {
                item_assert: Some(item_assert),
                ..
            } => {
                let item_assert =
                    parse_lit_to_tokenstream(item_assert).expect("Unable to parse item_assert");
                define_array_field_item_methods.push(quote!(.assert(#item_assert)));
            }
            MyFieldReceiver {
                item_assert_fn: Some(item_assert_fn),
                ..
            } => {
                define_array_field_item_methods.push(quote!(.assert(#item_assert_fn())));
            }
            _ => {}
        };

        // Gather default values
        match field_receiver {
            MyFieldReceiver {
                value: Some(_value),
                value_fn: Some(_value_fn),
                ..
            } => {
                return Err(syn::Error::new_spanned(
                    field_name_normalized,
                    "value and value_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into());
            }
            MyFieldReceiver {
                value: Some(value),
                type_: Some(type_),
                ..
            } => {
                let value = parse_lit_to_tokenstream(value).expect("unable to parse value");
                let field_type = type_.deref();
                let static_assertion = match field_type {
                    FieldType::Duration => quote!(#crate_name::sql::Duration::from(#value)),
                    FieldType::Uuid => quote!(#crate_name::sql::Uuid::from(#value)),
                    FieldType::Bytes => quote!(#crate_name::sql::Bytes::from(#value)),
                    FieldType::Null => quote!(#crate_name::sql::Value::Null),
                    // FieldType::Union(_) => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Union(_) => quote!(),
                    // FieldType::Option(_) => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Option(_) => quote!(),
                    FieldType::String => quote!(#crate_name::sql::String::from(#value)),
                    FieldType::Int => quote!(#crate_name::sql::Number::from(#value)),
                    FieldType::Float => quote!(#crate_name::sql::Number::from(#value)),
                    FieldType::Bool => quote!(#crate_name::sql::Bool::from(#value)),
                    FieldType::Array(_, _) => quote!(),
                    FieldType::Set(_, _) => quote!(),
                    // FieldType::Array => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Datetime => quote!(#crate_name::sql::Datetime::from(#value)),
                    FieldType::Decimal => quote!(#crate_name::sql::Number::from(#value)),
                    FieldType::Number => quote!(#crate_name::sql::Number::from(#value)),
                    FieldType::Object => quote!(),
                    // FieldType::Object => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Record(_) => quote!(#crate_name::sql::Thing::from(#value)),
                    FieldType::Geometry(_) => quote!(#crate_name::sql::Geometry::from(#value)),
                    FieldType::Any => quote!(#crate_name::sql::Value::from(#value)),
                };

                static_assertions.push(quote!(let _ = #static_assertion;));

                define_field_methods
                    // .push(quote!(.value(#crate_name::sql::Value::from(#type_of))));
                    .push(quote!(.value(#crate_name::sql::to_value(&#value).unwrap())));
            }
            MyFieldReceiver {
                value_fn: Some(value_fn),
                type_: Some(type_),
                ..
            } => {
                let field_type = type_.deref();
                let static_assertion = match field_type {
                    FieldType::Bytes => quote!(#crate_name::sql::Bytes::from(#value_fn())),
                    FieldType::Null => quote!(#crate_name::sql::Value::Null),
                    // FieldType::Union(_) => quote!(#crate_name::sql::Value::from(#value_fn())),
                    FieldType::Union(_) => quote!(),
                    // FieldType::Option(_) => quote!(#crate_name::sql::Value::from(#value_fn())),
                    FieldType::Option(_) => quote!(),
                    FieldType::Uuid => quote!(#crate_name::sql::Uuid::from(#value_fn())),
                    FieldType::Duration => quote!(#crate_name::sql::Duration::from(#value_fn())),
                    FieldType::String => quote!(#crate_name::sql::String::from(#value_fn())),
                    FieldType::Int => quote!(#crate_name::sql::Number::from(#value_fn())),
                    FieldType::Float => quote!(#crate_name::sql::Number::from(#value_fn())),
                    FieldType::Bool => quote!(#crate_name::sql::Bool::from(#value_fn())),
                    FieldType::Array(_, _) => quote!(),
                    FieldType::Set(_, _) => quote!(),
                    // FieldType::Array => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Datetime => quote!(#crate_name::sql::Datetime::from(#value_fn())),
                    FieldType::Decimal => quote!(#crate_name::sql::Number::from(#value_fn())),
                    FieldType::Number => quote!(#crate_name::sql::Number::from(#value_fn())),
                    FieldType::Object => quote!(),
                    // FieldType::Object => quote!(#crate_name::sql::Value::from(#value_fn())),
                    FieldType::Record(_) => quote!(#crate_name::sql::Thing::from(#value_fn())),
                    FieldType::Geometry(_) => quote!(#crate_name::sql::Geometry::from(#value_fn())),
                    FieldType::Any => quote!(#crate_name::sql::Value::from(#value_fn())),
                };
                static_assertions.push(quote!(let _ = #static_assertion;));

                define_field_methods
                    // .push(quote!(.value(#crate_name::sql::Value::from(#value_fn()))));
                    // .push(quote!(.value(#crate_name::sql::Value::from(#type_of))));
                    .push(quote!(.value(#crate_name::sql::to_value(&#value_fn()).unwrap())));
            }
            _ => {}
        };

        // Gather assertions
        match field_receiver {
            MyFieldReceiver {
                assert: Some(_),
                assert_fn: Some(_),
                ..
            } => {
                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    "assert and assert_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into());
            }
            MyFieldReceiver {
                assert: Some(assert),
                ..
            } => {
                let assert = parse_lit_to_tokenstream(assert).expect("unable to parse assert");
                define_field_methods.push(quote!(.assert(#assert)));
            }
            MyFieldReceiver {
                assert_fn: Some(assert_fn),
                ..
            } => {
                define_field_methods.push(quote!(.assert(#assert_fn())));
            }
            _ => {}
        };

        // Gather permissions
        match field_receiver {
            MyFieldReceiver {
                permissions: Some(_),
                permissions_fn: Some(_),
                ..
            } => {
                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    "permissions and permissions_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into());
            }
            MyFieldReceiver {
                permissions: Some(permissions),
                ..
            } => {
                define_field_methods.push(permissions.get_token_stream());
            }
            MyFieldReceiver {
                permissions_fn: Some(permissions_fn),
                ..
            } => {
                define_field_methods.push(permissions_fn.get_token_stream());
            }
            _ => {}
        };

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
        normalized_field_name: &NormalisedField,
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
