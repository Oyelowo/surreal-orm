pub(crate) mod block;
pub(crate) mod for_loop;
pub(crate) mod query_chain;
pub(crate) mod query_turbo;
pub(crate) mod return_statment;
pub(crate) mod statement_or_expr;
pub(crate) mod transaction;

pub use block::query_block;
pub use for_loop::for_loop;
pub use query_turbo::query_turbo;
pub use transaction::query_transaction;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use crate::query_builder::statement_or_expr::LetStatement;

use self::statement_or_expr::StmtOrExpr;

pub(crate) fn generate_query_chain_code(
    statements: &Vec<StmtOrExpr>,
) -> Vec<proc_macro2::TokenStream> {
    let crate_name = get_crate_name(false);

    let generated_code = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        StmtOrExpr::Statement(var_statement) => {
            let LetStatement { ident, expr, .. } = var_statement;
            quote! {
                let ref #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
            }
        }
        StmtOrExpr::Expr(expr) => quote! {
            #expr;
        },
    }).collect::<Vec<_>>();

    generated_code
}

pub(crate) fn generated_bound_query_chain(
    statements: &Vec<StmtOrExpr>,
) -> Vec<proc_macro2::TokenStream> {
    let crate_name = get_crate_name(false);

    let query_chain = statements
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_first = i == 0;
            let to_chain = match s {
                StmtOrExpr::Statement(LetStatement { ident, .. }) => quote!(#ident),
                StmtOrExpr::Expr(expr) => quote!(#expr),
            };

            if is_first {
                quote!(#crate_name::chain(#to_chain))
            } else {
                quote!(.chain(#to_chain))
            }
        })
        .collect::<Vec<_>>();
    query_chain
}
