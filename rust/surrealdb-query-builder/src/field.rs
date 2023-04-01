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
use surrealdb::{
    engine::local::Db,
    sql::{self, Number, Value},
};

use crate::{
    binding::{Binding, BindingsList, Parametric},
    clause::Empty,
    filter::Conditional,
    sql::{ArrayCustom, Buildable, Name, ToRawStatement},
    Clause, Erroneous, SurrealdbModel,
};

/// Represents a field in the database. This type wraps a `String` and
/// provides a convenient way to refer to a database fields.
///
/// # Examples
///
/// Creating a `Field`:
///
/// ```
/// use crate::query::field::Field;
///
/// let field = Field::new("name");
///
/// assert_eq!(field.to_string(), "name");
/// ```
#[derive(Debug, Clone)]
pub struct Field {
    field_name: sql::Idiom,
    pub condition_query_string: String,
    bindings: BindingsList,
}

impl Erroneous for Field {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Conditional for Field {
    fn get_condition_query_string(&self) -> String {
        format!("{}", self.condition_query_string)
    }
}

impl Parametric for Field {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl From<&Field> for Name {
    fn from(value: &Field) -> Self {
        Self::new(value.field_name.clone().into())
    }
}

impl From<&mut Field> for sql::Value {
    fn from(value: &mut Field) -> Operator {
        Self::Idiom(value.field_name.to_string().into())
    }
}

struct ValueCustom(sql::Value);

impl From<sql::Value> for ValueCustom {
    fn from(value: sql::Value) -> Operator {
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

impl Into<sql::Value> for &Field {
    fn into(self) -> Value {
        sql::Table(self.condition_query_string.to_string()).into()
    }
}

impl Into<sql::Idiom> for Field {
    fn into(self) -> sql::Idiom {
        self.field_name.into()
    }
}

impl From<Field> for sql::Value {
    fn from(val: Field) -> Operator {
        let mk = sql::Idiom::from(val.condition_query_string.to_string());
        // assert_eq!(sql::Value::from(mk).to_string(), "nawa".to_string());
        sql::Value::from(mk)
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
            fn from(value: $t) -> Operator {
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

impl Into<GeometryOrField> for Field {
    fn into(self) -> GeometryOrField {
        GeometryOrField::Field(self.into())
    }
}

impl Into<GeometryOrField> for &Field {
    fn into(self) -> GeometryOrField {
        GeometryOrField::Field(self.into())
    }
}

impl From<sql::Value> for GeometryOrField {
    fn from(value: Value) -> Operator {
        Self::Field(value)
    }
}

impl From<GeometryOrField> for sql::Value {
    fn from(val: GeometryOrField) -> Operator {
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
    fn from(value: sql::Datetime) -> Operator {
        Self::Datetime(value.into())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Ordinal {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Operator {
        Self::Datetime(value.into())
    }
}

macro_rules! impl_number_or_field_from {
    ($($t:ty),*) => {
        $(impl From<$t> for Ordinal {
            fn from(value: $t) -> Operator {
                Self::Number(sql::Number::from(value))
            }
        })*
    };
}

impl_number_or_field_from!(
    i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, BigDecimal
);

impl Into<Ordinal> for Field {
    fn into(self) -> Ordinal {
        Ordinal::Field(self.into())
    }
}

impl Into<Ordinal> for &Field {
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

impl<'a> From<Cow<'a, Self>> for Field {
    fn from(value: Cow<'a, Field>) -> Operator {
        match value {
            Cow::Borrowed(v) => v.clone(),
            Cow::Owned(v) => v,
        }
    }
}
impl<'a> From<&'a Field> for Cow<'a, Field> {
    fn from(value: &'a Field) -> Operator {
        Cow::Borrowed(value)
    }
}

impl From<Field> for Cow<'static, Field> {
    fn from(value: Field) -> Operator {
        Cow::Owned(value)
    }
}

impl From<String> for Field {
    fn from(value: String) -> Operator {
        Self::new(value)
    }
}
impl From<&Self> for Field {
    fn from(value: &Field) -> Operator {
        value.to_owned()
    }
}
impl From<&str> for Field {
    fn from(value: &str) -> Operator {
        let value: sql::Idiom = value.to_string().into();
        Self::new(Name::new(value))
    }
}

impl From<Field> for String {
    fn from(value: Field) -> Operator {
        value.condition_query_string
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            self.condition_query_string // self.condition_query_string.trim_start_matches("`")
        ))
    }
}

impl Field {
    pub fn new(field_name: impl Into<Name>) -> Operator {
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

    pub fn set_condition_query_string(mut self, connection_string: String) -> Operator {
        self.condition_query_string = connection_string;
        self
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
    /// use surrealdb::Field;
    ///
    /// let mut field = Field::new("name");
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
    /// use surrealdb::{Field, DbQuery};
    ///
    /// let field = Field::new("name");
    /// let query = field.__as__("name_alias");
    /// assert_eq!(query.to_string(), "name AS name_alias");
    /// ```
    pub fn __as__(&self, alias: impl std::fmt::Display) -> Operator {
        Self::new(format!("{} AS {}", self.condition_query_string, alias))
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
    pub fn between<T>(&self, lower_bound: T, upper_bound: T) -> Operator
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
    pub fn within<T>(&self, lower_bound: T, upper_bound: T) -> Operator
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
    ) -> Operator {
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
}

impl Operatable for Field {
    // fn ____________get_condition_string(&self) -> String {
    //     todo!()
    // }
    //
    // fn ____________update_condition_string(&mut self, condition_string: String) -> Operator {
    //     todo!()
    // }
    //
    // fn ____________update_many_bindings<'bi>(&self, bindings: impl Into<&'bi [Binding]>) -> Operator {
    //     todo!()
    // }

    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> Field
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let condition = format!(
            "{} {} {}",
            self.condition_query_string,
            operator,
            &binding.get_param_dollarised()
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

struct Fielda {
    value: sql::Idiom,
    bindings: BindingsList,
}

impl Fielda {
    fn new(value: impl Into<Name>) -> Operator {
        let value: sql::Idiom = value.into().into();
        // let value: sql::Idiom = value.clone().into();
        // println!("erer {}", value.clone());
        let bindings = dbg!(vec![Binding::new(sql::Value::from(value.clone()))]);
        Self { value, bindings }
    }
}

// impl ToRawStatement for Fielda {}

impl Parametric for Fielda {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for Fielda {
    fn build(&self) -> String {
        self.get_bindings().first().unwrap().get_param_dollarised()
        // self.0.to_string()
    }
}

#[test]
fn test_field() {
    let xx = Fielda::new("lowo");
    // let xx = Fielda::new(sql::Idiom::from("lowo".to_string()));
    let mm = xx.equal(34).less_than_or_equal(46);
    assert_eq!(mm.clone().to_raw().to_string(), "lowo = 34 <= 46");
    assert_eq!(mm.build(), "nawa");
}
impl Operational for Fielda {}

#[derive(Debug, Clone)]
pub struct Operator {
    query_string: String,
    bindings: BindingsList,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Operator {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}

impl Parametric for Operator {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Operational for Operator {}

pub trait Operational: Sized + Parametric + Buildable {
    fn equal<T>(&self, value: T) -> Operator
    where
        T: Into<sql::Value>,
    {
        self.generate_query(sql::Operator::Equal, value)
    }

    fn less_than_or_equal<T>(&self, value: T) -> Operator
    where
        T: Into<Ordinal>,
    {
        let value: Ordinal = value.into();
        self.generate_query(sql::Operator::LessThanOrEqual, value)
    }

    fn __update_bindings(&self, binding: Binding) -> Vec<Binding> {
        // let mut updated_params = Vec::with_capacity(self.bindings.len() + 1);
        // updated_params.extend(self.bindings.to_vec());
        // updated_params.extend([binding]);
        // updated_params
        [self.get_bindings().as_slice(), &[binding]].concat()
    }

    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> Operator
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let condition = format!(
            "{} {} ${}",
            self.build(),
            // self.condition_query_string,
            operator,
            &binding.get_param()
        );
        let updated_bindings = self.__update_bindings(binding);

        // let updated_bindings = self.__update_bindings(param, value);
        Operator {
            // condition_query_string: condition,
            query_string: condition,
            bindings: updated_bindings,
            // field_name: self.field_name.clone(),
        }
    }
}

pub trait Operatable: Sized + Parametric + Display {
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
    fn equal<T>(&self, value: T) -> Operator
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
    fn not_equal<T>(&self, value: T) -> Operator
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
    fn exactly_equal<T>(&self, value: T) -> Operator
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
    fn any_equal<T>(&self, value: T) -> Operator
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
    fn all_equal<T>(&self, value: T) -> Operator
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
    fn like<T>(&self, value: T) -> Operator
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
    fn not_like<T>(&self, value: T) -> Operator
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
    fn any_like<T>(&self, value: T) -> Operator
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
    fn all_like<T>(&self, value: T) -> Operator
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
    fn less_than<T>(&self, value: T) -> Operator
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
    fn less_than_or_equal<T>(&self, value: T) -> Operator
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
    fn greater_than<T>(&self, value: T) -> Operator
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
    fn greater_than_or_equal<T>(&self, value: T) -> Operator
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
    fn add<T>(&self, value: T) -> Operator
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
    fn subtract<T>(&self, value: T) -> Operator
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
    fn multiply<T>(&self, value: T) -> Operator
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
    fn divide<T>(&self, value: T) -> Operator
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
    fn truthy_and<T>(&self, value: T) -> Operator
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
    fn truthy_or<T>(&self, value: T) -> Operator
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
    fn and<T>(&self, value: T) -> Operator
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
    fn or<T>(&self, value: T) -> Operator
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
    fn is<T>(&self, value: T) -> Operator
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
    fn is_not<T>(&self, value: T) -> Operator
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
    fn contains<T>(&self, value: T) -> Operator
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
    fn contains_not<T>(&self, value: T) -> Operator
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
    fn contains_all<T>(&self, value: T) -> Operator
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
    fn contains_any<T>(&self, value: T) -> Operator
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
    fn contains_none<T>(&self, value: T) -> Operator
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
    fn inside<T>(&self, value: T) -> Operator
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
    fn not_inside<T>(&self, value: T) -> Operator
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
    fn all_inside<T>(&self, value: T) -> Operator
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
    fn any_inside<T>(&self, value: T) -> Operator
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
    fn none_inside<T>(&self, value: T) -> Operator
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
    fn outside<T>(&self, value: T) -> Operator
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
    fn intersects<T>(&self, value: T) -> Operator
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
    fn increment_by<T>(&self, value: T) -> Operator
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
    fn append<T>(&self, value: T) -> Operator
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
    fn decrement_by<T>(&self, value: T) -> Operator
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
    fn remove<T>(&self, value: T) -> Operator
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
    fn plus_equal<T>(&self, value: T) -> Operator
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
    fn minus_equal<T>(&self, value: T) -> Operator
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        self.generate_query("-=", value)
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

    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> Operator
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let condition = format!(
            "{} {} ${}",
            self.build(),
            // self.condition_query_string,
            operator,
            &binding.get_param()
        );
        let updated_bindings = self.__update_bindings(binding);

        // let updated_bindings = self.__update_bindings(param, value);
        Operator {
            // condition_query_string: condition,
            query_string: condition,
            bindings: updated_bindings,
            // field_name: self.field_name.clone(),
        }
    }
    // {
    //     let value: sql::Value = value.into();
    //     let binding = Binding::new(value);
    //     let condition = format!(
    //         "{} {} ${}",
    //         self.to_string(),
    //         // self.condition_query_string,
    //         operator,
    //         &binding.get_param()
    //     );
    //     let updated_bindings = self.__update_bindings(binding);
    //
    //     // let updated_bindings = self.__update_bindings(param, value);
    //     Operator {
    //         // condition_query_string: condition,
    //         query_string: condition,
    //         bindings: updated_bindings,
    //         // field_name: self.field_name.clone(),
    //     }
    // }
}
