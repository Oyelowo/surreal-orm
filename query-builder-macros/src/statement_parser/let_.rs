use std::fmt::Display;

use super::return_::ReturnExpr;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

#[derive(Debug, Clone)]
pub(crate) struct LetStatementParser {
    pub ident: Ident,
    pub _eq: Token![=],
    pub expr: ReturnExpr,
}

impl Parse for LetStatementParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _let: Token![let] = input.parse()?;
        let ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;

        let expr = input.parse()?;
        let is_if_else = matches!(expr, ReturnExpr::IfElse(_));

        let let_statement = LetStatementParser { ident, _eq, expr };
        // if else already has a semicolon
        if !is_if_else {
            let _semi: Token![;] = input.parse()?;
        }
        Ok(let_statement)
    }
}
