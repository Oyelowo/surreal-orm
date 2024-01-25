/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use syn::Ident;

use crate::models::CaseString;

// Used when explicit field casing not defined and cannot be inferred
#[derive(Debug, Clone)]
pub(crate) struct FieldIdentUnCased {
    pub(crate) uncased_field_name: Ident,
    pub(crate) casing: CaseString,
}

// Used when field casing is defined or can be inferred
#[derive(Debug, Clone)]
pub(crate) struct FieldIdentCased(String);

impl From<String> for FieldIdentCased {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<FieldIdentCased> for String {
    fn from(value: FieldIdentCased) -> Self {
        value.0
    }
}

impl From<FieldIdentUnCased> for FieldIdentCased {
    /// Converts the field identifier string to the specified case.
    /// Also, if rename_all attribute is not specified to change the casing,
    /// it defaults to exactly how the fields are written out.
    /// However, Field rename attribute overrides this.
    fn from(field_uncased: FieldIdentUnCased) -> Self {
        let field_name = field_uncased.uncased_field_name.to_string();

        match field_uncased.casing {
            CaseString::None => field_name,
            CaseString::Camel => field_name.to_case(Case::Camel),
            CaseString::Snake => field_name.to_case(Case::Snake),
            CaseString::Pascal => field_name.to_case(Case::Pascal),
            CaseString::Lower => field_name.to_case(Case::Lower),
            CaseString::Upper => field_name.to_case(Case::Upper),
            CaseString::ScreamingSnake => field_name.to_case(Case::ScreamingSnake),
            CaseString::Kebab => field_name.to_case(Case::Kebab),
            CaseString::ScreamingKebab => field_name.to_case(Case::ScreamingSnake),
        }
        .into()
    }
}
