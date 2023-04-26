use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::SurrealdbNode;

// Planet
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "planet")]
pub struct Planet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<sql::Thing>,
    pub name: String,
    // area: Polygon,
    pub population: u64,
    pub created: DateTime<Utc>,
    pub tags: Vec<String>,
}
