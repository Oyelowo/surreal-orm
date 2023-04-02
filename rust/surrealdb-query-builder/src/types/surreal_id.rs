use std::ops::Deref;

use serde::Deserialize;
use surrealdb::sql::{self, thing};

use crate::{
    errors::SurrealdbOrmError,
    traits::{Binding, BindingsList, Conditional, Erroneous, Parametric},
};

#[derive(Debug, Serialize, Clone)]
pub struct SurrealId(surrealdb::opt::RecordId);

impl Deref for SurrealId {
    type Target = surrealdb::opt::RecordId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Conditional for SurrealId {
    fn get_condition_query_string(&self) -> String {
        self.to_string()
    }
}

impl Erroneous for SurrealId {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Parametric for SurrealId {
    fn get_bindings(&self) -> BindingsList {
        let val: sql::Thing = self.to_owned().into();
        let val: sql::Value = val.into();
        vec![Binding::new(val)]
    }
}

impl ::std::fmt::Display for SurrealId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for SurrealId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SurrealId(thing(&s).map_err(serde::de::Error::custom)?))
    }
}

impl TryFrom<&str> for SurrealId {
    type Error = SurrealdbOrmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Improve error handling
        Ok(Self(thing(&value.to_string()).unwrap()))
    }
}

impl From<SurrealId> for sql::Thing {
    fn from(value: SurrealId) -> Self {
        value.0
    }
}

impl From<sql::Thing> for SurrealId {
    fn from(value: sql::Thing) -> Self {
        Self(value)
    }
}

// surrealdb::opt::RecordId is surrealdb::sql::Thing
// impl From<RecordId> for SurrealId {
//     fn from(value: RecordId) -> Self {
//         Self(value)
//     }
// }

impl Into<sql::Value> for SurrealId {
    fn into(self) -> sql::Value {
        self.0.into()
    }
}
