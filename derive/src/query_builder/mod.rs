use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Ident, Result as SynResult, Token,
};

use crate::models::get_crate_name;

struct LetStatement {
    ident: Ident,
    _eq: Token![=],
    expr: Expr,
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

enum Query {
    Statement(LetStatement),
    Expr(Expr),
}

impl Parse for Query {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![let]) {
            let var_statement = input.parse::<LetStatement>()?;
            Ok(Query::Statement(var_statement))
        } else {
            let expr = input.parse::<Expr>()?;
            let _end: Token![;] = input.parse()?;
            Ok(Query::Expr(expr))
        }
    }
}

struct QueriesInput {
    statements: Vec<Query>,
}

impl Parse for QueriesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();

        while !input.is_empty() && !input.peek(Token![return]) {
            statements.push(input.parse()?);
        }

        Ok(QueriesInput { statements })
    }
}

pub fn query_turbo(input: TokenStream) -> TokenStream {
    let QueriesInput { statements } = parse_macro_input!(input as QueriesInput);
    let crate_name = get_crate_name(false);

    let generated_code = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        Query::Statement(var_statement) => {
            let LetStatement { ident, expr, .. } = var_statement;
            quote! {
                let #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
            }
        }
        Query::Expr(expr) => quote! {
            #expr;
        },
    });

    let query_chain = statements
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_first = i == 0;
            let to_chain = match s {
                Query::Statement(LetStatement { ident, .. }) => quote!(#ident),
                Query::Expr(expr) => quote!(#expr),
            };

            if is_first {
                quote!(#crate_name::chain(#to_chain))
            } else {
                quote!(.chain(#to_chain))
            }
        })
        .collect::<Vec<_>>();

    quote! {
        {
        #( #generated_code )*

        #( #query_chain )*
        }
    }
    .into()
}

struct Block {
    statements: Vec<Query>,
    return_expr: Expr,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut queries_input = input.parse::<QueriesInput>()?;
        let return_token: Token![return] = input.parse()?;
        let return_expr = input.parse::<Expr>()?;
        let _ending_colon = input.parse::<Token![;]>();

        Ok(Block {
            statements: queries_input.statements,
            return_expr,
        })
    }
}

// ends with a return statement;
pub fn query_block(input: TokenStream) -> TokenStream {
    let Block {
        statements,
        return_expr,
    } = parse_macro_input!(input as Block);

    let crate_name = get_crate_name(false);

    let generated_code = statements.iter().map(|stmt_or_expr| match stmt_or_expr {
        Query::Statement(var_statement) => {
            let LetStatement { ident, expr, .. } = var_statement;
            quote! {
                let #ident = #crate_name::statements::let_(stringify!(#ident)).equal_to(#expr);
            }
        }
        Query::Expr(expr) => quote! {
            #expr;
        },
    });

    let query_chain = statements
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let is_first = i == 0;
            let to_chain = match s {
                Query::Statement(LetStatement { ident, .. }) => quote!(#ident),
                Query::Expr(expr) => quote!(#expr),
            };

            if is_first {
                quote!(#crate_name::chain(#to_chain))
            } else {
                quote!(.chain(#to_chain))
            }
        })
        .collect::<Vec<_>>();

    let has_only_return = statements.len() == 0;
    let whole_stmts = if has_only_return {
        quote!(#crate_name::statements::return_(#return_expr).as_block())
    } else {
        quote!(
                #( #query_chain )*
                .chain(#crate_name::statements::return_(#return_expr)).as_block()
        )
    };

    quote! {
        {
            #( #generated_code )*

            #whole_stmts
        }
    }
    .into()
}
