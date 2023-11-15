use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use crate::query_builder::generate_variable_name;

pub struct ReturnStatementParser {
    pub _return: Token![return],
    pub expr: Expr,
    pub _end: Token![;],
    pub generated_ident: Ident,
}

impl Parse for ReturnStatementParser {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _return = input.parse::<Token![return]>()?;
        let expr = input.parse::<Expr>()?;
        let _end = input.parse::<Token![;]>()?;
        let generated_ident = generate_variable_name();
        Ok(Self {
            _return,
            expr,
            _end,
            generated_ident,
        })
    }
}
