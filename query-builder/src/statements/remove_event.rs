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

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::{Event, Table},
};

/// Remove event statement
///
/// # Arguments
///
/// * `event` - The name of the event to be removed. Can be a string or an Event type.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::remove_event};
/// # let user = Table::new("user");
/// # let party = Event::new("party");
///
/// let statement = remove_event(party).on_table(user);
/// assert_eq!(statement.build(), "REMOVE EVENT party ON TABLE user;");
/// ```
pub fn remove_event(event: impl Into<Event>) -> RemoveEventStatement {
    RemoveEventStatement {
        table: None,
        event: event.into(),
    }
}

/// Remove event statement
pub struct RemoveEventStatement {
    event: Event,
    table: Option<Table>,
}

impl RemoveEventStatement {
    /// Set the table to remove the event from.
    ///
    /// # Arguments
    /// * `table` - The name of the table to remove the event from. Can be a string or a Table type.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::remove_event};
    /// # let user = Table::new("user");
    /// # let party = Event::new("party");
    ///
    /// remove_event(party).on_table(user);
    /// ```
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveEventStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE EVENT {}", self.event);
        if let Some(table) = &self.table {
            query = format!("{query} ON TABLE {}", table);
        }
        format!("{};", query)
    }
}
impl Display for RemoveEventStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveEventStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveEventStatement {}

impl Queryable for RemoveEventStatement {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_event() {
        let user = Table::new("user");
        let party = Event::new("party");

        let statement = remove_event(party).on_table(user);
        assert_eq!(statement.build(), "REMOVE EVENT party ON TABLE user;");
    }
}
