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

use crate::{BindingsList, Buildable, Erroneous, Field, Parametric, Queryable, Table};

/// Remove field statement
///
/// # Arguments
/// * `field` - The name of the field to be removed. Can be a string or a Field type.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_field};
/// # let user = Table::new("user");
/// # let name = Field::new("name");
///
/// let statement = remove_field(name).on_table(user);
/// assert_eq!(statement.build(), "REMOVE FIELD name ON TABLE user;");
/// ```
pub fn remove_field(field: impl Into<Field>) -> RemoveFieldStatement {
    RemoveFieldStatement {
        field: field.into(),
        table: None,
    }
}

/// Remove field statement
pub struct RemoveFieldStatement {
    field: Field,
    table: Option<Table>,
}

impl RemoveFieldStatement {
    /// Set the table to remove the field from
    /// # Arguments
    ///
    /// * `table` - The name of the table to remove the field from. Can be a string or a Table type.
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::remove_field};
    /// # let user = Table::new("user");
    /// # let name = Field::new("name");
    ///
    /// let statement = remove_field(name).on_table(user);
    /// assert_eq!(statement.build(), "REMOVE FIELD name ON TABLE user;");
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveFieldStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE FIELD {}", self.field);
        if let Some(table) = &self.table {
            query = format!("{} ON TABLE {}", query, table);
        }
        format!("{};", query)
    }
}

impl Display for RemoveFieldStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveFieldStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveFieldStatement {}

impl Queryable for RemoveFieldStatement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Field, Table};

    #[test]
    fn test_remove_field() {
        let user = Table::new("user");
        let name = Field::new("name");

        let statement = remove_field(name).on_table(user);
        assert_eq!(statement.build(), "REMOVE FIELD name ON TABLE user;");
    }
}
