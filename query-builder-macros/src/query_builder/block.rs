use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::statement_parser::query_chain::QueriesChainParser;

pub fn query_block(input: TokenStream) -> TokenStream {
    let queries_chain = parse_macro_input!(input as QueriesChainParser);
    let token: proc_macro2::TokenStream = queries_chain.to_tokenstream().into();
    if !queries_chain.is_likely_query_block() {
        return syn::Error::new_spanned(token, "query_block! macro must return an expression.")
            .into_compile_error()
            .into();
    }

    token.into()
}
