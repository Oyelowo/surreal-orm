#[macro_export]
macro_rules! create_ident_wrapper {
    ($ident:ident) => {
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

        impl From<::syn::Ident> for $ident {
            fn from(ident: ::syn::Ident) -> Self {
                Self(ident)
            }
        }

        impl ::quote::ToTokens for $ident {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                self.0.to_tokens(tokens)
            }
        }
    };
}
pub use create_ident_wrapper;

create_ident_wrapper!(FieldIdentOriginal);
create_ident_wrapper!(OldFieldName);
