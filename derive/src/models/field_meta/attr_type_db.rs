/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    str::FromStr,
};

use darling::FromMeta;
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use surreal_query_builder::FieldType;

use crate::models::*;

create_tokenstream_wrapper!(=> FieldTypeDbToken);

#[derive(Debug, Clone)]
pub struct DbFieldTypeAstMeta {
    pub field_type_db_original: Option<FieldType>,
    pub field_type_db_token: FieldTypeDbToken,
    pub static_assertion_token: StaticAssertionToken,
}

#[derive(Debug, Clone, Default)]
pub struct FieldTypeDb(pub FieldType);

impl ToTokens for FieldTypeDb {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_type_str = self.0.to_string();
        let crate_name = get_crate_name(false);
        let as_token = quote!(#crate_name::FieldTypeDb::from_str(#field_type_str).unwrap());

        as_token.to_tokens(tokens);
    }
}

impl FieldTypeDb {
    pub fn into_inner(self) -> FieldType {
        self.0
    }

    pub fn into_inner_ref(&self) -> &FieldType {
        &self.0
    }

    pub fn get_array_item_type(&self) -> Option<Self> {
        match self.0 {
            FieldType::Array(ref ft, _) => Some(Self(ft.deref().clone())),
            _ => None,
        }
    }

    pub fn as_db_sql_value_tokenstream(&self) -> SqlValueTokenStream {
        let crate_name = get_crate_name(false);
        let value = match self.into_inner_ref() {
            FieldType::Any => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Null => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Uuid => {
                quote!(#crate_name::sql::Uuid)
            }
            FieldType::Bytes => {
                quote!(#crate_name::sql::Bytes)
            }
            FieldType::Union(_) => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Option(ft) => {
                let val = Self(ft.deref().clone()).as_db_sql_value_tokenstream();
                quote!(::std::option::Option<#val>)
            }
            FieldType::String => {
                quote!(::std::string::String)
            }
            FieldType::Int => {
                // quote!(#crate_name::validators::Int)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Float => {
                // quote!(#crate_name::validators::Float)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Bool => {
                quote!(::std::convert::Into<::std::primitive::bool>)
            }
            FieldType::Array(_, _) => {
                // quote!(::std::iter::IntoIterator)
                // quote!(::std::convert::Into<#crate_name::sql::Array>)
                quote!(::std::vec::Vec<#crate_name::sql::Value>)
            }
            FieldType::Set(_, _) => {
                quote!(::std::collections::HashSet<#crate_name::sql::Value>)
            }
            FieldType::Datetime => {
                quote!(#crate_name::sql::Datetime)
            }
            FieldType::Decimal => {
                quote!(#crate_name::validators::Float)
            }
            FieldType::Duration => {
                quote!(#crate_name::sql::Duration)
            }
            FieldType::Number => {
                // quote!(#crate_name::validators::Num)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Object => {
                quote!(#crate_name::sql::Object)
            }
            FieldType::Record(_) => {
                quote!(#crate_name::sql::Thing)
            }
            FieldType::Geometry(_) => {
                quote!(#crate_name::sql::Geometry)
            }
        };
        value.into()
    }

    // Even if db type provided, it is still checked against the rust type
    // to make sure it's compatible
    pub fn generate_static_assertions(
        &self,
        rust_field_type: &CustomType,
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
                    #crate_name::validators::assert_type_is_int::<#rust_field_type>();
                    // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                )
            }
            FieldType::Float => {
                quote!(
                    #crate_name::validators::assert_type_is_float::<#rust_field_type>();
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
                    #crate_name::validators::assert_type_is_number::<#rust_field_type>();
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

impl Display for FieldTypeDb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for FieldTypeDb {
    type Target = FieldType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for FieldTypeDb {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        let field_type = match expr {
            syn::Expr::Lit(expr_lit) => {
                if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                    lit_str.value()
                } else {
                    return Err(darling::Error::custom("Expected a string literal for ty"));
                }
            }
            syn::Expr::Path(expr_path) => expr_path
                .path
                .get_ident()
                .map(|ident| ident.to_string())
                .ok_or_else(|| darling::Error::custom("Expected an identifier for ty"))?,
            syn::Expr::Verbatim(expr_verbatim) => expr_verbatim.to_string(),
            _ => return Err(darling::Error::custom("Expected a string literal for ty")),
        };
        let field_type = FieldType::from_str(&field_type).map_err(|_| {
            darling::Error::custom(format!(
                "Invalid db_field_type: {field_type}. Must be one of these: {}",
                FieldType::variants().join(", ")
            ))
        })?;
        Ok(Self(field_type))
    }
}
