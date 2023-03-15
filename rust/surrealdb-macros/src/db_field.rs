/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use bigdecimal::BigDecimal;
use serde::{
    de::{value, DeserializeOwned},
    Serialize,
};
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    fmt::{format, Display},
    ops::Deref,
    rc::Rc,
};

use proc_macro2::Span;
use surrealdb::sql::{self, Number, Value};

use crate::{
    query_define_token::Name, query_select::SelectStatement, value_type_wrappers::SurrealId,
    SurrealdbModel,
};

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
    field_name: sql::Idiom,
    condition_query_string: String,
    bindings: BindingsList,
}

pub type BindingsList = Vec<Binding>;
impl Parametric for DbField {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl From<&DbField> for Name {
    fn from(value: &DbField) -> Self {
        Self::new(value.field_name.clone().into())
    }
}

impl From<&mut DbField> for sql::Value {
    fn from(value: &mut DbField) -> Self {
        Self::Idiom(value.field_name.to_string().into())
    }
}

struct ValueCustom(sql::Value);

impl From<sql::Value> for ValueCustom {
    fn from(value: sql::Value) -> Self {
        ValueCustom(value)
    }
}

// impl<T: Into<sql::Value>> Deref for ValueCustom<T> {
//     type Target = sql::Value;
//
//     fn deref(&self) -> &Self::Target {
//         todo!()
//     }
// }

impl Into<sql::Value> for &DbField {
    fn into(self) -> Value {
        sql::Table(self.condition_query_string.to_string()).into()
    }
}

impl Into<sql::Value> for DbField {
    fn into(self) -> Value {
        sql::Table(self.condition_query_string.to_string()).into()
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

impl From<sql::Value> for GeometryOrField {
    fn from(value: Value) -> Self {
        Self::Field(value)
    }
}

impl From<GeometryOrField> for sql::Value {
    fn from(val: GeometryOrField) -> Self {
        match val {
            GeometryOrField::Geometry(g) => g.into(),
            GeometryOrField::Field(f) => f.into(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum Ordinal {
    Datetime(sql::Datetime),
    Number(sql::Number),
    Field(sql::Value),
}
impl From<sql::Datetime> for Ordinal {
    fn from(value: sql::Datetime) -> Self {
        Self::Datetime(value.into())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Ordinal {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::Datetime(value.into())
    }
}

macro_rules! impl_number_or_field_from {
    ($($t:ty),*) => {
        $(impl From<$t> for Ordinal {
            fn from(value: $t) -> Self {
                Self::Number(sql::Number::from(value))
            }
        })*
    };
}

impl_number_or_field_from!(
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, BigDecimal
);

impl Into<Ordinal> for DbField {
    fn into(self) -> Ordinal {
        Ordinal::Field(self.into())
    }
}

impl Into<Ordinal> for &DbField {
    fn into(self) -> Ordinal {
        Ordinal::Field(self.into())
    }
}
impl Into<sql::Value> for Ordinal {
    fn into(self) -> sql::Value {
        match self {
            Ordinal::Datetime(n) => n.into(),
            Ordinal::Number(n) => n.into(),
            Ordinal::Field(f) => f.into(),
        }
    }
}

impl Into<Ordinal> for sql::Number {
    fn into(self) -> Ordinal {
        Ordinal::Number(self)
    }
}

impl Into<DbFilter> for &DbField {
    fn into(self) -> DbFilter {
        DbFilter::new(self.clone())
    }
}

impl Into<DbFilter> for SelectStatement {
    fn into(self) -> DbFilter {
        let query_b: SelectStatement = self;
        DbFilter::new(query_b)
    }
}
impl From<SurrealId> for DbFilter {
    fn from(value: SurrealId) -> Self {
        // TODO: Check if to inline the string directly or stick with parametization and
        // autobinding
        DbFilter::new(value)
        // DbFilter::new(vec![value])
        // DbFilter::new(value.to_raw())
        // DbFilter::new("".into()).___update_bindings(&vec![b])
    }
}
impl From<DbField> for DbFilter {
    fn from(value: DbField) -> Self {
        Self {
            query_string: value.condition_query_string,
            bindings: value.bindings,
        }
    }
}

impl<'a> From<Cow<'a, Self>> for DbField {
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
        Self::new(value)
    }
}
impl From<&Self> for DbField {
    fn from(value: &DbField) -> Self {
        value.to_owned()
    }
}
impl From<&str> for DbField {
    fn from(value: &str) -> Self {
        let value: sql::Idiom = value.to_string().into();
        Self::new(Name::new(value))
    }
}

impl From<DbField> for String {
    fn from(value: DbField) -> Self {
        value.condition_query_string
    }
}

pub struct ArrayCustom(sql::Array);

pub struct NONE;

impl From<NONE> for sql::Value {
    fn from(value: NONE) -> Self {
        sql::Value::Idiom(value.into())
    }
}

impl Display for NONE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NONE")
    }
}

impl<T> From<Vec<T>> for ArrayCustom
where
    T: Into<sql::Value>,
{
    fn from(value: Vec<T>) -> Self {
        Self(
            value
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<sql::Value>>()
                .into(),
        )
    }
}

impl<T, const N: usize> From<&[T; N]> for ArrayCustom
where
    T: Into<sql::Value> + Clone,
{
    fn from(value: &[T; N]) -> Self {
        Self(
            value
                .into_iter()
                .map(|v| v.clone().into())
                .collect::<Vec<sql::Value>>()
                .into(),
        )
    }
}

impl From<ArrayCustom> for sql::Value {
    fn from(value: ArrayCustom) -> Self {
        Self::Array(value.0.into())
    }
}

impl std::fmt::Display for DbField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.condition_query_string))
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
#[derive(Debug, Clone, Default)]
pub struct DbFilter {
    query_string: String,
    bindings: BindingsList,
}

impl Parametric for DbFilter {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Binding {
    param: String,
    value: sql::Value,
    original_inline_name: String,
    raw_string: String,
    description: Option<String>,
}

impl Binding {
    pub fn new(value: impl Into<sql::Value>) -> Self {
        let value = value.into();
        let param_name = generate_param_name(&"param");
        let value_string = format!("{}", &value);
        Binding {
            param: param_name.clone(),
            value,
            original_inline_name: param_name.clone(),
            raw_string: value_string,
            description: None,
        }
    }

    pub fn with_raw(mut self, raw_string: String) -> Self {
        self.raw_string = raw_string;
        self
    }

    pub fn with_name(mut self, original_name: String) -> Self {
        self.original_inline_name = original_name;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn get_raw(&self) -> &String {
        &self.raw_string
    }

    pub fn get_original_name(&self) -> &String {
        &self.original_inline_name
    }

    pub fn get_param(&self) -> &String {
        &self.param
    }

    pub fn get_param_dollarised(&self) -> String {
        format!("${}", &self.param)
    }

    pub fn get_description(&self) -> String {
        format!("{}", self.description.as_ref().unwrap_or(&"".into()))
    }

    pub fn get_value(&self) -> &sql::Value {
        &self.value
    }
}

impl From<sql::Value> for Binding {
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

// impl From<(String, sql::Value)> for Binding {
//     fn from(value: (String, Value)) -> Self {
//         Self { field1: value }
//     }
// }

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
    DbFilter::new(Empty)
}

pub struct Empty;

impl From<Empty> for DbFilter {
    fn from(value: Empty) -> Self {
        DbFilter::new(value)
    }
}

impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(""))
    }
}

impl Parametric for Empty {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
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
    // pub fn new(query_string: String) -> Self {
    pub fn new(query: impl Parametric + std::fmt::Display) -> Self {
        let query_string = format!("{query}");
        let query_string = if query_string.is_empty() {
            "".into()
        } else {
            format!("({query_string})")
        };

        Self {
            query_string,
            bindings: query.get_bindings(),
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
        let new_params = self.___update_bindings(&filter);

        let ref filter: Self = filter.into();
        let query_string = format!("{precendence} OR ({filter})");

        DbFilter {
            query_string,
            bindings: new_params,
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
        let new_params = self.___update_bindings(&filter);

        let ref filter: Self = filter.into();
        let query_string = format!("{precendence} AND ({filter})");

        DbFilter {
            query_string,
            bindings: new_params,
        }
    }

    pub(crate) fn ___update_bindings(self, filter: &impl Parametric) -> Vec<Binding> {
        // let new_params = self
        //     .params
        //     .to_owned()
        //     .into_iter()
        //     .chain(filter.get_params().into_iter())
        //     .collect::<Vec<_>>(); // Consumed
        // let mut new_bindings = vec![];
        // new_bindings.extend(self.bindings);
        // new_bindings.extend(filter.get_bindings());
        // new_bindings
        [self.bindings.as_slice(), filter.get_bindings().as_slice()].concat()
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
        DbFilter {
            query_string: format!("({self})"),
            bindings: self.bindings.to_vec(),
        }
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

impl From<Option<Self>> for DbFilter {
    fn from(value: Option<DbFilter>) -> Self {
        match value {
            Some(v) => v,
            None => empty(),
        }
    }
}

impl From<String> for DbFilter {
    fn from(value: String) -> Self {
        Self {
            query_string: value,
            bindings: vec![],
        }
    }
}

impl std::fmt::Display for DbFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.query_string))
    }
}

fn generate_param_name(prefix: &str) -> String {
    let nil_id = uuid::Uuid::nil();
    #[cfg(test)]
    let sanitized_uuid = uuid::Uuid::nil();

    #[cfg(feature = "mock")]
    let sanitized_uuid = uuid::Uuid::nil();

    // #[cfg(not(test))]
    #[cfg(not(feature = "mock"))]
    let sanitized_uuid = uuid::Uuid::new_v4().simple();

    let mut param = format!("_{prefix}_{sanitized_uuid}");
    // TODO: this is temporary
    param.truncate(15);
    param
}

impl DbField {
    pub fn new(field_name: impl Into<Name>) -> Self {
        // let field: sql::Value = sql::Value::Idiom(field_name.into());
        let field_name: Name = field_name.into();
        let field_name: sql::Idiom = field_name.into();
        let field_name_str = format!("{}", &field_name);
        // let field: sql::Value = field_name.into();
        // This would be necessary if I decide to parametize and bind field names themselves
        // let binding = Binding::new(field_name.into());
        Self {
            field_name: field_name.into(),
            condition_query_string: field_name_str,
            bindings: vec![].into(),
            // TODO: Rethink if bindings should be used even for fields. If so, just uncomment
            // below in favour over above. This is more paranoid mode.
            // field_name: binding.get_param().to_string(),
            // bindings: vec![binding.into()].into(),
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
        self.condition_query_string.push_str(string)
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
        Self::new(format!("{} AS {}", self.condition_query_string, alias))
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
    /// use surrealdb::{DbField, DbQuery};
    ///
    /// let field = DbField::new("age");
    /// let query = field.not_equals(25);
    /// assert_eq!(query.to_string(), "age != 25");
    /// ```
    pub fn not_equal<T>(&self, value: T) -> Self
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
    pub fn exactly_equal<T>(&self, value: T) -> Self
    where
        T: Into<sql::Value>,
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
    pub fn like<T>(&self, value: T) -> Self
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
    pub fn not_like<T>(&self, value: T) -> Self
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
    pub fn any_like<T>(&self, value: T) -> Self
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
    pub fn all_like<T>(&self, value: T) -> Self
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
    pub fn less_than<T>(&self, value: T) -> Self
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
    pub fn less_than_or_equal<T>(&self, value: T) -> Self
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
    pub fn greater_than<T>(&self, value: T) -> Self
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
    pub fn greater_than_or_equal<T>(&self, value: T) -> Self
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
    pub fn add<T>(&self, value: T) -> Self
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
    pub fn subtract<T>(&self, value: T) -> Self
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
    pub fn multiply<T>(&self, value: T) -> Self
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
    pub fn divide<T>(&self, value: T) -> Self
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
    pub fn truthy_and<T>(&self, value: T) -> Self
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
    pub fn truthy_or<T>(&self, value: T) -> Self
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
    pub fn and<T>(&self, value: T) -> Self
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
    pub fn or<T>(&self, value: T) -> Self
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
    pub fn is<T>(&self, value: T) -> Self
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
    pub fn contains_not<T>(&self, value: T) -> Self
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
    pub fn contains_all<T>(&self, value: T) -> Self
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
    pub fn contains_any<T>(&self, value: T) -> Self
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
    pub fn contains_none<T>(&self, value: T) -> Self
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
    pub fn inside<T>(&self, value: T) -> Self
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
    pub fn not_inside<T>(&self, value: T) -> Self
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
    pub fn all_inside<T>(&self, value: T) -> Self
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
    pub fn any_inside<T>(&self, value: T) -> Self
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
    pub fn none_inside<T>(&self, value: T) -> Self
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
        T: Into<sql::Value>,
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
            self.condition_query_string,
            upper_bound_binding.get_param()
        );

        let lower_updated_params = self.__update_bindings(lower_bound_binding);
        let upper_updated_params = self.__update_bindings(upper_bound_binding);
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Self {
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
    pub fn within<T>(&self, lower_bound: T, upper_bound: T) -> Self
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
            self.condition_query_string,
            upper_bound_binding.get_param()
        );

        let lower_updated_params = self.__update_bindings(lower_bound_binding);
        let upper_updated_params = self.__update_bindings(upper_bound_binding);
        let updated_params = [lower_updated_params, upper_updated_params].concat();
        Self {
            condition_query_string: condition,
            bindings: updated_params,
            field_name: self.field_name.clone(),
        }
    }

    pub fn ____________update_many_bindings<'bi>(
        &self,
        bindings: impl Into<&'bi [Binding]>,
    ) -> Self {
        let bindings: &'bi [Binding] = bindings.into();
        // println!("bindingszz {bindings:?}");
        // updated_params.extend_from_slice(&self.bindings[..]);
        // updated_params.extend_from_slice(&bindings[..]);
        let updated_params = [&self.get_bindings().as_slice(), bindings].concat();
        Self {
            condition_query_string: self.condition_query_string.to_string(),
            bindings: updated_params,
            field_name: self.field_name.clone(),
        }
    }

    pub fn __update_bindings(&self, binding: Binding) -> Vec<Binding> {
        // let mut updated_params = Vec::with_capacity(self.bindings.len() + 1);
        // updated_params.extend(self.bindings.to_vec());
        // updated_params.extend([binding]);
        // updated_params
        [self.bindings.as_slice(), &[binding]].concat()
    }

    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> DbField
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let condition = format!(
            "{} {} ${}",
            self.condition_query_string,
            operator,
            &binding.get_param()
        );
        let updated_bindings = self.__update_bindings(binding);

        // let updated_bindings = self.__update_bindings(param, value);
        Self {
            condition_query_string: condition,
            bindings: updated_bindings,
            field_name: self.field_name.clone(),
        }
    }
}
