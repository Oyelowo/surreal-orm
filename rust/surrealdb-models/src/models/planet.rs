use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{SetterArray, SurrealId, SurrealdbNode};

// Planet
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "planet")]
pub struct Planet {
    pub id: SurrealId<Self>,
    pub name: String,
    // area: Polygon,
    // #[surrealdb(type = "int")]
    // #[surrealdb(type = "array", content_type = "int")]
    #[surrealdb(type = "array", content_type = "int")]
    pub population: PopArray,
    pub created: DateTime<Utc>,
    pub tags: Vec<u64>,
}

type PopArray = Vec<Population>;
type Population = u64;

fn srer() {
    Planet::schema().population.append(45u64);
}
