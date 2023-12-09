/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    token, Expr, Ident, Token,
};

use super::{helpers::generate_variable_name, query_chain::QueriesChainParser};

#[derive(Debug, Clone)]
pub struct Body(QueriesChainParser);

impl Body {
    pub fn has_return_statement(&self) -> bool {
        self.0.is_likely_query_block()
    }
}

impl ToTokens for Body {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Body(queries_chain_parser) = self;
        queries_chain_parser.to_tokenstream().to_tokens(tokens);
    }
}

impl std::ops::Deref for Body {
    type Target = QueriesChainParser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for Body {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body_content;
        let _ = syn::braced!(body_content in input);
        Ok(Self(body_content.parse()?))
    }
}

#[derive(Debug, Clone)]
pub enum IfElseCondExpression {
    Expr(Expr),
    Ident(Ident),
}

impl ToTokens for IfElseCondExpression {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            IfElseCondExpression::Expr(expr) => expr.to_tokens(tokens),
            IfElseCondExpression::Ident(ident) => ident.to_tokens(tokens),
        }
    }
}

impl Parse for IfElseCondExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        fork.parse::<Ident>()?;

        if (fork.is_empty()) || (fork.peek(token::Brace)) {
            let ident = input.parse()?;
            Ok(IfElseCondExpression::Ident(ident))
        } else {
            let expr = input.parse()?;
            Ok(IfElseCondExpression::Expr(expr))
        }
    }
}

#[derive(Debug, Clone)]
pub struct CondMeta {
    pub condition: IfElseCondExpression,
    pub body: Body,
}

impl Parse for CondMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(token::Paren) {
            let condition_content;
            let _paranthesized_iter_content_token = syn::parenthesized!(condition_content in input);
            let condition = condition_content.parse()?;

            let body = input.parse()?;

            Ok(CondMeta { condition, body })
        } else {
            let condition = input.parse()?;
            let body = input.parse()?;

            Ok(CondMeta { condition, body })
        }
    }
}
#[derive(Debug, Clone)]
pub struct IfMeta(CondMeta);

impl std::ops::Deref for IfMeta {
    type Target = CondMeta;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for IfMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

#[derive(Debug, Clone)]
pub struct ElseIfMeta(CondMeta);

impl std::ops::Deref for ElseIfMeta {
    type Target = CondMeta;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for ElseIfMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<syn::Token![else]>()?;
        let _ = input.parse::<syn::Token![if]>()?;

        Ok(Self(input.parse()?))
    }
}

#[derive(Debug, Clone)]
pub struct Else {
    pub body: Body,
}

impl Parse for Else {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<syn::Token![else]>()?;

        let body = input.parse()?;
        Ok(Else { body })
    }
}

#[derive(Debug, Clone)]
pub struct IfElseWithoutIfKeywordMetaAst {
    pub if_meta: IfMeta,
    pub else_if_meta: Vec<ElseIfMeta>,
    pub else_meta: Option<Else>,
    pub generated_ident: Ident,
}

impl IfElseWithoutIfKeywordMetaAst {
    #[allow(dead_code)]
    pub fn has_return_statement(&self) -> bool {
        self.if_meta.body.has_return_statement()
            || self
                .else_if_meta
                .iter()
                .any(|else_if_meta| else_if_meta.body.has_return_statement())
            || self
                .else_meta
                .as_ref()
                .map_or(false, |else_meta| else_meta.body.has_return_statement())
    }
}

impl ToTokens for IfElseWithoutIfKeywordMetaAst {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tokenized_if_else_content: proc_macro2::TokenStream =
            self.tokenize().query_chain.into();

        let if_else: proc_macro2::TokenStream = quote!(
            #tokenized_if_else_content
        );

        if_else.into_token_stream().to_tokens(tokens)
    }
}

impl Parse for IfElseWithoutIfKeywordMetaAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generated_ident = generate_variable_name();

        // The iteration parameter and the iterable in the start of the for loop
        let if_meta = input.parse::<IfMeta>()?;

        let mut else_if_meta = vec![];
        while input.peek(Token![else]) && input.peek2(Token![if]) {
            let else_if = input.parse::<ElseIfMeta>()?;
            else_if_meta.push(else_if);
        }

        let else_meta = if input.peek(Token![else]) {
            let else_meta = input.parse::<Else>()?;
            Some(else_meta)
        } else {
            None
        };

        let _ = input.parse::<Token![;]>()?;

        Ok(Self {
            if_meta,
            else_if_meta,
            else_meta,
            generated_ident,
        })
    }
}

#[derive(Debug, Clone)]
pub struct IfElseStatementAst {
    pub meta_content: Box<IfElseWithoutIfKeywordMetaAst>,
}

impl ToTokens for IfElseStatementAst {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // let tokenized_if_else_content: proc_macro2::TokenStream =
        //     self.meta_content.tokenize().query_chain.into();
        //
        // let if_else: proc_macro2::TokenStream = quote!(
        //     #tokenized_if_else_content
        // )
        // .into();

        self.meta_content.to_tokens(tokens)
    }
}

impl Parse for IfElseStatementAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![if]>()?;
        let for_loop = input.parse::<IfElseWithoutIfKeywordMetaAst>()?;
        Ok(IfElseStatementAst {
            meta_content: Box::new(for_loop),
        })
    }
}

pub struct TokenizedIfElseStmt {
    pub code_to_render: TokenStream,
    pub query_chain: TokenStream,
}

impl IfElseWithoutIfKeywordMetaAst {
    pub fn tokenize(&self) -> TokenizedIfElseStmt {
        let IfElseWithoutIfKeywordMetaAst {
            if_meta,
            else_if_meta,
            else_meta,
            generated_ident: _,
        } = self;
        let crate_name = get_crate_name(false);

        let if_cond_expr = &if_meta.condition;
        let if_body = &if_meta.body.generate_code();
        let if_body_to_render = &if_body.to_render;
        let query_chain_var_ident = &if_body.query_chain_var_ident;

        let if_code: proc_macro2::TokenStream = quote!(
            #crate_name::statements::if_(#if_cond_expr)
            .then({
                #(#if_body_to_render)*

                #query_chain_var_ident
            })
        );

        let else_if: proc_macro2::TokenStream = else_if_meta
            .iter()
            .map(|else_if_meta| {
                let cond_expr = &else_if_meta.condition;
                let body = &else_if_meta.body.generate_code();
                let body_to_render = &body.to_render;
                let query_chain_var_ident = &body.query_chain_var_ident;

                quote!(
                        .else_if(#cond_expr)
                        .then({
                            #(#body_to_render)*

                            #query_chain_var_ident
                        })
                )
            })
            .collect();

        let else_code: proc_macro2::TokenStream =
            else_meta.as_ref().map_or(quote!(), |else_meta| {
                let body = &else_meta.body.generate_code();
                let body_to_render = &body.to_render;
                let query_chain_var_ident = &body.query_chain_var_ident;

                quote!(
                    .else_({
                        #(#body_to_render)*

                        #query_chain_var_ident
                    })
                )
            });

        let to_render: proc_macro2::TokenStream = quote!(
            #if_code
            #else_if
            #else_code
            .end()
        );

        TokenizedIfElseStmt {
            code_to_render: to_render.clone().into(),
            query_chain: to_render.into(),
        }
    }
}

// TOOD:: complete the standalone top level if else statement parser implementation
#[allow(dead_code)]
pub fn if_else(input: TokenStream) -> TokenStream {
    let if_else = syn::parse_macro_input!(input as IfElseWithoutIfKeywordMetaAst);

    quote!(#if_else).into()
}
