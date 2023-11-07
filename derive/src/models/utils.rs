/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro2::{Span, TokenStream, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

use syn::{self, Error, Ident, LitStr};

pub fn parse_lit_to_tokenstream(lit: &LitStr) -> Result<TokenStream, Error> {
    let str = lit.value();
    let tokens: TokenStream = str.parse().map_err(syn::Error::from)?;
    Ok(quote! { (#tokens) })
}

pub fn get_crate_name(internal: bool) -> TokenStream {
    if internal {
        quote! { crate }
    } else {
        let name = match crate_name("surreal-orm") {
            Ok(FoundCrate::Name(name)) => name,
            Ok(FoundCrate::Itself) | Err(_) => "surreal_orm".to_string(),
        };
        TokenTree::from(Ident::new(&name, Span::call_site())).into()
    }
}

pub fn generate_nested_vec_type(
    foreign_node: &syn::Ident,
    nesting_level: usize,
) -> proc_macro2::TokenStream {
    if nesting_level == 0 {
        quote!(#foreign_node)
    } else {
        let inner_type = generate_nested_vec_type(foreign_node, nesting_level - 1);
        quote!(::std::vec::Vec<#inner_type>)
    }
}

pub fn count_vec_nesting(field_type: &syn::Type) -> usize {
    match field_type {
        syn::Type::Path(type_path) => {
            // Check if the outermost type is a `Vec`.
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Vec" {
                    // It's a Vec, now look at the inner type.
                    if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner_type)) =
                            angle_args.args.first()
                        {
                            // Recursively count nesting inside the Vec.
                            1 + count_vec_nesting(inner_type)
                        } else {
                            0 // No type inside Vec's angle brackets.
                        }
                    } else {
                        0 // Vec has no angle brackets, which should not happen for valid Vec usage.
                    }
                } else {
                    0 // The outermost type is not a Vec.
                }
            } else {
                0 // No segments in the type path.
            }
        }
        _ => 0, // Not a type path, so it can't be a Vec.
    }
}
