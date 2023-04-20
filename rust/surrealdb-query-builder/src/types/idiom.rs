/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{self, Display},
    ops::Deref,
};

use serde::{Deserialize, Serialize};
use surrealdb::sql::{self, thing};

use crate::{
    errors::SurrealdbOrmError,
    traits::{Conditional, Erroneous, Parametric},
};

/// Surrealdb namespace
#[derive(Debug, Clone)]
pub struct Namespace(sql::Idiom);

/// Surrealdb database
#[derive(Debug, Clone)]
pub struct Database(sql::Idiom);

/// Surrealdb login
#[derive(Debug, Clone)]
pub struct Login(sql::Idiom);

/// Surrealdb token
#[derive(Debug, Clone)]
pub struct Token(sql::Idiom);

/// Surrealdb scope
#[derive(Debug, Clone)]
pub struct Scope(sql::Idiom);

/// Surrealdb table
#[derive(Debug, Clone)]
pub struct Table(sql::Table);

/// Surrealdb event
#[derive(Debug, Clone)]
pub struct Event(sql::Idiom);

/// Surrealdb table index
#[derive(Debug, Clone)]
pub struct TableIndex(sql::Idiom);

impl Table {
    /// Create a new table
    pub fn new(name: impl Into<sql::Table>) -> Self {
        Self(name.into())
    }
}

impl From<sql::Table> for Table {
    fn from(value: sql::Table) -> Self {
        Self(value)
    }
}

impl From<Table> for sql::Table {
    fn from(value: Table) -> Self {
        value.0
    }
}
impl From<&Table> for sql::Table {
    fn from(value: &Table) -> Self {
        value.0.clone()
    }
}

impl Deref for Table {
    type Target = sql::Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Table> for Table {
    fn from(value: &Table) -> Self {
        Self(value.clone().into())
    }
}

/// A collection of tables
pub struct Tables(Vec<Table>);

impl From<Vec<Table>> for Tables {
    fn from(value: Vec<Table>) -> Self {
        Self(value)
    }
}

impl<'a, const N: usize> From<&[Table; N]> for Tables {
    fn from(value: &[Table; N]) -> Self {
        Self(value.to_vec())
    }
}

impl<'a, const N: usize> From<&[&Table; N]> for Tables {
    fn from(value: &[&Table; N]) -> Self {
        Self(value.map(Into::into).to_vec())
    }
}

impl From<Tables> for Vec<Table> {
    fn from(value: Tables) -> Self {
        value.0
    }
}

// impl Deref for Tables {
//     type Target = Vec<Table>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

macro_rules! impl_new_for_all {
    ($($types_:ty),*) => {
        $(
        impl $types_ {
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into().into())
            }
        }

        impl From<$types_> for sql::Idiom {
            fn from(value: $types_) -> Self {
                value.0
            }
        }
    )*
    };
}

impl_new_for_all!(Namespace, Database, Login, Token, Scope, Event, TableIndex);

macro_rules! impl_display_for_all {
    ($($types_:ty),*) => {
        $(
        impl Display for $types_ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl From<$types_> for String {
            fn from(value: $types_) -> Self {
                let value: String = value.0.to_string();
                value
            }
        }
        impl From<&str> for $types_ {
            fn from(value: &str) -> Self {
                Self(value.to_string().into())
            }
        }

        impl From<String> for $types_ {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }

        impl From<$types_> for sql::Value {
            fn from(value: $types_) -> Self {
                value.0.into()
            }
        }

    )*
    };
}
impl_display_for_all!(Namespace, Database, Login, Token, Scope, Table, Event, TableIndex);

pub struct Idiomx(sql::Idiom);

impl Display for Idiomx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Idiomx {
    pub fn new(name: sql::Idiom) -> Self {
        Self(name)
    }
}

impl From<Idiomx> for sql::Idiom {
    fn from(value: Idiomx) -> Self {
        value.0
    }
}

impl From<Idiomx> for sql::Value {
    fn from(value: Idiomx) -> Self {
        value.0.into()
    }
}

impl<T> From<T> for Idiomx
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into().into())
    }
}
