use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use crate::models::get_crate_name;

struct LetStatement {
    ident: Ident,
    _eq: Token![=],
    expr: Expr,
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

enum StmtOrExpr {
    Statement(LetStatement),
    Expr(Expr),
}

impl Parse for StmtOrExpr {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![let]) {
            let var_statement = input.parse::<LetStatement>()?;
            Ok(StmtOrExpr::Statement(var_statement))
        } else {
            let expr = input.parse::<Expr>()?;
            let _end: Token![;] = input.parse()?;
            Ok(StmtOrExpr::Expr(expr))
        }
    }
}

struct QueriesChain {
    statements: Vec<StmtOrExpr>,
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

fn generate_query_chain_code(statements: &Vec<StmtOrExpr>) -> Vec<proc_macro2::TokenStream> {
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

struct ReturnStatement {
    _return: Token![return],
    expr: Expr,
    _ending: Token![;],
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

struct Block {
    statements: Vec<StmtOrExpr>,
    return_expr: Expr,
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

enum TransactionEnding {
    Commit,
    Cancel,
}

impl Parse for TransactionEnding {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let ident = input.parse::<Ident>()?;
        let transaction_type = if ident.to_string().to_lowercase() == "commit" {
            Ok(TransactionEnding::Commit)
        } else if ident.to_string().to_lowercase() == "cancel" {
            Ok(TransactionEnding::Cancel)
        } else {
            Err(syn::Error::new_spanned(
                ident,
                "expected 'commit' or 'cancel' in lower or upper case",
            ))
        };

        let transaction_ident = input.parse::<Ident>()?;
        if transaction_ident.to_string().to_lowercase() != "transaction" {
            return Err(syn::Error::new_spanned(
                transaction_ident,
                "expected 'transaction' or 'TRANSACTION'",
            ));
        }

        input.parse::<Token![;]>()?;

        transaction_type
    }
}

struct Transaction {
    statements: Vec<StmtOrExpr>,
    transaction_ending: TransactionEnding,
}

struct BeginTransactionStatement;

impl Parse for BeginTransactionStatement {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let begin_statement = input.parse::<Ident>()?;
        if begin_statement.to_string().to_lowercase() != "begin" {
            return Err(syn::Error::new_spanned(
                begin_statement,
                "expected 'begin' or 'BEGIN'",
            ));
        }

        let statement_ident = input.parse::<Ident>()?;
        if statement_ident.to_string().to_lowercase() != "transaction" {
            return Err(syn::Error::new_spanned(
                statement_ident,
                "expected 'transaction' or 'TRANSACTION'",
            ));
        }
        input.parse::<Token![;]>()?;
        Ok(Self)
    }
}

impl Parse for Transaction {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input.parse::<BeginTransactionStatement>()?;
        let queries_input = input.parse::<QueriesChain>()?;
        let transaction_ending = input.parse::<TransactionEnding>()?;

        Ok(Transaction {
            statements: queries_input.statements,
            transaction_ending,
        })
    }
}

pub fn query_transaction(input: TokenStream) -> TokenStream {
    let Transaction {
        statements,
        transaction_ending,
    } = parse_macro_input!(input as Transaction);

    let crate_name = get_crate_name(false);

    let generated_code = generate_query_chain_code(&statements);

    let query_chain = generated_bound_query_chain(&statements);

    let ending = match transaction_ending {
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

fn generated_bound_query_chain(statements: &Vec<StmtOrExpr>) -> Vec<proc_macro2::TokenStream> {
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
