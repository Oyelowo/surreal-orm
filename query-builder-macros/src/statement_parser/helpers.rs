use proc_macro::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::quote;

use crate::{statement_parser::{for_::{ForLoopStatementParser, tokenize_for_loop}, let_::LetStatementParser, return_::ReturnStatementParser}, query_builder::generate_variable_name};

use super::query::QueryParser;

trait Tokenizer {
    fn generate_rendered_code(self) -> Vec<proc_macro2::TokenStream>;

    fn generate_chained_code(self) -> Vec<proc_macro2::TokenStream>;
}

impl Tokenizer for &Vec<QueryParser> {
    fn generate_rendered_code(self) -> Vec<proc_macro2::TokenStream> {
        let statements = self;
        let crate_name = get_crate_name(false);

        let generated_code = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        QueryParser::LetStatement(var_statement) => {
            let LetStatementParser { ident, expr, .. } = var_statement;
            quote! {
                let ref #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
            }
        }
        QueryParser::Expr {expr, generated_ident} => quote! {
            let #generated_ident = #expr;
        },
        QueryParser::ForLoop(for_loop_parser) => {
            let for_loop_content = &for_loop_parser.meta_content;
            let generated_ident = &for_loop_parser.generated_ident;
            // let meta_content = &for_loop_parser.meta_content;
                
            // let tokenized = tokenize_for_loop(for_loop_content.deref());
            let tokenized = tokenize_for_loop(for_loop_content);
            let query_chain: proc_macro2::TokenStream = tokenized.query_chain.into();
            let to_render: proc_macro2::TokenStream = tokenized.code_to_render.into();
                quote!(
                    let #generated_ident = #query_chain;
                )
            },
            QueryParser::BeginTransaction => {
                let generated_ident = generate_variable_name();
                quote! {
                    let ref #generated_ident = #crate_name::statements::begin_transaction();
                }
            },
            QueryParser::CommitTransaction => {
                quote! {
                    .commit_transaction();
                }
            },
            QueryParser::CancelTransaction => {
                quote! {
                    .cancel_transaction();
                }
            },
            QueryParser::ReturnStatement(r_stmt) => {
                let ReturnStatementParser { expr, generated_ident, _return, _end} = r_stmt;
                quote! {
                    let #generated_ident = #crate_name::statements::return_(#expr);
                }
            },
            QueryParser::BreakStatement => {
                let generated_ident = generate_variable_name();
                quote! {
                    let ref #generated_ident = #crate_name::statements::break_();
                }
            },
            QueryParser::ContinueStatement => {
                let generated_ident = generate_variable_name();
                quote! {
                    let ref #generated_ident = #crate_name::statements::continue_();
                }
            },
    }).collect::<Vec<_>>();

        generated_code
    }

    fn generate_chained_code(self) -> Vec<proc_macro2::TokenStream> {
        todo!()
    }
}

// pub(crate) fn generate_query_chain_code(
//     statements: &Vec<Query>,
// ) -> Vec<proc_macro2::TokenStream> {
//     let crate_name = get_crate_name(false);
//
//     let generated_code = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
//         Query::LetStatement(var_statement) => {
//             let LetStatement { ident, expr, .. } = var_statement;
//             quote! {
//                 let ref #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
//             }
//         }
//         Query::Expr {expr, generated_ident} => quote! {
//             let #generated_ident = #expr;
//         },
//         Query::ForLoop {for_loop: for_loop_content, generated_ident} => {
//             let tokenized = tokenize_for_loop(for_loop_content.deref());
//             let query_chain: proc_macro2::TokenStream = tokenized.query_chain.into();
//             let to_render: proc_macro2::TokenStream = tokenized.code_to_render.into();
//             quote!(
//
//             let #generated_ident = #query_chain;
//         )
//         },
//     }).collect::<Vec<_>>();
//
//     generated_code
// }
//
// pub(crate) fn generated_bound_query_chain(
//     statements: &Vec<Query>,
// ) -> Vec<proc_macro2::TokenStream> {
//     let crate_name = get_crate_name(false);
//
//     let query_chain = statements
//         .iter()
//         .enumerate()
//         .map(|(i, s)| {
//             let is_first = i == 0;
//             let to_chain = match s {
//                 Query::LetStatement(LetStatement { ident, .. }) => quote!(#ident),
//                 Query::Expr{generated_ident, ..} => quote!(#generated_ident),
//                 Query::ForLoop{generated_ident, ..} => quote!(#generated_ident),
//             };
//
//             if is_first {
//                 quote!(#crate_name::chain(#to_chain))
//             } else {
//                 quote!(.chain(#to_chain))
//             }
//         })
//         .collect::<Vec<_>>();
//     query_chain
// }
//
//
// pub fn generate_variable_name() -> Ident {
//     let sanitized_uuid = uuid::Uuid::new_v4().simple();
//     let crate_name = get_crate_name(false);
//     let name = format!("_{crate_name}__private__internal_variable_prefix__{sanitized_uuid}")
//         .to_case(Case::Camel);
//     let mut param = format_ident!("{name}");
//
//     // param.truncate(15);
//
//     param
// }
//
//
