use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb_orm::{SurrealId, SurrealSimpleId, SurrealdbNode, SurrealdbObject};

use serde_aux::prelude::deserialize_number_from_string;

// Weapon
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    // pub strength: u64,
    #[surrealdb(type = "int")]
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
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surrealdb(type = "int")]
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

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon_stats")]
pub struct WeaponStats {
    pub id: SurrealSimpleId<Self>,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub average_strength: f64,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "account")]
pub struct Account {
    pub id: SurrealId<Self, String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub balance: f64,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "balance")]
pub struct Balance {
    pub id: SurrealId<Self, String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub balance: f64,
}
