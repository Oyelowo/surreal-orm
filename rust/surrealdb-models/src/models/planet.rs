use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{SurrealId, SurrealdbNode};

// Planet
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "planet")]
pub struct Planet {
    pub id: SurrealId<Self>,
    pub name: String,
    // area: Polygon,
    #[surrealdb(type = "int")]
    pub population: Population,
    pub created: DateTime<Utc>,
    pub tags: Vec<String>,
}

type Population = u64;
