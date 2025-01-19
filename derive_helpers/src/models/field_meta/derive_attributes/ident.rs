/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

use std::ops::Deref;

use crate::models::*;
use convert_case::{Case, Casing};
use quote::format_ident;

use super::{FieldIdentNormalized, FieldNamePascalized, IdentCased, MyFieldReceiver};

impl MyFieldReceiver {
    pub fn ident(&self) -> ExtractorResult<FieldIdentOriginal> {
        Ok(self
            .ident
            .as_ref()
            .ok_or(darling::Error::custom("Field must have an identifier"))?
            .clone()
            .into())
    }

    pub fn field_ident_normalized(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<FieldIdentNormalized> {
        Ok(self.ident_meta(struct_casing)?.0)
    }

    pub fn db_field_name(&self, struct_casing: &StructLevelCasing) -> ExtractorResult<DbFieldName> {
        Ok(self.ident_meta(struct_casing)?.1)
    }

    pub fn field_name_pascalized(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<FieldNamePascalized> {
        let field_name_normalized = self.field_ident_normalized(struct_casing)?;

        let field_name_pascalized = format_ident!(
            "__{}__",
            field_name_normalized
                .to_string()
                .trim_start_matches("r#")
                .to_case(Case::Pascal)
        );

        Ok(field_name_pascalized.into())
    }

    fn ident_meta(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<(FieldIdentNormalized, DbFieldName)> {
        let field_ident_original = self.ident()?;
        let field_ident_cased =
            || Self::convert_case(field_ident_original.to_string(), struct_casing).to_string();

        let field_ident_normalized = &self
            .rename
            .as_ref()
            .and_then(|rename| rename.serialize.clone())
            .map_or_else(field_ident_cased, |renamed| renamed);

        let (field_ident_normalized, field_name_serialized) = if field_ident_normalized
            .starts_with("r#")
            || RustReservedKeyword::is_keyword(field_ident_normalized)
        {
            let field_ident_normalized = field_ident_normalized.trim_start_matches("r#");
            (
                syn::Ident::new_raw(field_ident_normalized, field_ident_original.span()),
                syn::Ident::new(field_ident_normalized, field_ident_original.span()),
            )
        } else {
            (
                syn::Ident::new(field_ident_normalized, field_ident_original.span()),
                syn::Ident::new(field_ident_normalized, field_ident_original.span()),
            )
        };

        Ok((field_ident_normalized.into(), field_name_serialized.into()))
    }

    fn convert_case(ident: impl Into<String>, casing: &StructLevelCasing) -> IdentCased {
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
