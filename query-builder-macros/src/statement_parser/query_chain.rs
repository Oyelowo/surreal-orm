use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};

use proc_macros_helpers::get_crate_name;

use crate::query_builder::tokenizer::QueryTypeToken;

use super::{
    helpers::generate_variable_name, let_::LetStatementParser, query::QueryParser,
    return_::ReturnStatementParser,
};

#[derive(Debug, Clone)]
pub struct QueriesChainParser {
    pub statements: Vec<QueryParser>,
}

impl QueriesChainParser {
    pub fn to_tokenstream(&self) -> proc_macro2::TokenStream {
        let tokenizer: QueryTypeToken = self.into();
        tokenizer.get_tokenstream()
    }

    pub fn is_valid_transaction_statement(&self) -> bool {
        let mut stmts = self
            .statements
            .iter()
            .filter(|stmt| stmt.is_transaction_stmt());

        let begin_stmt = stmts.next();
        let commit_or_cancel_stmt = stmts.next();
        let ending = stmts.next();

        match (begin_stmt, commit_or_cancel_stmt, ending) {
            (
                Some(QueryParser::BeginTransaction),
                Some(QueryParser::CommitTransaction | QueryParser::CancelTransaction),
                None,
            ) => true,
            _ => false,
        }
    }

    pub fn _is_definitely_query_block(&self) -> bool {
        let last_stmt = self.statements.last();

        last_stmt.map_or(false, |s| s.is_return_statement())
    }

    pub fn is_likely_query_block(&self) -> bool {
        self.statements.iter().any(|s| s.is_return_statement())
    }
}

pub struct GeneratedCode {
    pub to_render: Vec<proc_macro2::TokenStream>,
    pub query_chain_var_ident: Ident,
}

struct QueryLine {
    pub rendered: proc_macro2::TokenStream,
    pub var_ident: proc_macro2::TokenStream,
}

impl QueriesChainParser {
    pub fn generate_code(&self) -> GeneratedCode {
        let statements = &self.statements;
        let crate_name = get_crate_name(false);

        let query_chain_var_name = format_ident!("query_chain_{}", generate_variable_name());
        let lines = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        QueryParser::LetStatement(var_statement) => {
            let LetStatementParser { ident, expr, .. } = var_statement;
            Some(QueryLine {
                rendered: quote! (
                    let ref #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
                ),
                var_ident: quote!(#ident)
            })
        }
        QueryParser::Expr {expr, generated_ident} => Some(
                QueryLine {
                    rendered: quote! (
                        let #generated_ident = #expr;
                    ),
                    var_ident: quote!(#generated_ident)
                }
            ),
        QueryParser::ForLoop(for_loop_parser) => {
            let for_loop_content = &for_loop_parser.meta_content;
            let generated_ident = &for_loop_parser.generated_ident;
            let tokenized = for_loop_content.tokenize();
            let query_chain: proc_macro2::TokenStream = tokenized.query_chain.into();
                Some(QueryLine {
                    rendered: quote! (
                        let #generated_ident = #query_chain;
                    ),
                    var_ident: quote!(#generated_ident)
                })
            },
            QueryParser::IfEsle(if_else_meta) => {
                let tokenized = if_else_meta.meta_content.tokenize();
                let query_chain: proc_macro2::TokenStream = tokenized.query_chain.into();
                let generated_ident = &if_else_meta.meta_content.generated_ident;
                Some(
                    QueryLine {
                        rendered: quote! (
                            let #generated_ident = #query_chain;
                        ),
                        var_ident: quote!(#generated_ident)
                    }
                )
            },
            QueryParser::BeginTransaction | QueryParser::CommitTransaction | QueryParser::CancelTransaction => {
                None
            },
            QueryParser::ReturnStatement(r_stmt) => {
                let ReturnStatementParser { expr, generated_ident, _return, } = r_stmt;
                Some(
                    QueryLine {
                        rendered: quote! (
                            let #generated_ident = #crate_name::statements::return_(#expr);
                        ),
                        var_ident: quote!(#generated_ident)
                    }
                )
            },
            QueryParser::BreakStatement => {
                let generated_ident = generate_variable_name();
                Some(
                    QueryLine {
                        rendered: quote! (
                            let #generated_ident = #crate_name::statements::break_();
                        ),
                        var_ident: quote!(#generated_ident)
                    }
                )
            },
            QueryParser::ContinueStatement => {
                let generated_ident = generate_variable_name();
                Some(
                    QueryLine {
                        rendered: quote! (
                            let #generated_ident = #crate_name::statements::continue_();
                        ),
                        var_ident: quote!(#generated_ident)
                    }
                )
            },
   });

        let code = lines
            .filter(Option::is_some)
            .enumerate()
            .map(|(i, line)| {
                let QueryLine {
                    rendered,
                    var_ident,
                } = line.expect("Nonempty line should not be None");
                let is_first = i == 0;

                if is_first {
                    quote!(
                        #rendered
                        let #query_chain_var_name  = #crate_name::chain(#var_ident);
                    )
                } else {
                    quote!(
                        #rendered
                        let #query_chain_var_name  = #query_chain_var_name.chain(#var_ident);
                    )
                }
            })
            .collect::<Vec<_>>();

        GeneratedCode {
            to_render: code,
            query_chain_var_ident: query_chain_var_name,
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
