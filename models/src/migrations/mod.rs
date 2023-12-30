/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use chrono::Utc;
use serde::{Deserialize, Serialize};
use surreal_orm::{
    arr, cond, create_table_resources,
    statements::{define_event, define_index, select},
    *,
};

#[derive(Debug, Clone)]
pub struct Resources;

impl DbResources for Resources {
    create_table_resources!(Animal, Crop, AnimalEatsCrop, Student, Planet);
}

#[derive(Debug, Clone)]
pub struct ResourcesV2;

impl DbResources for ResourcesV2 {
    create_table_resources!(AnimalV2, Crop, AnimalEatsCropV2, PlanetV2, NewStuff);
}

#[derive(Debug, Clone)]
pub struct ResourcesV3;

impl DbResources for ResourcesV3 {
    create_table_resources!(Crop, Student);
}

#[derive(Debug, Clone)]
pub struct ResourcesV4;

impl DbResources for ResourcesV4 {
    create_table_resources!(Crop, Student);
}

#[derive(Debug, Clone)]
pub struct ResourcesV5;

impl DbResources for ResourcesV5 {
    create_table_resources!(Student);
}

#[derive(Debug, Clone)]
pub struct ResourcesV6;

impl DbResources for ResourcesV6 {}

#[derive(Debug, Clone)]
pub struct ResourcesV7;

impl DbResources for ResourcesV7 {
    create_table_resources!(Animal, Crop, AnimalEatsCrop, Planet);
}

#[derive(Debug, Clone)]
pub struct ResourcesV8;

impl DbResources for ResourcesV8 {}

#[derive(Debug, Clone)]
pub struct ResourcesV9;

impl DbResources for ResourcesV9 {}

#[derive(Debug, Clone)]
pub struct ResourcesV10;

impl DbResources for ResourcesV10 {
    create_table_resources!(Animal, Crop, Planet);
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "new_stuff", schemafull)]
pub struct NewStuff {
    // Test renaming tomorrow
    pub id: SurrealSimpleId<Self>,
    pub first_name: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl TableResources for NewStuff {}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "planet", schemafull)]
pub struct Planet {
    // Test renaming tomorrow
    pub id: SurrealSimpleId<Self>,
    pub first_name: String,
    pub population: u64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub labels: Vec<String>,
}

impl TableResources for Planet {}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "planet", schemafull, relax_table_name)]
pub struct PlanetV2 {
    // Test renaming tomorrow
    pub id: SurrealSimpleId<Self>,
    #[surreal_orm(old_name = "firstName")]
    pub new_name: String,
    pub population: u64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub labels: Vec<String>,
}

impl TableResources for PlanetV2 {}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student", schemafull)]
pub struct Student {
    pub id: SurrealSimpleId<Self>,
    pub university: String,
    pub age: u8,
    pub updated_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
}

impl TableResources for Student {}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "animal", schemafull)]
pub struct Animal {
    pub id: SurrealSimpleId<Self>,
    pub species: String,
    // Improve error message for old_nmae using word similarity algo
    pub attributes: Vec<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub velocity: u64,
}

impl TableResources for Animal {
    fn events_definitions() -> Vec<Raw> {
        let animal::Schema {
            species, velocity, ..
        } = Self::schema();

        let event1 = define_event("event1".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Erectus")).and(velocity.gt(545)))
            .then(select(All).from(Crop::table_name()))
            .to_raw();

        let event2 = define_event("event2".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Sapien")).and(velocity.lt(10)))
            .then(select(All).from(AnimalEatsCrop::table_name()))
            .to_raw();
        vec![event1, event2]
    }

    fn indexes_definitions() -> Vec<Raw> {
        let animal::Schema {
            species, velocity, ..
        } = Self::schema();

        let idx1 = define_index("species_speed_idx".to_string())
            .on_table(Self::table_name())
            .fields(arr![species, velocity])
            .unique()
            .to_raw();

        vec![idx1]
    }
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "animal", schemafull, relax_table_name)]
pub struct AnimalV2 {
    pub id: SurrealSimpleId<Self>,
    pub species: String,
    #[surreal_orm(old_name = "attributes")]
    pub characteristics: Vec<String>,
    pub velocity: u64,
}

impl TableResources for AnimalV2 {
    fn events_definitions() -> Vec<Raw> {
        let animal_v_2::Schema {
            species, velocity, ..
        } = Self::schema();

        let event1 = define_event("event1".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Habillis").and(velocity.gt(545))))
            .then(select(All).from(Crop::table_name()))
            .to_raw();

        vec![event1]
    }

    fn indexes_definitions() -> Vec<Raw> {
        let animal_v_2::Schema {
            characteristics,
            velocity,
            ..
        } = Self::schema();

        let idx1 = define_index("species_speed_idx".to_string())
            .on_table(Self::table_name())
            .fields(arr![velocity, characteristics])
            .unique()
            .to_raw();

        vec![idx1]
    }
}

#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "eats", schemafull)]
pub struct Eats<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: In,
    pub out: Out,
    pub place: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub type AnimalEatsCrop = Eats<Animal, Crop>;
impl TableResources for AnimalEatsCrop {}

#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "eats", schemafull, relax_table_name)]
pub struct EatsV2<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: In,
    pub out: Out,
    pub location: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub type AnimalEatsCropV2 = EatsV2<AnimalV2, Crop>;
impl TableResources for AnimalEatsCropV2 {}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "crop", schemafull)]
pub struct Crop {
    pub id: SurrealSimpleId<Self>,
    pub color: String,
}

impl TableResources for Crop {}
