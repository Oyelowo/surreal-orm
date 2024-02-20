/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

macro_rules! create_ident_wrapper {
    ($ident:ident) => {
        #[derive(Debug, Clone)]
        pub struct $ident(::syn::Ident);

        impl $ident {
            pub fn into_inner(self) -> ::syn::Ident {
                self.0
            }
        }

        impl ::std::ops::Deref for $ident {
            type Target = ::syn::Ident;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::convert::From<::syn::Ident> for $ident {
            fn from(ident: ::syn::Ident) -> Self {
                Self(ident)
            }
        }

        impl ::std::convert::From<$ident> for ::syn::Ident {
            fn from(ident: $ident) -> Self {
                ident.0
            }
        }

        impl ::quote::ToTokens for $ident {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                self.0.to_tokens(tokens)
            }
        }
    };
}
pub(crate) use create_ident_wrapper;
