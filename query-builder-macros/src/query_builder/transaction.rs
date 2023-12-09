/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::statement_parser::query_chain::QueriesChainParser;

pub fn query_transaction(input: TokenStream) -> TokenStream {
    let queries_chain = parse_macro_input!(input as QueriesChainParser);
    let token: proc_macro2::TokenStream = queries_chain.to_tokenstream();
    if !queries_chain.is_valid_transaction_statement() {
        return syn::Error::new_spanned(
            token,
            "query_transaction! macro can only be used with transaction queries",
        )
        .into_compile_error()
        .into();
    }

    token.into()
}
