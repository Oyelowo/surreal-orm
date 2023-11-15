use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::{for_loop::ForLoop, generate_variable_name};

pub(crate) struct LetStatement {
    pub ident: Ident,
    pub _eq: Token![=],
    pub expr: Expr,
}

impl Parse for LetStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _let: Token![let] = input.parse()?;
        let let_statement = LetStatement {
            ident: input.parse()?,
            _eq: input.parse()?,
            expr: input.parse()?,
        };
        let _semi: Token![;] = input.parse()?;
        Ok(let_statement)
    }
}

pub(crate) enum StmtOrExpr {
    Statement(LetStatement),
    Expr {
        generated_ident: Ident,
        expr: Expr,
    },
    ForLoop {
        generated_ident: Ident,
        for_loop: ForLoop,
    },
}

impl Parse for StmtOrExpr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![let]) {
            let var_statement = input.parse::<LetStatement>()?;
            Ok(StmtOrExpr::Statement(var_statement))
        } else if input.peek(Token![for]) {
            let for_loop = input.parse::<ForLoop>()?;
            Ok(StmtOrExpr::ForLoop {
                for_loop,
                generated_ident: generate_variable_name(),
            })
        } else {
            let expr = input.parse::<Expr>()?;
            let _end: Token![;] = input.parse()?;
            Ok(StmtOrExpr::Expr {
                generated_ident: generate_variable_name(),
                expr,
            })
        }
    }
}
