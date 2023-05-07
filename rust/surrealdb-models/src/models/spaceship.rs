use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{SurrealId, SurrealSimpleId, SurrealdbModel, SurrealdbNode};

// SpaceShip
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub created: DateTime<Utc>,
}
fn ere() {
    let x = SpaceShip::default().get_id();
}
