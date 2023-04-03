/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

mod errors;
pub mod functions;
mod helpers;
mod operators_macros;
pub mod statements;
mod traits;
mod types;

pub use errors::*;
pub use helpers::*;
pub use traits::*;
pub use types::*;

pub use surrealdb::sql::json;
pub use surrealdb::sql::Value;

pub use surrealdb::opt::RecordId;
pub mod utils {
    pub use super::helpers::*;
}
pub mod prelude {
    use super::errors::*;
    use super::function::*;
    use super::helpers::*;
    use super::statements::*;
    use super::traits::*;
    use super::types::*;
}
