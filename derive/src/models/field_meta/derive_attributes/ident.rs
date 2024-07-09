/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::Deref;

use crate::{errors::ExtractorResult, models::*};
use convert_case::{Case, Casing};
use quote::format_ident;

use super::{FieldIdentNormalized, FieldNamePascalized, IdentCased, MyFieldReceiver};

impl MyFieldReceiver {
    pub(crate) fn ident(&self) -> ExtractorResult<FieldIdentOriginal> {
        Ok(self
            .ident
            .as_ref()
            .ok_or(darling::Error::custom("Field must have an identifier"))?
            .clone()
            .into())
    }

    pub(crate) fn field_ident_normalized(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<FieldIdentNormalized> {
        Ok(self.ident_meta(struct_casing)?.0)
    }

    pub(crate) fn db_field_name(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<DbFieldName> {
        Ok(self.ident_meta(struct_casing)?.1)
    }

    pub fn field_name_pascalized(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<FieldNamePascalized> {
        let field_name_normalized = self.field_ident_normalized(struct_casing)?;

        let field_name_pascalized = format_ident!(
            "{}",
            field_name_normalized.to_string().to_case(Case::Pascal)
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

        let field_ident_normalised = &self
            .rename
            .as_ref()
            .map_or_else(field_ident_cased, |renamed| renamed.serialize);

        let (field_ident_normalized, field_name_serialized) =
            if field_ident_normalised.starts_with("r#") {
                let field_ident_normalized = field_ident_normalised.trim_start_matches("r#");
                (
                    format_ident!("{field_ident_normalised}_"),
                    field_ident_normalized.to_string(),
                )
            } else {
                (
                    format_ident!("{field_ident_normalised}"),
                    field_ident_normalised.to_string(),
                )
            };

        Ok((field_ident_normalized.into(), field_ident_normalized.into()))
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
