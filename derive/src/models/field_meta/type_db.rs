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
use quote::quote;
use surreal_query_builder::FieldType;

use crate::models::{DataType, RustFieldTypeSelfAllowed};

#[derive(Debug, Clone, Default)]
pub struct DbFieldTypeAst {
    pub(crate) db_field_type: TokenStream,
    pub(crate) static_assertion: TokenStream,
}

#[derive(Debug, Clone)]
pub struct DbFieldType(FieldType);

impl DbFieldType {
    pub fn into_inner(self) -> FieldType {
        self.0
    }
}

impl DbFieldType {
    pub fn generate_static_assertions(
        &self,
        rust_field_type: &RustFieldTypeSelfAllowed,
        model_type: &DataType,
    ) -> TokenStream {
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
        static_assertion
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
