use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{SurrealId2, SurrealdbNode};

// SpaceShip
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealId2<SpaceShip>,
    pub name: String,
    pub created: DateTime<Utc>,
}
