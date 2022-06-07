#![allow(dead_code)]

use darling::{ast, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use strum_macros::EnumString;
use syn::{self, parse_macro_input};



/// Options: "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case",
/// "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
#[derive(Debug, Clone, Copy, EnumString)]
pub(crate) enum CaseString {
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

impl Default for CaseString {
    fn default() -> Self {
        CaseString::Camel
    }
}