/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;
use proc_macro2::TokenStream;
use proc_macros_helpers::parse_lit_to_tokenstream;
use quote::{quote, ToTokens};
use syn::{Expr, ExprLit, Lit, LitStr, Path};

use crate::errors::ExtractorResult;

#[derive(Debug, Clone)]
pub enum Permissions {
    Full,
    None,
    FnName(Expr),
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
                // TODO: Remove this
                // let permissions = parse_lit_to_tokenstream(permissions)?;
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
            _ => Ok(Self::FnName(expr.clone())),
        }
    }

    fn from_value(value: &Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(str_lit) => {
                let value_str = str_lit.value();

                if value_str.to_lowercase() == "none" {
                    Ok(Self::None)
                } else if value_str.to_lowercase() == "full" {
                    Ok(Self::Full)
                } else {
                    Ok(Self::FnName(LitStr::new(&value_str, str_lit.span())))
                    // Ok(Self::FnName(str_lit.to_owned()))
                }
                // Ok(Self::FnName(LitStr::new(&value_str, str_lit.span())))
            }
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PermissionsFn {
    Full,
    None,
    FnPath(Path),
}

impl ToTokens for PermissionsFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let perms = match self {
            Self::Full => {
                quote!(.permissions_full())
            }
            Self::None => {
                quote!(.permissions_none())
            }
            Self::FnPath(permissions_fn) => {
                quote!(.permissions(#permissions_fn().to_raw()))
            }
        };
        tokens.extend(perms);
    }
}

impl FromMeta for PermissionsFn {
    fn from_expr(expr: &Expr) -> darling::Result<Self> {
        match expr {
            Expr::Path(path) => Ok(Self::FnPath(path.path.clone())),
            Expr::Lit(lit) => Self::from_value(&lit.lit),
            _ => Err(darling::Error::unexpected_expr_type(expr)),
        }
    }
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "full" => Ok(Self::Full),
            _ => Err(darling::Error::unexpected_type(value)),
        }
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(str) => Ok(Self::FnPath(syn::parse_str::<Path>(&str.value())?)),
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}
