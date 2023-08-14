use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{Node, SurrealSimpleId};

// Planet
#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "planet")]
pub struct Planet {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    // area: Polygon,
    // #[surreal_orm(type = "any")]
    // #[surreal_orm(type = "array")]
    // #[surreal_orm(type = "array", content_type = "int")]
    // #[surreal_orm(type = "array", content_type = "int")]
    // #[surreal_orm(type = "int")]
    // pub population: Population,
    // pub population: PopArray,
    // pub population: Vec<Population>,
    pub population: u64,
    pub created: DateTime<Utc>,
    pub tags: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PopArray(Vec<Population>);
// impl Intoiter for PopArray
impl IntoIterator for PopArray {
    type Item = Population;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// impl std::ops::Deref for PopArray {
//     type Target = Vec<Population>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl From<PopArray> for sql::Value {
//     fn from(v: PopArray) -> Self {
//         sql::Array(
//             v.iter()
//                 .map(|v| sql::Value::from(v.clone() as i64))
//                 .collect(),
//         )
//         .into()
//     }
// }
type Population = u64;

#[cfg(test)]
mod tests {
    use super::*;
    use surreal_orm::{SchemaGetter, SetterArray};

    #[test]
    fn test_auto_inference() {
        Planet::schema().tags.append(45u64);
    }
}
