/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

/*
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

use crate::{BindingsList, Buildable, Erroneous, Parametric, Queryable, Table};

/// Remove table statement
///
/// # Arguments
///
/// * `table` - The name of the table to be removed. Can be a string or a Table type.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_table};
/// # let table = Table::new("table");
/// let statement = remove_table(table);
/// assert_eq!(statement.build(), "REMOVE TABLE table;");
/// ```
pub fn remove_table(table: impl Into<Table>) -> RemoveTableStatement {
    RemoveTableStatement {
        table: table.into(),
    }
}

/// Remove table statement
pub struct RemoveTableStatement {
    table: Table,
}

impl Buildable for RemoveTableStatement {
    fn build(&self) -> String {
        format!("REMOVE TABLE {};", self.table)
    }
}

impl Display for RemoveTableStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveTableStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveTableStatement {}

impl Queryable for RemoveTableStatement {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_table() {
        let table = Table::new("table");
        let statement = remove_table(table);
        assert_eq!(statement.build(), "REMOVE TABLE table;");
    }
}
