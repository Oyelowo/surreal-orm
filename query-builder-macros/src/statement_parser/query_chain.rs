use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use crate::query_builder::tokenizer::QueryTypeToken;

use super::{query::QueryParser, let_::LetStatementParser, helpers::generate_variable_name, return_::ReturnStatementParser};


pub struct QueriesChainParser {
    pub statements: Vec<QueryParser>,
}

impl QueriesChainParser {
    pub fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let crate_name = get_crate_name(false);
        let tokenizer: QueryTypeToken = self.into();
        tokenizer.get_tokenstream()
    }
    
    pub fn is_valid_transaction_statement(&self) -> bool {
        let mut stmts = self.statements.iter().filter(|stmt| stmt.is_transaction_stmt());

        let begin_stmt = stmts.next();
        let commit_or_cancel_stmt = stmts.next();
        let ending = stmts.next();
        
        match (begin_stmt, commit_or_cancel_stmt, ending) {
            (Some(QueryParser::BeginTransaction), Some(QueryParser::CommitTransaction), None) => true,
            (Some(QueryParser::BeginTransaction), Some(QueryParser::CancelTransaction), None) => true,
            _ => false
        }
    }

    pub fn is_definitely_query_block(&self) -> bool {
        let mut last_stmt = self.statements.last();

        last_stmt.map_or(false, |s|s.is_return_statement())
    }

    pub fn is_likely_query_block(&self) -> bool {
         self.statements.iter().any(|s| s.is_return_statement())
    }
}

pub struct GeneratedCode {
    pub to_render: Vec<proc_macro2::TokenStream>,
    pub query_chain: Vec<proc_macro2::TokenStream>
}

impl QueriesChainParser {
    pub fn generate_code(&self) -> GeneratedCode {
        let statements = &self.statements;
        let crate_name = get_crate_name(false);
        

        let (rendered, query_chain) : (Vec<_>, Vec<_>)= statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        QueryParser::LetStatement(var_statement) => {
            let LetStatementParser { ident, expr, .. } = var_statement;
            (quote! {
                let ref #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
            }, quote!(#ident))
        }
        QueryParser::Expr {expr, generated_ident} => (
                quote! (let #generated_ident = #expr;),
                quote!(#generated_ident)
            ),
        QueryParser::ForLoop(for_loop_parser) => {
            let for_loop_content = &for_loop_parser.meta_content;
            let generated_ident = &for_loop_parser.generated_ident;
                
            let tokenized = for_loop_content.tokenize();
            let query_chain: proc_macro2::TokenStream = tokenized.query_chain.into();
            let to_render: proc_macro2::TokenStream = tokenized.code_to_render.into();
                (
                quote!(
                    let #generated_ident = #query_chain;
                ),
                    quote!(#generated_ident)
                )
            },
            QueryParser::IfEsle(if_else_meta) => {
                let tokenized = if_else_meta.tokenize();
                let query_chain: proc_macro2::TokenStream = tokenized.query_chain.into();
                let to_render: proc_macro2::TokenStream = tokenized.code_to_render.into();
                let generated_ident = &if_else_meta.generated_ident;
                (
                quote!(
                    let #generated_ident = #query_chain;
                ),
                    quote!(#generated_ident)
                )
                
            },
            QueryParser::BeginTransaction => {
                (quote!(), quote!())
            },
            QueryParser::CommitTransaction => {
                (quote!(), quote!())
            },
            QueryParser::CancelTransaction => {
                (quote!(), quote!())
            },
            QueryParser::ReturnStatement(r_stmt) => {
                let ReturnStatementParser { expr, generated_ident, _return, _end} = r_stmt;
                
                (
                quote! {
                    let #generated_ident = #crate_name::statements::return_(#expr);
                },
                    quote!(#generated_ident)
                )
            },
            QueryParser::BreakStatement => {
                let generated_ident = generate_variable_name();
                (
                quote! {
                    let ref #generated_ident = #crate_name::statements::break_();
                },
                    quote!(#generated_ident)
                )
            },
            QueryParser::ContinueStatement => {
                let generated_ident = generate_variable_name();
                (
                quote! {
                    let ref #generated_ident = #crate_name::statements::continue_();
                },
                    quote!(#generated_ident)
                )
            },
    }).unzip();

    let query_chain = query_chain
        .iter()
        .filter(| var_ident| !var_ident.is_empty())
        .enumerate()
        .map(|(i, var_ident)| {
            let is_first = i == 0;

            if is_first {
                quote!(#crate_name::chain(#var_ident))
            } else {
                quote!(.chain(#var_ident))
            }
        })
        .collect::<Vec<_>>();

        GeneratedCode {
            to_render: rendered,
            query_chain
        }
    }
    
}

impl Parse for QueriesChainParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();

        while !input.is_empty() {
            statements.push(input.parse()?);
        }

        Ok(QueriesChainParser { statements })
    }
}
