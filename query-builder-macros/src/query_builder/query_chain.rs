use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::statement_or_expr::StmtOrExpr;

pub(crate) struct QueriesChain {
    pub statements: Vec<StmtOrExpr>,
}

impl Parse for QueriesChain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();

        // this closure rather than direct assignment is necessary so we dont get stale result
        let is_transaction = |input: &ParseBuffer<'_>| {
            let input_str = input
                .to_string()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase();

            (input.peek(Ident) && input.peek2(Ident) && input.peek3(Token![;]))
                && (input_str.starts_with("begin transaction")
                    || input_str.starts_with("commit transaction")
                    || input_str.starts_with("cancel transaction"))
        };
        let is_last_return = |input: &ParseBuffer<'_>| input.peek(Token![return]);

        while !input.is_empty() && !is_last_return(input) && !is_transaction(input) {
            statements.push(input.parse()?);
        }

        Ok(QueriesChain { statements })
    }
}
