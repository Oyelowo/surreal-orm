/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

use convert_case::{Case, Casing};

use crate::models::*;

create_ident_wrapper!(TableNameIdent);

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
