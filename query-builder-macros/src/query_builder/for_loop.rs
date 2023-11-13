use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Expr, Ident,
};

use super::query_chain::QueriesChain;

struct ForLoop {
    iteration_param: Ident,
    iterable: Expr,
    body: QueriesChain,
}

impl Parse for ForLoop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // The iteration parameter and the iterable in the start of the for loop
        let iter_content;
        let _paranthesized_iter_content_token = syn::parenthesized!(iter_content in input);

        let iteration_param = iter_content.parse()?;

        iter_content.parse::<syn::Token![in]>()?;

        let iterable = iter_content.parse()?;

        // The body
        let content;
        let _brace_token: Brace = syn::braced!(content in input);

        let body = content.parse()?;

        input.parse::<syn::Token![;]>()?;

        Ok(ForLoop {
            iteration_param,
            iterable,
            body,
        })
    }
}

pub fn for_loop(input: TokenStream) -> TokenStream {
    let ForLoop {
        iteration_param,
        iterable,
        body,
    } = syn::parse_macro_input!(input as ForLoop);

    let generated_code = super::generate_query_chain_code(&body.statements);
    let query_chain = super::generated_bound_query_chain(&body.statements);

    let crate_name = super::get_crate_name(false);

    let whole_stmts = quote!(#crate_name::statements::for_(#iteration_param).in_(#iterable)
    .block(
        #( #query_chain )*
        .parenthesized()
    ));

    quote! {
        {
            #( #generated_code )*

            #whole_stmts
        }
    }
    .into()
}
