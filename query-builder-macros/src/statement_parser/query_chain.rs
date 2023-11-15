use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use proc_macros_helpers::get_crate_name;

use super::{query::QueryParser, let_::LetStatementParser, helpers::generate_variable_name, return_::ReturnStatementParser, for_::tokenize_for_loop};

// use super::statement_or_expr::Query;

pub(crate) struct QueriesChainParser {
    pub statements: Vec<QueryParser>,
}

pub struct GeneratedCode {
    rendered: Vec<proc_macro2::TokenStream>,
    query_chain: Vec<proc_macro2::TokenStream>
}

impl QueriesChainParser {
    pub fn generate_code(self) -> GeneratedCode {
        let statements = self.statements;
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
                
            // let tokenized = tokenize_for_loop(for_loop_content.deref());
            let tokenized = tokenize_for_loop(for_loop_content);
            let query_chain: proc_macro2::TokenStream = tokenized.query_chain.into();
            let to_render: proc_macro2::TokenStream = tokenized.code_to_render.into();
                (
                quote!(
                    let #generated_ident = #query_chain;
                ),
                    quote!(#generated_ident)
                )
            },
            QueryParser::BeginTransaction => {
                let generated_ident = generate_variable_name();
                (
                quote! {
                    let ref #generated_ident = #crate_name::statements::begin_transaction();
                },
                    quote!(#generated_ident)
                )
            },
            QueryParser::CommitTransaction => {
                (
                quote! {
                    .commit_transaction();
                },
                    quote!()
                )
            },
            QueryParser::CancelTransaction => {
                (
                quote! {
                    .cancel_transaction();
                },
                    quote!()
                )
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
            rendered,
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
