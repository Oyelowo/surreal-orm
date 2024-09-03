/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("{0}")]
    Syn(#[from] syn::Error),

    #[error("{0}")]
    Darling(#[from] darling::Error),
}

impl ExtractorError {
    #[allow(dead_code)]
    pub fn write_errors(self) -> proc_macro2::TokenStream {
        match self {
            ExtractorError::Syn(err) => err.to_compile_error(),
            ExtractorError::Darling(err) => err.write_errors(),
        }
    }
}

pub type ExtractorResult<T> = std::result::Result<T, ExtractorError>;
