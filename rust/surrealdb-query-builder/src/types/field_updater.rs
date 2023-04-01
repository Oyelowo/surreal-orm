/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surrealdb::sql::{self, Operator};

use crate::traits::{BindingsList, Buildable, Parametric};

use super::Field;

/// A helper struct for generating SQL update statements.
pub struct Updater {
    column_updater_string: String,
    ____bindings: BindingsList,
}

impl Parametric for Updater {
    fn get_bindings(&self) -> BindingsList {
        self.____bindings.to_vec()
    }
}

pub fn updater(field: impl Into<Field>) -> Updater {
    Updater::new(field)
}

impl Buildable for Updater {
    fn build(&self) -> String {
        self.column_updater_string
    }
}

impl std::fmt::Display for Updater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

pub enum Updateables {
    Updater(Updater),
    Updaters(Vec<Updater>),
}

impl Parametric for Updateables {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Updateables::Updater(up) => up.get_bindings(),
            Updateables::Updaters(ups) => ups
                .into_iter()
                .flat_map(|u| u.get_bindings())
                .collect::<Vec<_>>(),
        }
    }
}

impl Updater {
    /// Creates a new `Updater` instance with the given column update string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score = score + 1".to_string());
    /// ```
    pub fn new(field: impl Into<Field>) -> Self {
        let field = field.into();
        Self {
            column_updater_string: field.to_string(),
            ____bindings: vec![],
        }
    }
    /// Sets a field name
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Setter;
    /// let updater = Setter::new("score".to_string());
    /// let updated_updater = updater.equal(2);
    /// assert_eq!(updated_updater.to_string(), "score = 2");
    /// ```
    pub fn equal(&self, value: impl Into<sql::Value>) -> Self {
        let value: sql::Value = value.into();
        self._____update_field(Operator::Equal, value)
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
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score".to_string());
    /// let updated_updater = updater.increment_by(2);
    /// assert_eq!(updated_updater.to_string(), "score += 2");
    /// ```
    pub fn increment_by(&self, value: impl Into<sql::Number>) -> Self {
        let value: sql::Number = value.into();
        self._____update_field(Operator::Inc, value)
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
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("tags += 'rust'".to_string());
    /// let updated_updater = updater.remove("python");
    /// assert_eq!(updated_updater.to_string(), "tags += 'rust'");
    /// ```
    pub fn append(&self, value: impl Into<sql::Value>) -> Self {
        self._____update_field(Operator::Inc, value)
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
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score".to_string());
    /// let updated_updater = updater.decrement_by(2);
    /// assert_eq!(updated_updater.to_string(), "score -= 2");
    /// ```
    pub fn decrement_by(&self, value: impl Into<sql::Number>) -> Self {
        let value: sql::Number = value.into();
        self._____update_field(Operator::Dec, value)
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
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("tags -= 'rust'".to_string());
    /// let updated_updater = updater.remove("python");
    /// assert_eq!(updated_updater.to_string(), "tags -= 'rust'");
    /// ```
    pub fn remove(&self, value: impl Into<sql::Value>) -> Self {
        self._____update_field(Operator::Dec, value)
    }

    /// Returns a new `Updater` instance with the string to add the given value to the column.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score = 5".to_string());
    /// let updated_updater = updater.plus_equal(2);
    /// assert_eq!(updated_updater.to_string(), "score = 5 + 2");
    /// ```
    pub fn plus_equal(&self, value: impl Into<sql::Value>) -> Self {
        self._____update_field(Operator::Inc, value)
    }

    /// Returns a new `Updater` instance with the string to remove the given value from the column.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove from the column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("name = 'John'".to_string());
    /// let updated_updater = updater.minus_equal("ohn");
    /// assert_eq!(updated_updater.to_string(), "name = 'J'");
    /// ```
    pub fn minus_equal(&self, value: impl Into<sql::Value>) -> Self {
        self._____update_field(Operator::Dec, value)
    }

    fn _____update_field(&self, operator: sql::Operator, value: impl Into<sql::Value>) -> Updater {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let column_updater_string = format!("{self} {operator} {}", binding.get_param());
        Self {
            column_updater_string,
            ____bindings: vec![binding],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_by() {
        let updater = Updater::new("score".to_string());
        let updated_updater = updater.increment_by(10);
        assert_eq!(updated_updater.build(), "score += _param_00000000");
    }

    #[test]
    fn test_append() {
        let updater = Updater::new("names".to_string());
        let updated_updater = updater.append("Alice");
        assert_eq!(updated_updater.build(), "names += _param_00000000");
    }

    #[test]
    fn test_decrement_by() {
        let updater = Updater::new("score".to_string());
        let updated_updater = updater.decrement_by(5);
        assert_eq!(updated_updater.build(), "score -= _param_00000000");
    }

    #[test]
    fn test_remove() {
        let updater = Updater::new("names".to_string());
        let updated_updater = updater.remove("Alice");
        assert_eq!(updated_updater.build(), "names -= _param_00000000");
    }

    #[test]
    fn test_plus_equal() {
        let updater = Updater::new("score".to_string());
        let updated_updater = updater.plus_equal(10);
        assert_eq!(updated_updater.build(), "score += _param_00000000");
    }

    #[test]
    fn test_minus_equal() {
        let updater = Updater::new("names".to_string());
        let updated_updater = updater.minus_equal("Alice");
        assert_eq!(updated_updater.build(), "names -= _param_00000000");
    }
}
