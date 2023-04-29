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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WeaponUpdater {
    // #[serde(skip_serializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<sql::Thing>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
}
