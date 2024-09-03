/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

/*
 *
 *
REMOVE statement

Statement syntax
REMOVE [
    NAMESPACE @name
    | DATABASE @name
    | LOGIN @name ON [ NAMESPACE | DATABASE ]
    | TOKEN @name ON [ NAMESPACE | DATABASE ]
    | SCOPE @name
    | TABLE @name
    | EVENT @name ON [ TABLE ] @table
    | FIELD @name ON [ TABLE ] @table
    | INDEX @name ON [ TABLE ] @table
]
 * */

use std::fmt::{self, Display};

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::Database,
};

/// Remove database statement.
///
/// # Arguments
///
/// * `database` - The name of the database to be removed.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
///  use surreal_orm::{*, statements::remove_database};
///  assert_eq!(
///          remove_database("codebreather").build(),
///          "REMOVE DATABASE codebreather;"
///      );
///
///  let codebreather = Database::new("codebreather");
///  assert_eq!(
///          remove_database(codebreather).build(),
///          "REMOVE DATABASE codebreather;"
///      );
///  
///  let codebreather = Database::new("codebreather");
///  assert_eq!(
///          remove_database(codebreather).to_raw().build(),
///          "REMOVE DATABASE codebreather;"
///      );
/// ```
pub fn remove_database(database: impl Into<Database>) -> RemoveDatabaseStatement {
    RemoveDatabaseStatement::new(database)
}

/// A statement for removing a database.
pub struct RemoveDatabaseStatement {
    database: Database,
}

impl RemoveDatabaseStatement {
    fn new(database: impl Into<Database>) -> Self {
        Self {
            database: database.into(),
        }
    }
}

impl Buildable for RemoveDatabaseStatement {
    fn build(&self) -> String {
        format!("REMOVE DATABASE {};", self.database)
    }
}

impl Display for RemoveDatabaseStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveDatabaseStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveDatabaseStatement {}

impl Queryable for RemoveDatabaseStatement {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_statement() {
        assert_eq!(
            remove_database("oyelowo").build(),
            "REMOVE DATABASE oyelowo;"
        );
    }
}
