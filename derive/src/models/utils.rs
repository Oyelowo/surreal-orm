/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub use proc_macro2::TokenStream;
pub use proc_macros_helpers::get_crate_name;
pub use quote::{quote, ToTokens};

macro_rules! create_tokenstream_wrapper {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone)]
        pub struct $name(pub ::proc_macro2::TokenStream);

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

        impl ::std::convert::From<::proc_macro2::TokenStream> for $name {
            fn from(tokenstream: ::proc_macro2::TokenStream) -> Self {
                Self(tokenstream)
            }
        }

        impl ::std::convert::From<$name> for ::proc_macro2::TokenStream {
            fn from(tokenstream: $name) -> Self {
                tokenstream.0
            }
        }

        impl Eq for $name {}

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.to_string() == other.0.to_string()
            }
        }

        impl ::std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_string().hash(state);
            }
        }
    };
}

pub(crate) use create_tokenstream_wrapper;
