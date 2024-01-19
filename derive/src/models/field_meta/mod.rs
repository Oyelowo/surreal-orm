#![allow(missing_docs)]
#![allow(dead_code)]
/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

use crate::models::{
    replace_lifetimes_with_underscore, replace_self_in_type_str, GenericTypeExtractor,
};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    errors::ExtractorResult,
    field_rust_type::DbFieldTypeManager,
    get_crate_name, parse_lit_to_tokenstream,
    parser::DataType,
    relations::{NodeType, NodeTypeName},
    variables::VariablesModelMacro,
    FieldNameNormalized,
};
use darling::{ast::Data, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use surreal_query_builder::FieldType;
use syn::{Generics, Ident, Lit, LitStr, Path, Type};

#[derive(Debug, Clone, Default)]
pub struct DbFieldTypeMeta {
    pub(crate) db_field_type: TokenStream,
    pub(crate) static_assertion: TokenStream,
}
