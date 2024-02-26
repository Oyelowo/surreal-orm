/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use darling::FromMeta;
use quote::format_ident;
use syn::{Expr, Lit, Meta};

use crate::models::*;

create_ident_wrapper!(TableNameIdent);

impl FromMeta for TableNameIdent {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        match item {
            Meta::Path(path) => path
                .get_ident()
                .map(|ident| TableNameIdent(ident.clone()))
                .ok_or_else(|| darling::Error::custom("Expected an identifier.")),
            Meta::NameValue(nv) => match &nv.value {
                Expr::Lit(expr_lit) => {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        Ok(TableNameIdent(format_ident!("{}", lit_str.value())))
                    } else {
                        Err(darling::Error::custom("Expected a string literal."))
                    }
                }
                Expr::Verbatim(expr_verbatim) => {
                    let ident = syn::parse2(expr_verbatim.clone().into_token_stream())?;
                    Ok(TableNameIdent(ident))
                }
                Expr::Path(expr_path) => {
                    let ident = expr_path
                        .path
                        .get_ident()
                        .ok_or_else(|| darling::Error::custom("Expected an identifier."))?;
                    Ok(TableNameIdent(ident.clone()))
                }
                _ => Err(darling::Error::custom("Expected a string literal.")),
            },
            _ => Err(darling::Error::custom("Unsupported format for table name.")),
        }
    }
}

impl TableNameIdent {
    pub(crate) fn validate_and_return(
        &self,
        struct_name_ident: &StructIdent,
        relax_table_name: &Option<bool>,
    ) -> ExtractorResult<&Self> {
        let table_name = self;
        let expected_table_name = struct_name_ident.to_string().to_case(Case::Snake);
        if !relax_table_name.unwrap_or(false) && self.to_string() != expected_table_name {
            return Err(syn::Error::new(
                table_name.span(),
                format!(
                    "table name must be in snake case of the current struct name. 
        Try: `{expected_table_name}`.
        
        If you don't want to follow this convention, use attribute `relax_table_name`. "
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
        let table_name = format_ident!("{}", self.0);
        table_name.to_tokens(tokens);
    }
}
