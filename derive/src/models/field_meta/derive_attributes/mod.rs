/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

mod generics;
mod ident;
mod types;

pub use generics::FieldGenericsMeta;

use crate::{
    errors::ExtractorResult,
    models::{
        create_ident_wrapper, derive_attributes::TableDeriveAttributes,
        field_name_serialized::DbFieldName, CaseString, CustomType, DataType, DbFieldTypeAstMeta,
        FieldIdentOriginal, FieldTypeDb, LinkManyAttrType, LinkOneAttrType, LinkSelfAttrType,
        NestArrayAttrType, NestObjectAttrType, Relate, RelationType, Rename, StructLevelCasing,
    },
};

use super::{AttributeAssert, AttributeDefine, AttributeItemAssert, AttributeValue, Permissions};

create_ident_wrapper!(IdentCased);
create_ident_wrapper!(FieldIdentNormalized);
create_ident_wrapper!(FieldNamePascalized);
create_ident_wrapper!(FieldIdentOriginal);
create_ident_wrapper!(OldFieldName);

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
