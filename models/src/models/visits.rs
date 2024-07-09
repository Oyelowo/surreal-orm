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
    #[surreal_orm(link_one= "In", ty = "record<any>")]
    pub in_: LinkOne<In>,
    // #[surreal_orm(type_ = "record<planet>")]
    #[surreal_orm(link_one = "Out", ty = "record<any>")]
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
    #[surreal_orm(link_one = In)]
    pub in_: LinkOne<In>,

    #[serde(skip_serializing)]
    #[surreal_orm(link_one = Out)]
    // #[surreal_orm(ty = "record")]
    pub out: LinkOne<Out>,

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
    #[surreal_orm(link_one = Weapon)]
    weapon: LinkOne<Weapon>,

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
    #[surreal_orm(link_one = In)]
    pub r#in: LinkOne<In>,

    #[surreal_orm(link_one = Out)]
    pub out: LinkOne<Out>,
    pub hair_color: Option<String>,

    #[surreal_orm(ty = "duration")]
    pub time_visited: Duration,

    #[surreal_orm(link_one = "Planet<u64>")]
    pub mana: LinkOne<Planet<u64>>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_string: geo::LineString,
    pub multi_line_string: geo::MultiLineString,
    pub polygon: geo::Polygon,
    pub multi_polygon: geo::MultiPolygon,
    pub point: geo::Point,
    pub multi_point: geo::MultiPoint,
    pub geometry_collection: sql::Geometry,
    pub territory_area: geo::Polygon,
    pub home: geo::Point,
    
    #[surreal_orm(ty = "geometry<point>")]
    pub point_explicit: geo::Point,

    #[surreal_orm(ty = "geometry<multipoint>")]
    pub multi_point_explicit: geo::MultiPoint,

    #[surreal_orm(ty = "geometry<LineString>")]
    pub line_string_explicit: geo::LineString,

    #[surreal_orm(ty = "geometry<multiline>")]
    pub multi_line_string_explicit: geo::MultiLineString,

    #[surreal_orm(ty = "geometry<polygon>")]
    pub polygon_explicit: geo::Polygon,

    #[surreal_orm(ty = "geometry<multipolygon>")]
    pub multi_polygon_explicit: geo::MultiPolygon,

    #[surreal_orm(ty = "geometry<feature>")]
    pub geometry_collection_explicit: sql::Geometry,

    pub tags: Vec<String>,
    #[surreal_orm(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,
    // Again, we dont have to provide the type attribute, it can auto detect
    #[surreal_orm(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,
        //
        // TODO:: Prevent doing this at compile time
    // This is a read only field. This wouldnt make sense. as we are using in node also as edge.
        // e.g visit->visit->plant
    // #[surreal_orm(relate(model = "VistVisitsPlanet", connection = "->visits->planet"))]
    // #[serde(skip_serializing, default)]
    // pub visit_to_planet: Relate<Planet<u64>>,
}

