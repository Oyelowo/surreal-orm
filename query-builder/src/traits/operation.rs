/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{
    Aliasable, ArrayLike, Binding, BindingsList, Buildable, Conditional, Erroneous, ErrorList,
    GeometryLike, NumberLike, Ordinal, Parametric, Setter, StrandLike, ValueLike,
};
use std::fmt::Display;
use surrealdb::sql;

/// Defines the operations that can be performed on a field
#[derive(Debug, Clone)]
pub struct Operation {
    /// The query string
    pub query_string: String,
    /// The bindings used in the query
    pub bindings: BindingsList,
    /// The errors that occurred while building the query
    pub errors: ErrorList,
}

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

impl Buildable for &Operation {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}

impl Parametric for &Operation {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Aliasable for Operation {}

impl Operatable for Operation {}

impl Conditional for Operation {}
impl Conditional for &Operation {}

impl Erroneous for &Operation {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl Erroneous for Operation {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl From<Setter> for Operation {
    fn from(value: Setter) -> Self {
        Self {
            query_string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

/// Defines the operations that can be performed on a field or param or some other types
/// it is implemented for.
pub trait Operatable: Sized + Parametric + Buildable + Erroneous {
    /// `=`. Return a new `operation` that checks whether the field is equal to the specified value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for equality. Could be `sql::Value`, `Field` or `Param`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// let query = age.equal(25);
    /// assert_eq!(query.to_raw().build(), "age = 25");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_field = Field::new("valid_age_field");
    /// let query = age.equal(valid_age_field);
    /// assert_eq!(query.to_raw().build(), "age = valid_age_field");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_param = Param::new("valid_age_param");
    /// let query = age.equal(valid_age_param);
    /// assert_eq!(query.to_raw().build(), "age = $valid_age_param");
    /// ```
    fn equal<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Equal, value)
    }

    /// `=`. Alias for `equal`. Return a new `operation` that checks whether the field is equal to the specified value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for equality. Could be `sql::Value`, `Field` or `Param`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// let query = age.eq(25);
    /// assert_eq!(query.to_raw().build(), "age = 25");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_field = Field::new("valid_age_field");
    /// let query = age.eq(valid_age_field);
    /// assert_eq!(query.to_raw().build(), "age = valid_age_field");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_param = Param::new("valid_age_param");
    /// let query = age.eq(valid_age_param);
    /// assert_eq!(query.to_raw().build(), "age = $valid_age_param");
    /// ```
    fn eq<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Equal, value)
    }

    /// `!=`. Return a new `DbQuery` that checks whether the field is not equal to the specified value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for inequality. Could be `sql::Value`, `Field` or `Param`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// let query = age.not_equal(25);
    /// assert_eq!(query.to_raw().build(), "age != 25");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_field = Field::new("valid_age_field");
    /// let query = age.not_equal(valid_age_field);
    /// assert_eq!(query.to_raw().build(), "age != valid_age_field");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_param = Param::new("valid_age_param");
    /// let query = age.not_equal(valid_age_param);
    /// assert_eq!(query.to_raw().build(), "age != $valid_age_param");
    /// ```
    fn not_equal<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::NotEqual, value)
    }

    /// `!=`. Alias for `not_equal`. Return a new `DbQuery` that checks whether the field is not equal to the specified value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check for inequality. Could be `sql::Value`, `Field` or `Param`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// let query = age.neq(25);
    /// assert_eq!(query.to_raw().build(), "age != 25");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_field = Field::new("valid_age_field");
    /// let query = age.neq(valid_age_field);
    /// assert_eq!(query.to_raw().build(), "age != valid_age_field");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_param = Param::new("valid_age_param");
    /// let query = age.neq(valid_age_param);
    /// assert_eq!(query.to_raw().build(), "age != $valid_age_param");
    /// ```
    fn neq<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::NotEqual, value)
    }

    /// Constructs a query that checks whether the value of the column is exactly equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against. Could be `sql::Value`, `Field` or `Param`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// let query = age.exactly_equal(25);
    /// assert_eq!(query.to_raw().build(), "age == 25");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_field = Field::new("valid_age_field");
    /// let query = age.exactly_equal(valid_age_field);
    /// assert_eq!(query.to_raw().build(), "age == valid_age_field");
    ///
    /// # let age = Field::new("age");
    /// # let valid_age_param = Param::new("valid_age_param");
    /// let query = age.exactly_equal(valid_age_param);
    /// assert_eq!(query.to_raw().build(), "age == $valid_age_param");
    /// ```
    fn exactly_equal<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
        Self: Sized,
    {
        self.generate_query(sql::Operator::Exact, value)
    }

    /// Check whether any value in a arraa\y is equal to another value.
    ///
    /// # Arguments
    ///
    /// * `values` - An array of values to be checked for equality with the column. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let friends = Field::new("friends");
    /// let query = friends.any_equal("Alice");
    /// assert_eq!(query.to_raw().build(), "friends ?= 'Alice'");
    ///
    /// # let friends = Field::new("friends");
    /// # let valid_friends_field = Field::new("valid_friends_field");
    /// let query = friends.any_equal(valid_friends_field);
    /// assert_eq!(query.to_raw().build(), "friends ?= valid_friends_field");
    ///
    /// # let friends = Field::new("friends");
    /// # let valid_friends_param = Param::new("valid_friends_param");
    /// let query = friends.any_equal(valid_friends_param);
    /// assert_eq!(query.to_raw().build(), "friends ?= $valid_friends_param");
    /// ```
    fn any_equal<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::AnyEqual, value)
    }

    /// `*=` Check whether all values in an array is equals to another value.
    ///
    /// # Arguments
    ///
    /// * `values` - An array of values to be checked for equality with the column. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let friends = Field::new("friends");
    /// friends.all_equal("Alice");
    /// ```
    fn all_equal<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::AllEqual, value)
    }

    /// `~` Compare two values for equality using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match against. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let name = Field::new("name");
    /// let query = name.like("A");
    /// assert_eq!(query.to_raw().build(), "name ~ 'A'");
    /// ```
    fn like<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Like, value.into())
    }

    /// `!~` Compare two values for inequality using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match against. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let name = Field::new("name");
    /// let query = name.not_like("A");
    /// assert_eq!(query.to_raw().build(), "name !~ 'A'");
    /// ```
    fn not_like<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::NotLike, value)
    }

    /// `?~` Check whether any value in a set is equal to a value using fuzzy matching
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match against. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let name = Field::new("name");
    /// name.any_like("A");
    /// ```
    fn any_like<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::AnyLike, value)
    }

    /// `*~` Check whether all values in a set are equal to a value using fuzzy matching.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match against. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let name = Field::new("name");
    /// name.all_like("A");
    /// ```
    fn all_like<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::AllLike, value)
    }

    /// `<` Check whether the value of the field is less than the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.less_than(30);
    /// ```
    fn less_than<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::LessThan, value)
    }

    /// `<`. Alias for `less_than`. Check whether the value of the field is less than the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.lt(30);
    /// ```
    fn lt<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::LessThan, value)
    }

    /// `<=` Check whether the value of the field is less than or equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.less_than_or_equal(30);
    /// ```
    fn less_than_or_equal<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::LessThanOrEqual, value)
    }

    /// `<=`. Alias for `less_than_or_equal`. Check whether the value of the field is less than or equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field. Could be `sql::Value`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.lte(30);
    /// ```
    fn lte<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::LessThanOrEqual, value)
    }

    /// `>` Check whether the value of the field is greater than the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field. Could be `sql::Number`, `sql::Geometry`, `sql::Datetime`, `sql::Duration`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.greater_than(30);
    /// ```
    fn greater_than<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::MoreThan, value)
    }

    /// `>`. Alias for `greater_than`. Check whether the value of the field is greater than the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field. Could be `sql::Number`, `sql::Geometry`, `sql::Datetime`, `sql::Duration`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.gt(30);
    /// ```
    fn gt<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::MoreThan, value)
    }

    /// `>=` Check whether the value of the field is greater than or equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field. Could be `sql::Number`, `sql::Geometry`, `sql::Datetime`, `sql::Duration`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.greater_than_or_equal(30);
    /// ```
    fn greater_than_or_equal<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::MoreThanOrEqual, value)
    }

    /// `>=` Alias for `greater_than_or_equal`. Check whether the value of the field is greater than or equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compare against the field. Could be `sql::Number`, `sql::Geometry`, `sql::Datetime`, `sql::Duration`, `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.gte(30);
    /// ```
    fn gte<T>(&self, value: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::MoreThanOrEqual, value)
    }

    /// `+` Adds a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the current query. Could be `sql::Value`, `Field` or `Param
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.add(5);
    /// ```
    fn add<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Add, value)
    }

    /// `+` Alias for `add`. Adds a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be added to the current query. Could be `sql::Value`, `Field` or `Param
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.plus(5);
    /// ```
    fn plus<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Add, value)
    }

    /// `-` Subtracts a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be subtract to the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.subtract(5);
    /// ```
    fn subtract<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Sub, value)
    }

    /// `-`. Alias for `subtract`. Subtracts a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be subtract to the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.minus(5);
    /// ```
    fn minus<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Sub, value)
    }

    /// `*` Multiply a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be multiply to the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.multiply(5);
    /// ```
    fn multiply<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Mul, value)
    }

    /// `*` Alias for `multiply`. Multiply a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be multiply to the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.mul(5);
    /// ```
    fn mul<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Mul, value)
    }

    /// `/` Divide a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be divide to the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.divide(5);
    /// ```
    fn divide<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Div, value)
    }

    /// `/` Alias for `divide`. Divide a value to the current query.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be divide to the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// let age = Field::new("age");
    /// age.div(5);
    /// ```
    fn div<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Div, value)
    }

    /// `**` Raise the current query to the power of the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be raised to the power of the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// let age = Field::new("age");
    /// age.power(5);
    /// ```
    fn power<T>(&self, value: T) -> Operation
    where
        T: Into<NumberLike>,
    {
        self.generate_query(sql::Operator::Pow, value.into())
    }

    /// `**` Alias for `power`. Raise the current query to the power of the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be raised to the power of the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// let age = Field::new("age");
    /// age.pow(5);
    /// ```
    fn pow<T>(&self, value: T) -> Operation
    where
        T: Into<NumberLike>,
    {
        self.generate_query(sql::Operator::Pow, value.into())
    }

    /// `&&` Checks whether two values are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.truthy_and(5);
    /// ```
    fn truthy_and<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query("&&", value)
    }

    /// `||` Checks whether either of two values are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.truthy_or(5);
    /// ```
    fn truthy_or<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query("||", value)
    }

    /// `AND` Checks whether two values are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.and(5);
    /// ```
    fn and<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query("AND", value)
    }

    /// `OR` Checks whether either of two values are truthy.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.or(5);
    /// ```
    fn or<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        let value: ValueLike = value.into();
        self.generate_query("OR", value)
    }

    /// `IS` or `=` Check whether two values are equal.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.is(5);
    /// ```
    fn is<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        let value: ValueLike = value.into();
        self.generate_query("IS", value)
    }

    /// `IS NOT` or `!=` Check whether two values are not equal.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.is_not(5);
    /// ```
    fn is_not<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        let value: ValueLike = value.into();
        self.generate_query("IS NOT", value)
    }

    /// `CONTAINS` or `∋` Check whether a value contains another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let friends = Field::new("friends");
    /// friends.contains("Oyelowo");
    ///
    /// # let polygon = Field::new("polygon");
    /// polygon.contains("Oyelowo");
    /// ```
    fn contains<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        let value: ValueLike = value.into();
        self.generate_query(sql::Operator::Contain, value)
    }

    /// `CONTAINSNOT` or `∌` Check whether a value does not contain another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be any `sql::Value`, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let friends = Field::new("friends");
    /// friends.contains_not("Oyelowo");
    /// ```
    fn contains_not<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::NotContain, value)
    }

    /// `CONTAINSALL` or `⊇` Check whether a value contains all of multiple values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `Array, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let friends = Field::new("friends");
    /// friends.contains_all(&["Oyelowo", "Oyelowo"]);
    /// ```
    fn contains_all<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayLike>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::ContainAll, value)
    }

    /// `CONTAINSANY` or `⊃` Check whether a value contains any of multiple values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `Array, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let friends = Field::new("friends");
    /// friends.contains_any(&["Oyelowo", "Oyelowo"]);
    /// ```
    fn contains_any<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayLike>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::ContainAny, value)
    }

    /// `CONTAINSNONE` or `⊅` Check whether a value does not contain none of multiple values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `Array, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let friends = Field::new("friends");
    /// friends.contains_none(&["Oyelowo", "Oyelowo"]);
    /// ```
    fn contains_none<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayLike>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::ContainNone, value)
    }

    /// `INSIDE` or `∈` or `IN` Check whether a value is contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be any supported surrealdb value, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.inside(10);
    /// ```
    fn inside<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Inside, value)
    }

    /// `IN` or `∈` or `INSIDE` Check whether a value is contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be any supported surrealdb value, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// age.in_(10);
    /// ```
    fn in_<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::Inside, value)
    }

    /// `NOTINSIDE` or `∉` or `NOT IN` Check whether a value is not contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be any supported surrealdb value, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # use surrealdb::sql;
    /// use geo::polygon;
    /// let point = Field::new("point");
    /// let polygon = sql::Geometry::from(polygon!(
    ///             exterior: [
    ///                 (x: -111., y: 45.),
    ///                 (x: -111., y: 41.),
    ///                 (x: -104., y: 41.),
    ///                 (x: -104., y: 45.),
    ///             ],
    ///             interiors: [
    ///                 [
    ///                     (x: -110., y: 44.),
    ///                     (x: -110., y: 42.43),
    ///                     (x: -105., y: 42.),
    ///                     (x: -105., y: 44.),
    ///                 ],
    ///             ],
    ///         ));
    /// point.not_inside(polygon);
    /// ```
    fn not_inside<T>(&self, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        self.generate_query(sql::Operator::NotInside, value)
    }

    /// `ALLINSIDE` or `⊆` Check whether all of multiple values are contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `Array, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// let numbers = Field::new("numbers");
    /// numbers.all_inside(&[10, 20, 10]);
    /// ```
    fn all_inside<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayLike>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::AllInside, value)
    }

    /// `ANYINSIDE` or `⊂` Check whether any of multiple values are contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `Array, `Field` or `Param.C
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// let numbers = Field::new("numbers");
    /// numbers.any_inside(&[10, 20, 10]);
    /// ```
    fn any_inside<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayLike>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::AnyInside, value)
    }

    /// `NONEINSIDE` or `⊄` Check whether none of multiple values are contained within another value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be `Array, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// let numbers = Field::new("numbers");
    /// numbers.none_inside(vec![10, 20, 10]);
    /// ```
    fn none_inside<T>(&self, value: T) -> Operation
    where
        T: Into<ArrayLike>,
    {
        let value = value.into();
        self.generate_query(sql::Operator::NoneInside, value)
    }

    /// `OUTSIDE` Check whether a geometry value is outside another geometry value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be Geometry, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// use geo::polygon;
    ///
    /// let location = Field::new("location");
    /// let polygon_variable = polygon!(
    ///        exterior: [
    ///        (x: -0.38314819, y: 51.37692386),
    ///        (x: 0.1785278, y: 51.37692386),
    ///
    ///        (x: 0.1785278, y: 51.61460570),
    ///        (x: -0.38314819, y: 51.61460570),
    ///        (x: -0.38314819, y: 51.37692386),
    ///        ],
    ///        interiors: [],
    ///     );
    /// location.outside(polygon_variable);
    /// ```
    fn outside<T>(&self, value: T) -> Operation
    where
        T: Into<GeometryLike>,
    {
        let value: GeometryLike = value.into();
        self.generate_query(sql::Operator::Outside, value)
    }

    /// `INTERSECTS` Check whether a geometry value intersects annother geometry value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to compared with the current query. Could be Geometry, `Field` or `Param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// use geo::polygon;
    ///
    /// let location = Field::new("location");
    /// let polygon_variable = polygon!(
    ///        exterior: [
    ///        (x: -0.38314819, y: 51.37692386),
    ///        (x: 0.1785278, y: 51.37692386),
    ///
    ///        (x: 0.1785278, y: 51.61460570),
    ///        (x: -0.38314819, y: 51.61460570),
    ///        (x: -0.38314819, y: 51.37692386),
    ///        ],
    ///        interiors: [],
    ///     );
    /// location.intersects(polygon_variable);
    /// ```
    fn intersects<T>(&self, value: T) -> Operation
    where
        T: Into<GeometryLike>,
    {
        let value: GeometryLike = value.into();
        self.generate_query(sql::Operator::Intersects, value)
    }

    /// `@@` Checks whether the terms are found in a full-text indexed field
    ///
    /// # Arguments
    ///
    /// * `terms` - The terms to search for in the full-text indexed field.
    ///     can also be a `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    ///
    /// # let content = Field::new("content");
    /// let query = content.matches("hello world");
    /// assert_eq!(query.to_raw().build(), "content @@ 'hello world'");
    /// ```
    fn matches<T>(&self, terms: T) -> Operation
    where
        T: Into<StrandLike>,
    {
        self.generate_query(sql::Operator::Matches(None), terms.into())
    }

    /// `@[ref]@` Same as `matches` but using the matches operator with a reference checks whether the terms are found,
    /// highlights the searched terms, and computes the full-text score.
    ///
    /// # Arguments
    ///
    /// * `reference` - Optional. The specific reference or dictionary against which to perform the full-text search.
    ///     can also be a `Field` or `Param`.
    /// * `terms` - The terms to search for in the full-text indexed field.
    ///     can also be a `Field` or `Param`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    ///
    /// # let content = Field::new("content");
    ///
    /// let query = content.matches_with_ref(1, "hello world");
    /// assert_eq!(query.to_raw().build(), "content @1@ 'hello world'");
    /// ```
    fn matches_with_ref<T>(&self, reference: impl Into<NumberLike>, terms: T) -> Operation
    where
        T: Into<StrandLike>,
    {
        let reference: NumberLike = reference.into();
        let mut operation = self.generate_query(format!("@{}@", reference.build()), terms.into());

        operation.bindings.extend(reference.get_bindings());
        operation.errors.extend(reference.get_errors());

        operation
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
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    ///
    /// let age = Field::new("age");
    /// age.between(18, 30);
    /// ```
    fn between<T>(&self, lower_bound: T, upper_bound: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let lower_bound: ValueLike = lower_bound.into().into();
        let upper_bound: ValueLike = upper_bound.into().into();
        let condition = format!(
            "{} < {} < {}",
            lower_bound.build(),
            self.build(),
            upper_bound.build()
        );

        let lower_updated_params =
            self.__update_bindings(lower_bound.get_bindings().pop().expect("Must be one"));
        let upper_updated_params =
            self.__update_bindings(upper_bound.get_bindings().pop().expect("Must be one"));
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Operation {
            query_string: condition,
            bindings: updated_params,
            errors: vec![],
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
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    ///
    /// let age = Field::new("age");
    /// age.within(18, 30);
    /// ```
    fn within<T>(&self, lower_bound: T, upper_bound: T) -> Operation
    where
        T: Into<Ordinal>,
    {
        let lower_bound: ValueLike = lower_bound.into().into();
        let upper_bound: ValueLike = upper_bound.into().into();
        let condition = format!(
            "{} <= {} <= {}",
            lower_bound.build(),
            self.build(),
            upper_bound.build()
        );

        let lower_updated_params =
            self.__update_bindings(lower_bound.get_bindings().pop().expect("Must be one"));
        let upper_updated_params =
            self.__update_bindings(upper_bound.get_bindings().pop().expect("Must be one"));
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Operation {
            query_string: condition,
            bindings: updated_params,
            errors: vec![],
        }
    }

    /// Update the bindings of the current operation
    fn ____________update_many_bindings<'bi>(
        &self,
        bindings: impl Into<&'bi [Binding]>,
    ) -> Operation {
        let bindings: &'bi [Binding] = bindings.into();
        let updated_params = [&self.get_bindings().as_slice(), bindings].concat();
        Operation {
            query_string: self.build(),
            bindings: updated_params,
            errors: vec![],
        }
    }

    /// Update the bindings of the current operation
    fn __update_bindings(&self, binding: Binding) -> Vec<Binding> {
        [self.get_bindings().as_slice(), &[binding]].concat()
    }

    /// generates operation query string
    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> Operation
    where
        T: Into<ValueLike>,
    {
        let value: ValueLike = value.into();
        let condition = format!("{} {} {}", self.build(), operator, &value.build());
        let updated_bindings = [
            self.get_bindings().as_slice(),
            value.get_bindings().as_slice(),
        ]
        .concat();
        Operation {
            query_string: condition,
            bindings: updated_bindings,
            errors: vec![],
        }
    }
}
