/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    errors::ExtractorResult,
    models::{CaseString, DataType},
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
    pub(crate) ident: Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    pub(crate) ty: CustomType,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub(crate) old_name: Option<Ident>,

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
    pub(crate) type_: Option<DbFieldType>,

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

impl MyFieldReceiver {
    pub fn normalize_ident(&self, struct_level_casing: CaseString) -> NormalisedFieldMeta {
        NormalisedFieldMeta::from_receiever(self, struct_level_casing)
    }

    pub fn get_db_type(&self) -> ExtractorResult<DbFieldType> {
        // TODO: Handle error incase heuristics does not work and user does not specify
        Ok(self.type_.clone())
    }

    pub fn get_db_type_with_assertion(
        &self,
        field_name: &FieldIdentSerialized,
        model_type: &DataType,
        table: &Ident,
        // field_impl_generics: &syn::Generics,
        // field_ty_generics: &syn::Generics,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        // Infer/use user specified or error out
        // TODO: Add the compile time assertion/validations/checks for the dbtype here
        Ok(DbFieldTypeAstMeta {
            db_field_type: self.type_,
            static_assertion: todo!(),
        })
    }

    pub fn get_fallback_array_item_concrete_db_type(&self) -> ExtractorResult<DbfieldTypeToken> {
        let field_type = self
            .type_
            .clone()
            .map_or(FieldType::Any, |t| t.into_inner());

        let item_type = match field_type {
            FieldType::Array(item_type, _) => item_type,
            // TODO: Check if to error out here or just use Any
            _ => return Err(syn::Error::new_spanned(&self.ty, "Not an array").into()),
            // _ => Box::new(FieldType::Any),
        };

        let crate_name = get_crate_name(false);
        let value = match item_type.deref() {
            FieldType::Any => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Null => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Uuid => {
                quote!(#crate_name::sql::Uuid)
            }
            FieldType::Bytes => {
                quote!(#crate_name::sql::Bytes)
            }
            FieldType::Union(_) => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Option(_) => {
                quote!(::std::option::Option<#crate_name::sql::Value>)
            }
            FieldType::String => {
                quote!(::std::string::String)
            }
            FieldType::Int => {
                // quote!(#crate_name::validators::Int)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Float => {
                // quote!(#crate_name::validators::Float)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Bool => {
                quote!(::std::convert::Into<::std::primitive::bool>)
            }
            FieldType::Array(_, _) => {
                // quote!(::std::iter::IntoIterator)
                // quote!(::std::convert::Into<#crate_name::sql::Array>)
                quote!(::std::vec::Vec<#crate_name::sql::Value>)
            }
            FieldType::Set(_, _) => {
                quote!(::std::collections::HashSet<#crate_name::sql::Value>)
            }
            FieldType::Datetime => {
                quote!(#crate_name::sql::Datetime)
            }
            FieldType::Decimal => {
                quote!(#crate_name::validators::Float)
            }
            FieldType::Duration => {
                quote!(#crate_name::sql::Duration)
            }
            FieldType::Number => {
                // quote!(#crate_name::validators::Num)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Object => {
                quote!(#crate_name::sql::Object)
            }
            FieldType::Record(_) => {
                quote!(::std::convert::Option<#crate_name::sql::Thing>)
            }
            FieldType::Geometry(_) => {
                quote!(#crate_name::sql::Geometry)
            }
        };
        Ok(value.into())
    }

    pub fn is_numeric(&self) -> bool {
        let field_type = self
            .type_
            .clone()
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_numeric = matches!(
            field_type,
            FieldType::Int | FieldType::Float | FieldType::Decimal | FieldType::Number
        );
        explicit_ty_is_numeric || self.rust_field_type().is_numeric()
    }

    pub fn is_list(&self) -> bool {
        let field_type = self
            .type_
            .clone()
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list =
            matches!(field_type, FieldType::Array(_, _) | FieldType::Set(_, _));
        explicit_ty_is_list
            || self.rust_field_type().is_list()
            || self.type_.as_ref().map_or(false, |t| t.deref().is_array())
            || self.link_many.is_some()
    }

    pub fn rust_field_type(&self) -> RustFieldTypeSelfAllowed {
        self.ty
    }
}
