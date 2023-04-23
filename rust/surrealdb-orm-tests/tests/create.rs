use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{statements::create, *};

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "alien")]
struct Alien {
    id: Option<sql::Thing>,
    name: String,
    age: u8,
    created: sql::Datetime,
    life_expectancy: sql::Duration,
    home: sql::Geometry,
    tags: Vec<String>,
}
