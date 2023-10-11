/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use thiserror::Error;

pub(crate) fn validate_table_name<'a>(
    struct_name_ident: &proc_macro2::Ident,
    table_name: &'a Option<String>,
    relax_table_name: &Option<bool>,
) -> &'a String {
    let expected_table_name = struct_name_ident.to_string().to_case(Case::Snake);
    let table_name = table_name.as_ref().unwrap();
    if !relax_table_name.unwrap_or(false) && table_name != &expected_table_name {
        panic!(
            "table name must be in snake case of the current struct name. 
        Try: `{expected_table_name}`.
        
        If you don't want to follow this convention, use attribute `relax_table_name`. "
        );
    };

    table_name
}

#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("{0}")]
    Syn(#[from] syn::Error),

    #[error("{0}")]
    Darling(#[from] darling::Error),
}

impl ExtractorError {
    pub fn write_errors(self) -> proc_macro2::TokenStream {
        match self {
            ExtractorError::Syn(err) => err.to_compile_error(),
            ExtractorError::Darling(err) => err.write_errors(),
        }
    }
}

pub type ExtractorResult<T> = std::result::Result<T, ExtractorError>;
