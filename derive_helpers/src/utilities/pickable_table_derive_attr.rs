/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use super::ident::{FieldAttribute, FieldIdentNormalizedDeserialized};
use crate::models::{CaseString, ExtractorResult, Rename, StructGenerics, StructLevelCasing};

use darling::{ast::Data, util, FromDeriveInput};
use proc_macro2::TokenStream;
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
    pub(crate) rename_all: Option<Rename>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PickableMetadata<'a> {
    pub(crate) field_name_normalized_deserialized: Vec<FieldIdentNormalizedDeserialized>,
    pub(crate) field_type: Vec<&'a Type>,
}

impl TableDeriveAttributesPickable {
    pub fn casing(&self) -> ExtractorResult<StructLevelCasing> {
        let struct_level_casing = self
            .rename_all
            .as_ref()
            .map(|case| case.serialize.clone())
            .flatten()
            .map(|case| CaseString::from_str(case.as_str()));

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
        let struct_casing_de = self.casing()?;

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

        let pickable_name = format_ident!("{struct_name_ident}Pickable");
        tokens.extend(quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait #pickable_name {
                #(
                    type #field_name_normalized_deserialized ;
                ) *
            }

            impl #struct_impl_generics #pickable_name for #struct_name_ident #struct_ty_generics #struct_where_clause {
                #( 
                    type #field_name_normalized_deserialized = #field_type ;
                ) *
            }

        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use darling::FromDeriveInput;

    #[test]
    fn test_table_derive_attributes_pickable_without_generics() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person {
                name: String,
                age: u8,
                some: u32,
                another: String,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
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

    #[test]
    fn test_table_derive_attributes_pickable_with_single_lifetime_generics() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person<'a> {
                name: String,
                age: u8,
                some: &'a u32,
                another: &'a String,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl<'a> PersonPickable for Person<'a> {
                type name = String;
                type age = u8;
                type some = &'a u32;
                type another = &'a String;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_table_derive_attributes_pickable_with_multiple_lifetime_generics() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person<'a, 'b> {
                name: String,
                age: u8,
                some: &'a u32,
                another: &'b String,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl<'a, 'b> PersonPickable for Person<'a, 'b> {
                type name = String;
                type age = u8;
                type some = &'a u32;
                type another = &'b String;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_table_derive_attributes_pickable_with_single_generic() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person<T> {
                name: String,
                age: u8,
                some: T,
                another: String,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl<T> PersonPickable for Person<T> {
                type name = String;
                type age = u8;
                type some = T;
                type another = String;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_derive_table_attributes_pickable_with_multiple_generic() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person<T, U> {
                name: String,
                age: u8,
                some: T,
                another: U,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl<T, U> PersonPickable for Person<T, U> {
                type name = String;
                type age = u8;
                type some = T;
                type another = U;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_table_derive_attributes_pickable_with_single_lifetime_and_type_generic() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person<'a, T> {
                name: String,
                age: u8,
                some: &'a T,
                another: String,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl<'a, T> PersonPickable for Person<'a, T> {
                type name = String;
                type age = u8;
                type some = &'a T;
                type another = String;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_derive_table_attributes_pickable_with_multiple_lifetime_and_type_generic() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person<'a, 'b, T, U> {
                name: String,
                age: u8,
                some: &'a T,
                another: &'b U,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl<'a, 'b, T, U> PersonPickable for Person<'a, 'b, T, U> {
                type name = String;
                type age = u8;
                type some = &'a T;
                type another = &'b U;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_table_derive_attributes_pickable_with_single_lifetime_and_multiple_type_generic() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person<'a, T, U> {
                name: String,
                age: u8,
                some: &'a T,
                another: U,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl<'a, T, U> PersonPickable for Person<'a, T, U> {
                type name = String;
                type age = u8;
                type some = &'a T;
                type another = U;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_table_derive_attributes_pickable_with_lifetime_and_trait_bounds() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            struct Person<'a, 'b: 'a, T: Display + 'a, U: Debug + 'b> {
                name: String,
                age: u8,
                some: &'a T,
                another: U,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type name;
                type age;
                type some;
                type another;
            }

            impl<'a, 'b: 'a, T: Display + 'a, U: Debug + 'b> PersonPickable for Person<'a, 'b, T, U> {
                type name = String;
                type age = u8;
                type some = &'a T;
                type another = U;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_derive_table_attributes_pickable_with_rename_all() {
        let input = syn::parse_quote! {
            #[derive(Pickable)]
            #[serde(rename_all = "camelCase")]
            struct Person {
                first_name: String,
                age: u8,
                some: u32,
                another: String,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]
            pub trait PersonPickable {
                type firstName;
                type age;
                type some;
                type another;
            }

            impl PersonPickable for Person {
                type firstName = String;
                type age = u8;
                type some = u32;
                type another = String;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    // #[serde(rename(serialize = "ser_name"))]
    // #[serde(rename(deserialize = "de_name"))]
    // #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
    #[test]
    fn test_derive_table_attributes_pickable_with_rename_all_serialize() {
        let input = syn::parse_quote! {
            #[derive(Pickable, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Person {
                // Should use camelCase inheritted from the Container struct
                first_name: String,

                // Should use "simple_rename". Field attributes supersede/overrides struct
                // container attributes in this case i.e camelCase
                #[serde(rename = "simple_rename")]
                last_name: String,

                // should use "serialized_renmed_age" since that's what we're interested in
                #[serde(rename(serialize = "serialized_renmed_age"))]
                age: u8,

                // Should stay some since we are interested in the serialized name
                #[serde(rename(deserialize = "deserialized_renamed_field"))]
                some: u32,

                // Should use "ser_name" since that's what we're interested in
                #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
                another: String,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]pub trait PersonPickable {
                type firstName;
                type simple_rename;
                type serialized_renmed_age;
                type some;
                type ser_name;
            }

            impl PersonPickable for Person {
                type firstName = String;
                type simple_rename = String;
                type serialized_renmed_age = u8;
                type some = u32;
                type ser_name = String;
            }

        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }

    #[test]
    fn test_derive_table_attributes_pickable_with_rename_all_serialize_with_lifetime_and_type_generics(
    ) {
        let input = syn::parse_quote! {
            #[derive(Pickable, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Person<'a, T> {
                // Should use camelCase inheritted from the Container struct
                first_name: String,

                // Should use "simple_rename". Field attributes supersede/overrides struct
                // container attributes in this case i.e camelCase
                #[serde(rename = "simple_rename")]
                last_name: String,

                // should use "serialized_renmed_age" since that's what we're interested in
                #[serde(rename(serialize = "serialized_renmed_age"))]
                age: u8,

                // Should stay some since we are interested in the serialized name
                #[serde(rename(deserialize = "deserialized_renamed_field"))]
                some: &'a T,

                // Should use "ser_name" since that's what we're interested in
                #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
                another: T,
            }
        };

        let table_derive_attributes =
            TableDeriveAttributesPickable::from_derive_input(&input).unwrap();

        let expected = quote!(
            #[allow(non_camel_case_types, unused)]pub trait PersonPickable {
                type firstName;
                type simple_rename;
                type serialized_renmed_age;
                type some;
                type ser_name;
            }

            impl<'a, T> PersonPickable for Person<'a, T> {
                type firstName = String;
                type simple_rename = String;
                type serialized_renmed_age = u8;
                type some = &'a T;
                type ser_name = T;
            }
        );

        let mut tokens_input = TokenStream::new();
        table_derive_attributes.to_tokens(&mut tokens_input);
        assert_eq!(tokens_input.to_string(), expected.to_string());
    }
}
