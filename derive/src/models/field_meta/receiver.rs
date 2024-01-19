/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{errors::ExtractorResult, models::DataType};

use super::*;
use darling::FromField;
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use surreal_query_builder::FieldType;
use syn::*;

#[derive(Debug, FromField)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub(crate) ident: Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    // pub(crate) ty: syn::Type,
    pub(crate) ty: RustFieldType,
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
    pub(crate) link_one: Option<LinkRustFieldType>,

    // reference singular: LinkSelf<Account>
    #[darling(default)]
    pub(crate) link_self: Option<LinkRustFieldType>,

    // reference plural: LinkMany<Account>
    #[darling(default)]
    pub(crate) link_many: Option<LinkRustFieldType>,

    #[darling(default)]
    pub(crate) nest_array: Option<LinkRustFieldType>,

    #[darling(default)]
    pub(crate) nest_object: Option<LinkRustFieldType>,

    #[darling(default)]
    pub(crate) skip_serializing: bool,

    #[darling(default)]
    pub(crate) skip: bool,

    // #[darling(default)]
    // default: ::std::option::Option<syn::Expr>,
    // #[darling(default, rename = "type")]
    pub(crate) type_: Option<DbFieldType>,

    #[darling(default)]
    pub(crate) assert: Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) assert_fn: Option<syn::Path>,

    #[darling(default)]
    pub(crate) define: Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) define_fn: Option<syn::Path>,

    #[darling(default)]
    pub(crate) value: Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) value_fn: Option<syn::Path>,

    #[darling(default)]
    pub(crate) permissions: Option<Permissions>,

    #[darling(default)]
    pub(crate) permissions_fn: Option<PermissionsFn>,

    // #[darling(default)]
    // pub(crate) item_type: Option<FieldTypeWrapper>,
    #[darling(default)]
    item_assert: Option<syn::LitStr>,

    #[darling(default)]
    item_assert_fn: Option<syn::Path>,

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
    pub fn replace_self_in_type_str(
        &self,
        struct_name_ident: &Ident,
        struct_generics: &Generics,
    ) -> Type {
        let (_, struct_ty_generics, _) = struct_generics.split_for_impl();
        replace_self_in_type_str(&self.ty, struct_name_ident, &struct_ty_generics)
    }

    pub fn get_field_generics_meta<'a>(
        &self,
        struct_name_ident: &Ident,
        struct_generics: &Generics,
    ) -> FieldGenericsMeta<'a> {
        let (_, struct_ty_generics, _) = struct_generics.split_for_impl();
        let field_type =
            &replace_self_in_type_str(&self.ty, struct_name_ident, &struct_ty_generics);
        let mut field_extractor = GenericTypeExtractor::new(struct_generics);
        let (field_impl_generics, field_ty_generics, field_where_clause) = field_extractor
            .extract_generics_for_complex_type(&field_type)
            .split_for_impl();
        FieldGenericsMeta {
            field_impl_generics,
            field_ty_generics,
            field_where_clause,
        }
    }

    // pub fn get_db_type(&self) -> ExtractorResult<DbFieldTypeMeta> {}

    pub fn get_db_type(
        &self,
        field_name: &FieldNameNormalized,
        model_type: &DataType,
        table: &Ident,
        // field_impl_generics: &syn::Generics,
        // field_ty_generics: &syn::Generics,
    ) -> ExtractorResult<Option<DbFieldTypeAst>> {
        let db_field_type_string = self.type_;
    }

    pub fn get_fallback_array_item_concrete_type(&self) -> ExtractorResult<TokenStream> {
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
        Ok(value)
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
        explicit_ty_is_numeric || self.rust_type().is_numeric()
    }

    pub fn is_list(&self) -> bool {
        let field_type = self
            .type_
            .clone()
            .map_or(FieldType::Any, |t| t.into_inner());
        let explicit_ty_is_list =
            matches!(field_type, FieldType::Array(_, _) | FieldType::Set(_, _));
        explicit_ty_is_list
            || self.rust_type().is_list()
            || self.type_.as_ref().map_or(false, |t| t.deref().is_array())
            || self.link_many.is_some()
    }

    pub fn rust_type(&self) -> RustFieldType {
        let rust_type = RustFieldType::new(self.ty);
        rust_type
    }
}
