/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use surrealdb::sql::{self, Operator};

use crate::{BindingsList, Buildable, Parametric, Setter, ValueLike};

use super::Field;

/// A helper struct for generating SQL update statements.
#[derive(Debug, Clone)]
pub struct Updater {
    query_string: String,
    bindings: BindingsList,
}

impl Parametric for Updater {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

/// A helper struct for generating setters used in various statements
pub fn updater(field: impl Into<Field>) -> Updater {
    let field: Field = field.into();
    Updater {
        query_string: field.build(),
        bindings: field.get_bindings(),
    }
}

impl Buildable for Updater {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}

impl std::fmt::Display for Updater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// Things that can be updated
pub enum Updateables {
    /// Single updater
    Updater(Updater),
    /// Multiple updaters
    Updaters(Vec<Updater>),
}

impl From<Updater> for Updateables {
    fn from(value: Updater) -> Self {
        Self::Updater(value)
    }
}

impl From<Setter> for Updateables {
    fn from(value: Setter) -> Self {
        Self::Updater(Updater {
            query_string: value.build(),
            bindings: value.get_bindings(),
        })
    }
}

impl Parametric for Updateables {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Updateables::Updater(up) => up.get_bindings(),
            Updateables::Updaters(ups) => ups
                .iter()
                .flat_map(|u| u.get_bindings())
                .collect::<Vec<_>>(),
        }
    }
}

impl Updater {
    /// Sets a field name
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let score = Field::new("score");
    /// let updated_updater = updater(score).equal(2);
    /// assert_eq!(updated_updater.to_raw().build(), "score = 2");
    /// # assert_eq!(updated_updater.fine_tune_params(), "score = $_param_00000001");
    /// ```
    pub fn equal(&self, value: impl Into<sql::Value>) -> Self {
        let value: sql::Value = value.into();
        self.update_field(Operator::Equal, value)
    }

    /// Returns a new `Updater` instance with the string to increment the column by the given value.
    /// Alias for plus_equal but idiomatically for numbers
    ///
    /// # Arguments
    ///
    /// * `value` - The value to increment the column by.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let score = Field::new("score");
    /// let updated_updater = updater(score).increment_by(2);
    /// assert_eq!(updated_updater.to_raw().build(), "score += 2");
    ///
    /// # assert_eq!(updated_updater.fine_tune_params(), "score += $_param_00000001");
    /// ```
    pub fn increment_by(&self, value: impl Into<sql::Number>) -> Self {
        let value: sql::Number = value.into();
        self.update_field(Operator::Inc, value)
    }

    /// Returns a new `Updater` instance with the string to append the given value to a column that stores an array.
    /// Alias for plus_equal but idiomatically for an array
    ///
    /// # Arguments
    ///
    /// * `value` - The value to append to the column's array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let tags = Field::new("tags");
    /// let updated_updater = updater(tags).append("rust");
    /// assert_eq!(updated_updater.to_raw().build(), "tags += 'rust'");
    /// # assert_eq!(updated_updater.fine_tune_params(), "tags += $_param_00000001");
    /// ```
    pub fn append(&self, value: impl Into<sql::Value>) -> Self {
        self.update_field(Operator::Inc, value)
    }

    /// Returns a new `Updater` instance with the string to decrement the column by the given value.
    /// Alias for minus_equal but idiomatically for an number
    ///
    /// # Arguments
    ///
    /// * `value` - The value to decrement the column by.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let score = Field::new("score");
    /// let updated_updater = updater(score).decrement_by(2);
    /// assert_eq!(updated_updater.to_raw().build(), "score -= 2");
    /// # assert_eq!(updated_updater.fine_tune_params(), "score -= $_param_00000001");
    /// ```
    pub fn decrement_by(&self, value: impl Into<sql::Number>) -> Self {
        let value: sql::Number = value.into();
        self.update_field(Operator::Dec, value)
    }

    /// Returns a new `Updater` instance with the string to remove the given value from a column that stores an array.
    /// Alias for minus_equal but idiomatically for an array
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove from the column's array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let tags = Field::new("tags");
    /// let updated_updater = updater(tags).remove("rust");
    /// assert_eq!(updated_updater.to_raw().build(), "tags -= 'rust'");
    /// # assert_eq!(updated_updater.fine_tune_params(), "tags -= $_param_00000001");
    /// ```
    pub fn remove(&self, value: impl Into<sql::Value>) -> Self {
        self.update_field(Operator::Dec, value)
    }

    /// Returns a new `Updater` instance with the string to add the given value to the column.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the field or push to the field if it is an array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let tags = Field::new("tags");
    /// let updated_updater = updater(tags).plus_equal("rust");
    /// assert_eq!(updated_updater.to_raw().build(), "tags += 'rust'");
    /// # assert_eq!(updated_updater.fine_tune_params(), "tags += $_param_00000001");
    /// ```
    pub fn plus_equal(&self, value: impl Into<sql::Value>) -> Self {
        self.update_field(Operator::Inc, value)
    }

    /// Returns a new `Updater` instance with the string to remove the given value from the column.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to subtract from the field or remove from the field if it is an array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let tags = Field::new("tags");
    /// let updated_updater = updater(tags).minus_equal("rust");
    /// assert_eq!(updated_updater.to_raw().build(), "tags -= 'rust'");
    /// # assert_eq!(updated_updater.fine_tune_params(), "tags -= $_param_00000001");
    /// ```
    pub fn minus_equal(&self, value: impl Into<sql::Value>) -> Self {
        self.update_field(Operator::Dec, value)
    }

    fn update_field(&self, operator: sql::Operator, value: impl Into<ValueLike>) -> Updater {
        let value: ValueLike = value.into();
        let column_updater_string = format!("{self} {operator} {}", value.build());
        Self {
            query_string: column_updater_string,
            bindings: value.get_bindings(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ToRaw;

    use super::*;

    #[test]
    fn test_increment_by() {
        let score = Field::new("score");
        let updated_updater = updater(score).increment_by(5);
        assert_eq!(
            updated_updater.fine_tune_params(),
            "score += $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "score += 5");
    }

    #[test]
    fn test_append() {
        let names = Field::new("names");
        let updated_updater = updater(names).append("Oyelowo");
        assert_eq!(
            updated_updater.fine_tune_params(),
            "names += $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "names += 'Oyelowo'");
    }

    #[test]
    fn test_decrement_by() {
        let score = Field::new("score");
        let updated_updater = updater(score).decrement_by(5);
        assert_eq!(
            updated_updater.fine_tune_params(),
            "score -= $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "score -= 5");
    }

    #[test]
    fn test_remove() {
        let names = Field::new("names");
        let updated_updater = updater(names).remove("Oyelowo");
        assert_eq!(
            updated_updater.fine_tune_params(),
            "names -= $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "names -= 'Oyelowo'");
    }

    #[test]
    fn test_plus_equal() {
        let score = Field::new("score");
        let updated_updater = updater(score).plus_equal(10);
        assert_eq!(
            updated_updater.fine_tune_params(),
            "score += $_param_00000001"
        );
    }

    #[test]
    fn test_minus_equal() {
        let names = Field::new("names");
        let updated_updater = updater(names).minus_equal("Oyelowo");
        assert_eq!(
            updated_updater.fine_tune_params(),
            "names -= $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "names -= 'Oyelowo'");
    }
}
