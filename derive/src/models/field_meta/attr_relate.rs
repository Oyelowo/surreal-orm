/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;
use syn::Type;

use crate::models::{LinkRustFieldType, RustFieldTypeSelfAllowed};

#[derive(Debug, Clone)]
pub struct Relate {
    /// e.g ->writes->book
    pub connection_model: String,
    // #[darling(default)]
    /// e.g StudentWritesBook,
    /// derived from: type StudentWritesBook = Writes<Student, Book>;
    pub model: Option<LinkRustFieldType>,
}
//#[rename(se)]
impl FromMeta for Relate {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self {
            connection_model: value.into(),
            model: None,
        })
    }
    //TODO: Check to maybe remove cos I probably dont need this
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRelate {
            model: Type,
            connection: String,
        }

        impl From<FullRelate> for Relate {
            fn from(v: FullRelate) -> Self {
                let FullRelate {
                    connection, model, ..
                } = v;
                Self {
                    connection_model: connection,
                    model: Some(model),
                }
            }
        }
        FullRelate::from_list(items).map(Relate::from)
    }
}
