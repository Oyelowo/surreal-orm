/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

use std::{
    borrow::Cow,
    fmt::{format, Display},
};

#[derive(serde::Serialize, Debug, Clone, Default)]
pub struct DbField(String);

// impl<'a> From<&'a str> for &'a DbField {
//     fn from(s: &'a str) -> &'a DbField {
//         let db_field = DbField(s.to_string());
//         let reference = unsafe { std::mem::transmute::<&DbField, &'a DbField>(&db_field) };
//         reference
//     }
// }
// impl<'a> From<&str> for &'a DbField {
//     fn from(s: &str) -> &'a DbField {
//         todo!()
//     }
// }

// impl<'a> From<DbField> for &'a DbField {
//     fn from(value: DbField) -> &'a DbField {
//         Box::leak(Box::new(value))
//     }
// }

impl<'a> From<Cow<'a, DbField>> for DbField {
    fn from(value: Cow<'a, DbField>) -> Self {
        match value {
            Cow::Borrowed(v) => v.clone(),
            Cow::Owned(v) => v,
        }
    }
}
impl<'a> From<&'a DbField> for Cow<'a, DbField> {
    fn from(value: &'a DbField) -> Self {
        Cow::Borrowed(value)
    }
}

impl From<DbField> for Cow<'static, DbField> {
    fn from(value: DbField) -> Self {
        Cow::Owned(value)
    }
}
//
// impl<'a> From<DbField> for Cow<'a, DbField> {
//     fn from(value: DbField) -> Self {
//         Cow::Owned(value)
//     }
// }
//
// impl<'a> From<&DbField> for Cow<'a, DbField> {
//     fn from(value: &DbField) -> Self {
//         Cow::Borrowed(value)
//     }
// }
//
// impl<T: ToString> From<&T> for DbField {
//     fn from(value: &T) -> Self {
//         // DbField::Value(DbField(value.to_string()))
//         todo!()
//     }
// }
impl From<String> for DbField {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}
impl From<&str> for DbField {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl From<DbField> for String {
    fn from(value: DbField) -> Self {
        value.0
    }
}

impl std::fmt::Display for DbField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

/* impl std::fmt::Debug for DbField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
} */

#[derive(Debug, Clone)]
pub struct DbFilter {
    query_string: String,
}

pub fn empty() -> DbFilter {
    DbFilter::new("".into())
}

impl<'a> From<Cow<'a, DbFilter>> for DbFilter {
    fn from(value: Cow<'a, DbFilter>) -> Self {
        match value {
            Cow::Borrowed(v) => v.clone(),
            Cow::Owned(v) => v,
        }
    }
}
// impl From<Vec<String>> for DbQuery {
//     fn from(value: Vec<String>) -> Self {
//         Self::new(value.join(" "))
//     }
// }
impl From<Option<DbFilter>> for DbFilter {
    fn from(value: Option<DbFilter>) -> Self {
        match value {
            Some(v) => v,
            None => empty(),
        }
    }
}

impl From<String> for DbFilter {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
// impl Into<Vec<DbQuery>> for Vec<String> {
//     fn from(value: Vec<String>) -> Self {
//         todo!()
//     }
// }

impl std::fmt::Display for DbFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.query_string))
    }
}
impl DbFilter {
    pub fn new(query_string: String) -> Self {
        Self { query_string }
    }

    pub fn get_query_string(&self) -> &str {
        &self.query_string
    }

    pub fn or(&self, filter: Self) -> Self {
        let precendence = self._______bracket_if_not_already();
        DbFilter::new(format!("{precendence} OR ({filter})"))
    }

    fn _______bracket_if_not_already(&self) -> impl Display {
        let filter = self.to_string();
        match (filter.starts_with('('), filter.ends_with(')')) {
            (true, true) => format!("{self}"),
            _ => format!("({self})"),
        }
    }

    pub fn and(&self, filter: Self) -> Self {
        let precendence = self._______bracket_if_not_already();
        DbFilter::new(format!("{precendence} AND ({filter})"))
    }

    pub fn bracketed(&self) -> Self {
        DbFilter::new(format!("({self})"))
    }

    pub fn chain(&self, field: DbField) -> DbField {
        DbField::from(field)
    }
}

impl DbField {
    pub fn new(field_name: &str) -> Self {
        Self(field_name.to_owned())
    }
    /// Append the specified string to the field name
    ///
    /// # Arguments
    ///
    /// * `string` - The string to append
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbField;
    ///
    /// let mut field = DbField::new("name");
    /// field.push_str("_alias");
    /// ```
    // TODO: replace with long underscore to show it is an internal variable
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string)
    }

    /// Return a new `DbQuery` that renames the field with the specified alias
    ///
    /// # Arguments
    ///
    /// * `alias` - The alias to use for the field
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::{DbField, DbQuery};
    ///
    /// let field = DbField::new("name");
    /// let query = field.__as__("name_alias");
    /// assert_eq!(query.to_string(), "name AS name_alias");
    /// ```
    pub fn __as__(&self, alias: impl std::fmt::Display) -> DbFilter {
        DbFilter::new(format!("{} AS {}", self.0, alias))
    }

    /// Return a new `DbQuery` that checks whether the field is equal to the specified value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for equality
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::{DbField, DbQuery};
    ///
    /// let field = DbField::new("age");
    /// let query = field.equals(25);
    /// assert_eq!(query.to_string(), "age = 25");
    /// ```
    pub fn equals<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} = {}", self.0, value))
    }

    /// Return a new `DbQuery` that checks whether the field is not equal to the specified value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for inequality
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::{DbField, DbQuery};
    ///
    /// let field = DbField::new("age");
    /// let query = field.not_equals(25);
    /// assert_eq!(query.to_string(), "age != 25");
    /// ```
    pub fn not_equals<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} != {}", self.0, value))
    }

    /// Check whether the value of the field is greater than the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("age").greater_than(18);
    /// assert_eq!(query.to_string(), "age > 18");
    /// ```
    pub fn greater_than<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} > {}", self.0, value))
    }

    /// Check whether the value of the field is greater than or equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("age").greater_than_or_equals(18);
    /// assert_eq!(query.to_string(), "age >= 18");
    /// ```
    pub fn greater_than_or_equals<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} >= {}", self.0, value))
    }

    /// Check whether the value of the field is less than the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("age").less_than(30);
    /// assert_eq!(query.to_string(), "age < 30");
    /// ```
    pub fn less_than<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} < {}", self.0, value))
    }

    /// Check whether the value of the field is less than or equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("age").less_than_or_equals(30);
    /// assert_eq!(query.to_string(), "age <= 30");
    /// ```
    pub fn less_than_or_equals<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} <= {}", self.0, value))
    }

    /// Check whether the value of the field is between the given lower and upper bounds.
    ///
    /// # Arguments
    ///
    /// * `lower_bound` - The lower bound to compare against the field.
    /// * `upper_bound` - The upper bound to compare against the field.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("age").between(18, 30);
    /// assert_eq!(query.to_string(), "age BETWEEN 18 AND 30");
    /// ```
    pub fn between<T: Display, U: Display>(&self, lower_bound: T, upper_bound: U) -> DbFilter {
        DbFilter::new(format!(
            "{} BETWEEN {} AND {}",
            self.0, lower_bound, upper_bound
        ))
    }

    /// Constructs a LIKE query that checks whether the value of the column matches the given pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match against.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("name").like("A%");
    /// assert_eq!(query.to_string(), "name LIKE 'A%'");
    /// ```
    pub fn like(&self, pattern: &str) -> DbFilter {
        DbFilter::new(format!("{} LIKE '{}'", self.0, pattern))
    }

    /// Constructs a NOT LIKE query that checks whether the value of the column does not match the given pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match against.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("name").not_like("A%");
    /// assert_eq!(query.to_string(), "name NOT LIKE 'A%'");
    /// ```
    pub fn not_like(&self, pattern: &str) -> DbFilter {
        DbFilter::new(format!("{} NOT LIKE '{}'", self.0, pattern))
    }

    /// Constructs a query that checks whether the value of the column is null.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("age").is_null();
    /// assert_eq!(query.to_string(), "age IS NULL");
    /// ```
    pub fn is_null(&self) -> DbFilter {
        DbFilter::new(format!("{} IS NULL", self.0))
    }

    /// Constructs a query that checks whether the value of the column is not null.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("age").is_not_null();
    /// assert_eq!(query.to_string(), "age IS NOT NULL");
    /// ```
    pub fn is_not_null(&self) -> DbFilter {
        DbFilter::new(format!("{} IS NOT NULL", self.0))
    }

    /// Constructs a query that checks whether the value of the column is equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("age").equal(42);
    /// assert_eq!(query.to_string(), "age = 42");
    /// ```
    pub fn equal<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} = {}", self.0, value))
    }

    /// Constructs a query that checks whether the value of the column is not equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("age").not_equal(42);
    /// assert_eq!(query.to_string(), "age != 42");
    /// ```
    pub fn not_equal<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} != {}", self.0, value))
    }

    /// Constructs a query that checks whether the value of the column is exactly equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("age").exactly_equal(42);
    /// assert_eq!(query.to_string(), "age == 42");
    /// ```
    pub fn exactly_equal<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} == {}", self.0, value))
    }

    /// Check whether any value in a set is equal to a value.
    ///
    /// # Arguments
    ///
    /// * `values` - An array of values to be checked for equality with the column.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbColumn;
    ///
    /// let col = DbColumn::new("name");
    /// let query = col.any_equal(&["Alice", "Bob"]);
    /// assert_eq!(query.to_string(), "name ?= (Alice, Bob)");
    /// ```
    pub fn any_equal<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(", ");
        DbFilter::new(format!("{} ?= ({})", self.0, values_str))
    }

    /// Check whether all values in a set are equal to a value.
    ///
    /// # Arguments
    ///
    /// * `values` - An array of values to be checked for equality with the column.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbColumn;
    ///
    /// let col = DbColumn::new("age");
    /// let query = col.all_equal(&[20, 30, 40]);
    /// assert_eq!(query.to_string(), "age *= (20, 30, 40)");
    /// ```
    pub fn all_equal<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(", ");
        DbFilter::new(format!("{} *= ({})", self.0, values_str))
    }

    /// Compare two values for equality using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be compared with the column using fuzzy matching.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbColumn;
    ///
    /// let col = DbColumn::new("name");
    /// let query = col.fuzzy_equal("Alex");
    /// assert_eq!(query.to_string(), "name ~ Alex");
    /// ```
    pub fn fuzzy_equal<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} ~ {}", self.0, value))
    }

    /// Compare two values for inequality using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be compared with the column using fuzzy matching.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbColumn;
    ///
    /// let col = DbColumn::new("name");
    /// let query = col.fuzzy_not_equal("Alex");
    /// assert_eq!(query.to_string(), "name !~ Alex");
    /// ```
    pub fn fuzzy_not_equal<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} !~ {}", self.0, value))
    }

    /// Check whether any value in a set is equal to a value using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of values to match against.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::field("name").any_fuzzy_equal(&["foo", "bar"]);
    /// assert_eq!(query.to_string(), r#"name ?~ (foo, bar)"#);
    /// ```
    pub fn any_fuzzy_equal<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(", ");
        DbFilter::new(format!("{} ?~ ({})", self.0, values_str))
    }

    /// Check whether all values in a set are equal to a value using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of values to match against.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::field("name").all_fuzzy_equal(&["foo", "bar"]);
    /// assert_eq!(query.to_string(), r#"name *~ (foo, bar)"#);
    /// ```
    pub fn all_fuzzy_equal<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(", ");
        DbFilter::new(format!("{} *~ ({})", self.0, values_str))
    }

    /// Check whether a value is less than or equal to another value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value to compare against.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::field("age").less_than_or_equal(30);
    /// assert_eq!(query.to_string(), r#"age <= 30"#);
    /// ```
    pub fn less_than_or_equal<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} <= {}", self.0, value))
    }

    /// Check whether a value is greater than or equal to another value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value to compare against.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::field("age").greater_than_or_equal(30)
    /// assert_eq!(query.to_string(), r#"age => 30"#);
    pub fn greater_than_or_equal<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} >= {}", self.0, value))
    }

    /// Adds a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age + 5");
    /// ```
    pub fn add<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} + {}", self.0, value))
    }

    /// Checks whether the current query contains a given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be checked for containment in the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.contains("10-20");
    ///
    /// assert_eq!(new_query.to_string(), "age CONTAINS \"10-20\"");
    /// ```
    pub fn contains<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} CONTAINS {}", self.0, value))
    }

    /// Checks whether the current query does not contain a given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be checked for non-containment in the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.contains_not("10-20");
    ///
    /// assert_eq!(new_query.to_string(), "age CONTAINSNOT \"10-20\"");
    /// ```
    pub fn contains_not<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} CONTAINSNOT {}", self.0, value))
    }

    /// Checks whether the current query contains all of the given values.
    ///
    /// # Arguments
    ///
    /// * `values` - The values to be checked for containment in the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("tags".to_string());
    /// let new_query = query.contains_all(&["food", "pizza"]);
    ///
    /// assert_eq!(new_query.to_string(), "tags CONTAINSALL (\"food\",\"pizza\")");
    /// ```
    pub fn contains_all<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbFilter::new(format!("{} CONTAINSALL ({})", self.0, values_str))
    }

    /// Checks whether the current query contains any of the given values.
    ///
    /// # Arguments
    ///
    /// * `values` - The values to be checked for containment in the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("tags".to_string());
    /// let new_query = query.contains_all(&["food", "pizza"]);
    ///
    /// assert_eq!(new_query.to_string(), "tags CONTAINSANY (\"food\",\"pizza\")");
    pub fn contains_any<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbFilter::new(format!("{} CONTAINSANY ({})", self.0, values_str))
    }

    /// Checks whether the column value does not contain any of the specified values.
    ///
    /// # Arguments
    ///
    /// * `values` - An array of values to check if they are not contained in the column.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("my_column").contains_none(&[1, 2, 3]);
    /// assert_eq!(query.to_string(), "my_column CONTAINSNONE (1,2,3)");
    /// ```
    pub fn contains_none<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbFilter::new(format!("{} CONTAINSNONE ({})", self.0, values_str))
    }

    /// Checks whether the column value is contained in the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check if the column value is contained in.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("my_column").inside(10);
    /// assert_eq!(query.to_string(), "my_column INSIDE 10");
    /// ```
    pub fn inside<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} INSIDE {}", self.0, value))
    }

    /// Checks whether the column value is not contained in the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check if the column value is not contained in.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("my_column").not_inside("hello");
    /// assert_eq!(query.to_string(), "my_column NOTINSIDE hello");
    /// ```
    pub fn not_inside<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} NOTINSIDE {}", self.0, value))
    }

    /// Checks whether all of the specified values are contained in the column value.
    ///
    /// # Arguments
    ///
    /// * `values` - An array of values to check if they are all contained in the column value.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("my_column").all_inside(&[1, 2, 3]);
    /// assert_eq!(query.to_string(), "my_column ALLINSIDE (1,2,3)");
    /// ```
    pub fn all_inside<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbFilter::new(format!("{} ALLINSIDE ({})", self.0, values_str))
    }

    /// Checks whether any of the specified values are contained in the column value.
    ///
    /// # Arguments
    ///
    /// * `values` - An array of values to check if any of them are contained in the column value.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::column("my_column").all_inside(&[1, 2, 3]);
    /// assert_eq!(query.to_string(), "my_column ANYINSIDE (1,2,3)");
    /// ```
    pub fn any_inside<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbFilter::new(format!("{} ANYINSIDE ({})", self.0, values_str))
    }

    /// Checks whether none of the values are contained within the current field.
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of values of type `T` that are to be checked.
    ///
    /// # Examples
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("age").none_inside(&[18, 19, 20]);
    /// assert_eq!(query.to_string(), "age NONEINSIDE (18,19,20)");
    /// ```
    pub fn none_inside<T: Display>(&self, values: &[T]) -> DbFilter {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbFilter::new(format!("{} NONEINSIDE ({})", self.0, values_str))
    }

    /// Checks whether the current field is outside of the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value of type `T` that the current field is to be compared to.
    ///
    /// # Examples
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("location").outside("USA");
    /// assert_eq!(query.to_string(), "location OUTSIDE USA");
    /// ```
    pub fn outside<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} OUTSIDE {}", self.0, value))
    }

    /// Checks whether the current field intersects with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value of type `T` that the current field is to be compared to.
    ///
    /// # Examples
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("location").intersects("USA");
    /// assert_eq!(query.to_string(), "location INTERSECTS USA");
    /// ```
    pub fn intersects<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} INTERSECTS {}", self.0, value))
    }

    /// Checks whether any value in a set is equal to the current field using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of values of type `T` that the current field is to be compared to.
    ///
    /// # Examples
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("name").any_in_set(&["Oyelowo", "Oyedayo"]);
    /// assert_eq!(query.to_string(), "name ?= (Oyelowo, Oyedayo)");
    /// ```
    pub fn any_in_set<T: Display>(&self, values: &[T]) -> DbFilter {
        let value_str = values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        DbFilter::new(format!("{} ?= ({})", self.0, value_str))
    }

    /// Checks whether all values in a set are equal to the current field using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of values of type `T` that the current field is to be compared to.
    ///
    /// # Examples
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("name").all_in_set(&["Oyelowo", "Oyedayo"]);
    /// assert_eq!(query.to_string(), "name ?= (Oyelowo, Oyedayo)");
    /// ```
    pub fn all_in_set<T: Display>(&self, values: &[T]) -> DbFilter {
        let value_str = values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        DbFilter::new(format!("{} *= ({})", self.0, value_str))
    }

    /// Subtracts a value from the current query value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value to subtract from the current query value.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::from(10);
    /// let subtracted = query.subtract(5);
    /// assert_eq!(subtracted.to_string(), "10 - 5".to_string());
    /// ```
    pub fn subtract<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} - {}", self.0, value))
    }

    /// Multiplies the current query value with another value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value to multiply with the current query value.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::from(10);
    /// let multiplied = query.multiply(5);
    /// assert_eq!(multiplied.to_string(), "10 * 5".to_string());
    /// ```
    pub fn multiply<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} * {}", self.0, value))
    }

    /// Divides the current query value by another value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value to divide the current query value by.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::from(10);
    /// let divided = query.divide(5);
    /// assert_eq!(divided.to_string(), "10 / 5".to_string());
    /// ```
    pub fn divide<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} / {}", self.0, value))
    }

    /// Checks if the current query value is truthy.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::from(true);
    /// let is_truthy = query.is_truthy();
    /// assert_eq!(is_truthy.to_string(), "true && true".to_string());
    /// ```
    pub fn is_truthy(&self) -> DbFilter {
        DbFilter::new(format!("{} && true", self.0))
    }

    /// Checks if the current query value and another value are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - A value to check if it's truthy along with the current query value.
    ///
    /// # Example
    ///
    /// ```
    /// # use surrealdb::DbQuery;
    /// let query = DbQuery::from(true);
    /// let is_truthy_and = query.truthy_and(false);
    /// assert_eq!(is_truthy_and.to_string(), "true && false".to_string());
    /// ```
    pub fn truthy_and<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} && {}", self.0, value))
    }

    /// Checks if the current query value or another value are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - A value to check if it's truthy or the current query value.
    ///
    /// # Example
    /// ```
    /// use surrealdb::DbQuery;
    /// let query = DbQuery::new("column_name".to_string()).truthy_or(true);
    /// assert_eq!(query.to_string(), "column_name || true");
    /// ```
    pub fn truthy_or<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} || {}", self.0, value))
    }

    /// Check whether the value of the field is equal to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value of type T that implements the `Display` trait, representing the value to compare against.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("age").is(18);
    ///
    /// assert_eq!(query.to_string(), "age IS 18");
    /// ```
    pub fn is<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} IS {}", self.0, value))
    }

    /// Check whether the value of the field is not equal to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - A value of type T that implements the `Display` trait, representing the value to compare against.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::field("name").is_not("Alice");
    ///
    /// assert_eq!(query.to_string(), "name IS NOT Alice");
    /// ```
    pub fn is_not<T: Display>(&self, value: T) -> DbFilter {
        DbFilter::new(format!("{} IS NOT {}", self.0, value))
    }

    /// Check whether any value in a set is equal to a value using the "=" operator.
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of values to check for equality.
    ///
    /// # Example Usage
    ///
    /// ```
    /// use surrealdb::DbField;
    ///
    /// let field = DbField::new("age");
    /// let query = field.set_equal(&[20, 30, 40]);
    ///
    /// assert_eq!(query.to_string(), "age ?= {20, 30, 40}");
    /// ```
    pub fn set_equal<T: Display>(&self, values: &[T]) -> DbFilter {
        let joined_values = values
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(", ");
        DbFilter::new(format!("{} ?= {{{}}}", self.0, joined_values))
    }

    /// Check whether all values in a set are equal to a value using the "*=" operator.
    ///
    /// # Arguments
    ///
    /// * `values` - A slice of values to check for equality.
    ///
    /// # Example Usage
    ///
    /// ```
    /// use surrealdb::DbField;
    ///
    /// let field = DbField::new("age");
    /// let query = field.set_all_equal(&[20, 20, 20]);
    ///
    /// assert_eq!(query.to_string(), "age *= {20, 20, 20}");
    /// ```
    pub fn set_all_equal<T: Display>(&self, values: &[T]) -> DbFilter {
        let joined_values = values
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(", ");
        DbFilter::new(format!("{} *= {{{}}}", self.0, joined_values))
    }

    /// Combine this field with another using the "AND" operator.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `DbField` to combine with this one.
    ///
    /// # Example Usage
    ///
    /// ```
    /// use surrealdb::DbField;
    ///
    /// let field1 = DbField::new("age");
    /// let field2 = DbField::new("gender");
    /// let query = field1.and(&field2);
    ///
    /// assert_eq!(query.to_string(), "age AND gender");
    /// ```
    pub fn and(&self, other: &DbField) -> DbFilter {
        DbFilter::new(format!("{} AND {}", self.0, other.0))
    }

    /// Combine this field with another using the "OR" operator.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `DbField` to combine with this one.
    ///
    /// # Example Usage
    ///
    /// ```
    /// use surrealdb::DbField;
    ///
    /// let field1 = DbField::new("age");
    /// let field2 = DbField::new("gender");
    /// let query = field1.or(&field2);
    ///
    /// assert_eq!(query.to_string(), "age OR gender");
    /// ```
    pub fn or(&self, other: &DbField) -> DbFilter {
        DbFilter::new(format!("{} OR {}", self.0, other.0))
    }
}
