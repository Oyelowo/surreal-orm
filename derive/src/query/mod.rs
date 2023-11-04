use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use surreal_query_builder::sql;
use syn::parse::{Parse, ParseStream};
use syn::parse2;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use syn::{Expr, LitStr, Result, Token};

struct Binding {
    key: Ident,
    value: Expr,
}

struct Bindings {
    pairs: Punctuated<Binding, Comma>,
}

impl Parse for Binding {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: Expr = input.parse()?;
        Ok(Binding { key, value })
    }
}

impl Parse for Bindings {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _brace_token: Brace = syn::braced!(content in input);
        let pairs = Punctuated::parse_terminated(&content)?;
        Ok(Bindings { pairs })
    }
}

fn parse_args(input: TokenStream) -> Result<(Expr, String, Vec<Binding>)> {
    let input2: TokenStream = input.clone().into();
    let mut iter = input2.into_iter();

    let database_connection = iter
        .next()
        .ok_or_else(|| syn::Error::new(Span::call_site(), "Expected a db connection"))
        .and_then(|tt| syn::parse2::<Expr>(tt.into()))?;

    match iter.next() {
        Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => {}
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Expected a comma after the DB connection",
            ))
        }
    }

    let query = match iter.next() {
        Some(proc_macro2::TokenTree::Literal(lit)) => syn::parse2::<LitStr>(lit.to_token_stream())?,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Expected a SQL query string literal",
            ))
        }
    };

    let query = query.value();
    let sql = sql::parse(query.as_str());

    match sql {
        Ok(value) => value,
        Err(value) => return Err(syn::Error::new_spanned(input, value)),
    };

    // Check if there is a comma after the SQL query string literal
    if let Some(token) = iter.next() {
        if token.to_string() != "," {
            return Err(syn::Error::new(
                Span::call_site(),
                "Expected a comma after the SQL query string literal",
            ));
        }
    } else {
        return Err(syn::Error::new(
            Span::call_site(),
            "Expected arguments after the SQL query string literal",
        ));
    }

    let bindings_tokens = iter.collect::<TokenStream>();
    let Bindings { pairs } = parse2(bindings_tokens)?;
    let bindings = pairs.into_iter().collect::<Vec<_>>();

    Ok((database_connection, query, bindings))
}

fn validate_and_parse_sql_query(query: &str, bindings: &[Binding]) -> syn::Result<String> {
    for binding in bindings {
        let placeholder = format!("${}", binding.key.to_string());
        if !query.contains(&placeholder) {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "No placeholder found in SQL query for binding '{}'",
                    binding.key.to_string()
                ),
            ));
        }
    }

    // Ensure that every placeholder in the query has a binding
    let mut placeholders = query.match_indices("$").peekable();

    // Verify that each `$` in the query corresponds to a binding.
    let mut last_index = 0;
    while let Some(start) = query[last_index..].find('$').map(|i| last_index + i) {
        let end = query[start + 1..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| start + i + 1) // `+1` to include the character at the end position.
            .unwrap_or_else(|| query.len());

        // Ensure that `start` is less than `end` before slicing to prevent panic.
        if start >= end {
            return Err(syn::Error::new(
                Span::call_site(),
                "Placeholder parsing error: end index is not greater than start index",
            ));
        }

        let placeholder_name = &query[start + 1..end];
        if !bindings.iter().any(|b| b.key == placeholder_name) {
            return Err(syn::Error::new(
                Span::call_site(),
                format!("No binding found for placeholder '${}'", placeholder_name),
            ));
        }

        // Update `last_index` to continue searching the rest of the string.
        last_index = end;
    }

    Ok(query.to_owned())
}

pub fn query(args: TokenStream) -> TokenStream {
    let (db_con, query_str, bindings) = parse_args(args).expect("Failed to parse arguments");

    let sql_query =
        validate_and_parse_sql_query(&query_str, &bindings).expect("Failed to validate SQL query");

    let mut output = TokenStream::new();

    output.extend(quote::quote! {
        #db_con.query(#sql_query)
    });

    for Binding { key, value } in bindings {
        let key = key.to_string();
        output.extend(quote::quote! {
            .bind((#key, #value))
        });
    }

    output
}
