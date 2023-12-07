use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Result as SynResult, Token,
};

use super::{helpers::generate_variable_name, if_else::IfElseStatementAst};

#[derive(Debug, Clone)]
pub struct ReturnStatementParser {
    pub _return: Token![return],
    pub expr: ReturnExpr,
    // pub _end: Token![;],
    pub generated_ident: Ident,
}

#[derive(Debug, Clone)]
pub enum ReturnExpr {
    Expr(Expr),
    // Ident(Ident),
    IfElse(IfElseStatementAst),
}

impl ToTokens for ReturnExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Expr(expr) => expr.to_tokens(tokens),
            // Self::Ident(ident) => ident.to_tokens(tokens),
            Self::IfElse(if_else) => tokens.extend(quote! {
                 #if_else
            }),
        }
    }
}

impl Parse for ReturnExpr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![if]) {
            Ok(Self::IfElse(input.parse()?))
        }
        // else if lookahead.peek(Ident) {
        //     Ok(Self::Ident(input.parse()?))
        // }
        else {
            Ok(Self::Expr(input.parse()?))
        }
    }
}

impl Parse for ReturnStatementParser {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _return = input.parse::<Token![return]>()?;
        let expr = input.parse::<ReturnExpr>()?;
        if input.peek(Token![;]) {
            let _end = input.parse::<Token![;]>()?;
        }
        let generated_ident = generate_variable_name();

        Ok(Self {
            _return,
            expr,
            // _end: Token![;],
            generated_ident,
        })
    }
}
