/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::{Expr, Meta, MetaNameValue};

use crate::models::*;

#[derive(Clone, Debug)]
pub enum ExprOrPath {
    Expr(Expr),
    Path(syn::Path),
}

impl FromMeta for ExprOrPath {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        Ok(Self::Expr(expr.clone()))
    }

    fn from_meta(item: &Meta) -> Result<Self, darling::Error> {
        match item {
            Meta::Path(ref path) => Ok(ExprOrPath::Path(path.clone())),
            Meta::NameValue(MetaNameValue { value, .. }) => match value {
                Expr::Path(expr_path) => {
                    if expr_path.path.segments.len() > 0 {
                        Ok(ExprOrPath::Path(expr_path.path))
                    } else {
                        Err(darling::Error::custom("Path cannot be empty"))
                    }
                }
                _ => Err(darling::Error::custom(
                    "Expected a valid Rust path or an expression",
                )),
            },
            _ => Err(darling::Error::unsupported_shape(
                "Expected a path or a name-value pair",
            )),
        }
    }
}

impl ToTokens for ExprOrPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExprOrPath::Expr(expr) => expr.to_tokens(tokens),
            ExprOrPath::Path(path) => path.to_tokens(tokens),
        }
    }
}

#[derive(Debug)]
pub struct AttributeValue(ExprOrPath);
#[derive(Debug)]
pub struct AttributeAssert(ExprOrPath);
#[derive(Debug)]
pub struct AttributeItemAssert(ExprOrPath);
#[derive(Debug)]
pub struct AttributeAs(ExprOrPath);
#[derive(Debug)]
pub struct AttributeDefine(ExprOrPath);

macro_rules! impl_from_expr_or_path {
    ($ty:ty) => {
        impl FromMeta for $ty {
            fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
                Ok(Self(ExprOrPath::Expr(expr.clone())))
            }

            fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
                Ok(Self(ExprOrPath::from_meta(item)?))
            }
        }

        impl ToTokens for $ty {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                self.0.to_tokens(tokens)
            }
        }
    };
}

impl_from_expr_or_path!(AttributeValue);
impl_from_expr_or_path!(AttributeAssert);
impl_from_expr_or_path!(AttributeItemAssert);
impl_from_expr_or_path!(AttributeAs);
impl_from_expr_or_path!(AttributeDefine);

impl AttributeValue {
    pub fn get_static_assertion(&self, db_field_type: FieldType) -> StaticAssertionToken {
        let crate_name = &get_crate_name(false);
        let value_expr = &self;

        let convertible_values_to_db_type = match db_field_type {
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

        quote!(let _ = #convertible_values_to_db_type;).into()
    }
}
