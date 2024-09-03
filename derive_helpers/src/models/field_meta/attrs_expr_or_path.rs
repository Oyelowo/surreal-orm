/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

use darling::FromMeta;
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::{Expr, Lit, Meta, MetaNameValue};

use crate::models::*;

#[derive(Clone, Debug)]
pub enum ExprOrPath {
    Expr(Expr),
    Path(syn::Path),
}

impl FromMeta for ExprOrPath {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        match expr {
            syn::Expr::Lit(lit) => Self::from_value(&lit.lit),
            _ => Ok(ExprOrPath::Expr(expr.clone())),
        }
    }

    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        match item {
            Meta::Path(ref path) => Ok(ExprOrPath::Path(path.clone())),
            Meta::NameValue(MetaNameValue { value, .. }) => match value {
                Expr::Path(expr_path) => Ok(ExprOrPath::Path(expr_path.path.clone())),
                Expr::Lit(lit) => Self::from_value(&lit.lit),
                _ => Ok(ExprOrPath::Expr(value.clone())),
                // _ => Err(darling::Error::custom(
                //     "Expected a valid Rust path or an expression",
                // )),
            },
            _ => Err(darling::Error::unsupported_shape(
                "Expected a path or a name-value pair",
            )),
        }
    }

    fn from_value(value: &Lit) -> darling::Result<Self> {
        match value {
            Lit::Byte(_)
            | Lit::Char(_)
            | Lit::Int(_)
            | Lit::Float(_)
            | Lit::Bool(_)
            | Lit::Str(_) => Ok(ExprOrPath::Expr(syn::parse2(quote!(#value))?)),
            // Lit::Str(str_lit) => {
            //     let value_str = str_lit.value();
            //     // just take the string directly as value the attribute is stringified i.e quoted
            //     // e.g #[surreal_orm(value = "18")]
            //     Ok(ExprOrPath::Expr(syn::parse_str::<Expr>(&value_str)?))
            //
            //     // match syn::parse_str::<Path>(&value_str) {
            //     //     Ok(path) => Ok(ExprOrPath::Path(path)),
            //     //     Err(_) => Ok(syn::parse_str::<Expr>(&value_str).map(ExprOrPath::Expr)?),
            //     // }
            // }
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}

//
// impl FromMeta for ExprOrPath {
//     fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
//         // TODO: Check if it makes sense to allow path alone of it that works OOTB
//         Ok(Self::Expr(expr.clone()))
//     }
//
//     // fn from_meta(item: &Meta) -> Result<Self, darling::Error> {
//     //     match item {
//     //         Meta::Path(ref path) => Ok(ExprOrPath::Path(path.clone())),
//     //         Meta::NameValue(MetaNameValue { value, .. }) => match value {
//     //             Expr::Path(expr_path) => {
//     //                 if expr_path.path.segments.is_empty() {
//     //                     Err(darling::Error::custom("Path cannot be empty"))
//     //                 } else {
//     //                     Ok(ExprOrPath::Path(expr_path.path.clone()))
//     //                 }
//     //             }
//     //             Expr::Lit(expr_lit) => {
//     //                 let lit = &expr_lit.lit;
//     //                 match lit {
//     //                     syn::Lit::Str(lit_str) => {
//     //                         let path_str = lit_str.value();
//     //                         let path = syn::parse_str::<syn::Path>(&path_str)
//     //                             .map_err(|_| darling::Error::custom("Invalid path"))?;
//     //                         Ok(ExprOrPath::Path(path))
//     //                     }
//     //                     _ => Err(darling::Error::custom("Expected a string literal")),
//     //                 }
//     //             }
//     //             _ => {
//     //                 // panic!("Value: {:?}", value);
//     //                 Err(darling::Error::custom(
//     //                                 "Expected a valid Rust path or an expression",
//     //                             ))
//     //             },
//     //         },
//     //         _ => Err(darling::Error::unsupported_shape(
//     //             "Expected a path or a name-value pair",
//     //         )),
//     //     }
//     // }
//     //
// }
//
impl ToTokens for ExprOrPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExprOrPath::Expr(expr) => expr.to_tokens(tokens),
            ExprOrPath::Path(path) => {
                quote!(#path()).to_tokens(tokens);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AttributeValue(ExprOrPath);
#[derive(Clone, Debug)]
pub struct AttributeAssert(ExprOrPath);
#[derive(Clone, Debug)]
pub struct AttributeItemAssert(ExprOrPath);
#[derive(Clone, Debug)]
pub struct AttributeAs(ExprOrPath);
#[derive(Clone, Debug)]
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

            fn from_value(value: &Lit) -> darling::Result<Self> {
                Ok(Self(ExprOrPath::from_value(value)?))
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
    pub fn get_default_value_static_assertion(
        &self,
        db_field_type: FieldType,
    ) -> StaticAssertionToken {
        let crate_name = &get_crate_name(false);
        let value_expr = &self;

        let convertible_values_to_db_type = match db_field_type {
            FieldType::Bytes => quote!(
                #crate_name::validators::assert_value_is_bytes(#value_expr)
            ),
            FieldType::Null => quote!(
                #crate_name::validators::assert_value_is_null(#value_expr)
            ),
            FieldType::Union(_) => quote!(),
            FieldType::Option(_) => quote!(
                #crate_name::validators::assert_value_is_option(#value_expr)
            ),
            FieldType::Uuid => quote!(
                #crate_name::validators::assert_value_is_uuid(#value_expr)
            ),
            FieldType::Duration => quote!(
                #crate_name::validators::assert_value_is_duration(#value_expr)
            ),
            FieldType::String => {
                quote!(#crate_name::validators::assert_value_is_string(#value_expr))
            }
            FieldType::Int => quote!(
                    #crate_name::validators::assert_value_is_int(#value_expr)
            ),
            FieldType::Float => quote!(
                    #crate_name::validators::assert_value_is_float(#value_expr)
            ),
            FieldType::Bool => quote!(
                        #crate_name::validators::assert_value_is_bool(#value_expr)
            ),
            FieldType::Array(_, _) => quote!(
                 #crate_name::validators::assert_value_is_array(#value_expr)
            ),
            FieldType::Set(_, _) => {
                quote!(#crate_name::validators::assert_value_is_set(#value_expr))
            }
            FieldType::Datetime => quote!(
                    #crate_name::validators::assert_value_is_datetime(#value_expr)
            ),
            FieldType::Decimal => quote!(
                #crate_name::validators::assert_value_is_number(#value_expr)
            ),
            FieldType::Number => quote!(
                #crate_name::validators::assert_value_is_number(#value_expr)
            ),
            FieldType::Object => quote!(),
            FieldType::Record(_) => quote!(
                #crate_name::validators::assert_value_is_thing(#value_expr)
            ),
            FieldType::Geometry(_) => quote!(
                #crate_name::validators::assert_value_is_geometry(#value_expr)
            ),
            FieldType::Any => quote!(
                #crate_name::validators::assert_value_is_any(#value_expr)
            ),
        };

        quote!(let _ = #convertible_values_to_db_type;).into()
    }
}
