use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb_orm::SurrealId;
use surrealdb_orm::SurrealdbNode;
use surrealdb_orm::SurrealdbObject;

// Weapon
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealId<Self>,
    pub name: String,
    pub strength: Strength,
    pub created: DateTime<Utc>,
    #[surrealdb(nest_object = "Rocket")]
    pub rocket: Rocket,
}
type Strength = u64;
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon", relax_table_name)]
pub struct WeaponOld {
    pub id: SurrealId<Self>,
    pub name: String,
    pub strength: Strength,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
    #[surrealdb(nest_object = "Rocket")]
    pub rocket: Rocket,
}

#[derive(SurrealdbObject, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rocket {
    pub name: String,
    pub strength: u64,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
}
