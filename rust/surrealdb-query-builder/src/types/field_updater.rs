/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surrealdb::sql::{self, Operator};

use crate::traits::{Binding, BindingsList, Buildable, Parametric};

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

pub fn updater(field: impl Into<Field>) -> Updater {
    Updater::new(field)
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

pub enum Updateables {
    Updater(Updater),
    Updaters(Vec<Updater>),
}

impl From<Updater> for Updateables {
    fn from(value: Updater) -> Self {
        Self::Updater(value)
    }
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
        let field: Field = field.into();
        let bindings = vec![field.get_bindings()];
        Self {
            query_string: field.to_string(),
            bindings: vec![],
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
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score".to_string());
    /// let updated_updater = updater.increment_by(2);
    /// assert_eq!(updated_updater.to_string(), "score += 2");
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
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("tags += 'rust'".to_string());
    /// let updated_updater = updater.remove("python");
    /// assert_eq!(updated_updater.to_string(), "tags += 'rust'");
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
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("score".to_string());
    /// let updated_updater = updater.decrement_by(2);
    /// assert_eq!(updated_updater.to_string(), "score -= 2");
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
    /// ```
    /// # use my_cool_db::Updater;
    /// let updater = Updater::new("tags -= 'rust'".to_string());
    /// let updated_updater = updater.remove("python");
    /// assert_eq!(updated_updater.to_string(), "tags -= 'rust'");
    /// ```
    pub fn remove(&self, value: impl Into<sql::Value>) -> Self {
        self.update_field(Operator::Dec, value)
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
        self.update_field(Operator::Inc, value)
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
        self.update_field(Operator::Dec, value)
    }

    fn update_field(&self, operator: sql::Operator, value: impl Into<sql::Value>) -> Updater {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let column_updater_string = format!("{self} {operator} {}", binding.get_param_dollarised());
        Self {
            query_string: column_updater_string,
            bindings: vec![binding],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::ToRaw;

    use super::*;

    #[test]
    fn test_increment_by() {
        let score = Field::new("score");
        let updated_updater = updater(score).increment_by(5);
        assert_eq!(
            updated_updater.fine_tune_params(),
            "score += $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().to_string(), "score += 5");
    }

    #[test]
    fn test_append() {
        let names = Field::new("names");
        let updated_updater = updater(names).append("Oyelowo");
        assert_eq!(
            updated_updater.fine_tune_params(),
            "names += $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().to_string(), "names += 'Oyelowo'");
    }

    #[test]
    fn test_decrement_by() {
        let score = Field::new("score");
        let updated_updater = updater(score).decrement_by(5);
        assert_eq!(
            updated_updater.fine_tune_params(),
            "score -= $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().to_string(), "score -= 5");
    }

    #[test]
    fn test_remove() {
        let names = Field::new("names");
        let updated_updater = updater(names).remove("Oyelowo");
        assert_eq!(
            updated_updater.fine_tune_params(),
            "names -= $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().to_string(), "names -= 'Oyelowo'");
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
        assert_eq!(updated_updater.to_raw().to_string(), "names -= 'Oyelowo'");
    }
}
