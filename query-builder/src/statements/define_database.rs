/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::Database,
};

/// Define a new database statement.
///
/// # Arguments
///
/// * `database` - The name of the database to be defined.
///
/// # Example
/// ```rust
///  use surreal_query_builder::{*, statements::define_database};
///  assert_eq!(
///          define_database("codebreather").build(),
///          "DEFINE DATABASE codebreather;"
///      );
///
///  let codebreather = Database::new("codebreather");
///  assert_eq!(
///          define_database(codebreather).build(),
///          "DEFINE DATABASE codebreather;"
///      );
///  
///  let codebreather = Database::new("codebreather");
///  assert_eq!(
///          define_database(codebreather).to_raw().build(),
///          "DEFINE DATABASE codebreather;"
///      );
/// ```
pub fn define_database(database: impl Into<Database>) -> DefineDatabaseStatement {
    DefineDatabaseStatement {
        database: database.into().into(),
        bindings: vec![],
    }
}

/// A statement for defining a database.
pub struct DefineDatabaseStatement {
    database: String,
    bindings: BindingsList,
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

impl Parametric for DefineDatabaseStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Queryable for DefineDatabaseStatement {}
impl Erroneous for DefineDatabaseStatement {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_statement() {
        assert_eq!(
            define_database("oyelowo").build(),
            "DEFINE DATABASE oyelowo;"
        );
    }
}
