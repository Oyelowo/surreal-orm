use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::{
    generate_query_chain_code, generated_bound_query_chain, query_chain::QueriesChain,
    statement_or_expr::Query,
};

pub enum TransactionEnding {
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

pub(crate) struct Transaction {
    pub statements: Vec<Query>,
    pub transaction_ending: TransactionEnding,
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
