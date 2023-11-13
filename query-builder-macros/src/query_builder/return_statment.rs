use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

pub(crate) struct ReturnStatement {
    pub(crate) _return: Token![return],
    pub(crate) expr: Expr,
    pub _ending: Token![;],
}

impl Parse for ReturnStatement {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _return: Token![return] = input.parse()?;
        let expr = input.parse::<Expr>()?;
        let _ending: Token![;] = input.parse()?;

        Ok(ReturnStatement {
            _return,
            expr,
            _ending,
        })
    }
}
