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

    let has_placeholders = query.contains('$');
    let mut bindings = Vec::new();

    if has_placeholders {
        // Expect a comma after the SQL query string literal
        match iter.next() {
            Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => {}
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Expected a comma after the SQL query string literal",
                ))
            }
        }

        // Expect curly braces for bindings
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

        // After parsing the bindings, there should be no more tokens.
        if iter.next().is_some() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Unexpected tokens after bindings block",
            ));
        }
    } else {
        // If there are no placeholders, bindings are optional.
        // Check for an optional comma and bindings block.
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

                // After parsing the bindings, there should be no more tokens.
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

    while let Some((start, _)) = placeholders.peek().cloned() {
        // Peek the next placeholder without advancing the iterator to check if it's consecutive.
        placeholders.next(); // Consume the current placeholder since we've now processed it.

        // Find the end of the placeholder name by looking for the next non-alphanumeric character.
        let end = query[start + 1..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map_or(query.len(), |i| start + i + 1);

        let placeholder_name = &query[start + 1..end];

        // Check if there's a corresponding binding for the placeholder.
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

        let (db_conn, query, bindings) = parse_args(input.into()).unwrap();

        assert_eq!(
            db_conn.to_token_stream().to_string(),
            expected_db_conn.to_token_stream().to_string()
        );
        assert_eq!(query, expected_query);
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
        let expected_bindings = vec![Binding {
            key: parse_quote! { id },
            value: parse_quote! { 1 },
        }];

        let (db_conn, query, bindings) = parse_args(input.into()).unwrap();

        assert_eq!(
            db_conn.to_token_stream().to_string(),
            expected_db_conn.to_token_stream().to_string()
        );
        assert_eq!(query, expected_query);
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

        assert!(parse_args(input.into()).is_err());
    }
}
