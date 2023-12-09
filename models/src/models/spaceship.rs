/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{Model, Node, SurrealId};
use surrealdb::sql;

// #[derive(Serialize, Deserialize)]
// struct SpaceShipId(SurrealId<SpaceShip, String>);
// impl Default for SpaceShipId {
//     fn default() -> Self {
//         SpaceShip::create_id("default")
//     }
// }

// SpaceShip
#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
}

impl Default for SpaceShip {
    fn default() -> Self {
        Self {
            id: Self::create_id(sql::Uuid::new_v4().to_string()),
            name: Default::default(),
            created: Default::default(),
        }
    }
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "test_raw_id")]
pub struct TestRawId {
    pub id: SurrealId<Self, i32>,
    pub name: String,
}
