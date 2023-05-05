use serde::{Deserialize, Serialize};
use surrealdb_orm::SurrealdbObject;

// Configuration
#[derive(SurrealdbObject, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    length: u64,
    #[surrealdb(type = "int")]
    shape: Shape,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum Shape {
    #[default]
    Circle,
    Square,
    Triangle,
}
