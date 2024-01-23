use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Clone)]
pub struct TokenStreamHashable(TokenStream);

impl ToTokens for TokenStreamHashable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.clone());
    }
}

impl Deref for TokenStreamHashable {
    type Target = TokenStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TokenStream> for TokenStreamHashable {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}

impl Eq for TokenStreamHashable {}

impl PartialEq for TokenStreamHashable {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl std::hash::Hash for TokenStreamHashable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}
