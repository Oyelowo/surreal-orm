/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use proc_macro2::Ident;
use quote::format_ident;
use syn::spanned::Spanned;

use crate::errors::ExtractorResult;

pub(crate) fn validate_table_name<'a>(
    struct_name_ident: &proc_macro2::Ident,
    table_name: &'a Option<Ident>,
    relax_table_name: &Option<bool>,
) -> ExtractorResult<Ident> {
    let expected_table_name = struct_name_ident.to_string().to_case(Case::Snake);
    let table_name = table_name
        .as_ref()
        .expect("table name must be provided")
        .to_string();
    if !relax_table_name.unwrap_or(false) && table_name != expected_table_name {
        return Err(syn::Error::new(
            table_name.span(),
            "table name must be in snake case of the current struct name. 
        Try: `{expected_table_name}`.
        
        If you don't want to follow this convention, use attribute `relax_table_name`. ",
        )
        .into());
    };

    Ok(format_ident!("{table_name}"))
}
