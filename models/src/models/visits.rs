/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::*;

use crate::{Alien, SpaceShip, Weapon};

use super::planet::Planet;

// Visits

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanet = Visits<Alien, Planet<u64>>;

// VisitsExplicit
#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = visits_explicit)]
pub struct VisitsExplicit<In: Node, Out: Node> {
    #[surreal_orm(ty = "record<visits_explicit>")]
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    #[surreal_orm(ty = "record")]
    pub in_: LinkOne<In>,
    #[surreal_orm(ty = "record")]
    // #[surreal_orm(type_ = "record<planet>")]
    pub out: LinkOne<Out>,
    #[surreal_orm(ty = "duration")]
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanetExplicit = VisitsExplicit<Alien, Planet<u64>>;

#[derive(Edge, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = visits_with_explicit_attributes)]
pub struct VisitsWithExplicitAttributes<In: Node, Out: Node> {
    // #[surreal_orm(ty = "record<visits_with_explicit_attributes>")]
    pub id: SurrealSimpleId<Self>,

    #[serde(rename = "in", skip_serializing)]
    #[surreal_orm(link_many = "In")]
    pub in_: LinkMany<In>,

    #[serde(skip_serializing)]
    #[surreal_orm(link_many = "Out")]
    // #[surreal_orm(ty = "record")]
    pub out: LinkMany<Out>,

    // #[surreal_orm(ty = "string")]
    name: String,

    // #[surreal_orm(ty = "int")]
    age: u8,

    // #[surreal_orm(ty = "datetime")]
    created: DateTime<Utc>,

    // #[surreal_orm(ty = "duration")]
    life_expectancy: Duration,

    // #[surreal_orm(ty = "geometry<polygon>")]
    territory_area: geo::Polygon,

    // #[surreal_orm(ty = "geometry<point>")]
    home: geo::Point,

    // #[surreal_orm(ty = "array<string>")]
    tags: Vec<String>,

    // #[surreal_orm(link_one = "Weapon", ty = "record<weapon>")]
    #[surreal_orm(link_many = "Weapon")]
    weapon: LinkMany<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    // #[surreal_orm(link_many = "SpaceShip", ty = "array<record<space_ship>>")]
    #[surreal_orm(link_many = "SpaceShip")]
    space_ships: LinkMany<SpaceShip>,
}

pub type AlienVisitsPlanetWithExplicitAttributes = VisitsWithExplicitAttributes<Alien, Planet<u64>>;

#[derive(Edge, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = visits)]
pub struct Visits<In: Node, Out>
where
    Out: Node,
{
    pub id: SurrealSimpleId<Self>,
    // pub id: SurrealSimpleId<Self>,
    #[serde(skip_serializing)]
    #[surreal_orm(link_many = "In")]
    pub r#in: LinkMany<In>,

    #[serde(skip_serializing)]
    #[surreal_orm(link_many = "Out")]
    pub out: LinkMany<Out>,
    pub name: String,
    pub hair_color: Option<&'static str>,
    #[surreal_orm(ty = "duration")]
    pub time_visited: Duration,
    #[surreal_orm(link_one = "Planet<u64>")]
    pub mana: LinkOne<Planet<u64>>,
    // pub age: &'a u8,
    // pub zdf: &'static u8,
    // pub zdf: &'static str,
    // pub name: &'a String,
    // pub name2: &'a str,
    // pub created: DateTime<Utc>,
    // pub life_expectancy: Duration,
    // pub line_polygon: geo::LineString,
    // pub territory_area: geo::Polygon,
    // pub home: geo::Point,
    // pub tags: Vec<String>,
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


