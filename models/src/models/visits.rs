/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{Edge, LinkMany, LinkOne, Model, Node, Relate, SurrealSimpleId};

use crate::{Planet, SpaceShip, Weapon};

// Visits

// Connects Alien to Planet via Visits
// pub type AlienVisitsPlanet = Visits<Alien, Planet<u64>>;

// // VisitsExplicit
// #[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "visits_explicit")]
// pub struct VisitsExplicit<In: Node, Out: Node> {
//     #[surreal_orm(type_ = "record<visits_explicit>")]
//     pub id: SurrealSimpleId<Self>,
//     #[serde(rename = "in")]
//     #[surreal_orm(type_ = "record")]
//     pub in_: LinkOne<In>,
//     #[surreal_orm(type_ = "record")]
//     // #[surreal_orm(type_ = "record<planet>")]
//     pub out: LinkOne<Out>,
//     #[surreal_orm(type_ = "duration")]
//     pub time_visited: Duration,
// }
//
// // Connects Alien to Planet via Visits
// pub type AlienVisitsPlanetExplicit = VisitsExplicit<Alien, Planet<u64>>;
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// // #[surreal_orm(table_name = "visits_with_explicit_attributes")]
// pub struct VisitsWithExplicitAttributes<In: Node, Out: Node> {
//     // #[surreal_orm(type_ = "record<visits_with_explicit_attributes>")]
//     pub id: SurrealSimpleId<Self>,
//
//     #[serde(rename = "in")]
//     // #[surreal_orm(type_ = "record")]
//     pub in_: LinkOne<In>,
//     // #[surreal_orm(type_ = "record")]
//     pub out: LinkOne<Out>,
//
//     // #[surreal_orm(type_ = "string")]
//     name: String,
//
//     // #[surreal_orm(type_ = "int")]
//     age: u8,
//
//     // #[surreal_orm(type_ = "datetime")]
//     created: DateTime<Utc>,
//
//     // #[surreal_orm(type_ = "duration")]
//     life_expectancy: Duration,
//
//     // #[surreal_orm(type_ = "geometry<polygon>")]
//     territory_area: geo::Polygon,
//
//     // #[surreal_orm(type_ = "geometry<point>")]
//     home: geo::Point,
//
//     // #[surreal_orm(type_ = "array<string>")]
//     tags: Vec<String>,
//
//     // #[surreal_orm(link_one = "Weapon", type_ = "record<weapon>")]
//     weapon: LinkOne<Weapon>,
//
//     // Again, we dont have to provide the type attribute, it can auto detect
//     // #[surreal_orm(link_many = "SpaceShip", type_ = "array<record<space_ship>>")]
//     space_ships: LinkMany<SpaceShip>,
// }
//
// pub type AlienVisitsPlanetWithExplicitAttributes = VisitsWithExplicitAttributes<Alien, Planet<u64>>;
//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Nma<'a> {
    // pub id: SurrealSimpleId<Self>,
    pub time_visited: Duration,
    pub dfd: &'static str,
    pub age: &'a str,
}

#[derive(Edge, Serialize, Deserialize, Debug, Clone)]
// #[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "visits")]
pub struct Visits<'a, In: Node, Out>
where
    Out: Node,
{
    // pub id: SurrealSimpleId<Self>,
    pub id: SurrealSimpleId<Self>,
    // #[serde(rename = "in")]
    pub in_: In,
    pub out: Out,
    pub time_visited: Duration,
    // pub age: &'a u8,
    // pub zdf: &'static u8,
    // pub zdf: &'static str,
    pub name: &'a String,
    pub name2: &'a str,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_polygon: geo::LineString,
    pub territory_area: geo::Polygon,
    pub home: geo::Point,
    pub tags: Vec<String>,
    // #[surreal_orm(link_one = "Weapon")]
    // pub weapon: LinkOne<Weapon>,
    // Again, we dont have to provide the type attribute, it can auto detect
    // #[surreal_orm(link_many = "SpaceShip")]
    // pub space_ships: LinkMany<SpaceShip>,
    // This is a read only field
    // #[surreal_orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    // #[serde(skip_serializing, default)]
    // pub planets_to_visit: Relate<Planet<u64>>,
}

//
//
//
//
//
//
//
//
