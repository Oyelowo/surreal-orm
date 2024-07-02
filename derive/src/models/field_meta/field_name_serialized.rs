/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{Display, Formatter};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::models::{create_ident_wrapper, DataType};

#[derive(Debug, PartialEq, Eq)]
pub struct DbFieldName(String);

create_ident_wrapper!(DbFieldNameAsIdent);

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
    pub fn as_ident(&self) -> DbFieldNameAsIdent {
        format_ident!("{self}").into()
    }

    pub fn is_id(&self) -> bool {
        self.0 == "id"
    }

    pub fn is_in_edge_node(&self, model_type: DataType) -> bool {
        model_type.is_edge() && self.0 == "in"
    }

    pub fn is_out_edge_node(&self, model_type: DataType) -> bool {
        model_type.is_edge() && self.0 == "out"
    }

    pub fn is_in_or_out_edge_node(&self, model_type: &DataType) -> bool {
        model_type.is_edge() && (self.0 == "in" || self.0 == "out")
    }


    // TODO: Confirm in new surrealdb version if we can now update in and out fields
    // in edge tables and id field in all tables(i doubt we should be able to for id field but
    // maybe for in and out).
    pub fn is_updateable_by_default(&self, model_type: &DataType) -> bool {
        let not_updateable = self.is_id() || self.is_in_or_out_edge_node(model_type);
        !not_updateable
    }
}

impl From<Ident> for DbFieldName {
    fn from(s: Ident) -> Self {
        Self(s.to_string())
    }
}
