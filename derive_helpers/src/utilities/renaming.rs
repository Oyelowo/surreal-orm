/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use crate::models::CaseString;

use darling::{util, FromMeta};

#[derive(Debug, Clone)]
pub struct RenameDeserialize {
    pub(crate) deserialize: String,
}

/// This enables us to handle potentially nested values i.e
///   #[serde(rename = "simple_name")]
///    or
///   #[serde(rename(deserialize = "age"))]
///  #[serde(rename(serialize = "ser_name_nested", deserialize = "deser_name_nested"))]
/// However, We dont care about deserialized name from serde, so we just ignore that.
impl FromMeta for RenameDeserialize {
    fn from_string(value: &str) -> ::darling::Result<Self> {
        Ok(Self {
            deserialize: value.into(),
        })
    }

    fn from_list(items: &[darling::ast::NestedMeta]) -> ::darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRename {
            deserialize: String,

            #[darling(default)]
            #[allow(dead_code)]
            serialize: util::Ignored, // Ignore deserialize since we only care about the serialized string
        }

        impl From<FullRename> for RenameDeserialize {
            fn from(v: FullRename) -> Self {
                let FullRename { deserialize, .. } = v;
                Self { deserialize }
            }
        }
        FullRename::from_list(items).map(RenameDeserialize::from)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StructLevelCasingDeserialize(CaseString);

impl From<CaseString> for StructLevelCasingDeserialize {
    fn from(value: CaseString) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for StructLevelCasingDeserialize {
    type Target = CaseString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
