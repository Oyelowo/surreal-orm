/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// pub mod surreal_orm {
//     pub use surreal_derive::*;
//     pub use surreal_query_builder::*;
// }

pub mod migrations;
mod models;
// pub use migrations::*;
pub use models::alien::*;
pub use models::attributes::*;
pub use models::configuration::*;
pub use models::planet::*;
pub use models::spaceship::*;
pub use models::user::*;
pub use models::visits::*;
pub use models::weapon::*;
// pub use models::*;

// use surreal_orm::{
//     serde::{Deserialize, Serialize},
//     Node, SurrealId,
// };

// #[derive(Node, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(table = balance)]
// pub struct Balance {
//     pub id: SurrealId<Self, String>,
//     #[surreal_orm(ty = "string")]
//     pub amount: &'static str,
//     // pub amount: Option<f64>,
// }
