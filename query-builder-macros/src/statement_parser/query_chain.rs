use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::query::QueryParser;

// use super::statement_or_expr::Query;

pub(crate) struct QueriesChain {
    pub statements: Vec<QueryParser>,
}

impl Parse for QueriesChain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();

        while !input.is_empty() {
            statements.push(input.parse()?);
        }

        Ok(QueriesChain { statements })
    }
}
