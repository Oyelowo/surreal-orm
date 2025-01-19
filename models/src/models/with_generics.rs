use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use surreal_orm::*;

#[derive(Node, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = planet_with_generics)]
pub struct PlanetWithGenerics<'a, T: Serialize + Default + Clone + surreal_orm::validators::Int> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[orm(ty = "float")]
    pub strength: Strength,

    #[orm(ty = int)]
    pub something: T,

    #[orm(nest_object = "RocketWithGenerics<'a, T>")]
    pub rocket: RocketWithGenerics<'a, T>,

    #[orm(ty = "option<array<float>>")]
    pub score: Option<Vec<f64>>,
}
type Strength = f64;

#[derive(Object, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
pub struct RocketWithGenerics<'a, T: Serialize + Default + Clone + surreal_orm::validators::Int> {
    name: String,
    #[orm(ty = "int")]
    something: T,

    #[orm(ty = "option<string>")]
    something2: Option<&'a str>,

    nana: &'static str,
    #[serde(rename = "lowo")]
    fav_number: Option<i32>,
    #[orm(ty = "set<int>")]
    field_set: HashSet<i32>,

    // TODO: Do a compile check for the array size against the declared field type
    #[orm(ty = "array<float, 2>")]
    must_number: [Strength; 3],
}

fn _test_partial_ob() {
    let rocket = RocketWithGenerics::partial_builder()
        // .name("Sugar".to_string())
        .something(43)
        .fav_number(None)
        // .something2(None)
        // .fav_number(Some(1919))
        // .must_number(1919)
        .nana("ewe")
        .build();

    let _x = PlanetWithGenerics::partial_builder().rocket(rocket).build();
}
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
