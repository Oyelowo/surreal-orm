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
use syn::spanned::Spanned;

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

    pub fn array_item_type(&self) -> Option<Self> {
        match self.0 {
            FieldType::Array(ref ft, _) => Some(Self(ft.deref().clone())),
            _ => None,
        }
    }

    pub fn set_item_type(&self) -> Option<Self> {
        match self.0 {
            FieldType::Set(ref ft, _) => Some(Self(ft.deref().clone())),
            _ => None,
        }
    }

    pub fn as_db_sql_value_tokenstream(&self) -> SqlValueTokenStream {
        let crate_name = get_crate_name(false);
        let value = self.into_inner_ref().clone().to_string();
        let value = quote!(#crate_name::FieldType::from_str(#value).unwrap());
        value.into()
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
                    return Err(darling::Error::custom("Expected a string literal for ty")
                        .with_span(&expr.span()));
                }
            }
            syn::Expr::Path(expr_path) => expr_path
                .path
                .get_ident()
                .map(|ident| ident.to_string())
                .ok_or_else(|| {
                    darling::Error::custom("Expected an identifier for ty").with_span(&expr.span())
                })?,
            syn::Expr::Verbatim(expr_verbatim) => expr_verbatim.to_string(),
            _ => {
                return Err(darling::Error::custom("Expected a string literal for ty")
                    .with_span(&expr.span()))
            }
        };
        let field_type = FieldType::from_str(&field_type).map_err(|_| {
            darling::Error::custom(format!(
                "Invalid db_field_type: {field_type}. Must be one of these: {}",
                FieldType::variants().join(", ")
            ))
            .with_span(&expr.span())
        })?;
        Ok(Self(field_type))
    }
}
