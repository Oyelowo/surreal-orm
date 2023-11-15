use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

pub struct BeginTransactionStatementParser;

impl Parse for BeginTransactionStatementParser {
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

pub struct CommitTransactionStatementParser;

impl Parse for CommitTransactionStatementParser {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let commit_statement = input.parse::<Ident>()?;
        if commit_statement.to_string().to_lowercase() != "commit" {
            return Err(syn::Error::new_spanned(
                commit_statement,
                "expected 'commit' or 'COMMIT'",
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

pub struct CancelTransactionStatementParser;

impl Parse for CancelTransactionStatementParser {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let cancel_statement = input.parse::<Ident>()?;
        if cancel_statement.to_string().to_lowercase() != "cancel" {
            return Err(syn::Error::new_spanned(
                cancel_statement,
                "expected 'cancel' or 'CANCEL'",
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
