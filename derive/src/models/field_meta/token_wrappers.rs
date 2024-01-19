#[derive(Debug, Clone)]
struct DbfieldTypeToken(TokenStream);

impl Default for DbfieldTypeToken {
    fn default() -> Self {
        let crate_name = get_crate_name(false);
        Self(quote!(#crate_name::FieldType::Any))
    }
}

impl From<TokenStream> for DbfieldTypeToken {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}
impl ToTokens for DbfieldTypeToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

#[derive(Debug, Clone, Default)]
struct StaticAssertionToken(TokenStream);
impl ToTokens for StaticAssertionToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}
impl From<TokenStream> for StaticAssertionToken {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}
