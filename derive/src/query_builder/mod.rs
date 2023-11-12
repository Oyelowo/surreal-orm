use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    token::Abstract,
    Expr, Ident, Lit, LitStr, Result as SynResult, Token,
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

enum Query {
    Statement(LetStatement),
    Expr(Expr),
}

impl Parse for Query {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![let]) {
            let var_statement = input.parse::<LetStatement>()?;
            Ok(Query::Statement(var_statement))
        } else {
            let expr = input.parse::<Expr>()?;
            let _end: Token![;] = input.parse()?;
            Ok(Query::Expr(expr))
        }
    }
}

struct QueriesInput {
    statements: Vec<Query>,
}

impl Parse for QueriesInput {
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

        // while !input.is_empty() {
        //     // Peek the next token, if it's an identifier, check if it's the specific keyword we're looking for
        //     if input.peek(Ident) {
        //         let ident: Ident = input.parse()?;
        //         if ident == "begin" {
        //             // Handle the 'begin' keyword
        //             // You can add your logic here for what should happen when 'begin' is encountered
        //             // ...
        //             break;
        //         } else if ident == "return" {
        //             // Handle the 'return' keyword or break the loop, as per your requirement
        //             break;
        //         } else {
        //             // If it's not 'begin' or 'return', parse other parts of the input normally
        //             statements.push(input.parse()?);
        //         }
        //     } else {
        //         // If the next token is not an identifier, parse the next part of the input normally
        //         statements.push(input.parse()?);
        //     }
        // }
        Ok(QueriesInput { statements })
    }
}

pub fn query_turbo(input: TokenStream) -> TokenStream {
    let QueriesInput { statements } = parse_macro_input!(input as QueriesInput);
    let crate_name = get_crate_name(false);

    let generated_code = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        Query::Statement(var_statement) => {
            let LetStatement { ident, expr, .. } = var_statement;
            quote! {
                let ref #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
            }
        }
        Query::Expr(expr) => quote! {
            #expr;
        },
    });

    let query_chain = statements
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_first = i == 0;
            let to_chain = match s {
                Query::Statement(LetStatement { ident, .. }) => quote!(#ident),
                Query::Expr(expr) => quote!(#expr),
            };

            if is_first {
                quote!(#crate_name::chain(#to_chain))
            } else {
                quote!(.chain(#to_chain))
            }
        })
        .collect::<Vec<_>>();

    quote! {
        {
        #( #generated_code )*

        #( #query_chain )*
        }
    }
    .into()
}

struct Block {
    statements: Vec<Query>,
    return_expr: Expr,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut queries_input = input.parse::<QueriesInput>()?;
        let return_token: Token![return] = input.parse()?;
        let return_expr = input.parse::<Expr>()?;
        let _ending_colon = input.parse::<Token![;]>();

        Ok(Block {
            statements: queries_input.statements,
            return_expr,
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

    let generated_code = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        Query::Statement(var_statement) => {
            let LetStatement { ident, expr, .. } = var_statement;
            quote! {
                let ref #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
            }
        }
        Query::Expr(expr) => quote! {
            #expr;
        },
    });

    let query_chain = statements
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_first = i == 0;
            let to_chain = match s {
                Query::Statement(LetStatement { ident, .. }) => quote!(#ident),
                Query::Expr(expr) => quote!(#expr),
            };

            if is_first {
                quote!(#crate_name::chain(#to_chain))
            } else {
                quote!(.chain(#to_chain))
            }
        })
        .collect::<Vec<_>>();

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
    begin_statement: bool,
    statements: Vec<Query>,
    transaction_ending: TransactionEnding,
}

impl Parse for Transaction {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let begin_statement = input.parse::<Ident>()?;
        if begin_statement.to_string().to_lowercase() != "begin" {
            return Err(syn::Error::new_spanned(
                begin_statement,
                "expected 'begin' or 'BEGIN'",
            ));
        }
        // input.parse::<Abstract>()?;
        let statement_ident = input.parse::<Ident>()?;
        if statement_ident.to_string().to_lowercase() != "transaction" {
            return Err(syn::Error::new_spanned(
                statement_ident,
                "expected 'transaction' or 'TRANSACTION'",
            ));
        }
        input.parse::<Token![;]>()?;

        let mut queries_input = input.parse::<QueriesInput>()?;
        let transaction_ending = input.parse::<TransactionEnding>()?;

        Ok(Transaction {
            begin_statement: true,
            statements: queries_input.statements,
            transaction_ending,
        })
    }
}

pub fn query_transaction(input: TokenStream) -> TokenStream {
    let Transaction {
        begin_statement,
        statements,
        transaction_ending,
    } = parse_macro_input!(input as Transaction);

    let crate_name = get_crate_name(false);

    let generated_code = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        Query::Statement(var_statement) => {
            let LetStatement { ident, expr, .. } = var_statement;
            quote! {
                let ref #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
            }
        }
        Query::Expr(expr) => quote! {
            #expr;
        },
    });

    let query_chain = statements
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_first = i == 0;
            let to_chain = match s {
                Query::Statement(LetStatement { ident, .. }) => quote!(#ident),
                Query::Expr(expr) => quote!(#expr),
            };

            if is_first {
                quote!(#crate_name::chain(#to_chain))
            } else {
                quote!(.chain(#to_chain))
            }
        })
        .collect::<Vec<_>>();

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
