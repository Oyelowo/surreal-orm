use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{SurrealId, SurrealdbModel, SurrealdbNode};

// #[derive(Serialize, Deserialize)]
// struct SpaceShipId(SurrealId<SpaceShip, String>);
// impl Default for SpaceShipId {
//     fn default() -> Self {
//         SpaceShip::create_id("default")
//     }
// }

// SpaceShip
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealId<SpaceShip, String>,
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

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "test_raw_id")]
pub struct TestRawId {
    pub id: SurrealId<Self, i32>,
    pub name: String,
}

// // Rust doc test compile fail
// /// ```rust, compile_fail
// /// use surrealdb_models::models::spaceship::SpaceShip;
// /// let result: SurrealId<SpaceShip, i32> = TestRawId::create_id("dff");
// /// ```
// /// ```rust
// /// use surrealdb_models::models::spaceship::SpaceShip;
// /// let result: SurrealId<SpaceShip, i32> = TestRawId::create_id(112);
// /// ```
// fn ere() {
//     let x = SpaceShip::default().get_id();
//     // let xx: SurrealId<SpaceShip, &str> = SpaceShip::create_id("dff");
//
//     let c = TestRawId {
//         id: TestRawId::create_id(34),
//         name: "dff".to_string(),
//     };
// }
