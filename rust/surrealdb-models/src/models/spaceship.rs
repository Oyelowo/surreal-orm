use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::SurrealdbNode;

// SpaceShip
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<sql::Thing>,
    pub name: String,
    pub created: DateTime<Utc>,
}
