use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::{
    block::Block,
    generate_query_chain_code, generated_bound_query_chain,
    query_chain::QueriesChain,
    return_statment::ReturnStatement,
    transaction::{BeginTransactionStatement, Transaction, TransactionEnding},
};

enum QueryType {
    Chain,
    Block,
    Transaction,
}

enum QueryTypeInner {
    Chain(QueriesChain),
    Block(Block),
    Transaction(Transaction),
}

struct QueriesTurbo {
    inner: QueryTypeInner,
}

impl Parse for QueriesTurbo {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let is_likely_transaction_stmt =
            input.peek(Ident) && input.peek2(Ident) && input.peek3(Token![;]);

        if is_likely_transaction_stmt {
            input.parse::<BeginTransactionStatement>()?;
        }

        let queries_chain = input.parse::<QueriesChain>()?;

        let is_likely_block = |input: &ParseBuffer<'_>| input.peek(Token![return]);

        if is_likely_block(input) {
            let return_stmt = input.parse::<ReturnStatement>()?;
            return Ok(QueriesTurbo {
                inner: QueryTypeInner::Block(Block {
                    statements: queries_chain.statements,
                    return_expr: return_stmt.expr,
                }),
            });
        } else if is_likely_transaction_stmt {
            let ending = input.parse::<TransactionEnding>()?;

            return Ok(QueriesTurbo {
                inner: QueryTypeInner::Transaction(Transaction {
                    statements: queries_chain.statements,
                    transaction_ending: ending,
                }),
            });
        }

        Ok(QueriesTurbo {
            inner: QueryTypeInner::Chain(queries_chain),
        })
    }
}

pub fn query_turbo(input: TokenStream) -> TokenStream {
    let QueriesTurbo { inner } = parse_macro_input!(input as QueriesTurbo);
    let crate_name = get_crate_name(false);

    let bound_queries = match inner {
        QueryTypeInner::Chain(chain) => {
            let statements = chain.statements;
            let generated_code = generate_query_chain_code(&statements);
            let query_chain = generated_bound_query_chain(&statements);

            quote! {
                {
                #( #generated_code )*

                #( #query_chain )*
                }
            }
        }
        QueryTypeInner::Block(block) => {
            let statements = block.statements;
            let generated_code = generate_query_chain_code(&statements);
            let query_chain = generated_bound_query_chain(&statements);

            let has_only_return = statements.len() == 0;
            let return_expr = block.return_expr;

            let chained_bound_stmts = if has_only_return {
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

                    #chained_bound_stmts
                }
            }
            .into()
        }
        QueryTypeInner::Transaction(transaction) => {
            let statements = transaction.statements;
            let generated_code = generate_query_chain_code(&statements);
            let query_chain = generated_bound_query_chain(&statements);

            let ending = match transaction.transaction_ending {
                TransactionEnding::Commit => quote!(.commit_transaction()),
                TransactionEnding::Cancel => quote!(.cancel_transaction()),
            };
            let transaction = quote! {
                #crate_name::statements::begin_transaction()
                .query(#( #query_chain) *)
                #ending
            };

            quote! {
                {
                    #( #generated_code )*

                    #transaction

                }
            }
            .into()
        }
    };

    bound_queries.into()
}
