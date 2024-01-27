/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub(crate) mod casing;
pub(crate) mod edge;
pub(crate) mod errors;
pub(crate) mod field_meta;
pub(crate) mod object;
pub(crate) mod parser;
pub(crate) mod table_meta;
pub(crate) mod utils;
pub(crate) mod variables;

pub use casing::*;
pub use casing::*;
pub use field_meta::*;
pub use parser::*;
pub use table_meta::*;
pub(crate) use utils::*;

#[derive(Debug, Clone)]
pub enum DataType {
    Node,
    Edge,
    Object,
}

impl DataType {
    pub fn is_edge(&self) -> bool {
        matches!(self, Self::Edge)
    }
}
