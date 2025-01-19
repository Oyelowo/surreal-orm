/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{Node, Object, SurrealId, SurrealSimpleId};

// Weapon
#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = weapon)]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[orm(ty = float)]
    pub strength: Strength,
    pub created: DateTime<Utc>,
    #[orm(nest_object = "Rocket")]
    pub rocket: Rocket,
}
type Strength = f64;

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = weapon, relax_table)]
pub struct WeaponOld {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[orm(ty = "float")]
    pub strength: Strength,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
    #[orm(nest_object = "Rocket")]
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

#[allow(dead_code)]
struct Mana<T>
where
    T: Clone,
{
    pub name: String,
    pub strength: u64,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
    pub ala: T,
}

// #[derive(Object, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// pub struct Rocket2<T: Clone> {
//     pub name: String,
//     pub strength: u64,
//     pub nice: bool,
//     pub bunch_of_other_fields: i32,
//     pub created: DateTime<Utc>,
//     pub ala: T,
// }

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = "weapon_stats")]
pub struct WeaponStats {
    pub id: SurrealSimpleId<Self>,
    pub average_strength: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = "account")]
pub struct Account {
    pub id: SurrealId<Self, String>,
    pub balance: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = balance)]
pub struct Balance {
    pub id: SurrealId<Self, String>,
    // #[orm(ty = "string")]
    // pub amount: &'static str,
    pub amount: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = test_stuff)]
pub struct TestStuff {
    pub id: SurrealSimpleId<Self>,
    #[orm(ty = "option<int>")]
    pub amt: Option<Strength>,
    #[orm(ty = "option<int>")]
    pub amt9: Option<Strength>,
    // Would be autoinferred
    pub amt2: Option<u64>,
    #[orm(ty = "array<int>")]
    pub amt3: Vec<u64>,
    // #[orm(type_ = "int")]
    pub count: u64,
}
