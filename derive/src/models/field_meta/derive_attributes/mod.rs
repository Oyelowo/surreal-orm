/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

mod generics;
mod ident;
mod types;

use darling::FromField;
use proc_macro2::Ident;
use syn::Type;

use crate::models::*;

use super::{AttributeAssert, AttributeDefine, AttributeItemAssert, AttributeValue, Permissions};

create_ident_wrapper!(IdentCased);
create_ident_wrapper!(FieldIdentNormalized);
create_ident_wrapper!(FieldNamePascalized);
create_ident_wrapper!(FieldIdentOriginal);
create_ident_wrapper!(OldFieldName);

#[allow(dead_code)]
#[derive(Clone, Debug, FromField)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<Ident>,
    /// This magic field name pulls the type from the input.
    ty: Type,
    attrs: Vec<syn::Attribute>,

    /// Explicity specify the array or set item/element rust type
    #[darling(default, rename= "item_rust_ty")]
    pub(crate) array_item_ty_specified: Option<ArrayItemType>,

    /// Old name of field when renaming
    #[darling(default)]
    pub(crate) old_name: Option<OldFieldName>,

    /// New name of field when renaming
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
