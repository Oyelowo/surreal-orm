/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use quote::format_ident;
use std::fmt::Display;
use std::ops::Deref;
use syn::Ident;

use crate::errors::ExtractorResult;
use crate::models::field_name_serialized;
use crate::models::{
    casing::*, create_ident_wrapper, derive_attributes::TableDeriveAttributes,
    field_name_serialized::FieldNameSerialized, CaseString, StructLevelCasing,
};

use super::MyFieldReceiver;

create_ident_wrapper!(IdentCased);
create_ident_wrapper!(FieldIdentNormalized);

impl MyFieldReceiver {
    pub(crate) fn field_ident_normalized(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<FieldIdentNormalized> {
        Ok(self.ident_meta(struct_casing)?.0)
    }

    pub(crate) fn field_name_serialized(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<FieldNameSerialized> {
        Ok(self.ident_meta(struct_casing)?.1)
    }

    fn ident_meta(
        &self,
        struct_casing: &StructLevelCasing,
    ) -> ExtractorResult<(FieldIdentNormalized, FieldNameSerialized)> {
        let field_ident_original = self
            .ident
            .as_ref()
            .ok_or(darling::Error::custom("Field must have an identifier]"))?;
        let field_ident_cased =
            || Self::convert_case(field_ident_original.to_string(), struct_casing).to_string();

        let field_ident_normalised = &self
            .rename
            .as_ref()
            .map_or_else(field_ident_cased, |renamed| renamed.serialize);

        let (field_ident_normalized, field_name_serialized) =
            if field_name_normalised.starts_with("r#") {
                let field_ident_normalized = field_ident_normalised.trim_start_matches("r#");
                (
                    format_ident!("{field_ident_normalised}_"),
                    field_ident_normalized.to_string(),
                )
            } else {
                (
                    field_ident_normalised.to_owned(),
                    field_ident_normalised.to_string(),
                )
            };

        Ok((field_ident_normalized.into(), field_ident_normalized.into()))
    }

    fn convert_case(ident: impl Into<String>, casing: &StructLevelCasing) -> IdentCased {
        let ident: String = ident.into();
        match casing.deref() {
            CaseString::None => ident,
            CaseString::Camel => ident.to_case(Case::Camel),
            CaseString::Snake => ident.to_case(Case::Snake),
            CaseString::Pascal => ident.to_case(Case::Pascal),
            CaseString::Lower => ident.to_case(Case::Lower),
            CaseString::Upper => ident.to_case(Case::Upper),
            CaseString::ScreamingSnake => ident.to_case(Case::ScreamingSnake),
            CaseString::Kebab => ident.to_case(Case::Kebab),
            CaseString::ScreamingKebab => ident.to_case(Case::ScreamingSnake),
        }
        .into()
    }
}
