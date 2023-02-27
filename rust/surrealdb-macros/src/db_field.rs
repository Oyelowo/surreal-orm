/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

use bigdecimal::BigDecimal;
use serde::{de::value, Serialize};
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
    bindings: BindingsList,
}
pub type BindingsList = Vec<Binding>;
impl Parametric for DbField {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
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
    Field(sql::Value),
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
            params: value.bindings,
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
            bindings: vec![],
        }
    }
}
impl From<&str> for DbField {
    fn from(value: &str) -> Self {
        Self {
            field_name: value.into(),
            bindings: vec![],
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
    params: Vec<Binding>,
}

impl Parametric for DbFilter {
    fn get_bindings(&self) -> BindingsList {
        self.params.to_vec()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Binding(pub (String, Value));

impl From<(String, Value)> for Binding {
    fn from(value: (String, Value)) -> Self {
        Self(value)
    }
}

/// Can have parameters which can be bound
pub trait Parametric {
    fn get_bindings(&self) -> BindingsList;
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
pub fn cond(filterable: impl Into<DbFilter> + Parametric) -> DbFilter {
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
            query_string: format!("({query_string})"),
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
    pub fn or(self, filter: impl Into<Self> + Parametric) -> Self {
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
    pub fn and(self, filter: impl Into<Self> + Parametric) -> Self {
        let precendence = self._______bracket_if_not_already();
        let new_params = self.___update_params(&filter);

        let ref filter: Self = filter.into();
        let query_string = format!("{precendence} AND ({filter})");

        DbFilter {
            query_string,
            params: new_params,
        }
    }

    fn ___update_params(self, filter: &impl Parametric) -> Vec<Binding> {
        // let new_params = self
        //     .params
        //     .to_owned()
        //     .into_iter()
        //     .chain(filter.get_params().into_iter())
        //     .collect::<Vec<_>>(); // Consumed
        let mut new_params = vec![];
        new_params.extend(self.params);
        new_params.extend(filter.get_bindings());
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

fn generate_param_name(prefix: &str) -> String {
    let sanitized_uuid = Uuid::new_v4().simple();
    let mut param = format!("_{prefix}_{sanitized_uuid}");
    // TODO: this is temporary
    param.truncate(15);
    param
}

impl DbField {
    pub fn new(field_name: impl Display) -> Self {
        Self {
            field_name: field_name.to_string(),
            bindings: vec![].into(),
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
    pub fn exactly_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn any_equal<T>(&self, value: T) -> Self
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
    pub fn all_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn like<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        self.generate_query(sql::Operator::Like, value)
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
    pub fn not_like<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn any_like<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn all_like<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn less_than<T>(&self, value: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let value: NumberOrField = value.into();
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
    pub fn less_than_or_equal<T>(&self, value: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let value: NumberOrField = value.into();
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
    pub fn greater_than<T>(&self, value: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let value: NumberOrField = value.into();
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
    pub fn greater_than_or_equal<T>(&self, value: T) -> Self
    where
        T: Into<NumberOrField>,
    {
        let value: NumberOrField = value.into();
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
    pub fn add<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn subtract<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn multiply<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn divide<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn truthy_and<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn truthy_or<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn and<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn or<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn is<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn is_not<T>(&self, value: T) -> Self
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
    pub fn contains<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn contains_not<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn contains_all<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
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
    /// let new_query = query.contains_all([10, 20, 10]);
    ///
    /// assert_eq!(new_query.to_string(), "number_counts CONTAINSANY [10, 20, 10]");
    /// ```
    pub fn contains_any<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        self.generate_query(sql::Operator::ContainAny, value)
    }

    /// Check whether a value does not contain any of multiple values.
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
    /// assert_eq!(new_query.to_string(), "number_counts CONTAINSNONE [10, 20, 10]");
    /// ```
    pub fn contains_none<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
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
    pub fn inside<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn not_inside<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
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
    pub fn all_inside<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
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
    pub fn any_inside<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
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
    pub fn none_inside<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
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
    pub fn outside<T>(&self, value: T) -> Self
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
    pub fn intersects<T>(&self, value: T) -> Self
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
    pub fn increment_by<T>(&self, value: T) -> Self
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
    pub fn append<T>(&self, value: T) -> Self
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
    pub fn decrement_by<T>(&self, value: T) -> Self
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
    pub fn remove<T>(&self, value: T) -> Self
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
    pub fn plus_equal<T>(&self, value: T) -> Self
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
    pub fn minus_equal<T>(&self, value: T) -> Self
    where
        T: Into<Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query("-=", value)
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
        let lower_param = generate_param_name(&self.field_name);
        let upper_param = generate_param_name(&self.field_name);
        let condition = format!("{} < {} < {}", lower_param, self.field_name, upper_param);

        let lower_updated_params = self.__update_bindings(lower_param, lower_bound);
        let upper_updated_params = self.__update_bindings(upper_param, upper_bound);
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Self {
            field_name: condition,
            bindings: updated_params,
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
        let lower_param = generate_param_name(&self.field_name);
        let upper_param = generate_param_name(&self.field_name);
        let condition = format!("{} <= {} <= {}", lower_param, self.field_name, upper_param);

        let lower_updated_params = self.__update_bindings(lower_param, lower_bound);
        let upper_updated_params = self.__update_bindings(upper_param, upper_bound);
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Self {
            field_name: condition,
            bindings: updated_params,
        }
    }

    fn __update_bindings(&self, param: String, value: sql::Value) -> Vec<Binding> {
        let mut updated_params = vec![];
        updated_params.extend(self.bindings.to_vec());
        updated_params.extend([(param, value).into()]);
        updated_params
    }

    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> DbField
    where
        T: Into<Value>,
    {
        let value: Value = value.into();
        let param = generate_param_name(&"param");
        let condition = format!("{} {} ${}", self.field_name, operator, &param);

        let updated_params = self.__update_bindings(param, value);
        Self {
            field_name: condition,
            bindings: updated_params,
        }
    }
}
