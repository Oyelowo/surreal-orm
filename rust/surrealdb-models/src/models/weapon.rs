use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::DataUpdater;
use surrealdb_orm::SurrealId2;
use surrealdb_orm::SurrealdbNode;

// Weapon
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealId2<Weapon>,
    pub name: String,
    pub strength: u64,
    pub created: DateTime<Utc>,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon", relax_table_name)]
pub struct WeaponOld {
    pub id: SurrealId2<WeaponOld>,
    pub name: String,
    pub strength: u64,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WeaponNonNullUpdater {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
}
