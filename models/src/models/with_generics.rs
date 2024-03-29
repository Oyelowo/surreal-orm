use core::num;
use std::{collections::HashSet, marker::PhantomData};

use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surreal_orm::{
    statements::{define_field, DefineFieldStatement},
    Node, Object, PartialUpdater, SurrealId, SurrealSimpleId, Updater,
};
// use surreal_orm::sql::U

struct Mana<T: surreal_orm::validators::Int>(T);
fn fef() {
    Mana(45);
    // Mana(45.8);
    // Mana("re");
}

// #[derive(Serialize, Deserialize)]
// struct User<'a> {
//     id: u32,
//     name: &'a str,
//     screen_name: &'a str,
//     location: &'a str,
// }
//
// Weapon
#[derive(Node, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
// #[surreal_orm(table = "weapon")]
#[surreal_orm(table = weapon)]
pub struct Weapon<'a, T: Serialize + Default + Clone + surreal_orm::validators::Int> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    // #[surreal_orm(ty = "option<number>", define = 2 + 2)]
    // #[surreal_orm(ty = "option<number>", define = define_field)]
    // #[surreal_orm(ty = option<number>)]
    #[surreal_orm(ty = "float")]
    // #[surreal_orm(ty = float)]
    pub strength: Strength,

    #[surreal_orm(ty = int)]
    pub something: T,
    // pub created: DateTime<Utc>,
    // #[surreal_orm(nest_object = "Rocket")]
    #[surreal_orm(nest_object = "Rocket<'a, T>")]
    // #[surreal_orm(nest_object = "<Rocket::<'a, T> as Mana>::Tx", ty = object)]
    // #[surreal_orm(reff = Rocket<'a, T>)]
    pub rocket: Rocket<'a, T>,

    // #[surreal_orm(ty = "option<array<float>>")]
    // #[surreal_orm(ty = "option<array<float>>")]
    pub score: Option<Vec<f64>>,
}
type Strength = f64;

#[derive(Object, Serialize, Deserialize, Debug, Clone, Default)]
pub struct Rocket<'a, T: Serialize + Default + Clone + surreal_orm::validators::Int> {
    name: String,
    #[surreal_orm(ty = "int")]
    something: T,

    #[surreal_orm(ty = "option<string>")]
    something2: Option<&'a str>,

    nana: &'static str,
    fav_number: Option<i32>,
    #[surreal_orm(ty = "set<int>")]
    field_set: HashSet<i32>,

    // #[surreal_orm(ty = "array<any>")]
    #[surreal_orm(ty = "array<float>")]
    must_number: [Strength; 3],
}

// fn test_partial_ob() {
//     let rocket = Rocket::partial_builder()
//         // .name("Sugar".to_string())
//         .something(43)
//         // .something2(None)
//         // .fav_number(Some(1919))
//         // .must_number(1919)
//         .nana("ewe")
//         .build();
//
//     let x = Weapon::partial_builder().rocket(rocket).build();
// }
//
// type Lala<'a, T> = <Weapon<'a, T> as PartialUpdater>::StructPartial;
// fn xfd(arg1: String) -> DefineFieldStatement {
//     let x = &mut Weapon::partial_builder()
//         .name("Oyelowo".into())
//         .something(45)
//         // .strength(2.0)
//         .rocket(Rocket::partial_builder().something2(None).build())
//         .build();
//     // Weapon::pa
//     define_field("strength").permissions_full()
// }

// #[derive(Deserialize)]
// struct User<'a> {
//     id: u32,
//     name: &'a str,
//     screen_name: &'a str,
//     location: &'a str,
// }
//
//
//
//
