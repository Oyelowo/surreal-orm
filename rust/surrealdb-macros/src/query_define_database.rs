/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql::{self, Ident};

use crate::{
    binding::BindingsList,
    sql::{Buildable, Database, Queryable, Runnables},
    Parametric,
};

pub fn define_database(database: impl Into<Database>) -> DefineDatabaseStatement {
    DefineDatabaseStatement::new(database)
}

// DEFINE DATABASE @name
pub struct DefineDatabaseStatement {
    database: String,
    bindings: BindingsList,
}

// Musings: Perhaps, definitions should not be parametized
impl DefineDatabaseStatement {
    pub fn new(database: impl Into<Database>) -> Self {
        Self {
            database: database.into().into(),
            bindings: vec![],
        }
    }
}

impl Buildable for DefineDatabaseStatement {
    fn build(&self) -> String {
        format!("DEFINE DATABASE {};", self.database)
    }
}

impl Display for DefineDatabaseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Runnables for DefineDatabaseStatement {}

impl Parametric for DefineDatabaseStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for DefineDatabaseStatement {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_let_statement() {
        assert_eq!(
            define_database("oyelowo").build(),
            "DEFINE DATABASE oyelowo;"
        );
    }
}
