/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

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

struct Queries {
    query_strings: Vec<LitStr>,
}

impl Parse for Queries {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _bracket_token: syn::token::Bracket = syn::bracketed!(content in input);
        let query_strings = Punctuated::<LitStr, Comma>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Queries { query_strings })
    }
}

fn parse_args(input: TokenStream) -> Result<(Expr, Vec<String>, Vec<Binding>)> {
    let input2: TokenStream = input.clone();
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

    let queries = match iter.next() {
        Some(proc_macro2::TokenTree::Literal(lit)) => {
            vec![syn::parse2::<LitStr>(lit.to_token_stream())?.value()]
        }
        Some(TokenTree::Group(group)) if group.delimiter() == proc_macro2::Delimiter::Bracket => {
            let queries_tokens = group.into_token_stream();
            parse2::<Queries>(queries_tokens)?
                .query_strings
                .into_iter()
                .map(|lit_str| lit_str.value())
                .collect()
        }
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Expected a SQL query string literal",
            ))
        }
    };

    for query in &queries {
        let sql = sql::parse(query.trim());
        match sql {
            Ok(value) => value,
            Err(value) => {
                return Err(syn::Error::new_spanned(input.clone(), value));
            }
        };
    }

    let has_placeholders = queries.join(";").contains('$');
    let mut bindings = Vec::new();

    if has_placeholders {
        match iter.next() {
            Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => {}
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Expected a comma after the SQL query string literal",
                ))
            }
        }

        let bindings_tokens = match iter.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == proc_macro2::Delimiter::Brace => {
                group.into_token_stream()
            }
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Expected curly braces for bindings",
                ))
            }
        };

        let Bindings { pairs } = parse2(bindings_tokens)?;
        bindings = pairs.into_iter().collect::<Vec<_>>();

        if iter.next().is_some() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Unexpected tokens after bindings block",
            ));
        }
    } else {
        // If there are no placeholders, bindings are optional.
        if let Some(TokenTree::Punct(ref punct)) = iter.next() {
            if punct.as_char() == ',' {
                let bindings_tokens = match iter.next() {
                    Some(TokenTree::Group(group))
                        if group.delimiter() == proc_macro2::Delimiter::Brace =>
                    {
                        group.into_token_stream()
                    }
                    _ => {
                        return Err(syn::Error::new(
                            Span::call_site(),
                            "Expected curly braces for bindings",
                        ))
                    }
                };

                let Bindings { pairs } = parse2(bindings_tokens)?;
                bindings = pairs.into_iter().collect::<Vec<_>>();

                if iter.next().is_some() {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "Unexpected tokens after bindings block",
                    ));
                }
            } else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Expected a comma after the SQL query string literal",
                ));
            }
        }
    }
    Ok((database_connection, queries, bindings))
}

fn validate_and_parse_sql_query(query: &str, bindings: &[Binding]) -> syn::Result<String> {
    for binding in bindings {
        let placeholder = format!("${}", binding.key);
        if !query.contains(&placeholder) {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "No placeholder found in SQL query for binding '{}'",
                    binding.key
                ),
            ));
        }
    }

    let placeholders = query.match_indices('$').peekable();

    for (start, _) in placeholders {
        let end = query[start + 1..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map_or(query.len(), |i| start + i + 1);

        let placeholder_name = &query[start + 1..end];

        if !bindings.iter().any(|b| b.key == placeholder_name) {
            return Err(syn::Error::new(
                Span::call_site(),
                format!("No binding found for placeholder '${}'", placeholder_name),
            ));
        }
    }

    Ok(query.to_owned())
}

pub fn query(args: TokenStream) -> TokenStream {
    let (db_con, query_strs, bindings) = parse_args(args).expect("Failed to parse arguments");
    let mut output = TokenStream::new();

    let _sql_queries = validate_and_parse_sql_query(&query_strs.join(";"), &bindings)
        .expect("Failed to validate SQL query");

    for (i, query_str) in query_strs.iter().enumerate() {
        let is_first = i == 0;
        if is_first {
            output.extend(quote::quote! {
                #db_con.query(#query_str)
            });
        } else {
            output.extend(quote::quote! {
                .query(#query_str)
            });
        }
    }

    for Binding { key, value } in bindings {
        let key = key.to_string();
        output.extend(quote::quote! {
            .bind((#key, #value))
        });
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_parse_args_without_placeholders() {
        let input = quote! {
            my_db, "SELECT * FROM users"
        };

        let expected_db_conn: Expr = parse_quote! { my_db };
        let expected_query = "SELECT * FROM users".to_string();
        let expected_bindings: Vec<Binding> = vec![];

        let (db_conn, query, bindings) = parse_args(input).unwrap();

        assert_eq!(
            db_conn.to_token_stream().to_string(),
            expected_db_conn.to_token_stream().to_string()
        );
        assert_eq!(query, vec![expected_query]);
        assert_eq!(bindings.len(), expected_bindings.len());
    }

    #[test]
    fn test_parse_args_with_placeholders() {
        let input = quote! {
            my_db, "SELECT * FROM users WHERE id = $id", {
                id: 1
            }
        };

        let expected_db_conn: Expr = parse_quote! { my_db };
        let expected_query = "SELECT * FROM users WHERE id = $id".to_string();
        let expected_bindings = [Binding {
            key: parse_quote! { id },
            value: parse_quote! { 1 },
        }];

        let (db_conn, query, bindings) = parse_args(input).unwrap();

        assert_eq!(
            db_conn.to_token_stream().to_string(),
            expected_db_conn.to_token_stream().to_string()
        );
        assert_eq!(query, vec![expected_query]);
        assert_eq!(bindings.len(), expected_bindings.len());
        assert_eq!(bindings[0].key, expected_bindings[0].key);
        assert_eq!(
            bindings[0].value.to_token_stream().to_string(),
            expected_bindings[0].value.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_parse_args_unexpected_tokens() {
        let input = quote! {
            my_db, "SELECT * FROM users", {} something_else
        };

        assert!(parse_args(input).is_err());
    }

    #[test]
    fn test_parse_args_multiple_queries() {
        let input = quote! {
            my_db,
            [
                "SELECT * FROM users WHERE id = $id",
                "CREATE user:oyelowo SET name = 'Oyelowo', company = 'Codebreather', skills = ['Rust', 'python', 'typescript']"
            ], {
                id: 1
            }
        };

        let expected_db_conn: Expr = parse2(quote! { my_db }).unwrap();
        let expected_queries = vec![
            "SELECT * FROM users WHERE id = $id".to_string(),
            "CREATE user:oyelowo SET name = 'Oyelowo', company = 'Codebreather', skills = ['Rust', 'python', 'typescript']".to_string()
        ];
        let expected_bindings = [Binding {
            key: parse2(quote! { id }).unwrap(),
            value: parse2(quote! { 1 }).unwrap(),
        }];

        let (db_conn, queries, bindings) = parse_args(input).expect("Failed to parse arguments");

        assert_eq!(
            quote! { #db_conn }.to_string(),
            quote! { #expected_db_conn }.to_string(),
            "Database connection parsing failed or mismatched."
        );

        assert_eq!(
            queries, expected_queries,
            "Queries parsing failed or mismatched."
        );
        assert_eq!(
            bindings.len(),
            expected_bindings.len(),
            "Number of bindings parsed does not match expected."
        );

        for (binding, expected_binding) in bindings.iter().zip(expected_bindings.iter()) {
            let binding_key = binding.key.to_string();
            let binding_value = binding.value.to_token_stream().to_string();

            let expected_binding_key = expected_binding.key.to_string();
            let expected_binding_value = expected_binding.value.to_token_stream().to_string();

            assert_eq!(binding_key, expected_binding_key, "Binding key mismatch.");
            assert_eq!(
                binding_value, expected_binding_value,
                "Binding value mismatch."
            );
        }
    }
}
