/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use syn::{
    self, parse_quote, spanned::Spanned, visit_mut::VisitMut, GenericArgument, Ident, Lifetime,
    Path, PathArguments, PathSegment, Type, TypeReference,
};

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, DataType},
};

use super::*;

#[derive(Debug, Clone)]
pub struct DestinationNodeTypeOriginal(pub CustomType);

impl DestinationNodeTypeOriginal {
    pub fn replace_self_with_current_struct_ident(
        &self,
        table_def: &TableDeriveAttributes,
    ) -> DestinationNodeTypeNoSelf {
        DestinationNodeTypeNoSelf(self.0.replace_self_with_current_struct_ident(table_def))
    }
}

impl FromMeta for DestinationNodeTypeOriginal {
    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        let ty = match item {
            syn::Meta::Path(path) => {
                let ty = path
                    .segments
                    .last()
                    .ok_or_else(|| darling::Error::custom("Expected a type"))?;
                syn::parse_str::<syn::Type>(&ty.to_string())?
            }
            _ => return Err(darling::Error::custom("Expected a type").with_span(&item.span())),
        };
        Ok(Self(RustFieldTypeSelfAllowed::new(ty)))
    }
}

impl ToTokens for DestinationNodeTypeOriginal {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

#[derive(Debug, Clone)]
pub struct DestinationNodeTypeNoSelf(pub CustomTypeNoSelf);

impl DestinationNodeTypeNoSelf {
    pub fn to_basic_type(self) -> Type {
        self.0.to_basic_type()
    }
}

impl ToTokens for DestinationNodeTypeNoSelf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_basic_type().to_tokens(tokens)
    }
}
