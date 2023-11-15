use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self, Brace},
    Expr, Ident, Token,
};

use super::{
    helpers::generate_variable_name,
    query::QueryParser,
    query_chain::{GeneratedCode, QueriesChainParser},
};

pub struct IfElseMetaParser {
    pub iteration_param: Ident,
    pub iterable: Expr,
    pub body: QueriesChainParser,
    pub generated_ident: Ident,
}

impl Parse for IfElseMetaParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generated_ident = generate_variable_name();

        // The iteration parameter and the iterable in the start of the for loop
        if input.peek(token::Paren) {
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

            return Ok(IfElseMetaParser {
                iteration_param,
                iterable,
                body,
                generated_ident,
            });
        } else {
            let iteration_param = input.parse()?;

            input.parse::<syn::Token![in]>()?;

            let iterable = input.parse()?;
            // The body
            let content;
            let _brace_token: Brace = syn::braced!(content in input);

            let body = content.parse()?;

            input.parse::<syn::Token![;]>()?;

            return Ok(IfElseMetaParser {
                iteration_param,
                iterable,
                body,
                generated_ident,
            });
        };
    }
}

pub struct IfElseStatementParser {
    pub meta_content: Box<IfElseMetaParser>,
}

impl std::ops::Deref for IfElseStatementParser {
    type Target = IfElseMetaParser;

    fn deref(&self) -> &Self::Target {
        &self.meta_content
    }
}

impl Parse for IfElseStatementParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![if]>()?;
        let for_loop = input.parse::<IfElseMetaParser>()?;
        Ok(IfElseStatementParser {
            meta_content: Box::new(for_loop),
        })
    }
}

pub struct TokenizedForLoop {
    pub code_to_render: TokenStream,
    pub query_chain: TokenStream,
}

impl IfElseMetaParser {
    pub fn tokenize(&self) -> TokenizedForLoop {
        let IfElseMetaParser {
            iteration_param,
            iterable,
            body,
            generated_ident,
        } = self;

        let GeneratedCode {
            query_chain,
            to_render,
        } = body.generate_code();

        let crate_name = get_crate_name(false);

        let whole_stmts = quote!(
        {
            let #iteration_param = #crate_name::Param::new(stringify!(#iteration_param));

            #( #to_render )*

            #crate_name::statements::for_(#iteration_param).in_(#iterable)
            .block(
                #( #query_chain )*
                .parenthesized()
            )
        });

        let to_render = quote! {
            {
                #( #to_render )*

                #whole_stmts
            }
        }
        .into();
        let to_chain = quote!(#whole_stmts);

        TokenizedForLoop {
            code_to_render: to_render,
            query_chain: to_chain.into(),
        }
    }
}

pub fn for_loop(input: TokenStream) -> TokenStream {
    let for_loop_content = syn::parse_macro_input!(input as IfElseMetaParser);

    let z = &for_loop_content.tokenize();
    let to_render: proc_macro2::TokenStream = z.code_to_render.clone().into();
    let to_chain: proc_macro2::TokenStream = z.query_chain.clone().into();

    quote!(
        #to_render

        #to_chain

    )
    .into()
}
