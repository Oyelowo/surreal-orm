/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{fmt::Display, str::FromStr};

use darling::{ast::Data, util, FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use proc_macros_helpers::{get_crate_name, parse_lit_to_tokenstream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, GenericArgument, Ident, Path, PathArguments, Type};

use crate::{
    errors::ExtractorResult,
    models::{
        create_ident_wrapper, AttributeAs, AttributeDefine, CaseString, CustomType,
        MyFieldReceiver, Permissions, Rename, StructGenerics, StructLevelCasing,
    },
};

// Struct name
create_ident_wrapper!(StructIdent);

impl StructIdent {
    pub fn is_same_name(&self, other: impl Into<CustomType>) -> ExtractorResult<bool> {
        let other: CustomType = other.into();
        Ok(self.to_string() == other.type_name()?.to_string())
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct TableDeriveAttributes {
    // pub(crate) ident: syn::Ident,
    pub(crate) ident: StructIdent,
    // pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: StructGenerics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: Data<util::Ignored, MyFieldReceiver>,

    #[darling(default)]
    pub(crate) rename_all: ::std::option::Option<Rename>,

    // #[darling(default)]
    pub(crate) table_name: Ident,

    #[darling(default)]
    pub(crate) relax_table_name: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) schemafull: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) drop: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) flexible: ::std::option::Option<bool>,

    // #[darling(default, rename = "as_")]
    pub(crate) as_: ::std::option::Option<AttributeAs>,

    #[darling(default)]
    pub(crate) permissions: ::std::option::Option<Permissions>,

    #[darling(default)]
    pub(crate) define: ::std::option::Option<AttributeDefine>,
}

impl TableDeriveAttributes {
    pub fn table_name(&self) -> ExtractorResult<Ident> {
        Ok(self.table_name)
    }

    pub fn casing(&self) -> ExtractorResult<StructLevelCasing> {
        let struct_level_casing = self
            .rename_all
            .as_ref()
            .map(|case| CaseString::from_str(case.serialize.as_str()));

        let casing = match struct_level_casing {
            Some(Ok(case)) => case,
            Some(Err(e)) => return Err(darling::Error::custom(e.to_string()).into()),
            None => CaseString::None,
        };
        Ok(casing.into())
    }

    pub fn struct_as_path_no_bounds(&self) -> Path {
        // let replacement_path: Path = parse_quote!(#struct_name #ty_generics);
        self.construct_type_without_bounds()
            .replace_self_with_struct_concrete_type(self)
            .to_path()
    }

    pub fn construct_type_without_bounds(&self) -> RustFieldTypeSelfAllowed {
        let mut path = Path::from(self.ident.clone());
        let generics = self.generics;

        // Process generics, excluding bounds
        if !generics.params.is_empty() {
            let args = generics
                .params
                .iter()
                .map(|param| match param {
                    syn::GenericParam::Type(type_param) => {
                        GenericArgument::Type(parse_quote!(#type_param))
                    }
                    syn::GenericParam::Lifetime(lifetime_def) => {
                        GenericArgument::Lifetime(lifetime_def.lifetime.clone())
                    }
                    syn::GenericParam::Const(const_param) => {
                        // TODO: Test this in struct
                        GenericArgument::Const(
                            const_param
                                .default
                                .clone()
                                .expect("absent const expression"),
                        )
                    }
                })
                .collect();

            path.segments.last_mut().unwrap().arguments =
                PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: generics.lt_token.unwrap(),
                    args,
                    gt_token: generics.gt_token.unwrap(),
                });
        }

        Type::Path(syn::TypePath { qself: None, path }).into()
    }
    pub fn get_table_definition_token(&self) -> ExtractorResult<TokenStream> {
        let TableDeriveAttributes {
            ref drop,
            ref flexible,
            ref schemafull,
            ref as_,
            ref permissions,
            ref define,
            ..
        } = *self;

        let crate_name = get_crate_name(false);

        if (define.is_some())
            && (drop.is_some()
                || as_.is_some()
                || schemafull.is_some()
                || flexible.is_some()
                || permissions.is_some())
        {
            return Err(
                syn::Error::new_spanned(
                    self.ident.clone(),
                    "Invalid combination. When `define`, the following attributes cannot be use in combination to prevent confusion:
                            drop,
                            flexible,
                            as,
                            schemafull,
                            permissions",
                )
                .into(),
            );
        }

        let mut define_table: Option<TokenStream> = None;
        let mut define_table_methods: Vec<TokenStream> = vec![];

        if let Some(define) = define {
            define_table = Some(quote!(#define.to_raw()));
        }

        if let Some(_drop) = drop {
            define_table_methods.push(quote!(.drop()))
        }

        if let Some(_flexible) = flexible {
            define_table_methods.push(quote!(.flexible()))
        }

        if let Some(as_) = as_ {
            define_table_methods.push(quote!(.as_(#as_)))
        }

        if let Some(_schemafull) = schemafull {
            define_table_methods.push(quote!(.schemafull()))
        }

        if let Some(permissions) = permissions {
            define_table_methods.push(permissions.to_token_stream());
        }

        Ok(define_table.unwrap_or_else(|| {
            quote!(
                #crate_name::statements::define_table(Self::table_name())
                #( #define_table_methods) *
                .to_raw()
            )
        }))
    }
}
