/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use darling::FromMeta;
use quote::format_ident;

use crate::models::*;

use super::derive_attributes::StructIdent;

create_ident_wrapper!(TableNameIdent);

impl FromMeta for TableNameIdent {}

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
                "table name must be in snake case of the current struct name. 
        Try: `{expected_table_name}`.
        
        If you don't want to follow this convention, use attribute `relax_table_name`. ",
            )
            .into());
        };

        Ok(self)
    }

    pub(crate) fn as_string(&self) -> TableNameAsString {
        TableNameAsString(self.to_string())
    }
}

struct TableNameAsString(String);

impl ToTokens for TableNameAsString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let table_name = format_ident!("{}", self.0);
        table_name.to_tokens(tokens);
    }
}
