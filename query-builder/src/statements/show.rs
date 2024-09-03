/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    DatetimeLike, NumberLike, TableLike,
};

/// Creates a SHOW CHANGES statement builder.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::show_changes_for_table};
///
/// let timestamp = chrono::DateTime::from_timestamp(4343434434, 232).unwrap();
/// let statement = show_changes_for_table("reading").since(timestamp).limit(10);
/// ```
pub fn show_changes_for_table(table: impl Into<TableLike>) -> ShowChangesStatement {
    ShowChangesStatement {
        table: table.into(),
        since: None,
        limit: None,
    }
}

/// Represents the initial state for building a SHOW CHANGES statement.
pub struct ShowChangesStatement {
    table: TableLike,
    since: Option<DatetimeLike>,
    limit: Option<NumberLike>,
}

impl ShowChangesStatement {
    /// Sets the SINCE clause for the SHOW CHANGES statement.
    pub fn since(mut self, timestamp: impl Into<DatetimeLike>) -> ShowChangesStatement {
        let timestamp: DatetimeLike = timestamp.into();
        self.since = Some(timestamp);
        self
    }

    /// Sets the LIMIT clause for the SHOW CHANGES statement.
    pub fn limit(mut self, limit: impl Into<NumberLike>) -> ShowChangesStatement {
        let limit: NumberLike = limit.into();
        self.limit = Some(limit);
        self
    }
}

impl Queryable for ShowChangesStatement {}

impl Erroneous for ShowChangesStatement {
    fn get_errors(&self) -> crate::ErrorList {
        let mut errors = vec![];
        errors.extend(self.table.get_errors());

        if let Some(timestamp) = &self.since {
            errors.extend(timestamp.get_errors());
        }

        if let Some(limit) = &self.limit {
            errors.extend(limit.get_errors());
        }
        errors
    }
}

impl Parametric for ShowChangesStatement {
    fn get_bindings(&self) -> BindingsList {
        let mut bindings = vec![];
        bindings.extend(self.table.get_bindings());

        if let Some(timestamp) = &self.since {
            bindings.extend(timestamp.get_bindings());
        }
        if let Some(limit) = &self.limit {
            bindings.extend(limit.get_bindings());
        }
        bindings
    }
}

impl Buildable for ShowChangesStatement {
    fn build(&self) -> String {
        let mut query = format!("SHOW CHANGES FOR TABLE {}", self.table.build());
        if let Some(timestamp) = &self.since {
            query += &format!(" SINCE \"{}\"", timestamp.build());
        }
        if let Some(limit) = &self.limit {
            query += &format!(" LIMIT {}", limit.build());
        }
        query + ";"
    }
}

impl fmt::Display for ShowChangesStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Buildable, ToRaw};

    #[test]
    fn test_show_changes_for_table_build() {
        let timestamp = chrono::DateTime::from_timestamp(4343434434, 232).unwrap();
        let statement = show_changes_for_table("reading").since(timestamp).limit(10);
        assert_eq!(statement.fine_tune_params(), "SHOW CHANGES FOR TABLE $_param_00000001 SINCE \"$_param_00000002\" LIMIT $_param_00000003;");
        assert_eq!(
            statement.to_raw().build(),
            "SHOW CHANGES FOR TABLE reading SINCE \"'2107-08-22T05:33:54.000000232Z'\" LIMIT 10;"
        );
    }
}
