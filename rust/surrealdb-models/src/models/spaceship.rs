use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{
    SurrealId, SurrealSimpleId, SurrealUlid, SurrealUuid, SurrealdbModel, SurrealdbNode,
};

// SpaceShip
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealId<Self, i32>,
    pub name: String,
    pub created: DateTime<Utc>,
}
fn ere() {
    let x = SpaceShip::default().get_id();
    // let xx: SurrealId<SpaceShip, &str> = SpaceShip::create_id("dff");

    let c = SpaceShip {
        id: SpaceShip::create_id("545"),
        ..Default::default()
    };
}
