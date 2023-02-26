/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

use bigdecimal::BigDecimal;
use serde::Serialize;
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    fmt::{format, Display},
    ops::Deref,
    rc::Rc,
};

use proc_macro2::Span;
use surrealdb::sql::{self, Number, Uuid, Value};

use crate::query_select::QueryBuilder;

/// Represents a field in the database. This type wraps a `String` and
/// provides a convenient way to refer to a database fields.
///
/// # Examples
///
/// Creating a `DbField`:
///
/// ```
/// use crate::query::field::DbField;
///
/// let field = DbField::new("name");
///
/// assert_eq!(field.to_string(), "name");
/// ```
#[derive(Debug, Clone)]
pub struct DbField {
    field_name: String,
    params: ParamList,
}
pub type ParamList = Vec<Param>;
impl ParamsExtractor for DbField {
    fn get_params(&self) -> ParamList {
        self.params.to_vec()
    }
}

// impl From<&mut DbField> for sql::Value {
//     fn from(value: &mut DbField) -> Self {
//         // sql::Value(value.field_name.to_string())
//         Self::Table(value.field_name.to_string().into())
//     }
// }
impl From<&mut DbField> for sql::Table {
    fn from(value: &mut DbField) -> Self {
        sql::Table(value.field_name.to_string())
    }
}

impl From<DbField> for sql::Table {
    fn from(value: DbField) -> Self {
        sql::Table(value.field_name)
    }
}

impl Into<Value> for &DbField {
    fn into(self) -> Value {
        sql::Table(self.field_name.to_string()).into()
    }
}

impl Into<Value> for DbField {
    fn into(self) -> Value {
        sql::Table(self.field_name.to_string()).into()
    }
}

impl Into<Number> for &DbField {
    fn into(self) -> Number {
        Value::Table(self.field_name.to_string().into())
            .as_string()
            .into()
    }
}
#[derive(serde::Serialize, Debug, Clone)]
pub enum GeometryOrField {
    Geometry(sql::Geometry),
    Field(Value),
}

macro_rules! impl_geometry_or_field_from {
    ($($t:ty),*) => {
        $(impl From<$t> for GeometryOrField {
            fn from(value: $t) -> Self {
                Self::Geometry(sql::Geometry::from(value))
            }
        })*
    };
}

impl_geometry_or_field_from!(
    geo::Polygon,
    geo::Point,
    geo::LineString,
    geo::MultiPoint,
    geo::MultiPolygon,
    geo::MultiLineString
);

impl Into<GeometryOrField> for DbField {
    fn into(self) -> GeometryOrField {
        GeometryOrField::Field(self.into())
    }
}

impl Into<GeometryOrField> for &DbField {
    fn into(self) -> GeometryOrField {
        GeometryOrField::Field(self.into())
    }
}

impl From<Value> for GeometryOrField {
    fn from(value: Value) -> Self {
        Self::Field(value)
    }
}

impl From<GeometryOrField> for Value {
    fn from(val: GeometryOrField) -> Self {
        match val {
            GeometryOrField::Geometry(g) => g.into(),
            GeometryOrField::Field(f) => f.into(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum NumberOrField {
    Number(sql::Number),
    Field(Value),
}

macro_rules! impl_number_or_field_from {
    ($($t:ty),*) => {
        $(impl From<$t> for NumberOrField {
            fn from(value: $t) -> Self {
                Self::Number(sql::Number::from(value))
            }
        })*
    };
}

impl_number_or_field_from!(
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, BigDecimal
);

impl Into<NumberOrField> for DbField {
    fn into(self) -> NumberOrField {
        NumberOrField::Field(self.into())
    }
}

impl Into<NumberOrField> for &DbField {
    fn into(self) -> NumberOrField {
        NumberOrField::Field(self.into())
    }
}
impl Into<Value> for NumberOrField {
    fn into(self) -> Value {
        match self {
            NumberOrField::Number(n) => n.into(),
            NumberOrField::Field(f) => f.into(),
        }
    }
}

// impl std::fmt::Display for NumberOfField {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let num_field: sql::Value = match self {
//             NumberOfField::Number(n) => n.to_owned().into,
//             NumberOfField::Field(f) => f.clone().into(),
//         };
//         f.write_fmt(format_args!("{}", self.))
//     }
// }

impl Into<NumberOrField> for sql::Number {
    fn into(self) -> NumberOrField {
        NumberOrField::Number(self)
    }
}

impl Into<Number> for DbField {
    fn into(self) -> Number {
        // sql::Strand::from(self.field_name.into())
        // let xx = sql::Strand::from(self.field_name.into());
        Value::Strand(self.field_name.into()).as_string().into()
    }
}

impl Into<DbFilter> for &DbField {
    fn into(self) -> DbFilter {
        DbFilter::new(self.to_string())
    }
}

impl<'a> Into<DbFilter> for QueryBuilder<'a> {
    fn into(self) -> DbFilter {
        let query_b: QueryBuilder = self;
        DbFilter::new(query_b.to_string())
    }
}
impl From<DbField> for DbFilter {
    fn from(value: DbField) -> Self {
        Self {
            query_string: value.field_name,
            params: value.params,
        }
    }
}
// impl Into<DbFilter> for DbField {
//     fn into(self) -> DbFilter {
//         DbFilter::new(self.into())
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

impl From<String> for DbField {
    fn from(value: String) -> Self {
        Self {
            field_name: value.into(),
            params: vec![],
        }
    }
}
impl From<&str> for DbField {
    fn from(value: &str) -> Self {
        Self {
            field_name: value.into(),
            params: vec![],
        }
    }
}

impl From<DbField> for String {
    fn from(value: DbField) -> Self {
        value.field_name
    }
}

impl std::fmt::Display for DbField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.field_name))
    }
}

/// This module provides functionality for building complex filters for database queries.
///
/// A `DbFilter` struct represents a filter that can be composed of subfilters using logical
/// operators like `AND` and `OR`. Filters can be created using the `empty` function or by
/// converting a string using `DbFilter::new`.
///
/// The `cond` function is used to create a new filter from a given `filterable` input, which
/// can be of type `DbFilter`.
///
/// Methods on a `DbFilter` instance are used to combine filters with logical operators or to
/// modify the filter using methods like `bracketed`.
///
/// # Examples
///
/// ```
/// use crate::query::filter::{DbFilter, cond};
///
/// let filter1 = DbFilter::new("name = 'John'".to_string());
/// let filter2 = DbFilter::new("age > 18".to_string());
///
/// // Combine two filters using the 'AND' operator
/// let combined_filter = filter1.and(filter2);
///
/// assert_eq!(combined_filter.to_string(), "(name = 'John') AND (age > 18)");
///
/// // Create a filter from a string
/// let filter3 = DbFilter::new("name like '%Doe%'".to_string());
///
/// // Combine multiple filters using the 'OR' operator
/// let all_filters = cond(filter1).or(filter2).or(filter3);
///
/// assert_eq!(all_filters.to_string(), "(name = 'John') OR (age > 18) OR (name like '%Doe%')");
/// ```
#[derive(Debug, Clone)]
pub struct DbFilter {
    query_string: String,
    params: Vec<Param>,
}

impl ParamsExtractor for DbFilter {
    fn get_params(&self) -> ParamList {
        self.params.to_vec()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Param(pub (String, Value));

impl From<(String, Value)> for Param {
    fn from(value: (String, Value)) -> Self {
        Self(value)
    }
}

pub trait ParamsExtractor {
    fn get_params(&self) -> ParamList;
}
/// Creates a new filter from a given `filterable` input.
///
/// # Arguments
///
/// * `filterable` - A value that can be converted into a `DbFilter`.
///
/// # Example
///
/// ```
/// use crate::query::filter::{DbFilter, cond};
///
/// let filter = DbFilter::new("name = 'John'".to_string());
///
/// let combined_filter = cond(filter).and("age > 18");
///
/// assert_eq!(combined_filter.to_string(), "(name = 'John') AND (age > 18)");
/// ```
pub fn cond(filterable: impl Into<DbFilter> + ParamsExtractor) -> DbFilter {
    // let filterable: DbFilter = filterable.into();
    // DbFilter {
    //     query_string: filterable,
    //     params: filterable.get_params(),
    // }
    filterable.into()
}

/// Creates an empty filter.
///
/// # Example
///
/// ```
/// use crate::query::filter::DbFilter;
///
/// let empty_filter = DbFilter::empty();
///
/// assert_eq!(empty_filter.to_string(), "");
///
pub fn empty() -> DbFilter {
    DbFilter::new("".into())
}

impl DbFilter {
    /// Creates a new `DbFilter` instance.
    ///
    /// # Arguments
    ///
    /// * `query_string` - The query string used to initialize the filter.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::query::filter::DbFilter;
    ///
    /// let filter = DbFilter::new("name = 'John'".to_string());
    ///
    /// assert_eq!(filter.to_string(), "name = 'John'");
    /// ```
    pub fn new(query_string: String) -> Self {
        Self {
            query_string,
            params: vec![].into(),
        }
    }

    /// Combines the current filter with another filter using a logical OR operator.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter to be combined with the current filter.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::query::filter::{DbFilter, cond};
    ///
    /// let filter = cond(DbFilter::new("name = 'John'".to_string())).or(
    ///     cond(DbFilter::new("age > 30".to_string()))
    /// );
    ///
    /// assert_eq!(filter.to_string(), "(name = 'John') OR (age > 30)");
    /// ```
    pub fn or(self, filter: impl Into<Self> + ParamsExtractor) -> Self {
        let precendence = self._______bracket_if_not_already();
        let new_params = self.___update_params(&filter);

        let ref filter: Self = filter.into();
        let query_string = format!("{precendence} OR ({filter})");

        DbFilter {
            query_string,
            params: new_params,
        }
    }

    /// Combines this `DbFilter` instance with another using the `AND` operator.
    ///
    /// # Arguments
    ///
    /// * `filter` - The `DbFilter` instance to combine with this one.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::query::filter::{DbFilter, cond};
    ///
    /// let filter1 = cond(DbFilter::new("name = 'John'"));
    /// let filter2 = cond(DbFilter::new("age > 30"));
    /// let combined = filter1.and(filter2);
    ///
    /// assert_eq!(combined.to_string(), "(name = 'John') AND (age > 30)");
    /// ```
    pub fn and(self, filter: impl Into<Self> + ParamsExtractor) -> Self {
        let precendence = self._______bracket_if_not_already();
        let new_params = self.___update_params(&filter);

        let ref filter: Self = filter.into();
        let query_string = format!("{precendence} AND ({filter})");

        DbFilter {
            query_string,
            params: new_params,
        }
    }

    fn ___update_params(self, filter: &impl ParamsExtractor) -> Vec<Param> {
        // let new_params = self
        //     .params
        //     .to_owned()
        //     .into_iter()
        //     .chain(filter.get_params().into_iter())
        //     .collect::<Vec<_>>(); // Consumed
        let mut new_params = vec![];
        new_params.extend(self.params);
        new_params.extend(filter.get_params());
        new_params
    }

    /// Wraps this `DbFilter` instance in a set of brackets.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::query::filter::{DbFilter, cond};
    ///
    /// let filter = cond(DbFilter::new("name = 'John'")).or(cond(DbFilter::new("age > 30")));
    /// let wrapped = filter.bracketed();
    ///
    /// assert_eq!(wrapped.to_string(), "((name = 'John') OR (age > 30))");
    /// ```
    pub fn bracketed(&self) -> Self {
        DbFilter::new(format!("({self})"))
    }

    /// Wraps this `DbFilter` instance in a set of brackets if it isn't already wrapped.
    fn _______bracket_if_not_already(&self) -> impl Display {
        let filter = self.to_string();
        match (filter.starts_with('('), filter.ends_with(')')) {
            (true, true) => format!("{self}"),
            _ => format!("({self})"),
        }
    }
}

impl<'a> From<Cow<'a, DbFilter>> for DbFilter {
    fn from(value: Cow<'a, DbFilter>) -> Self {
        match value {
            Cow::Borrowed(v) => v.clone(),
            Cow::Owned(v) => v,
        }
    }
}

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

impl std::fmt::Display for DbFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.query_string))
    }
}

fn generate_param_name() -> String {
    let sanitized_uuid = Uuid::new_v4().simple();
    format!("_{sanitized_uuid}")
}

impl DbField {
    pub fn new(field_name: impl Display) -> Self {
        Self {
            field_name: field_name.to_string(),
            // params: Rc::new(vec![].into()),
            params: vec![].into(),
        }
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
        self.field_name.push_str(string)
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
    pub fn __as__(&self, alias: impl std::fmt::Display) -> Self {
        Self::new(format!("{} AS {}", self.field_name, alias))
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
    pub fn equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        let param = generate_param_name();
        let condition = format!("{} = {}", self.field_name, param);

        let updated_params = self.__update_params(param, value);
        Self {
            field_name: condition,
            params: updated_params,
        }
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
    pub fn not_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        let param = generate_param_name();
        let condition = format!("{} != {}", self.field_name, value);

        let updated_params = self.__update_params(param, value);
        Self {
            field_name: condition,
            params: updated_params,
        }
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
    pub fn greater_than<T>(&self, value: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let value: NumberOrField = value.into();
        let value: Value = value.into();
        let param = generate_param_name();
        let condition = format!("{} > ${}", self.field_name, &param);

        let updated_params = self.__update_params(param, value);
        Self {
            field_name: condition,
            params: updated_params,
        }
    }

    fn __update_params(&self, param: String, value: Value) -> Vec<Param> {
        let mut updated_params = vec![];
        updated_params.extend(self.params.to_vec());
        updated_params.extend([(param, value).into()]);
        updated_params
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
    pub fn greater_than_or_equal<T>(&self, value: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let value: NumberOrField = value.into();
        let value: Value = value.into();
        let param = generate_param_name();
        let condition = format!("{} >= ${}", self.field_name, &param);

        let updated_params = self.__update_params(param, value);
        Self {
            field_name: condition,
            params: updated_params,
        }
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
    pub fn less_than<T>(&self, value: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let value: NumberOrField = value.into();
        let value: Value = value.into();
        let param = generate_param_name();
        let condition = format!("{} < {}", self.field_name, &param);
        let updated_params = self.__update_params(param, value);

        Self {
            field_name: condition,
            params: updated_params,
        }
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
    pub fn less_than_or_equals<T>(&self, value: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let value: NumberOrField = value.into();
        let value: Value = value.into();
        let param = generate_param_name();
        let condition = format!("{} <= {}", self.field_name, param);

        let updated_params = self.__update_params(param, value);
        Self {
            field_name: condition,
            params: updated_params,
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
    /// let query = DbQuery::field("age").between(18, 30);
    /// assert_eq!(query.to_string(), "age < 18 AND age < 30");
    /// ```
    pub fn between<T>(&self, lower_bound: T, upper_bound: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let lower_bound: NumberOrField = lower_bound.into();
        let lower_bound: Value = lower_bound.into();
        let upper_bound: NumberOrField = upper_bound.into();
        let upper_bound: Value = upper_bound.into();
        let lower_param = generate_param_name();
        let upper_param = generate_param_name();
        let condition = format!("{} < {} < {}", lower_param, self.field_name, upper_param);

        let lower_updated_params = self.__update_params(lower_param, lower_bound);
        let upper_updated_params = self.__update_params(upper_param, upper_bound);
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Self {
            field_name: condition,
            params: updated_params,
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
    pub fn within<T>(&self, lower_bound: T, upper_bound: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let lower_bound: NumberOrField = lower_bound.into();
        let lower_bound: Value = lower_bound.into();
        let upper_bound: NumberOrField = upper_bound.into();
        let upper_bound: Value = upper_bound.into();
        let lower_param = generate_param_name();
        let upper_param = generate_param_name();
        let condition = format!("{} <= {} <= {}", lower_param, self.field_name, upper_param);

        let lower_updated_params = self.__update_params(lower_param, lower_bound);
        let upper_updated_params = self.__update_params(upper_param, upper_bound);
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Self {
            field_name: condition,
            params: updated_params,
        }
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
    pub fn like<T>(&self, pattern: T) -> Self
    where
        T: Into<Value>,
    {
        let pattern: Value = pattern.into();
        Self::new(format!("{} LIKE {}", self.field_name, pattern))
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
    pub fn not_like<T>(&self, pattern: T) -> Self
    where
        T: Into<Value>,
    {
        let pattern: Value = pattern.into();
        Self::new(format!("{} NOT LIKE {}", self.field_name, pattern))
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
    pub fn is_null(&self) -> Self {
        Self::new(format!("{} IS NULL", self.field_name))
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
    pub fn is_not_null(&self) -> Self {
        Self::new(format!("{} IS NOT NULL", self.field_name))
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
    pub fn exactly_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} == {}", self.field_name, value))
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
    pub fn any_equal<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} ?= ({})", self.field_name, values_str))
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
    pub fn all_equal<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} *= ({})", self.field_name, values_str))
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
    pub fn fuzzy_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value> + Clone,
    {
        let value: Value = value.into();
        Self::new(format!("{} ~ {}", self.field_name, value))
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
    pub fn fuzzy_not_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} !~ {}", self.field_name, value))
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
    pub fn any_fuzzy_equal<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} ?~ ({})", self.field_name, values_str))
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
    pub fn all_fuzzy_equal<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} *~ ({})", self.field_name, values_str))
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
    pub fn less_than_or_equal<T>(&self, value: T) -> Self
    where
        T: Into<Number>,
    {
        let value: Number = value.into();
        Self::new(format!("{} <= {}", self.field_name, value))
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
    pub fn add<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} + {}", self.field_name, value))
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
    pub fn contains<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} CONTAINS {}", self.field_name, value))
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
    pub fn contains_not<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} CONTAINSNOT {}", self.field_name, value))
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
    pub fn contains_all<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} CONTAINSALL ({})", self.field_name, values_str))
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
    pub fn contains_any<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} CONTAINSANY ({})", self.field_name, values_str))
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
    pub fn contains_none<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} CONTAINSNONE ({})", self.field_name, values_str))
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
    pub fn inside<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} INSIDE {}", self.field_name, value))
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
    pub fn not_inside<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} NOTINSIDE {}", self.field_name, value))
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
    pub fn all_inside<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} ALLINSIDE ({})", self.field_name, values_str))
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
    pub fn any_inside<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} ANYINSIDE ({})", self.field_name, values_str))
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
    pub fn none_inside<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} NONEINSIDE ({})", self.field_name, values_str))
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
    pub fn outside<T>(&self, value: T) -> Self
    where
        T: Into<GeometryOrField>,
    {
        let value: GeometryOrField = value.into();
        let value: Value = value.into();
        let xx = Self::new(format!("OUTSIDE {}", value));
        println!("koko {}", xx);
        Self::new(format!("{} OUTSIDE {}", self.field_name, value))
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
    pub fn intersects<T>(&self, value: T) -> Self
    where
        T: Into<GeometryOrField>,
    {
        let value: GeometryOrField = value.into();
        let value: Value = value.into();
        Self::new(format!("{} INTERSECTS {}", self.field_name, value))
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
    pub fn any_in_set<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} ?= ({})", self.field_name, values_str))
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
    pub fn all_in_set<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let values_str = collect_query_values(values);
        Self::new(format!("{} *= ({})", self.field_name, values_str))
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
    pub fn subtract<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} - {}", self.field_name, value))
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
    pub fn multiply<T>(&self, value: T) -> Self
    where
        T: Into<Number>,
    {
        let value: Number = value.into();
        Self::new(format!("{} * {}", self.field_name, value))
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
    pub fn divide<T>(&self, value: T) -> Self
    where
        T: Into<Number>,
    {
        let value: Number = value.into();
        Self::new(format!("{} / {}", self.field_name, value))
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
    pub fn is_truthy(&self) -> Self {
        Self::new(format!("{} && true", self.field_name))
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
    pub fn truthy_and<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} && {}", self.field_name, value))
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
    pub fn truthy_or<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} || {}", self.field_name, value))
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
    pub fn is<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} IS {}", self.field_name, value))
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
    pub fn is_not<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        Self::new(format!("{} IS NOT {}", self.field_name, value))
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
    pub fn set_equal<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let joined_values = collect_query_values(values);
        Self::new(format!("{} ?= {{{}}}", self.field_name, joined_values))
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
    pub fn set_all_equal<T>(&self, values: &[T]) -> Self
    where
        T: Into<Value> + Clone,
    {
        let joined_values = collect_query_values(values);
        Self::new(format!("{} *= {{{}}}", self.field_name, joined_values))
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
    pub fn and<T>(&self, other: T) -> Self
    where
        T: Into<Value>,
    {
        let other: Value = other.into();
        Self::new(format!("{} AND {}", self.field_name, other))
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
    pub fn or<T>(&self, other: T) -> Self
    where
        T: Into<Value>,
    {
        let other: Value = other.into();
        Self::new(format!("{} OR {}", self, other))
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
    pub fn increment_by<T>(&self, value: T) -> Self
    where
        T: Into<Number>,
    {
        let value: Number = value.into();
        let increment_string = format!("{self} += {}", value);
        // let other = serde_json::to_string(&other).unwrap();
        // let other = sql::json(&other).unwrap();
        // println!("PAOELEEEE {}", &other);
        // println!("PAOELEEEE {}", serde_json::to_string(&other).unwrap());
        // println!("{}", Self::new(format!("{self} += {other}")));
        Self::new(increment_string)
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
    pub fn append<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        let add_string = format!("{self} += {}", value);
        Self::new(add_string)
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
    pub fn decrement_by<T>(&self, value: T) -> Self
    where
        T: Into<Number>,
    {
        let value: Number = value.into();
        let decrement_string = format!("{self} -= {}", value);
        Self::new(decrement_string)
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
    pub fn remove<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        let remove_string = format!("{self} -= {}", value);
        Self::new(remove_string)
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
    pub fn plus_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        let increment_string = format!("{self} += {}", value);
        Self::new(increment_string)
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
    pub fn minus_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        let remove_string = format!("{self} -= {}", value);
        Self::new(remove_string)
    }
}

fn collect_query_values<T>(values: &[T]) -> String
where
    T: Into<Value> + Clone,
{
    let values_str = values
        .to_vec()
        .into_iter()
        .map(|value| {
            let value: Value = value.into();
            format!("{}", value)
        })
        .collect::<Vec<String>>()
        .join(", ");
    values_str
}
