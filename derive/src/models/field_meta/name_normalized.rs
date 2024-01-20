/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use quote::format_ident;
use syn::Ident;

use crate::models::casing::*;

use super::*;

pub(crate) struct NormalisedField {
    pub(crate) field_ident_raw_to_underscore_suffix: Ident,
    pub(crate) field_ident_serialized_fmt: String,
}

impl NormalisedField {
    pub(crate) fn from_receiever(
        field_receiver: &MyFieldReceiver,
        struct_level_casing: CaseString,
    ) -> Self {
        let field_ident = field_receiver
            .ident
            .as_ref()
            .expect("Field ident is required");

        let field_ident_cased = FieldIdentCased::from(FieldIdentUnCased {
            uncased_field_name: field_ident.clone(),
            casing: struct_level_casing,
        });

        // get the field's proper serialized format. Renaming should take precedence
        let original_field_name_normalised = &field_receiver.rename.as_ref().map_or_else(
            || field_ident_cased.into(),
            |renamed| renamed.clone().serialize,
        );
        let field_ident_normalised = &format_ident!("{original_field_name_normalised}");

        let (field_ident_raw_to_underscore_suffix, field_ident_serialized_fmt) =
            if original_field_name_normalised.trim_start_matches("r#") == "in" {
                (format_ident!("in_"), "in".to_string())
            } else {
                (
                    field_ident_normalised.to_owned(),
                    field_ident_normalised.to_string(),
                )
            };

        Self {
            field_ident_raw_to_underscore_suffix,
            field_ident_serialized_fmt,
        }
    }
}
