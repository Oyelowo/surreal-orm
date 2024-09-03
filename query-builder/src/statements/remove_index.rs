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

use crate::{BindingsList, Buildable, Erroneous, Parametric, Queryable, Table, TableIndex};

/// Remove index statement
///
/// # Arguments
/// * `index` - The name of the index to be removed. Can be a string or a TableIndex type.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_index};
/// # let user = Table::new("user");
/// # let party = TableIndex::new("party");
///
/// let statement = remove_index(party).on_table(user);
/// assert_eq!(statement.build(), "REMOVE INDEX party ON TABLE user;");
/// ```
pub fn remove_index(index: impl Into<TableIndex>) -> RemoveIndexStatement {
    RemoveIndexStatement {
        index: index.into(),
        table: None,
    }
}

/// Remove index statement
pub struct RemoveIndexStatement {
    index: TableIndex,
    table: Option<Table>,
}

impl RemoveIndexStatement {
    /// Set the table to remove the index from
    /// # Arguments
    ///
    /// * `table` - The name of the table to remove the index from. Can be a string or a Table type.
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::remove_index};
    /// # let user = Table::new("user");
    /// # let party = TableIndex::new("party");
    ///  remove_index(party).on_table(user);
    /// ```
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveIndexStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE INDEX {}", self.index);
        if let Some(table) = &self.table {
            query = format!("{} ON TABLE {}", query, table);
        }
        format!("{};", query)
    }
}

impl Display for RemoveIndexStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveIndexStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveIndexStatement {}

impl Queryable for RemoveIndexStatement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TableIndex;

    #[test]
    fn test_remove_index() {
        let user = Table::new("user");
        let party = TableIndex::new("party");

        let statement = remove_index(party).on_table(user);
        assert_eq!(statement.build(), "REMOVE INDEX party ON TABLE user;");
    }
}
