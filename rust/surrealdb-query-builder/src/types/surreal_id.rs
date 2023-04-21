use std::ops::Deref;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{self, thing};

use crate::{
    errors::SurrealdbOrmError,
    traits::{Binding, BindingsList, Buildable, Conditional, Erroneous, Operatable, Parametric},
};

/// Wrapper around surrealdb::sql::Thing to extend its capabilities
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SurrealId(sql::Thing);

impl Deref for SurrealId {
    type Target = sql::Thing;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Erroneous for SurrealId {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl ::std::fmt::Display for SurrealId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for SurrealId {
    type Error = SurrealdbOrmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        thing(&value.to_string())
            .map(SurrealId)
            .map_err(|e| SurrealdbOrmError::InvalidId(e.into()))
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

impl Into<sql::Value> for SurrealId {
    fn into(self) -> sql::Value {
        self.0.into()
    }
}
