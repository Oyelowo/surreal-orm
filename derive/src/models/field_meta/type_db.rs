/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

use darling::FromMeta;
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::Expr;

use crate::models::{DataType, MainFieldTypeSelfAllowed, StaticAssertionToken};

#[derive(Debug, Clone, Default)]
pub struct DbFieldTypeAstMeta {
    pub(crate) db_field_type: DbFieldType,
    pub(crate) static_assertion: StaticAssertionToken,
}

#[derive(Debug, Clone, Default)]
pub struct DbFieldType(FieldType);

impl DbFieldType {
    pub fn get_static_assertions_for_value_attr(&self, value_expr: Expr) -> StaticAssertionToken {
        let convertible_values_to_db_type = match field_type {
            FieldType::Bytes => quote!(#crate_name::sql::Bytes::from(#value_expr)),
            FieldType::Null => quote!(#crate_name::sql::Value::Null),
            // FieldType::Union(_) => quote!(#crate_name::sql::Value::from(#value_expr)),
            FieldType::Union(_) => quote!(),
            // FieldType::Option(_) => quote!(#crate_name::sql::Value::from(#value_expr)),
            FieldType::Option(_) => quote!(),
            FieldType::Uuid => quote!(#crate_name::sql::Uuid::from(#value_expr)),
            FieldType::Duration => quote!(#crate_name::sql::Duration::from(#value_expr)),
            FieldType::String => quote!(#crate_name::sql::String::from(#value_expr)),
            FieldType::Int => quote!(#crate_name::sql::Number::from(#value_expr)),
            FieldType::Float => quote!(#crate_name::sql::Number::from(#value_expr)),
            FieldType::Bool => quote!(#crate_name::sql::Bool::from(#value_expr)),
            FieldType::Array(_, _) => quote!(),
            FieldType::Set(_, _) => quote!(),
            // FieldType::Array => quote!(#crate_name::sql::Value::from(#value)),
            FieldType::Datetime => quote!(#crate_name::sql::Datetime::from(#value_expr)),
            FieldType::Decimal => quote!(#crate_name::sql::Number::from(#value_expr)),
            FieldType::Number => quote!(#crate_name::sql::Number::from(#value_expr)),
            FieldType::Object => quote!(),
            // FieldType::Object => quote!(#crate_name::sql::Value::from(#value_expr)),
            FieldType::Record(_) => quote!(#crate_name::sql::Thing::from(#value_expr)),
            FieldType::Geometry(_) => quote!(#crate_name::sql::Geometry::from(#value_expr)),
            FieldType::Any => quote!(#crate_name::sql::Value::from(#value_expr)),
        };
        tokens.extend(convertible_values_to_db_type);

        let x = quote!(let _ = #static_assertion;);
    }
}

impl ToTokens for DbFieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}

impl DbFieldType {
    pub fn into_inner(self) -> FieldType {
        self.0
    }
}

impl DbFieldType {
    pub fn generate_static_assertions(
        &self,
        rust_field_type: &MainFieldTypeSelfAllowed,
        model_type: &DataType,
    ) -> StaticAssertionToken {
        let rust_field_type = &mut rust_field_type.clone();
        let crate_name = get_crate_name(false);

        let static_assertion = match self.0 {
            FieldType::Any => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Value>);)
            }
            FieldType::Null => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Value>);)
            }
            FieldType::Uuid => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Uuid>);)
            }
            FieldType::Bytes => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Bytes>);)
            }
            FieldType::Union(_) => {
                // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                quote!()
            }
            FieldType::Option(_) => {
                // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                quote!()
            }
            FieldType::String => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<::std::string::String>);)
            }
            FieldType::Int => {
                quote!(
                    #crate_name::validators::is_int::<#rust_field_type>();
                    // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                )
            }
            FieldType::Float => {
                quote!(
                    #crate_name::validators::is_float::<#rust_field_type>();
                    // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                )
                // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
            }
            FieldType::Bool => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<::std::primitive::bool>);)
            }
            FieldType::Array(_, _) => {
                quote!(
                    #crate_name::validators::assert_is_vec::<#rust_field_type>();
                )
            }
            FieldType::Set(_, _) => {
                quote!(
                    #crate_name::validators::assert_is_vec::<#rust_field_type>();
                )
            }
            FieldType::Datetime => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Datetime>);)
            }
            FieldType::Decimal => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Number>);)
            }
            FieldType::Duration => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Duration>);)
            }
            FieldType::Number => {
                quote!(
                    #crate_name::validators::is_number::<#rust_field_type>();
                    // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                )
                // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
            }
            FieldType::Object => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Object>);)
            }
            FieldType::Record(_) => {
                if model_type.is_edge() {
                    quote!()
                } else {
                    quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<Option<#crate_name::sql::Thing>>);)
                }
            }
            FieldType::Geometry(_) => {
                quote!(#crate_name::validators::assert_impl_one!(#rust_field_type: ::std::convert::Into<#crate_name::sql::Geometry>);)
            }
        };
        static_assertion.into()
    }
}

impl Display for DbFieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for DbFieldType {
    type Target = FieldType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for DbFieldType {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.parse::<FieldType>() {
            Ok(f) => Ok(Self(f)),
            Err(e) => Err(darling::Error::unknown_value(&e)),
        }
    }
}
