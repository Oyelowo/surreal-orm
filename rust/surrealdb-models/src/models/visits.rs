use std::time::Duration;

use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{LinkOne, SurrealdbEdge, SurrealdbNode};

use crate::{Alien, Planet};

// Visits
#[derive(SurrealdbEdge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "visits")]
pub struct Visits<In: SurrealdbNode, Out: SurrealdbNode> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<sql::Thing>,
    #[serde(rename = "in")]
    pub in_: LinkOne<In>,
    pub out: LinkOne<Out>,
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanet = Visits<Alien, Planet>;
