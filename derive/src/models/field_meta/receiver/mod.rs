/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

mod core;
mod define_statement;
mod field_ident;
mod field_value_setter;
mod generics;
mod link_methods;
mod relate;
mod serialized_field_fmts;
mod simple;
mod updater_non_null;

pub use core::*;
pub use define_statement::*;
pub use field_value_setter::*;
pub use generics::*;
pub use link_methods::*;
pub use relate::*;
pub use serialized_field_fmts::*;
pub use simple::*;
pub use updater_non_null::*;

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, CaseString, DataType},
};

use super::*;
use darling::FromField;
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::quote;
use surreal_query_builder::FieldType;
use syn::*;

#[derive(Debug, FromField)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub(crate) ident: Option<FieldIdentOriginal>,
    /// This magic field name pulls the type from the input.
    pub(crate) ty: CustomType,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub(crate) old_name: Option<OldFieldName>,

    #[darling(default)]
    pub(crate) rename: Option<Rename>,

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) relate: Option<Relate>,

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) link_one: Option<LinkOneAttrType>,

    // reference singular: LinkSelf<Account>
    #[darling(default)]
    pub(crate) link_self: Option<LinkSelfAttrType>,

    // reference plural: LinkMany<Account>
    #[darling(default)]
    pub(crate) link_many: Option<LinkManyAttrType>,

    #[darling(default)]
    pub(crate) nest_array: Option<NestArrayAttrType>,

    #[darling(default)]
    pub(crate) nest_object: Option<NestObjectAttrType>,

    #[darling(default)]
    pub(crate) skip_serializing: bool,

    #[darling(default)]
    pub(crate) skip: bool,

    #[darling(default, rename = "ty")]
    pub(crate) field_type_db: Option<FieldTypeDb>,

    #[darling(default)]
    pub(crate) assert: Option<AttributeAssert>,

    #[darling(default)]
    pub(crate) define: Option<AttributeDefine>,

    #[darling(default)]
    pub(crate) value: Option<AttributeValue>,

    #[darling(default)]
    pub(crate) permissions: Option<Permissions>,

    #[darling(default)]
    pub(crate) item_assert: Option<AttributeItemAssert>,

    #[darling(default)]
    skip_serializing_if: ::darling::util::Ignored,

    #[darling(default)]
    with: ::darling::util::Ignored,

    #[darling(default)]
    deserialize_with: ::darling::util::Ignored,

    #[darling(default)]
    default: ::darling::util::Ignored,
}
