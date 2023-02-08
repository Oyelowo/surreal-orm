/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use convert_case::{Case, Casing};
use strum_macros::EnumString;

/// Options: "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case",
/// "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
#[derive(Debug, Clone, Copy, EnumString)]
pub enum CaseString {
    #[strum(serialize = "camelCase")]
    Camel,
    #[strum(serialize = "snake_case")]
    Snake,
    // Normal,
    #[strum(serialize = "PascalCase")]
    Pascal,

    #[strum(serialize = "lowercase")]
    Lower,

    #[strum(serialize = "UPPERCASE")]
    Upper,

    #[strum(serialize = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,

    #[strum(serialize = "kebab-case")]
    Kebab,

    #[strum(serialize = "SCREAMING-KEBAB-CASE")]
    ScreamingKebab,
}

#[derive(Debug, Clone)]
pub(crate) struct FieldIdentUnCased {
    pub(crate) uncased_field_name: String,
    pub(crate) casing: Option<CaseString>,
}

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
        let field_name = field_uncased.uncased_field_name;

        match field_uncased.casing {
            None => field_name,
            Some(CaseString::Camel) => field_name.to_case(Case::Camel),
            Some(CaseString::Snake) => field_name.to_case(Case::Snake),
            Some(CaseString::Pascal) => field_name.to_case(Case::Pascal),
            Some(CaseString::Lower) => field_name.to_case(Case::Lower),
            Some(CaseString::Upper) => field_name.to_case(Case::Upper),
            Some(CaseString::ScreamingSnake) => field_name.to_case(Case::ScreamingSnake),
            Some(CaseString::Kebab) => field_name.to_case(Case::Kebab),
            Some(CaseString::ScreamingKebab) => field_name.to_case(Case::ScreamingSnake),
        }
        .into()
    }
}
