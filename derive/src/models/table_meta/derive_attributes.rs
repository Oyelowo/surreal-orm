/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::{ast::Data, util, FromDeriveInput};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use syn::Ident;

use super::table_name::TableNameIdent;
use crate::models::*;

create_ident_wrapper!(StructIdent);

impl StructIdent {
    pub fn is_same_name(&self, other: impl Into<CustomType>) -> ExtractorResult<bool> {
        let other: CustomType = other.into();
        Ok(self.to_string() == other.type_name()?.to_string())
    }
}

#[derive(Clone, Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct TableDeriveAttributes {
    pub(crate) ident: Ident,
    // pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: StructGenerics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: Data<util::Ignored, MyFieldReceiver>,

    #[darling(default)]
    pub(crate) rename_all: Option<Rename>,

    // #[darling(default)]
    pub(crate) table: TableNameIdent,

    #[darling(default)]
    pub(crate) relax_table_name: Option<bool>,

    #[darling(default)]
    pub(crate) schemafull: Option<bool>,

    #[darling(default)]
    pub(crate) drop: Option<bool>,

    #[darling(default)]
    pub(crate) flexible: Option<bool>,

    // #[darling(default, rename = "as_")]
    pub(crate) as_: Option<AttributeAs>,

    pub(crate) permissions: Option<Permissions>,

    #[darling(default)]
    pub(crate) define: Option<AttributeDefine>,
}

impl TableDeriveAttributes {
    pub fn ident(&self) -> StructIdent {
        StructIdent(self.ident.clone())
    }

    pub fn generics(&self) -> &StructGenerics {
        &self.generics
    }

    pub fn table_name(&self) -> ExtractorResult<&TableNameIdent> {
        // TODO: Ask during alpha release if specifying table name explicitly
        // should be optional since it can be inferred from the struct name
        // as the snake case version of the struct name.
        self.table
            .validate_and_return(&self.ident(), &self.relax_table_name)
    }

    pub fn get_table_definition_token(&self) -> ExtractorResult<TableDefinitions> {
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

        Ok(define_table
            .unwrap_or_else(|| {
                quote!(
                    #crate_name::statements::define_table(Self::table_name())
                    #( #define_table_methods) *
                    .to_raw()
                )
            })
            .into())
    }
}
