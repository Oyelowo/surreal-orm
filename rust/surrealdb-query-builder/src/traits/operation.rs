/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql;

use super::{BindingsList, Buildable, Parametric};

#[derive(Debug, Clone)]
pub struct Operation {
    query_string: String,
    bindings: BindingsList,
}

// impl Operator {
//     pub fn new(query_string: String, bindings: BindingsList) -> Self {
//         Self {
//             query_string,
//             bindings,
//         }
//     }
//     /// Append the specified string to the field name
//     ///
//     /// # Arguments
//     ///
//     /// * `string` - The string to append
//     ///
//     /// # Example
//     ///
//     /// ```
//     /// use surrealdb::Field;
//     ///
//     /// let mut field = Field::new("name");
//     /// field.push_str("_alias");
//     /// ```
//     // TODO: replace with long underscore to show it is an internal variable
//     pub fn push_str(&mut self, string: &str) {
//         self.query_string.push_str(string)
//     }
// }

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Operation {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}

impl Parametric for Operation {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Operatable for Operation {}

pub trait Operatable: Sized + Parametric + Buildable {
    /// Return a new `DbQuery` that checks whether the field is equal to the specified value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for equality
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::{Field, DbQuery};
    ///
    /// let field = Field::new("age");
    /// let query = field.equals(25);
    /// assert_eq!(query.to_string(), "age = 25");
    /// ```
    fn equal<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::Equal, value)
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
    /// use surrealdb::{Field, DbQuery};
    ///
    /// let field = Field::new("age");
    /// let query = field.not_equals(25);
    /// assert_eq!(query.to_string(), "age != 25");
    /// ```
    fn not_equal<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::NotEqual, value)
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
    fn exactly_equal<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
        Self: Sized,
    {
        self.generate_query(sql::Operator::Exact, value)
    }

    /// Check whether any value in a arraa\y is equal to another value.
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
    /// let col = DbColumn::new("friends");
    /// let query = col.any_equal("Alice");
    /// assert_eq!(query.to_string(), friends ?= Alice)");
    /// ```
    fn any_equal<T>(&self, value: T) -> Operation
    where
        T: Into<Value>,
    {
        self.generate_query(sql::Operator::AnyEqual, value)
    }

    /// Check whether all values in an array is equals to another value.
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
    /// let col = DbColumn::new("friends");
    /// let query = col.any_equal("Alice");
    /// assert_eq!(query.to_string(), friends *= Alice)");
    /// ```
    fn all_equal<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::AllEqual, value)
    }

    /// Compare two values for equality using fuzzy matching.
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
    /// let query = DbQuery::column("name").like("A");
    /// assert_eq!(query.to_string(), "name ~ 'A'");
    /// ```
    fn like<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::Like, value.into())
    }

    /// Compare two values for inequality using fuzzy matching.
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
    /// let query = DbQuery::column("name").not_like("A");
    /// assert_eq!(query.to_string(), "name !~ 'A'");
    /// ```
    fn not_like<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::NotLike, value)
    }

    /// Check whether any value in a set is equal to a value using fuzzy matching
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
    /// let query = DbQuery::column("name").all_like("A");
    /// assert_eq!(query.to_string(), "name ?~ 'A'");
    /// ```
    fn any_like<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::AnyLike, value)
    }

    /// Check whether all values in a set are equal to a value using fuzzy matching.
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
    /// let query = DbQuery::column("name").all_like("A");
    /// assert_eq!(query.to_string(), "name *~ 'A'");
    /// ```
    fn all_like<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::AllLike, value)
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
    fn less_than<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::LessThan, value)
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
    fn less_than_or_equal<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::LessThanOrEqual, value)
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
    fn greater_than<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::MoreThan, value)
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
    fn greater_than_or_equal<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::MoreThanOrEqual, value)
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
    fn add<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::Add, value)
    }

    /// Subtracts a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be subtract to the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age - 5");
    /// ```
    fn subtract<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::Sub, value)
    }

    /// Multiply a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be multiply to the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age * 5");
    /// ```
    fn multiply<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::Mul, value)
    }

    /// Divide a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be divide to the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age / 5");
    /// ```
    fn divide<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::Div, value)
    }

    /// Checks whether two values are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age && 5");
    /// ```
    fn truthy_and<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query("&&", value)
    }

    /// Checks whether either of two values are truthy.

    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age || 5");
    /// ```
    fn truthy_or<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query("||", value)
    }

    /// Checks whether two values are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age AND 5");
    /// ```
    fn and<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::And, value)
    }

    /// Checks whether either of two values are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age OR 5");
    /// ```
    fn or<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query(sql::Operator::Or, value)
    }

    /// Check whether two values are equal.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age IS 5");
    /// ```
    fn is<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query("IS", value)
    }

    /// Check whether two values are not equal.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age".to_string());
    /// let new_query = query.add(5);
    ///
    /// assert_eq!(new_query.to_string(), "age IS NOT 5");
    /// ```
    fn is_not<T>(&self, value: T) -> Operation
    where
        T: Into<Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query("IS NOT", value)
    }

    /// Check whether a value contains another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("friends".to_string());
    /// let new_query = query.contains("Oyelowo")
    ///
    /// assert_eq!(new_query.to_string(), "friends CONTAINS 'Oyelowo'");
    /// ```
    fn contains<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query(sql::Operator::Contain, value)
    }

    /// Check whether a value does not contain another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("friends".to_string());
    /// let new_query = query.contains_not("Oyelowo")
    ///
    /// assert_eq!(new_query.to_string(), "friends CONTAINSNOT 'Oyelowo'");
    /// ```
    fn contains_not<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::NotContain, value)
    }

    /// Check whether a value contains all of multiple values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("number_counts");
    /// let new_query = query.contains_all([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts CONTAINSALL [10, 20, 10]");
    /// ```
    fn contains_all<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayCustom>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::ContainAll, value)
    }

    /// Check whether a value contains any of multiple values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("number_counts");
    /// let new_query = query.contains_any([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts CONTAINSANY [10, 20, 10]");
    /// ```
    fn contains_any<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayCustom>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::ContainAny, value)
    }

    /// Check whether a value does not contain none of multiple values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("number_counts");
    /// let new_query = query.contains_none([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts CONTAINSNONE [10, 20, 10]");
    /// ```
    fn contains_none<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayCustom>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::ContainNone, value)
    }

    /// Check whether a value is contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age");
    /// let new_query = query.inside([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts INSIDE [10, 20, 10]");
    /// ```
    fn inside<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::Inside, value)
    }

    /// Check whether a value is not contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("age");
    /// let new_query = query.not_inside([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts NOTINSIDE [10, 20, 10]");
    /// ```
    fn not_inside<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::NotInside, value)
    }

    /// Check whether all of multiple values are contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("ages");
    /// let new_query = query.not_inside([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts NOTINSIDE [10, 20, 10]");
    /// ```
    fn all_inside<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayCustom>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::AllInside, value)
    }

    /// Check whether any of multiple values are contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("ages");
    /// let new_query = query.not_inside([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts ANYINSIDE [10, 20, 10]");
    /// ```
    fn any_inside<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayCustom>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::AllInside, value)
    }

    /// Check whether none of multiple values are contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("ages");
    /// let new_query = query.none_inside([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts NONEINSIDE [10, 20, 10]");
    /// ```
    fn none_inside<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayCustom>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::NoneInside, value)
    }

    /// Check whether a geometry value is outside another geometry value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("location");
    /// let new_query = query.outside(polygon_variable);
    ///
    /// assert_eq!(new_query.to_string(), "location OUTSIDE {
    /// 	type: "Polygon",
    /// 	coordinates: [[
    /// 		[-0.38314819, 51.37692386], [0.1785278, 51.37692386],
    /// 		[0.1785278, 51.61460570], [-0.38314819, 51.61460570],
    /// 		[-0.38314819, 51.37692386]
    /// 	]]
    ///   };
    /// ");
    /// ```
    fn outside<T>(&self, value: T) -> Operation
    where
        T: Into<GeometryOrField>,
    {
        let value: GeometryOrField = value.into();
        self.generate_query(sql::Operator::Outside, value)
    }

    /// Check whether a geometry value intersects annother geometry value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::DbQuery;
    ///
    /// let query = DbQuery::new("location");
    /// let new_query = query.intersects(polygon_variable);
    ///
    /// assert_eq!(new_query.to_string(), "location INTERSECTS {
    /// 	type: "Polygon",
    /// 	coordinates: [[
    /// 		[-0.38314819, 51.37692386], [0.1785278, 51.37692386],
    /// 		[0.1785278, 51.61460570], [-0.38314819, 51.61460570],
    /// 		[-0.38314819, 51.37692386]
    /// 	]]
    ///   };
    /// ");
    /// ```
    fn intersects<T>(&self, value: T) -> Operation
    where
        T: Into<GeometryOrField>,
    {
        let value: GeometryOrField = value.into();
        self.generate_query(sql::Operator::Intersects, value)
    }

    // UPDATER METHODS
    //
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
    /// let updater = Updater::new("score = 5".to_string());
    /// let updated_updater = updater.increment_by(2);
    /// assert_eq!(updated_updater.to_string(), "score = 5 + 2");
    /// ```
    fn increment_by<T>(&self, value: T) -> Operation
    where
        T: Into<Number>,
    {
        let value: Number = value.into();
        self.generate_query("+=", value)
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
    /// let updater = Updater::new("tags = ARRAY['rust']".to_string());
    /// let updated_updater = updater.append("python");
    /// assert_eq!(updated_updater.to_string(), "tags = ARRAY['rust', 'python']");
    /// ```
    fn append<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query("+=", value)
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
    /// let updater = Updater::new("score = 5".to_string());
    /// let updated_updater = updater.decrement_by(2);
    /// assert_eq!(updated_updater.to_string(), "score = 5 - 2");
    /// ```
    fn decrement_by<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Number>,
    {
        let value: sql::Number = value.into();
        self.generate_query("-=", value)
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
    /// let updater = Updater::new("tags = ARRAY['rust', 'python']".to_string());
    /// let updated_updater = updater.remove("python");
    /// assert_eq!(updated_updater.to_string(), "tags = ARRAY['rust']");
    /// ```
    fn remove<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query("-=", value)
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
    fn plus_equal<T>(&self, value: T) -> Operation
    where
        T: Into<Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query("+=", value)
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
    fn minus_equal<T>(&self, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query("-=", value)
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
    /// use surrealdb::{Field, DbQuery};
    ///
    /// let field = Field::new("name");
    /// let query = field.__as__("name_alias");
    /// assert_eq!(query.to_string(), "name AS name_alias");
    /// ```
    pub fn __as__(&self, alias: impl std::fmt::Display) -> Operation {
        Operation::new(format!("{} AS {}", self.build(), alias))
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
    /// assert_eq!(query.to_string(), "age < 18 AND age < 30");
    /// ```
    pub fn between<T>(&self, lower_bound: T, upper_bound: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let lower_bound: Ordinal = lower_bound.into();
        let lower_bound: Value = lower_bound.into();
        let upper_bound: Ordinal = upper_bound.into();
        let upper_bound: Value = upper_bound.into();
        let lower_bound_binding = Binding::new(lower_bound);
        let upper_bound_binding = Binding::new(upper_bound);
        let condition = format!(
            "{} < {} < {}",
            lower_bound_binding.get_param(),
            self.build(),
            upper_bound_binding.get_param()
        );

        let lower_updated_params = self.__update_bindings(lower_bound_binding);
        let upper_updated_params = self.__update_bindings(upper_bound_binding);
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Operation {
            condition_query_string: condition,
            bindings: updated_params,
            field_name: self.field_name.clone(),
        }
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
    /// let query = DbQuery::field("age").within(18, 30);
    /// assert_eq!(query.to_string(), "age <= 18 AND age <= 30");
    /// ```
    pub fn within<T>(&self, lower_bound: T, upper_bound: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let lower_bound: Ordinal = lower_bound.into();
        let lower_bound: Value = lower_bound.into();
        let upper_bound: Ordinal = upper_bound.into();
        let upper_bound: Value = upper_bound.into();
        let lower_bound_binding = Binding::new(lower_bound);
        let upper_bound_binding = Binding::new(upper_bound);
        let condition = format!(
            "{} <= {} <= {}",
            lower_bound_binding.get_param(),
            self.build(),
            upper_bound_binding.get_param()
        );

        let lower_updated_params = self.__update_bindings(lower_bound_binding);
        let upper_updated_params = self.__update_bindings(upper_bound_binding);
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Operation {
            query_string: condition,
            bindings: updated_params,
        }
    }

    pub fn ____________update_many_bindings<'bi>(
        &self,
        bindings: impl Into<&'bi [Binding]>,
    ) -> Operation {
        let bindings: &'bi [Binding] = bindings.into();
        // println!("bindingszz {bindings:?}");
        // updated_params.extend_from_slice(&self.bindings[..]);
        // updated_params.extend_from_slice(&bindings[..]);
        let updated_params = [&self.get_bindings().as_slice(), bindings].concat();
        Operation {
            query_string: self.build(),
            bindings: updated_params,
        }
    }
    // fn ____________get_condition_string(&self) -> String;
    //
    // fn ____________update_condition_string(&mut self, condition_string: String) -> Operator;
    //
    // fn ____________update_many_bindings<'bi>(&self, bindings: impl Into<&'bi [Binding]>) -> Operator;
    // let bindings: &'bi [Binding] = bindings.into();
    // // println!("bindingszz {bindings:?}");
    // // updated_params.extend_from_slice(&self.bindings[..]);
    // // updated_params.extend_from_slice(&bindings[..]);
    // let updated_params = [&self.get_bindings().as_slice(), bindings].concat();
    // Self {
    //     condition_query_string: self.condition_query_string.to_string(),
    //     bindings: updated_params,
    //     field_name: self.field_name.clone(),
    //     }
    // }

    fn __update_bindings(&self, binding: Binding) -> Vec<Binding> {
        // let mut updated_params = Vec::with_capacity(self.bindings.len() + 1);
        // updated_params.extend(self.bindings.to_vec());
        // updated_params.extend([binding]);
        // updated_params
        [self.get_bindings().as_slice(), &[binding]].concat()
    }

    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> Operation
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let condition = format!("{} {} ${}", self.build(), operator, &binding.get_param());
        let updated_bindings = self.__update_bindings(binding);

        Operation {
            query_string: condition,
            bindings: updated_bindings,
        }
    }
}
