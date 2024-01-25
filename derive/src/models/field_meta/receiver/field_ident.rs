/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use quote::format_ident;
use std::fmt::Display;
use syn::Ident;

use crate::errors::ExtractorResult;
use crate::models::derive_attributes::TableDeriveAttributes;
use crate::models::field_name_serialized::FieldNameSerialized;
use crate::models::CaseString;
use crate::models::{casing::*, create_ident_wrapper};

use super::MyFieldReceiver;

create_ident_wrapper!(IdentCased);
create_ident_wrapper!(FieldIdentNormalized);

impl MyFieldReceiver {
    pub(crate) fn field_ident_normalized(
        &self,
        table_attr: &TableDeriveAttributes,
    ) -> ExtractorResult<FieldIdentNormalized> {
        Ok(self.ident_meta(table_attr)?.0)
    }

    pub(crate) fn field_name_serialized(
        &self,
        table_attr: &TableDeriveAttributes,
    ) -> ExtractorResult<FieldNameSerialized> {
        Ok(self.ident_meta(table_attr)?.1)
    }

    fn ident_meta(
        &self,
        table_attr: &TableDeriveAttributes,
    ) -> ExtractorResult<(FieldIdentNormalized, FieldNameSerialized)> {
        let struct_level_casing = table_attr.struct_level_casing();
        let field_ident_original = self.ident.as_ref().expect("Field ident is required");
        let field_ident_cased = || {
            Self::convert_case(field_ident_original.to_string(), struct_level_casing).to_string()
        };

        let original_field_name_normalised = &self
            .rename
            .as_ref()
            .map_or_else(field_ident_cased, |renamed| renamed.serialize);
        let field_ident_normalised = &format_ident!("{original_field_name_normalised}");

        let (field_ident_normalized, field_name_serialized) =
            if original_field_name_normalised.trim_start_matches("r#") == "in" {
                (format_ident!("in_"), "in".to_string())
            } else {
                (
                    field_ident_normalised.to_owned(),
                    field_ident_normalised.to_string(),
                )
            };

        Ok((field_ident_normalized.into(), field_ident_normalized.into()))
    }

    fn convert_case(ident: impl Into<String>, casing: CaseString) -> IdentCased {
        let ident: String = ident.into();
        match casing {
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
