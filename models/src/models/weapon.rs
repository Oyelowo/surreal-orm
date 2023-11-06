use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{Node, Object, SurrealId, SurrealSimpleId};

// Weapon
#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    // pub strength: u64,
    #[surreal_orm(type_ = "int")]
    pub strength: Strength,
    pub created: DateTime<Utc>,
    #[surreal_orm(nest_object = "Rocket")]
    pub rocket: Rocket,
}
type Strength = u64;

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "weapon", relax_table_name)]
pub struct WeaponOld {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surreal_orm(type_ = "int")]
    pub strength: Strength,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
    #[surreal_orm(nest_object = "Rocket")]
    pub rocket: Rocket,
}

#[derive(Object, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rocket {
    pub name: String,
    pub strength: u64,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "weapon_stats")]
pub struct WeaponStats {
    pub id: SurrealSimpleId<Self>,
    pub average_strength: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "account")]
pub struct Account {
    pub id: SurrealId<Self, String>,
    pub balance: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "balance")]
pub struct Balance {
    pub id: SurrealId<Self, String>,
    #[surreal_orm(type_ = "option<string>")]
    pub amount: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "test_stuff")]
pub struct TestStuff {
    pub id: SurrealSimpleId<Self>,
    #[surreal_orm(type_ = "option<string>")]
    pub amt: Option<Strength>,
    #[surreal_orm(type_ = "option<geometry<polygon>>")]
    pub amt2: Option<i32>,
    #[surreal_orm(type_ = "int")]
    pub amt3: Strength,
}
// DEFINE FIELD amt TYPE option<int> on TABLE test_stuff;
