/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::quote;
use syn::Ident;

use super::*;
use crate::{
    errors::ExtractorResult,
    models::{
        derive_attributes::TableDeriveAttributes, variables::VariablesModelMacro, DataType,
        LinkRustFieldType,
    },
};

#[derive(Default, Clone)]
pub(crate) struct ReferencedNodeMeta {
    pub(crate) foreign_node_type: TokenStream,
    pub(crate) foreign_node_schema_import: TokenStream,
    pub(crate) record_link_default_alias_as_method: TokenStream,
    pub(crate) foreign_node_type_validator: TokenStream,
    pub(crate) field_definition: TokenStream,
    pub(crate) field_type_validation_asserts: Vec<TokenStream>,
}

impl ReferencedNodeMeta {
    pub(crate) fn from_simple_array(normalized_field_name: &::syn::Ident) -> Self {}

