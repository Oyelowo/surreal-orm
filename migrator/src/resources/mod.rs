/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::*;

pub mod comparison_init;
pub mod meta;
pub mod table_events;
pub mod table_fields;
pub mod table_indexes;
pub mod tables;

pub use comparison_init::*;
pub use meta::*;
pub use tables::*;

pub enum DeltaType {
    NoChange,
    Create {
        right: DefineStatementRaw,
    },
    Remove {
        left: DefineStatementRaw,
    },
    Update {
        left: DefineStatementRaw,
        right: DefineStatementRaw,
    },
}

impl From<(Option<DefineStatementRaw>, Option<DefineStatementRaw>)> for DeltaType {
    fn from(value: (Option<DefineStatementRaw>, Option<DefineStatementRaw>)) -> Self {
        match value {
            (None, Some(r)) => DeltaType::Create { right: r },
            (Some(l), None) => DeltaType::Remove { left: l },
            (Some(l), Some(r)) => {
                if l.trim() != r.trim() {
                    DeltaType::Update { left: l, right: r }
                } else {
                    DeltaType::NoChange
                }
            }
            (None, None) => unreachable!(),
        }
    }
}
