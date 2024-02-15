use std::fmt::{Display, Formatter};

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::Ident;

use crate::models::DataType;

#[derive(Debug, PartialEq, Eq)]
pub struct DbFieldName(String);

impl Display for DbFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToTokens for DbFieldName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = &self.to_string();
        tokens.extend(quote!(#s));
    }
}

impl DbFieldName {
    pub fn is_id(&self) -> bool {
        self.0 == "id"
    }

    pub fn is_in_edge_node(&self, model_type: DataType) -> bool {
        model_type.is_edge() && self.0 == "in"
    }

    pub fn is_out_edge_node(&self, model_type: DataType) -> bool {
        model_type.is_edge() && self.0 == "out"
    }

    pub fn is_orig_or_dest_edge_node(&self, model_type: &DataType) -> bool {
        model_type.is_edge() && (self.0 == "in" || self.0 == "out")
    }

    pub fn is_updateable_by_default(&self, model_type: &DataType) -> bool {
        let not_updateable = self.is_id() || self.is_orig_or_dest_edge_node(model_type);
        !not_updateable
    }
}

impl From<Ident> for DbFieldName {
    fn from(s: Ident) -> Self {
        Self(s.to_string())
    }
}
