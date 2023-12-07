use super::return_::ReturnExpr;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

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
