/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#[macro_export]
macro_rules! string_wrapper {
    ($name:ident) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct $name(String);

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::quote::ToTokens for $name {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                tokens.extend(::quote::quote!(#self.0));
            }
        }
    };
}

pub use string_wrapper;

string_wrapper!(SerializedFieldNamesNormalised);
