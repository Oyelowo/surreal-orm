use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::{
    generate_query_chain_code, generated_bound_query_chain, query_chain::QueriesChain,
    return_statment::ReturnStatement, statement_or_expr::Query,
};

pub(crate) struct Block {
    pub statements: Vec<Query>,
    pub return_expr: Expr,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let queries_input = input.parse::<QueriesChain>()?;
        let return_stmt = input.parse::<ReturnStatement>()?;

        Ok(Block {
            statements: queries_input.statements,
            return_expr: return_stmt.expr,
        })
    }
}

// ends with a return statement;
pub fn query_block(input: TokenStream) -> TokenStream {
    let Block {
        statements,
        return_expr,
    } = parse_macro_input!(input as Block);

    let crate_name = get_crate_name(false);

    let generated_code = generate_query_chain_code(&statements);
    let query_chain = generated_bound_query_chain(&statements);

    let has_only_return = statements.len() == 0;
    let whole_stmts = if has_only_return {
        quote!(#crate_name::statements::return_(#return_expr).as_block())
    } else {
        quote!(
                #( #query_chain )*
                .chain(#crate_name::statements::return_(#return_expr)).as_block()
        )
    };

    quote! {
        {
            #( #generated_code )*

            #whole_stmts
        }
    }
    .into()
}
