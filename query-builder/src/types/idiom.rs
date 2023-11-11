/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{self, Display},
    ops::Deref,
};

use surrealdb::sql;

/// Surreal namespace
#[derive(Debug, Clone)]
pub struct Namespace(sql::Idiom);

/// Surreal database
#[derive(Debug, Clone)]
pub struct Database(sql::Idiom);

/// Surreal login
#[derive(Debug, Clone)]
pub struct Login(sql::Idiom);

/// Surreal token
#[derive(Debug, Clone)]
pub struct Token(sql::Idiom);

/// Surreal token
#[derive(Debug, Clone)]
pub struct User(sql::Idiom);

/// Surreal scope
#[derive(Debug, Clone)]
pub struct Scope(sql::Idiom);

/// Surreal table
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct Table(sql::Table);

impl Ord for Table {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

/// Surreal event
#[derive(Debug, Clone)]
pub struct Event(sql::Idiom);

/// Surreal table index
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

impl<const N: usize> From<&[Table; N]> for Tables {
    fn from(value: &[Table; N]) -> Self {
        Self(value.to_vec())
    }
}

impl<const N: usize> From<[Table; N]> for Tables {
    fn from(value: [Table; N]) -> Self {
        Self(value.to_vec())
    }
}

impl<const N: usize> From<&[&Table; N]> for Tables {
    fn from(value: &[&Table; N]) -> Self {
        Self(value.map(Into::into).to_vec())
    }
}

impl From<Tables> for Vec<Table> {
    fn from(value: Tables) -> Self {
        value.0
    }
}

macro_rules! impl_new_for_all {
    ($($types_:ty),*) => {
        $(
        impl $types_ {
            /// Create instance of type
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

impl_new_for_all!(Namespace, Database, Login, Token, User, Scope, Event, TableIndex);

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
impl_display_for_all!(Namespace, Database, Login, Token, User, Scope, Table, Event, TableIndex);

/// Wrapper around Surreal idiom. X suffix stands for extra.
pub struct Idiomx(sql::Idiom);

impl Display for Idiomx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Idiomx {
    /// Create a new idiom
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
