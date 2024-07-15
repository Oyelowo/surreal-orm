/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{fmt::Display, ops::Deref};

use crate::models::*;
use convert_case::{Case, Casing};
use quote::format_ident;

use super::{FieldAttribute, StructLevelCasingDeserialize};

// use super::{FieldIdentNormalized, FieldNamePascalized, IdentCased, MyFieldReceiver};

create_ident_wrapper!(FieldIdentNormalizedDeserialized);
create_ident_wrapper!(DeserializedFieldName);

// struct DeserializedFieldName(String);

// impl Display for DeserializedFieldName {
//     #[inline]
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         self.0.fmt(f)
//     }
// }
//
// impl ToTokens for DeserializedFieldName {
//     #[inline]
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         self.0.to_tokens(tokens)
//     }
// }


impl FieldAttribute {
    pub fn ident(&self) -> ExtractorResult<FieldIdentOriginal> {
        Ok(self
            .ident
            .as_ref()
            .ok_or(darling::Error::custom("Field must have an identifier"))?
            .clone()
            .into())
    }

    pub fn field_ident_normalized_deserialized_rawable(
        &self,
        struct_casing: &StructLevelCasingDeserialize,
    ) -> ExtractorResult<FieldIdentNormalizedDeserialized> {
        Ok(self.ident_meta_deserialized(struct_casing)?.0)
    }

    pub fn field_name_normaized_de_no_raw(&self, struct_casing: &StructLevelCasingDeserialize) -> ExtractorResult<DeserializedFieldName> {
        Ok(self.ident_meta_deserialized(struct_casing)?.1)
    }

    pub fn field_name_pascalized(
        &self,
        struct_casing: &StructLevelCasingDeserialize,
    ) -> ExtractorResult<FieldNamePascalized> {
        let field_name_normalized = self.field_ident_normalized_deserialized_rawable(struct_casing)?;

        let field_name_pascalized = format_ident!(
            "__{}__",
            field_name_normalized
                .to_string()
                .trim_start_matches("r#")
                .to_case(Case::Pascal)
        );

        Ok(field_name_pascalized.into())
    }

    fn ident_meta_deserialized(
        &self,
        struct_casing: &StructLevelCasingDeserialize,
    ) -> ExtractorResult<(FieldIdentNormalizedDeserialized, DeserializedFieldName)> {
        let field_ident_original = self.ident()?;
        let field_ident_cased =
            || Self::convert_case(field_ident_original.to_string(), struct_casing).to_string();

        let field_ident_normalized_de = &self
            .rename
            .as_ref()
            .map_or_else(field_ident_cased, |renamed| renamed.deserialize.clone());

        let (field_ident_normalized, field_name_serialized) = if field_ident_normalized_de
            .starts_with("r#")
            || RustReservedKeyword::is_keyword(field_ident_normalized_de)
        {
            let field_ident_normalized = field_ident_normalized_de.trim_start_matches("r#");
            (
                syn::Ident::new_raw(field_ident_normalized, field_ident_original.span()),
                syn::Ident::new(field_ident_normalized, field_ident_original.span()),
            )
        } else {
            (
                syn::Ident::new(field_ident_normalized_de, field_ident_original.span()),
                syn::Ident::new(field_ident_normalized_de, field_ident_original.span()),
            )
        };

        Ok((field_ident_normalized.into(), field_name_serialized.into()))
    }

    fn convert_case(ident: impl Into<String>, casing: &StructLevelCasingDeserialize) -> IdentCased {
        let ident: String = ident.into();
        let ident = match casing.deref() {
            CaseString::None => ident,
            CaseString::Camel => ident.to_case(Case::Camel),
            CaseString::Snake => ident.to_case(Case::Snake),
            CaseString::Pascal => ident.to_case(Case::Pascal),
            CaseString::Lower => ident.to_case(Case::Lower),
            CaseString::Upper => ident.to_case(Case::Upper),
            CaseString::ScreamingSnake => ident.to_case(Case::ScreamingSnake),
            CaseString::Kebab => ident.to_case(Case::Kebab),
            CaseString::ScreamingKebab => ident.to_case(Case::ScreamingSnake),
        };
        format_ident!("{ident}").into()
    }
}
