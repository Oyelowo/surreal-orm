/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::statement_parser::query_chain::QueriesChainParser;

pub fn query_turbo(input: TokenStream) -> TokenStream {
    let queries_chain = parse_macro_input!(input as QueriesChainParser);
    queries_chain.to_tokenstream().into()
}
