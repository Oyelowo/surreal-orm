use mongodb::bson::{to_bson, Bson};
use serde::{de::DeserializeOwned, Serialize};

pub fn as_bson<T: DeserializeOwned + Serialize>(field: &T) -> Bson {
    to_bson(field).expect("problem converting to bson")
}