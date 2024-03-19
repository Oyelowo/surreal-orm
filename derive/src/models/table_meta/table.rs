/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use darling::FromMeta;
use proc_macro2::Ident;
use syn::{Expr, Lit};

use crate::models::*;

create_ident_wrapper!(TableNameIdent);

impl FromMeta for TableNameIdent {
    fn from_expr(expr: &Expr) -> darling::Result<Self> {
        match expr {
            Expr::Lit(expr) => {
                if let Lit::Str(lit_str) = &expr.lit {
                    Ok(TableNameIdent(Ident::new(&lit_str.value(), lit_str.span())))
                } else {
                    Err(darling::Error::custom("Expected a string literal."))
                }
            }
            // Expr::Verbatim(expr_verbatim) => {
            //     let ident = syn::parse2(expr_verbatim.clone().into_token_stream())?;
            //     Ok(TableNameIdent(ident))
            // }
            Expr::Path(expr_path) => {
                let ident = expr_path
                    .path
                    .get_ident()
                    .ok_or_else(|| darling::Error::custom("Expected an identifier."))?;
                Ok(TableNameIdent(ident.clone()))
            }
            _ => Err(darling::Error::custom("Expected a string literal.")),
        }
    }
    // fn from_meta(item: &Meta) -> darling::Result<Self> {
    //     match item {
    //         Meta::Path(path) => path
    //             .get_ident()
    //             .map(|ident| TableNameIdent(ident.clone()))
    //             .ok_or_else(|| darling::Error::custom("Expected an identifier.")),
    //         Meta::NameValue(nv) => match &nv.value {
    //             Expr::Lit(expr_lit) => {
    //                 if let Lit::Str(lit_str) = &expr_lit.lit {
    //                     Ok(TableNameIdent(Ident::new(&lit_str.value(), lit_str.span())))
    //                 } else {
    //                     Err(darling::Error::custom("Expected a string literal."))
    //                 }
    //             }
    //             Expr::Verbatim(expr_verbatim) => {
    //                 let ident = syn::parse2(expr_verbatim.clone().into_token_stream())?;
    //                 Ok(TableNameIdent(ident))
    //             }
    //             Expr::Path(expr_path) => {
    //                 let ident = expr_path
    //                     .path
    //                     .get_ident()
    //                     .ok_or_else(|| darling::Error::custom("Expected an identifier."))?;
    //                 Ok(TableNameIdent(ident.clone()))
    //             }
    //             _ => Err(darling::Error::custom("Expected a string literal.")),
    //         },
    //         _ => Err(darling::Error::custom("Unsupported format for table name.")),
    //     }
    // }
}

impl TableNameIdent {
    pub(crate) fn validate_and_return(
        &self,
        struct_name_ident: &StructIdent,
        relax_table: &Option<bool>,
    ) -> ExtractorResult<&Self> {
        let table = self;
        let expected_table = struct_name_ident.to_string().to_case(Case::Snake);
        if !relax_table.unwrap_or(false) && self.to_string() != expected_table {
            return Err(syn::Error::new(
                table.span(),
                format!(
                    "table name must be in snake case of the current struct name. 
        Try: `{expected_table}`.
        If you don't want to follow this convention, use attribute `relax_table`. "
                ),
            )
            .into());
        };

        Ok(self)
    }

    pub(crate) fn as_string(&self) -> TableNameAsString {
        TableNameAsString(self.to_string())
    }
}

pub struct TableNameAsString(String);

impl ToTokens for TableNameAsString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let table = format!("{}", self.0);
        let table = quote!(#table);
        table.to_tokens(tokens);
    }
}
