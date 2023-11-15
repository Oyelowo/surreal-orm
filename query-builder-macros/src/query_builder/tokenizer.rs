use crate::statement_parser::query_chain::{GeneratedCode, QueriesChainParser};
use proc_macros_helpers::get_crate_name;
use quote::quote;

pub enum QueryTypeToken {
    Chain(proc_macro2::TokenStream),
    Block(proc_macro2::TokenStream),
    Transaction(proc_macro2::TokenStream),
}

impl QueryTypeToken {
    pub fn get_tokenstream(self) -> proc_macro2::TokenStream {
        match self {
            QueryTypeToken::Chain(t) => t,
            QueryTypeToken::Block(t) => t,
            QueryTypeToken::Transaction(t) => t,
        }
    }
}

impl From<&QueriesChainParser> for QueryTypeToken {
    fn from(value: &QueriesChainParser) -> Self {
        let GeneratedCode {
            to_render,
            query_chain,
        } = value.generate_code();
        let crate_name = get_crate_name(false);

        if value.is_valid_transaction_statement() {
            let ending = value.statements.last().expect("No statements found");
            let ending = if ending.is_commit_transaction() {
                quote! {
                    .commit_transaction()
                }
            } else if ending.is_cancel_transaction() {
                quote! {
                    .cancel_transaction()
                }
            } else {
                // shouldnt happen since this should now be a valid tx. Probably can be modelled
                // better
                panic!("Invalid transaction ending")
            };

            let transaction = quote! {
                #crate_name::statements::begin_transaction()
                .query(#( #query_chain) *)
                #ending
            };

            QueryTypeToken::Transaction(quote! {
                {
                    #( #to_render )*

                    #transaction
                }
            })
        } else if value.is_likely_query_block() {
            QueryTypeToken::Block(quote! {
                {
                    #( #to_render )*

                    #( #query_chain )*
                    .as_block()
                }
            })
        } else {
            QueryTypeToken::Chain(quote! {
                {
                    #( #to_render )*

                    #( #query_chain )*
                }
            })
        }
    }
}
