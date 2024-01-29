/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub use proc_macro2::TokenStream;
pub use proc_macros_helpers::get_crate_name;
pub use quote::{quote, ToTokens};

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

#[macro_export]
macro_rules! create_tokenstream_wrapper {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Default)]
        pub struct $name(::proc_macro2::TokenStream);

        impl $name {
            pub fn new(tokenstream: ::proc_macro2::TokenStream) -> Self {
                Self(tokenstream)
            }
        }

        impl ::quote::ToTokens for $name {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                tokens.extend(self.0.clone());
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = ::proc_macro2::TokenStream;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<::proc_macro2::TokenStream> for $name {
            fn from(tokenstream: ::proc_macro2::TokenStream) -> Self {
                Self(tokenstream)
            }
        }

        impl From<$name> for ::proc_macro2::TokenStream {
            fn from(tokenstream: $name) -> Self {
                tokenstream.0
            }
        }
    };
}

pub use create_tokenstream_wrapper;
