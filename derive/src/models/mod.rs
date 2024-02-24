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
pub(crate) mod node;
pub(crate) mod object;
pub(crate) mod table_meta;
pub(crate) mod token_codegen;
pub(crate) mod utils;
pub(crate) mod variables;

pub use casing::*;
pub use errors::*;
pub use field_meta::*;
pub use table_meta::*;
pub use token_codegen::*;
pub use utils::*;
pub use variables::*;

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
