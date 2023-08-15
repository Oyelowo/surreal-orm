use std::time::Duration;

use serde::{Deserialize, Serialize};
use surreal_orm::{Edge, LinkOne, Node, SurrealSimpleId};

use crate::{Alien, Planet};

// Visits
#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "visits")]
pub struct Visits<In: Node, Out: Node> {
    // pub id: SurrealId<Visits<In, Out>>,
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: LinkOne<In>,
    pub out: LinkOne<Out>,
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanet = Visits<Alien, Planet>;

// VisitsExplicit
#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "visits_explicit")]
pub struct VisitsExplicit<In: Node, Out: Node> {
    #[surreal_orm(type = "record(visits_explicit)")]
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    #[surreal_orm(type = "record()")]
    pub in_: LinkOne<In>,
    #[surreal_orm(type = "record()")]
    pub out: LinkOne<Out>,
    #[surreal_orm(type = "duration")]
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanetExplicit = Visits<Alien, Planet>;
