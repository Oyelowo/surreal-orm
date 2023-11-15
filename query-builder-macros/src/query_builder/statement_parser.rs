use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::generate_variable_name;

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

struct ReturnStatement {
    pub _return: Token![return],
    pub expr: Expr,
    pub _end: Token![;],
    pub generated_ident: Ident,
}

impl Parse for ReturnStatement {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _return = input.parse::<Token![return]>()?;
        let expr = input.parse::<Expr>()?;
        let _end = input.parse::<Token![;]>()?;
        let generated_ident = generate_variable_name();
        Ok(Self {
            _return,
            expr,
            _end,
            generated_ident,
        })
    }
}

pub struct BeginTransactionStatement;

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

pub struct CommitTransactionStatement;

impl Parse for CommitTransactionStatement {
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

pub struct CancelTransactionStatement;

impl Parse for CancelTransactionStatement {
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
