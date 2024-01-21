/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};

#[derive(Debug, Clone)]
pub struct DbfieldTypeToken(TokenStream);

impl Default for DbfieldTypeToken {
    fn default() -> Self {
        let crate_name = get_crate_name(false);
        Self(quote!(#crate_name::FieldType::Any))
    }
}

impl From<TokenStream> for DbfieldTypeToken {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}
impl ToTokens for DbfieldTypeToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

#[derive(Debug, Clone, Default)]
pub struct StaticAssertionToken(TokenStream);
impl ToTokens for StaticAssertionToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}
impl From<TokenStream> for StaticAssertionToken {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}
