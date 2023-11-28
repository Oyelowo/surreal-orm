use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};
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

// if (condition -> expression) {
//  body -> query chain
// } else if  (condition -> expression) {
// body -> query chain
// } else {
// body -> query chain
// }

pub struct Body(QueriesChainParser);

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

pub enum Expression {
    Expr(Expr),
    Ident(Ident),
}

impl ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Expression::Expr(expr) => expr.to_tokens(tokens),
            Expression::Ident(ident) => ident.to_tokens(tokens),
        }
    }
}

impl Parse for Expression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        fork.parse::<Ident>()?;

        if (fork.is_empty()) || (fork.peek(token::Brace)) {
            let ident = input.parse()?;
            Ok(Expression::Ident(ident))
        } else {
            let expr = input.parse()?;
            Ok(Expression::Expr(expr))
        }
    }
}

pub struct CondMeta {
    pub condition: Expression,
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

pub struct IfElseMetaParser {
    pub if_meta: IfMeta,
    pub else_if_meta: Vec<ElseIfMeta>,
    pub else_meta: Option<Else>,
    pub generated_ident: Ident,
}

impl Parse for IfElseMetaParser {
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

pub struct TokenizedIfElseStmt {
    pub code_to_render: TokenStream,
    pub query_chain: TokenStream,
}

impl IfElseMetaParser {
    pub fn tokenize(&self) -> TokenizedIfElseStmt {
        let IfElseMetaParser {
            if_meta,
            else_if_meta,
            else_meta,
            generated_ident,
        } = self;
        let crate_name = get_crate_name(false);

        // let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
        //     .then(statement1)
        //     .else_if(name.like("Oyelowo Oyedayo"))
        //     .then(statement2)
        //     .else_if(cond(country.is("Canada")).or(country.is("Norway")))
        //     .then("Cold")
        //     .else_("Hot")
        //     .end();

        let ref if_cond_expr = if_meta.condition;
        let if_body = &if_meta.body.generate_code();
        let if_body_to_render = &if_body.to_render;
        let query_chain = &if_body.query_chain;

        let if_code: proc_macro2::TokenStream = quote!(
            #crate_name::statements::if_(#if_cond_expr)
            .then({
                #(#if_body_to_render)*

                #(#query_chain)*
            })
        );

        let else_if: proc_macro2::TokenStream = else_if_meta
            .iter()
            .map(|else_if_meta| {
                let cond_expr = &else_if_meta.condition;
                let body = &else_if_meta.body.generate_code();
                let body_to_render = &body.to_render;
                let query_chain = &body.query_chain;

                quote!(
                        .else_if(#cond_expr)
                        .then({
                            #(#body_to_render)*

                            #(#query_chain)*
                        })
                )
            })
            .collect();

        let else_code: proc_macro2::TokenStream =
            else_meta.as_ref().map_or(quote!(), |else_meta| {
                let body = &else_meta.body.generate_code();
                let body_to_render = &body.to_render;
                let query_chain = &body.query_chain;

                quote!(
                    .else_({
                        #(#body_to_render)*
                        #(#query_chain)*
                    })
                )
            });

        let to_render: proc_macro2::TokenStream = quote!(
            #if_code
            #else_if
            #else_code
            .end()
        )
        .into();

        TokenizedIfElseStmt {
            code_to_render: to_render.clone().into(),
            query_chain: to_render.into(),
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
