use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::SurrealdbNode;

// Weapon
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon")]
pub struct Weapon {
    // #[serde(skip_serializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<sql::Thing>,
    pub name: String,
    pub strength: u64,
    pub created: DateTime<Utc>,
}
