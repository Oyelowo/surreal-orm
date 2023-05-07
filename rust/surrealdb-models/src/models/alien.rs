use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{Last, LinkMany, LinkOne, LinkSelf, Relate, SurrealSimpleId, SurrealdbNode, E};

use crate::{AlienVisitsPlanet, Planet, Rocket, SpaceShip, Weapon};

// Alien
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_polygon: sql::Geometry,
    pub territory_area: sql::Geometry,
    pub home: sql::Geometry,
    pub tags: Vec<String>,
    // database type attribute is autogenerated for all links of the struct. But you can also provide it
    #[surrealdb(link_self = "Alien", type = "record(alien)")]
    pub ally: LinkSelf<Alien>,

    // #[serde(skip_serializing)]
    #[surrealdb(link_one = "Weapon", type = "record(weapon)")]
    pub weapon: LinkOne<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    // #[serde(skip_serializing)]
    #[surrealdb(
        link_many = "SpaceShip",
        type = "array",
        content_type = "record(space_ship)"
    )]
    pub space_ships: LinkMany<SpaceShip>,

    // This is a read only field
    #[surrealdb(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    #[serde(skip_serializing, default)]
    pub planets_to_visit: Relate<Planet>,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "alien_2")]
pub struct Alien2 {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_polygon: sql::Geometry,
    pub territory_area: sql::Geometry,
    pub home: sql::Geometry,
    pub tags: Vec<String>,

    #[surrealdb(nest_object = "Rocket")]
    pub weapon: Rocket,

    // Again, we dont have to provide the type attribute, it can auto detect
    // #[serde(skip_serializing)]
    #[surrealdb(
        link_many = "SpaceShip",
        type = "array",
        content_type = "record(space_ship)"
    )]
    pub space_ships: LinkMany<SpaceShip>,
}
