/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use super::{ident::{FieldAttribute, FieldIdentNormalizedDeserialized}, renaming};
use crate::models::{CaseString, ExtractorResult, StructGenerics};

use darling::{ast::Data, util, FromDeriveInput};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};
use std::str::FromStr;
use syn::{Ident, Type};


#[derive(Clone, Debug, FromDeriveInput)]
#[darling(attributes(pick, serde), forward_attrs(allow, doc, cfg))]
pub struct TableDeriveAttributesPickable {
    pub(crate) ident: Ident,
    // pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: StructGenerics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: Data<util::Ignored, FieldAttribute>,

    #[darling(default)]
    pub(crate) rename_all: Option<renaming::RenameDeserialize>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PickableMetadata<'a> {
    pub(crate) field_name_normalized_deserialized: Vec<FieldIdentNormalizedDeserialized>,
    pub(crate) field_type: Vec<&'a Type>,
}

impl TableDeriveAttributesPickable {
    pub fn casing_deserialize(&self) -> ExtractorResult<renaming::StructLevelCasingDeserialize> {
        let struct_level_casing = self
            .rename_all
            .as_ref()
            .map(|case| CaseString::from_str(case.deserialize.as_str()));

        let casing = match struct_level_casing {
            Some(Ok(case)) => case,
            Some(Err(e)) => return Err(darling::Error::custom(e.to_string()).into()),
            None => CaseString::None,
        };
        Ok(casing.into())
    }

    pub(crate) fn get_meta(&self) -> ExtractorResult<PickableMetadata> {
        let fields =
            self.data.as_ref().take_struct().ok_or(
                darling::Error::custom("Only structs are supported").with_span(&self.ident),
            )?;
        let struct_casing_de = self.casing_deserialize()?;

        let mut meta = PickableMetadata::default();

        for field_attr in fields {
            let f = field_attr.field_ident_normalized_deserialized_rawable(&struct_casing_de)?;
            meta.field_name_normalized_deserialized.push(f);
            meta.field_type.push(&field_attr.ty);
        }

        Ok(meta)
    }
}

impl ToTokens for TableDeriveAttributesPickable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_name = get_crate_name(false);
        let table_derive_attributes = self;
        let struct_name_ident = &table_derive_attributes.ident;
        let (struct_impl_generics, struct_ty_generics, struct_where_clause) =
            &table_derive_attributes.generics.split_for_impl();
        let meta = match table_derive_attributes.get_meta() {
            Ok(meta) => meta,
            Err(err) => return tokens.extend(err.write_errors()),
        };
        let PickableMetadata {
            field_name_normalized_deserialized,
            field_type,
        } = meta;

        // use std::any::Any;
        //
        // struct Person<'a, T: 'a, U: 'a> {
        //     name: String,
        //     age: u8,
        //     some: &'a T,
        //     another: &'a U,
        // }
        //
        // trait PersonPickable {
        //     type name;
        //     type age;
        //     type some;
        //     type another;
        // }
        //
        // impl<'a, T: 'a, U: 'a> PersonPickable for Person<'a, T, U> {
        //     type name = String;
        //     type age = u8;
        //     type some = &'a T;
        //     type another = &'a U;
        // }
        let pickable_name = format_ident!("{struct_name_ident}Pickable");
        tokens.extend(quote!(
            pub trait #pickable_name {
                #( type #field_name_normalized_deserialized ;) *
            }

            impl #struct_impl_generics #pickable_name for #struct_name_ident #struct_ty_generics #struct_where_clause {
                #( type #field_name_normalized_deserialized = #field_type ;) *
            }

        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use darling::FromDeriveInput;

    #[test]
    fn test_table_derive_attributes_pickable() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person {
                name: String,
                age: u8,
                some: u32,
                another: String,
            }
        };

        let table_derive_attributes = TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl PersonPickable for Person {
                type name = String;
                type age = u8;
                type some = u32;
                type another = String;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }
}
