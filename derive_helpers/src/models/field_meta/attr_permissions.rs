/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, Lit, Meta, MetaNameValue, Path};

use super::ExprOrPath;

#[derive(Debug, Clone)]
pub enum Permissions {
    /// permissions = full
    Full,
    /// permissions = none
    None,
    /// permissions = get_permissions or permissions = get_permissions() or permissions = get_permissions("some_arg")
    FnName(ExprOrPath),
}

impl ToTokens for Permissions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let permission = match self {
            Self::Full => {
                quote!(.permissions_full())
            }
            Self::None => {
                quote!(.permissions_none())
            }
            Self::FnName(permissions) => {
                quote!(.permissions(#permissions.to_raw()))
            }
        };

        tokens.extend(permission);
    }
}

impl FromMeta for Permissions {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        match expr {
            syn::Expr::Lit(lit) => Self::from_value(&lit.lit),
            _ => Ok(Self::FnName(ExprOrPath::Expr(expr.clone()))),
        }
    }

    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        match item {
            Meta::Path(ref path) => {
                let path_str = path.into_token_stream().to_string();
                match path_str.to_lowercase().as_ref() {
                    "none" => Ok(Self::None),
                    "full" => Ok(Self::Full),
                    _ => Ok(Self::FnName(ExprOrPath::Path(path.clone()))),
                }
            }
            Meta::NameValue(MetaNameValue { value, .. }) => match value {
                Expr::Path(expr_path) => {
                    let path_str = expr_path.path.to_token_stream().to_string();
                    match path_str.as_ref() {
                        "none" => Ok(Self::None),
                        "full" => Ok(Self::Full),
                        _ => Ok(Self::FnName(ExprOrPath::Path(expr_path.path.clone()))),
                    }
                }
                Expr::Lit(lit) => Self::from_value(&lit.lit),
                _ => Ok(Self::FnName(ExprOrPath::Expr(value.clone()))),
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
            Lit::Str(str_lit) => {
                let value_str = str_lit.value();
                match value_str.to_lowercase().as_str() {
                    "none" => Ok(Self::None),
                    "full" => Ok(Self::Full),
                    _ => match syn::parse_str::<Path>(&value_str) {
                        Ok(path) => Ok(Self::FnName(ExprOrPath::Path(path))),
                        Err(_) => Ok(syn::parse_str::<Expr>(&value_str)
                            .map(ExprOrPath::Expr)
                            .map(Self::FnName)?),
                    },
                }
            }
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}
