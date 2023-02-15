use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct SurId((String, String));

impl Serialize for SurId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let table_id_joined = format!("{}:{}", self.0 .0, self.0 .1);
        serializer.serialize_str(&table_id_joined)
    }
}

impl Display for SurId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SurId((table_name, id_part)) = self;
        f.write_fmt(format_args!("{table_name}:{id_part}"))
    }
}

impl SurId {
    pub fn new(table_name: &str, id_part: &str) -> SurId {
        SurId((table_name.into(), id_part.into()))
    }

    pub fn id(self) -> (String, String) {
        self.0
    }
    pub fn from_string(str: String) -> (String, String) {
        Self::from(str).0
    }
}

impl From<SurId> for String {
    fn from(value: SurId) -> Self {
        let SurId((table_name, id_part)) = value;
        format!("{table_name}:{id_part}",)
    }
}

impl From<SurId> for (String, String) {
    fn from(value: SurId) -> Self {
        value.0
    }
}

#[derive(Error, Debug)]
pub enum SurrealdbOrmError {
    #[error("the id - `{0}` - you have provided is invalid or belongs to another table. Surrealdb Is should be in format: <table_name:column>")]
    InvalidId(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    // #[error("unknown data store error")]
    // Unknown,
}
impl TryFrom<&str> for SurId {
    type Error = SurrealdbOrmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut spl = value.split(':');
        match (spl.next(), spl.next(), spl.next()) {
            (Some(table), Some(id), None) => Ok(Self((table.into(), id.into()))),
            _ => Err(SurrealdbOrmError::InvalidId(value.to_string())),
        }
    }
}

// impl From<&str> for SurId {
//     fn from(value: &str) -> Self {
//         let mut spl = value.split(':');
//         match (spl.next(), spl.next(), spl.next()) {
//             (Some(table), Some(id), None) => Self((table.into(), id.into())),
//             _ => panic!(),
//         }
//     }
// }

impl From<String> for SurId {
    fn from(value: String) -> Self {
        let mut spl = value.split(':');
        match (spl.next(), spl.next(), spl.next()) {
            (Some(table), Some(id), None) => Self((table.into(), id.into())),
            _ => panic!(),
        }
    }
}

// impl IntoResource for SurId {
//     fn into_resource(self) -> Result<surrealdb::opt::Resource> {
//         todo!()
//     }
// }
