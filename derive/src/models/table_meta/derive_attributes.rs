/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::str::FromStr;

use darling::{ast::Data, util, FromDeriveInput};
use proc_macro2::TokenStream;
use proc_macros_helpers::{get_crate_name, parse_lit_to_tokenstream};
use quote::quote;
use syn::{parse_quote, GenericArgument, Ident, Path, PathArguments, Type};

use crate::{
    errors::ExtractorResult,
    models::{
        CaseString, MyFieldReceiver, Permissions, PermissionsFn, Rename, RustFieldTypeSelfAllowed,
    },
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct TableDeriveAttributes {
    pub(crate) ident: syn::Ident,
    // pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: Data<util::Ignored, MyFieldReceiver>,

    #[darling(default)]
    pub(crate) rename_all: ::std::option::Option<Rename>,

    #[darling(default)]
    pub(crate) table_name: ::std::option::Option<Ident>,

    #[darling(default)]
    pub(crate) relax_table_name: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) schemafull: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) drop: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) flexible: ::std::option::Option<bool>,

    // #[darling(default, rename = "as_")]
    pub(crate) as_: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) as_fn: ::std::option::Option<syn::Path>,

    #[darling(default)]
    pub(crate) permissions: ::std::option::Option<Permissions>,

    #[darling(default)]
    pub(crate) permissions_fn: ::std::option::Option<PermissionsFn>,

    #[darling(default)]
    pub(crate) define: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) define_fn: ::std::option::Option<syn::Path>,
}

impl TableDeriveAttributes {
    pub fn struct_level_casing(&self) -> ExtractorResult<CaseString> {
        let struct_level_casing = self
            .rename_all
            .as_ref()
            .map(|case| CaseString::from_str(case.serialize.as_str()));

        let casing = match struct_level_casing {
            Some(Ok(case)) => case,
            Some(Err(e)) => return Err(darling::Error::custom(e.to_string()).into()),
            None => CaseString::Snake,
        };
        Ok(casing)
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
            ref as_fn,
            ref permissions,
            ref permissions_fn,
            ref define,
            ref define_fn,
            ..
        } = *self;

        let crate_name = get_crate_name(false);

        if (define_fn.is_some() || define.is_some())
            && (drop.is_some()
                || as_.is_some()
                || as_fn.is_some()
                || schemafull.is_some()
                || flexible.is_some()
                || permissions.is_some()
                || permissions_fn.is_some())
        {
            return Err(
                syn::Error::new_spanned(
                    self.ident.clone(),
                    "Invalid combination. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:
                            drop,
                            flexible,
                            as,
                            as_fn,
                            schemafull,
                            permissions,
                            permissions_fn",
                )
                .into(),
            );
        }

        let mut define_table: Option<TokenStream> = None;
        let mut define_table_methods: Vec<TokenStream> = vec![];

        match (define, define_fn){
            (Some(define), None) => {
                let define = parse_lit_to_tokenstream(define).map_err(|e| darling::Error::custom("invalid define statement"))?;
                define_table = Some(quote!(#define.to_raw()));
            },
            (None, Some(define_fn)) => {
                define_table = Some(quote!(#define_fn().to_raw()));
            },
            (Some(_), Some(_)) => return Err(
                syn::Error::new_spanned(
                    self.ident.clone(),
                    "define and define_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                )
                .into(),
            ),
            (None, None) => (),
        };

        if let Some(_drop) = drop {
            define_table_methods.push(quote!(.drop()))
        }

        if let Some(_flexible) = flexible {
            define_table_methods.push(quote!(.flexible()))
        }

        match (as_, as_fn){
            (Some(as_), None) => {
                let as_ = parse_lit_to_tokenstream(as_).map_err(|e| darling::Error::custom(format!("Invalid as expression: {e}")))?;
                define_table_methods.push(quote!(.as_(#as_)))
            },
            (None, Some(as_fn)) => {
                    define_table_methods.push(quote!(.as_(#as_fn())));
            },
            (Some(_), Some(_)) => return Err(
                darling::Error::custom(
                    "as and as_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into()
            ),
            (None, None) => (),
        };

        if let Some(_schemafull) = schemafull {
            define_table_methods.push(quote!(.schemafull()))
        }

        match (permissions, permissions_fn){
            (None, Some(p_fn)) => {
                    define_table_methods.push(p_fn.get_token_stream());
            },
            (Some(p), None) => {
                    define_table_methods.push(p.get_token_stream()?);
            },
            (Some(_), Some(_)) => return Err(darling::Error::custom("permissions and permissions_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.").into()),
            (None, None) => (),
        };

        Ok(define_table.unwrap_or_else(|| {
            quote!(
                #crate_name::statements::define_table(Self::table_name())
                #( #define_table_methods) *
                .to_raw()
            )
        }))
    }
}
