/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

// use syn::Expr::
macro_rules! create_ident_wrapper {
    ($ident:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $ident(::syn::Ident);

        impl $ident {
            pub fn into_inner(self) -> ::syn::Ident {
                self.0
            }
        }

        impl ::darling::FromMeta for $ident {
            fn from_expr(expr: &::syn::Expr) -> ::darling::Result<Self> {
                match expr {
                    ::syn::Expr::Lit(expr) => {
                        if let ::syn::Lit::Str(lit_str) = &expr.lit {
                            Ok($ident(::syn::Ident::new(&lit_str.value(), lit_str.span())))
                        } else {
                            Err(darling::Error::custom("Expected a string literal."))
                        }
                    }
                    // Expr::Verbatim(expr_verbatim) => {
                    //     let ident = syn::parse2(expr_verbatim.clone().into_token_stream())?;
                    //     Ok(TableNameIdent(ident))
                    // }
                    ::syn::Expr::Path(expr_path) => {
                        let ident = expr_path
                            .path
                            .get_ident()
                            .ok_or_else(|| ::darling::Error::custom("Expected an identifier."))?;
                        Ok($ident(ident.clone()))
                    }
                    _ => Err(darling::Error::custom("Expected a string literal.")),
                }
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
