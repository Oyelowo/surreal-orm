/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromField;
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::quote;
use surreal_query_builder::FieldType;
use syn::*;

mod aliases;
mod define_statement;
mod field_connection_build;
mod field_metadata;
mod field_type_assertions;
mod field_value_setter;
mod link_methods;
mod relate;
mod serialized_field_fmts;
mod simple;
mod updater_non_null;

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, CaseString, DataType},
};
