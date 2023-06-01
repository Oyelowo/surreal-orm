/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

/*
 * Syntax from specs:https://surrealdb.com/docs/surrealql/statements/select
 * SELECT @projections
    FROM @targets
    [  @condition ]
    [ SPLIT [ AT ] @field ... ]
    [ GROUP [ BY ] @field ... ]
    [ ORDER [ BY ]
        @field [
            RAND()
            | COLLATE
            | NUMERIC
        ] [ ASC | DESC ] ...
    ] ]
    [ LIMIT [ BY ] @limit ]
    [ START [ AT ] @start ]
    [ FETCH @field ... ]
    [ TIMEOUT @duration ]
    [ PARALLEL ]
; */
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    ops::Deref,
};

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    Aliasable, All, Binding, BindingsList, Buildable, Conditional, DurationLike, Erroneous,
    ErrorList, Field, Filter, Function, NumberLike, Parametric, Queryable, ReturnableSelect,
    ReturnableStandard, SurrealId, SurrealSimpleId, SurrealUlid, SurrealUuid, SurrealdbModel,
    Table, ToRaw, Valuex,
};

use super::Subquery;

/// Creates a new `Order` instance with the specified database field.
///
/// To sort records, SurrealDB allows ordering on multiple fields and nested fields. Use the ORDER
/// BY clause to specify a comma-separated list of field names which should be used to order the
/// resulting records. The ASC and DESC keywords can be used to specify whether results should be
/// sorted in an ascending or descending manner. The COLLATE keyword can be used to use unicode
/// collation when ordering text in string values, ensuring that different cases, and different
/// languages are sorted in a consistent manner. Finally the NUMERIC can be used to correctly sort
/// text which contains numeric values.
///
/// # Arguments
///
/// * `field` - A reference to a `Field` instance to be used as the ordering field.
///
/// # Example
///
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, statements::{order, select}};
/// # let age = Field::new("age");
/// # let score = Field::new("score");
/// # let name = Field::new("name");
///
/// order(age).numeric().asc();
/// order(score).rand().desc();
/// order(name).collate().desc();
/// ```
pub fn order(field: impl Into<Field>) -> Order {
    let field: Field = field.into();
    Order {
        field: field.clone(),
        direction: None,
        option: None,
    }
}

/// Represents an ordering field, direction, and options for a database query.
#[derive(Debug, Clone)]
pub struct Order {
    field: Field,
    direction: Option<OrderDirection>,
    option: Option<OrderOption>,
}

impl Parametric for Order {
    fn get_bindings(&self) -> BindingsList {
        self.field.get_bindings()
    }
}

impl Parametric for &[Order] {
    fn get_bindings(&self) -> BindingsList {
        self.into_iter()
            .flat_map(|o| o.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Parametric for Vec<Order> {
    fn get_bindings(&self) -> BindingsList {
        self.into_iter()
            .flat_map(|o| o.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Parametric for Orderables {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Orderables::Order(o) => o.get_bindings(),
            Orderables::OrdersList(ol) => ol.get_bindings(),
        }
    }
}

/// an order or list of orders
pub enum Orderables {
    /// Single order
    Order(Order),
    /// Multiple orders
    OrdersList(Vec<Order>),
}

impl From<Order> for Orderables {
    fn from(value: Order) -> Self {
        Self::Order(value)
    }
}

impl From<Vec<Order>> for Orderables {
    fn from(value: Vec<Order>) -> Self {
        Self::OrdersList(value)
    }
}

impl<const N: usize> From<&[Order; N]> for Orderables {
    fn from(value: &[Order; N]) -> Self {
        Self::OrdersList(value.to_vec())
    }
}

impl From<Orderables> for Vec<Order> {
    fn from(value: Orderables) -> Self {
        match value {
            Orderables::Order(o) => vec![o.into()],
            Orderables::OrdersList(ol) => ol,
        }
    }
}

impl Order {
    /// Sets the direction of the ordering to ascending.
    pub fn asc(mut self) -> Self {
        self.direction = Some(OrderDirection::Asc);
        self
    }

    /// Sets the direction of the ordering to descending.
    pub fn desc(mut self) -> Self {
        self.direction = Some(OrderDirection::Desc);
        self
    }

    /// Sets the ordering option to random.
    pub fn rand(mut self) -> Self {
        self.option = Some(OrderOption::Rand);
        self
    }

    /// Sets the ordering option to collate.
    pub fn collate(mut self) -> Self {
        self.option = Some(OrderOption::Collate);
        self
    }

    /// Sets the ordering option to sort the values numerically instead of as strings.
    pub fn numeric(mut self) -> Self {
        self.option = Some(OrderOption::Numeric);
        self
    }
}

impl Display for &Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut query = self.field.build();
        if let Some(option) = &self.option {
            query.push_str(&format!(" {}", option));
        }

        if let Some(direction) = &self.direction {
            query.push_str(&format!(" {}", direction));
        }
        query.fmt(f)
    }
}

trait CanOrder: Into<Field> + Clone {
    fn asc(&self) -> Order {
        let field: Field = self.clone().into();
        Order {
            field,
            direction: Some(OrderDirection::Asc),
            option: None,
        }
    }

    fn desc(&self) -> Order {
        let field: Field = self.clone().into();
        Order {
            field,
            direction: Some(OrderDirection::Desc),
            option: None,
        }
    }

    fn rand(&self) -> Order {
        let field: Field = self.clone().into();
        Order {
            field,
            direction: None,
            option: Some(OrderOption::Rand),
        }
    }

    fn collate(&self) -> Order {
        let field: Field = self.clone().into();
        Order {
            field,
            direction: None,
            option: Some(OrderOption::Collate),
        }
    }

    fn numeric(&self) -> Order {
        let field: Field = self.clone().into();
        Order {
            field,
            direction: None,
            option: Some(OrderOption::Numeric),
        }
    }
}

impl<T> CanOrder for T where T: Into<Field> + Clone {}

#[derive(Debug, Clone, Copy)]
enum OrderDirection {
    Asc,
    Desc,
}

impl Display for OrderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderDirection::Asc => write!(f, "ASC"),
            OrderDirection::Desc => write!(f, "DESC"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OrderOption {
    Rand,
    Collate,
    Numeric,
}
impl Display for OrderOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderOption::Rand => write!(f, "RAND()"),
            OrderOption::Collate => write!(f, "COLLATE"),
            OrderOption::Numeric => write!(f, "NUMERIC"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TargettablesForSelect {
    Table(sql::Table),
    Tables(Vec<sql::Table>),
    SurrealId(sql::Thing),
    SurrealIds(Vec<sql::Thing>),
    // Should already be bound
    Subquery(Subquery),
    Function(Function),
    RecordRange(Valuex),
}

impl From<Vec<sql::Thing>> for TargettablesForSelect {
    fn from(value: Vec<sql::Thing>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

impl From<&sql::Thing> for TargettablesForSelect {
    fn from(value: &sql::Thing) -> Self {
        Self::SurrealId(value.to_owned().into())
    }
}

impl From<sql::Thing> for TargettablesForSelect {
    fn from(value: sql::Thing) -> Self {
        Self::SurrealId(value.into())
    }
}
impl From<Vec<sql::Table>> for TargettablesForSelect {
    fn from(value: Vec<sql::Table>) -> Self {
        Self::Tables(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

// from surrealid to TargettablesForSelect
impl<T, Id> From<&SurrealId<T, Id>> for TargettablesForSelect
where
    T: SurrealdbModel,
    Id: Into<sql::Id>,
{
    fn from(value: &SurrealId<T, Id>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<const N: usize, T, Id> From<&[SurrealId<T, Id>; N]> for TargettablesForSelect
where
    T: SurrealdbModel,
    Id: Into<sql::Id>,
{
    fn from(value: &[SurrealId<T, Id>; N]) -> Self {
        Self::SurrealIds(value.into_iter().map(|v| v.into()).collect())
    }
}

impl<T, Id> From<Vec<&SurrealId<T, Id>>> for TargettablesForSelect
where
    T: SurrealdbModel,
    Id: Into<sql::Id>,
{
    fn from(value: Vec<&SurrealId<T, Id>>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<const N: usize, T, Id> From<&[&SurrealId<T, Id>; N]> for TargettablesForSelect
where
    T: SurrealdbModel,
    Id: Into<sql::Id>,
{
    fn from(value: &[&SurrealId<T, Id>; N]) -> Self {
        Self::SurrealIds(value.into_iter().map(|&t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<&[sql::Thing; N]> for TargettablesForSelect {
    fn from(value: &[sql::Thing; N]) -> Self {
        Self::SurrealIds(
            value
                .into_iter()
                .map(|t| t.to_owned().into())
                .collect::<Vec<_>>(),
        )
    }
}

impl<T, Id> From<Vec<SurrealId<T, Id>>> for TargettablesForSelect
where
    T: SurrealdbModel,
    Id: Into<sql::Id>,
{
    fn from(value: Vec<SurrealId<T, Id>>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

impl<T, Id> From<SurrealId<T, Id>> for TargettablesForSelect
where
    T: SurrealdbModel,
    Id: Into<sql::Id>,
{
    fn from(value: SurrealId<T, Id>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T: SurrealdbModel> From<SurrealSimpleId<T>> for TargettablesForSelect {
    fn from(value: SurrealSimpleId<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T: SurrealdbModel> From<&SurrealSimpleId<T>> for TargettablesForSelect {
    fn from(value: &SurrealSimpleId<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T: SurrealdbModel> From<Vec<SurrealSimpleId<T>>> for TargettablesForSelect {
    fn from(value: Vec<SurrealSimpleId<T>>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<Vec<&SurrealSimpleId<T>>> for TargettablesForSelect {
    fn from(value: Vec<&SurrealSimpleId<T>>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<&[SurrealSimpleId<T>]> for TargettablesForSelect {
    fn from(value: &[SurrealSimpleId<T>]) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<&[&SurrealSimpleId<T>]> for TargettablesForSelect {
    fn from(value: &[&SurrealSimpleId<T>]) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<&SurrealUuid<T>> for TargettablesForSelect {
    fn from(value: &SurrealUuid<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T: SurrealdbModel> From<SurrealUuid<T>> for TargettablesForSelect {
    fn from(value: SurrealUuid<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T: SurrealdbModel> From<Vec<SurrealUuid<T>>> for TargettablesForSelect {
    fn from(value: Vec<SurrealUuid<T>>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<Vec<&SurrealUuid<T>>> for TargettablesForSelect {
    fn from(value: Vec<&SurrealUuid<T>>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<&[SurrealUuid<T>]> for TargettablesForSelect {
    fn from(value: &[SurrealUuid<T>]) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<&[&SurrealUuid<T>]> for TargettablesForSelect {
    fn from(value: &[&SurrealUuid<T>]) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

// from surrealUlid
impl<T: SurrealdbModel> From<&SurrealUlid<T>> for TargettablesForSelect {
    fn from(value: &SurrealUlid<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T: SurrealdbModel> From<SurrealUlid<T>> for TargettablesForSelect {
    fn from(value: SurrealUlid<T>) -> Self {
        Self::SurrealId(value.to_thing())
    }
}

impl<T: SurrealdbModel> From<Vec<SurrealUlid<T>>> for TargettablesForSelect {
    fn from(value: Vec<SurrealUlid<T>>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<Vec<&SurrealUlid<T>>> for TargettablesForSelect {
    fn from(value: Vec<&SurrealUlid<T>>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<&[SurrealUlid<T>]> for TargettablesForSelect {
    fn from(value: &[SurrealUlid<T>]) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl<T: SurrealdbModel> From<&[&SurrealUlid<T>]> for TargettablesForSelect {
    fn from(value: &[&SurrealUlid<T>]) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_thing()).collect::<Vec<_>>())
    }
}

impl From<&Table> for TargettablesForSelect {
    fn from(value: &Table) -> Self {
        Self::Table(value.into())
    }
}
impl From<Table> for TargettablesForSelect {
    fn from(value: Table) -> Self {
        Self::Table(value.into())
    }
}

impl From<Vec<&Table>> for TargettablesForSelect {
    fn from(value: Vec<&Table>) -> Self {
        Self::Tables(
            value
                .into_iter()
                .map(|t| t.clone().into())
                .collect::<Vec<_>>(),
        )
    }
}

impl From<&sql::Table> for TargettablesForSelect {
    fn from(value: &sql::Table) -> Self {
        Self::Table(value.to_owned())
    }
}
impl From<Vec<&sql::Table>> for TargettablesForSelect {
    fn from(value: Vec<&sql::Table>) -> Self {
        Self::Tables(value.into_iter().map(|t| t.to_owned()).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<&[&sql::Table; N]> for TargettablesForSelect {
    fn from(value: &[&sql::Table; N]) -> Self {
        Self::Tables(value.into_iter().map(|&t| t.to_owned()).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<&[&Table; N]> for TargettablesForSelect {
    fn from(value: &[&Table; N]) -> Self {
        Self::Tables(value.to_vec().into_iter().map(|v| v.into()).collect())
    }
}

impl<const N: usize> From<&[sql::Table; N]> for TargettablesForSelect {
    fn from(value: &[sql::Table; N]) -> Self {
        Self::Tables(value.to_vec())
    }
}

impl From<sql::Table> for TargettablesForSelect {
    fn from(value: sql::Table) -> Self {
        Self::Table(value)
    }
}

impl<T: Into<Subquery>> From<T> for TargettablesForSelect {
    fn from(value: T) -> Self {
        Self::Subquery(value.into())
    }
}

impl From<Function> for TargettablesForSelect {
    fn from(value: Function) -> Self {
        Self::Function(value.clone())
    }
}

/// Single field or multiple fields to split by
#[derive(Clone, Debug)]
pub enum Splittables {
    /// single field to split by
    Field(Field),
    /// Multiple fields to split by
    Fields(Vec<Field>),
}

impl<'a, const N: usize> From<&[&Field; N]> for Splittables {
    fn from(value: &[&Field; N]) -> Self {
        Self::Fields(value.map(Into::into).to_vec())
    }
}

impl<'a, const N: usize> From<&[Field; N]> for Splittables {
    fn from(value: &[Field; N]) -> Self {
        Self::Fields(value.to_vec())
    }
}

impl From<Vec<Field>> for Splittables {
    fn from(value: Vec<Field>) -> Self {
        Self::Fields(value)
    }
}

impl From<Vec<&Field>> for Splittables {
    fn from(value: Vec<&Field>) -> Self {
        Self::Fields(value.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<Vec<Valuex>> for Splittables {
    fn from(value: Vec<Valuex>) -> Self {
        Self::Fields(
            value
                .into_iter()
                .map(|v| Field::new(v.build()).with_bindings(v.get_bindings()))
                .collect::<Vec<_>>(),
        )
    }
}

type Groupables = Splittables;
pub(crate) type Fetchables = Groupables;

impl<T: Into<Field>> From<T> for Splittables {
    fn from(value: T) -> Self {
        let value: Field = value.into();
        Self::Field(value)
    }
}
/// Items that can be selected

pub struct Selectables(Valuex);

impl<T: Into<Field>> From<T> for Selectables {
    fn from(value: T) -> Self {
        let value: Field = value.into();
        Self(Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

impl From<Vec<Valuex>> for Selectables {
    fn from(value: Vec<Valuex>) -> Self {
        Self(Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

impl From<&All> for Selectables {
    fn from(_value: &All) -> Self {
        Self(Valuex {
            string: "*".into(),
            bindings: vec![],
            errors: vec![],
        })
    }
}

impl From<All> for Selectables {
    fn from(_value: All) -> Self {
        Self(Valuex {
            string: "*".into(),
            bindings: vec![],
            errors: vec![],
        })
    }
}

impl<'a, const N: usize> From<&[Field; N]> for Selectables {
    fn from(value: &[Field; N]) -> Self {
        Self(Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

impl<'a, const N: usize> From<&[&Field; N]> for Selectables {
    fn from(value: &[&Field; N]) -> Self {
        Self(Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

impl From<Vec<&Field>> for Selectables {
    fn from(value: Vec<&Field>) -> Self {
        Self(Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

impl From<Vec<Field>> for Selectables {
    fn from(value: Vec<Field>) -> Self {
        Self(Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

// impl From<Field> for Selectables {
//     fn from(value: Field) -> Self {
//         Self(Valuex {
//             string: value.build(),
//             bindings: value.get_bindings(),
//         })
//     }
// }
//
// impl From<&Field> for Selectables {
//     fn from(value: &Field) -> Self {
//         Self(Valuex {
//             string: value.build(),
//             bindings: value.get_bindings(),
//         })
//     }
// }

impl From<Function> for Selectables {
    fn from(value: Function) -> Self {
        Self(Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

impl From<&Function> for Selectables {
    fn from(value: &Function) -> Self {
        Self(Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        })
    }
}

impl Deref for Selectables {
    type Target = Valuex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
enum SelectionType {
    Select,
    SelectValue,
}

/// The query builder struct used to construct complex database queries.
#[derive(Debug, Clone)]
pub struct SelectStatement {
    selection_type: SelectionType,
    projections: String,
    targets: Vec<String>,
    where_: Option<String>,
    split: Vec<String>,
    group_by: Vec<String>,
    order_by: Vec<Order>,
    limit: Option<String>,
    start: Option<String>,
    fetch: Vec<String>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
    errors: ErrorList,
}

impl Aliasable for SelectStatement {
    fn build_aliasable(&self) -> String {
        format!("({})", self.build().trim_end_matches(";"))
    }
}

impl Queryable for SelectStatement {}

impl ReturnableSelect for SelectStatement {}

impl Conditional for SelectStatement {
    fn get_condition_query_string(&self) -> String {
        format!("{}", self)
    }
}

impl Erroneous for SelectStatement {
    fn get_errors(&self) -> Vec<String> {
        self.errors.to_vec()
    }
}

impl Parametric for SelectStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

/// Creates a SELECT statement.
///
/// The SELECT statement can be used for selecting and querying data in a database.
/// Each SELECT statement supports selecting from multiple targets, which can include
/// tables, records, edges, subqueries, paramaters, s, objects, and other values.
///
/// Examples
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, statements::{order, select}, functions::{math}};
/// # let name = Field::new("name");
/// # let age = Field::new("age");
/// # let country = Field::new("country");
/// # let city = Field::new("city");
/// # let fake_id = TestUser::create_id("oyelowo");
/// # let fake_id2 = TestUser::create_id("oyedayo");
///
///  select(All)
///     .from(fake_id)
///     .where_(cond(city.is("Prince Edward Island"))
///                 .and(city.is("NewFoundland"))
///                 .or(city.like("Toronto"))
///     )
///     .order_by(order(&age).numeric())
///     .limit(153)
///     .start(10)
///     .parallel();
///
///  select(All)
///     .from(fake_id2)
///     .where_(country.is("INDONESIA"))
///     .order_by(order(&age).numeric())
///     .limit(20)
///     .start(5);
///
///  // Selecting heterogenous types e.g field, alias and all
///  # let user = Table::new("user");
///  # let country = Field::new("country");
///  # let gender = Field::new("gender");
///  # let total = AliasName::new("total");
///  select(arr![count!().__as__(total), math::sum!(age), &gender, &country])
///     .from(user)
///     .group_by(&[gender, country]);
///  
///  // Select reference of reference
///  # let user = Table::new("user");
///  # let country = Field::new("country");
///  # let age = Field::new("age");
///  # let gender = Field::new("gender");
///  # let city = Field::new("city");
///  select(&[&gender, &country, &city])
///     .from(user)
///     .group_by(&[gender, country, city]);
///  
///  // Select reference of owned types
///  # let user = Table::new("user");
///  # let country = Field::new("country");
///  # let gender = Field::new("gender");
///  # let city = Field::new("city");
///  select(&[gender, country, city])
///     .from(user);
///     
///  // Select vector of homogenous fields
///  # let user = Table::new("user");
///  # let country = Field::new("country");
///  # let gender = Field::new("gender");
///  select(vec![gender, country])
///     .from(user);
/// ```
pub fn select(selectables: impl Into<Selectables>) -> SelectStatement {
    let selectables: Selectables = selectables.into();

    SelectStatement {
        selection_type: SelectionType::Select,
        projections: selectables.build(),
        targets: vec![],
        where_: None,
        split: vec![],
        group_by: vec![],
        order_by: vec![],
        limit: None,
        start: None,
        fetch: vec![],
        timeout: None,
        parallel: false,
        bindings: selectables.get_bindings(),
        errors: selectables.get_errors(),
    }
}

/// Just like normal select statement but useful for selecting a single value out of the returned object.
pub fn select_value(selectable_value: impl Into<Field>) -> SelectStatement {
    let selectables: Field = selectable_value.into();

    SelectStatement {
        selection_type: SelectionType::SelectValue,
        projections: selectables.build(),
        targets: vec![],
        where_: None,
        split: vec![],
        group_by: vec![],
        order_by: vec![],
        limit: None,
        start: None,
        fetch: vec![],
        timeout: None,
        parallel: false,
        bindings: selectables.get_bindings(),
        errors: selectables.get_errors(),
    }
}

impl SelectStatement {
    /// Specifies the table to select from.
    ///
    /// # Arguments
    ///
    /// * `targettables` - which can include tables, records, edges, subqueries, paramaters, s, objects, and other values.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// # use surrealdb_orm::{*, statements::{order, select}};
    /// # let user = Table::new("user");
    /// # let alien = Table::new("alien");
    /// # let user_id1 = TestUser::create_id("oyelowo");
    ///  //  Can select from a table name
    ///  select(All).from(user);
    ///
    ///  //  Can also select from an id
    ///  select(All).from(user_id1);
    ///
    ///  //  or select fromj a subquery
    ///  select(All).from(select(All).from(alien));
    ///
    /// ```
    /// ```rust, ignore
    ///  // or a list of tables, ids or subqueries
    ///  select(All).from(![user, user_id, select(All).from(alien)]);
    /// ```
    pub fn from(mut self, targettables: impl Into<TargettablesForSelect>) -> Self {
        let targets: TargettablesForSelect = targettables.into();
        let mut targets_bindings = vec![];

        let target_names = match targets {
            TargettablesForSelect::Table(table) => {
                vec![table.to_string()]
            }
            TargettablesForSelect::Tables(tables) => tables
                .into_iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>(),
            // Should already be bound
            TargettablesForSelect::Subquery(query) => {
                targets_bindings.extend(query.get_bindings());
                vec![query.build()]
            }
            TargettablesForSelect::SurrealId(id) => {
                let binding = Binding::new(id.to_owned());
                let param = binding.get_param_dollarised();
                targets_bindings.push(binding);
                vec![param]
            }

            TargettablesForSelect::SurrealIds(ids) => {
                let mut params = vec![];

                ids.into_iter().for_each(|id| {
                    let binding = Binding::new(id.to_owned());
                    let param = binding.get_param_dollarised();
                    targets_bindings.push(binding);
                    params.push(param);
                });
                params
            }
            TargettablesForSelect::Function(function) => {
                targets_bindings.extend(function.get_bindings());
                vec![function.build()]
            }
            TargettablesForSelect::RecordRange(r) => {
                targets_bindings.extend(r.get_bindings());
                vec![r.build()]
            }
        };

        self.update_bindings(targets_bindings);
        self.targets.extend(target_names);
        self
    }

    /// Adds a condition to the `` clause of the SQL query.
    ///
    /// # Arguments
    ///
    /// * `condition` - A reference to a filter condition.
    ///
    /// # Example
    ///
    /// Examples
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// # use surrealdb_orm::{*, statements::{order, select}};
    /// # let name = Field::new("name");
    /// # let age = Field::new("age");
    /// # let country = Field::new("country");
    /// # let city = Field::new("city");
    /// # let fake_id = TestUser::create_id("oyelowo");
    /// # let fake_id2 = TestUser::create_id("oyedayo");
    /// // Supports simpler where clause without `cond` helper function
    /// # let select2 = select(All)
    /// #   .from(fake_id2)
    ///     .where_(country.is("INDONESIA"));
    ///
    /// // Supports more complex where clause using `cond` helper function
    /// # let select1 = select(All)
    ///     .where_(cond(city.is("Prince Edward Island"))
    ///                 .and(city.is("NewFoundland"))
    ///                 .or(city.like("Toronto"))
    ///     );
    pub fn where_(mut self, condition: impl Conditional + Clone) -> Self {
        self.update_bindings(condition.get_bindings());
        let condition = Filter::new(condition);
        self.where_ = Some(condition.build());
        self
    }

    fn update_bindings(&mut self, bindings: BindingsList) -> &mut Self {
        // let mut updated_params = vec![];
        // updated_params.extend(self.________params_accumulator.to_vec());
        // updated_params.extend(parametric_value.get_bindings());
        self.bindings.extend(bindings);
        self
    }

    /// Adds a field or multiple fields to the `SPLIT BY` clause of the query.
    /// As SurrealDB supports s and nested fields within arrays,
    /// it is possible to split the result on a specific field name,
    /// returning each value in an  as a separate value, along with the record content itself.
    /// This is useful in data analysis contexts.
    ///
    /// # Arguments
    ///
    /// * `splittables` - The name of the field or  or vector of fields to add to the `SPLIT BY` clause.
    ///
    /// # Examples: For multiple fields
    ///
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// # use surrealdb_orm::{*, statements::{order, select}};
    /// # let user = Table::new("user");
    /// # let country = Table::new("country");
    /// # let emails = Field::new("emails");
    /// # let cities = Field::new("cities");
    // // Split the results by each value in an
    ///  select(All)
    ///     .from(user)
    ///     .split(emails);
    ///
    /// ```
    /// ```rust, ignore
    /// // Split the results by each value in a nested
    ///  let locations = Country::schema();
    ///  select(All)
    ///     .from(country)
    ///     .split(locations.cities);
    /// ```
    pub fn split(mut self, splittables: impl Into<Splittables>) -> Self {
        let fields: Splittables = splittables.into();

        let fields = match fields {
            Splittables::Field(one_field) => vec![one_field],
            Splittables::Fields(many_fields) => many_fields,
        };

        fields.iter().for_each(|f| {
            self.split.push(f.to_string());
        });
        self
    }

    /// Sets the GROUP BY clause for the query.
    /// SurrealDB supports data aggregation and grouping, with support for multiple fields, nested fields, and aggregate functions.
    /// In SurrealDB, every field which appears in the field projections of the select statement
    /// (and which is not an aggregate function), must also be present in the GROUP BY clause.
    ///
    /// # Arguments
    ///
    /// * `field(s)` - The name or names of the field to group by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// # use surrealdb_orm::{*, statements::{order, select}, functions::{count, math}};
    /// # let user = Table::new("user");
    /// # let country = Field::new("country");
    ///  
    ///  // Group records by a single field
    ///  select(All)
    ///     .from(user)
    ///     .group_by(country);
    ///  
    /// // Group results by list borrowed list of referenced fields
    /// # let user = Table::new("user");
    /// # let country = Field::new("country");
    /// # let age = Field::new("age");
    /// # let gender = Field::new("gender");
    /// # let city = Field::new("city");
    ///  select(All)
    ///     .from(user)
    ///     .group_by(&[&gender, &country, &city]);
    ///  
    /// // Group results by list borrowed list of owned fields
    /// # let user = Table::new("user");
    ///  select(All)
    ///     .from(user)
    ///     .group_by(&[gender, country, city]);
    ///     
    /// # let user = Table::new("user");
    /// # let country = Field::new("country");
    /// # let gender = Field::new("gender");
    /// # let total = AliasName::new("total");
    /// // Group results with aggregate functions
    ///  select(All)
    ///     .from(user)
    ///     .group_by(vec![gender, country]);
    ///
    /// // Group results by chaining
    /// # let user = Table::new("user");
    /// # let country = Field::new("country");
    /// # let gender = Field::new("gender");
    /// select(All)
    ///     .from(user)
    ///     .group_by(gender)
    ///     .group_by(country);
    /// ```
    /// ```rust, ignore
    /// // Group results by a nested field
    /// let settings = Article::schem();
    /// select(settings.published)
    ///     .from(article)
    ///     .group_by(settings.published);
    /// ```
    pub fn group_by(mut self, groupables: impl Into<Groupables>) -> Self {
        let fields: Groupables = groupables.into();

        let fields = match fields {
            Groupables::Field(one_field) => vec![one_field],
            Groupables::Fields(many_fields) => many_fields,
        };

        fields.iter().for_each(|f| {
            self.group_by.push(f.to_string());
        });
        self
    }

    /// Sets the ORDER BY clause for the query. Multiple values can also be set within same call.
    /// Repeated calls are accumulated
    ///
    /// To sort records, SurrealDB allows ordering on multiple fields and nested fields.
    /// Use the ORDER BY clause to specify a comma-separated list of field names which
    /// should be used to order the resulting records. The ASC and DESC keywords can be
    /// used to specify whether results should be sorted in an ascending or descending manner.
    /// The COLLATE keyword can be used to use unicode collation when ordering text in string values,
    /// ensuring that different cases, and different languages are sorted in a consistent manner.
    /// Finally the NUMERIC can be used to correctly sort text which contains numeric values.
    ///
    /// # Arguments
    ///
    /// * `orderables` - The field(s) and direction to order by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// # use surrealdb_orm::{*, statements::{order, select}};
    /// # let user = Table::new("user");
    /// # let age = Field::new("age");
    /// # let country = Field::new("country");
    /// # let city = Field::new("city");
    /// # let state = Field::new("state");
    /// // Order by single field
    /// select(All)
    ///     .from(user)
    ///     .order_by(order(age).numeric().desc());
    ///
    /// # let user = Table::new("user");
    /// # let age = Field::new("age");
    /// // Order by multiple fields by using a list. Vector and `!` helper also work
    /// select(All)
    ///     .from(user)
    ///     .order_by(&[order(age).numeric().desc(), order(city).rand().asc()]);
    ///     
    /// # let user = Table::new("user");
    /// # let age = Field::new("age");
    /// // Order by multiple fields by chainging to accumulate
    /// select(All)
    ///     .from(user)
    ///     .order_by(order(age).numeric().desc())
    ///     .order_by(order(state).collate().asc());
    /// ```
    pub fn order_by(mut self, orderables: impl Into<Orderables>) -> Self {
        let orderables: Orderables = orderables.into();
        self.update_bindings(orderables.get_bindings());

        let orders: Vec<Order> = orderables.into();
        self.order_by.extend(orders);
        self
    }

    /// Sets the LIMIT clause for the query.
    /// To limit the number of records returned, use the LIMIT clause.
    ///
    /// When using the LIMIT clause, it is possible to paginate results by using the START clause to start from a specific record from the result set.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of rows to return.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// # use surrealdb_orm::{*, statements::select};
    /// # let user = Table::new("user");
    /// select(All)
    ///     .from(user)
    ///     .limit(100);
    ///
    /// // When using the LIMIT clause, it is possible to paginate results by using the START clause to start from a specific record from the result set.
    /// # let user = Table::new("user");
    /// select(All)
    ///     .from(user)
    ///     .start(50)
    ///     .limit(50);
    /// ```
    pub fn limit(mut self, limit: impl Into<NumberLike>) -> Self {
        let limit: NumberLike = limit.into();
        self.limit = Some(limit.build());
        self.update_bindings(limit.get_bindings());
        self
    }

    /// Adds a start offset to the current query.
    ///
    /// # Arguments
    ///
    /// * `start` - An unsigned 64-bit integer representing the starting offset.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// # use surrealdb_orm::{*, statements::select};
    /// # let user = Table::new("user");
    ///
    /// // When using the LIMIT clause, it is possible to paginate results by using the START clause to start from a specific record from the result set.
    /// select(All)
    ///     .from(user)
    ///     .start(50)
    ///     .limit(50);
    /// ```
    pub fn start(mut self, start: impl Into<NumberLike>) -> Self {
        let start: NumberLike = start.into();
        self.start = Some(start.build());
        self.update_bindings(start.get_bindings());
        self
    }

    /// Adds a field or many fields to the list of fields to fetch in the current query.
    /// You can add as list in a single `fetch` call or chain to accumulate fields to fetch.
    ///
    /// One of the most powerful functions in SurrealDB is the related records and graph connections.
    /// Instead of pulling data from multiple tables and merging that data together,
    /// SurrealDB allows you to traverse related records efficiently without needing to use JOINs.
    /// To fetch and replace records with the remote record data, use the FETCH clause to specify the fields
    /// and nested fields which should be fetched in-place, and returned in the final statement response output.
    /// # Arguments
    ///
    /// * `fetchables` - A reference to a field/fields to be fetched in the query.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surrealdb_query_builder as surrealdb_orm;
    /// # use surrealdb_orm::{*, statements::select};
    /// # let user = Table::new("user");
    /// # let account = Field::new("account");
    /// # let friend = Field::new("friend");
    ///
    /// // Fetch single field
    /// select(All)
    ///     .from(user)
    ///     .fetch(account);
    ///
    /// // Fetch multiple field using a list
    /// # let user = Table::new("user");
    /// # let account = Field::new("account");
    /// select(All)
    ///     .from(user)
    ///     .fetch(&[account, friend]);
    ///
    /// // Fetch multiple field by chaining fetch method calls
    /// # let user = Table::new("user");
    /// # let account = Field::new("account");
    /// # let friend = Field::new("friend");
    /// select(All)
    ///     .from(user)
    ///     .fetch(account)
    ///     .fetch(friend);
    /// ```
    /// ```rust, ignore
    /// let account = Person::schema().account;
    /// select(All)
    ///     .from(user)
    ///     // Fetch nested field
    ///     .fetch(&[account, account.users]);
    /// ```
    pub fn fetch(mut self, fetchables: impl Into<Fetchables>) -> Self {
        let fields: Fetchables = fetchables.into();

        let fields = match fields {
            Fetchables::Field(one_field) => vec![one_field],
            Fetchables::Fields(many_fields) => many_fields,
        };

        fields.iter().for_each(|f| {
            self.fetch.push(f.build());
            self.bindings.extend(f.get_bindings());
            self.errors.extend(f.get_errors());
        });
        self
    }

    /// Sets the timeout duration for the query.
    ///
    /// # Arguments
    ///
    /// * `duration` - a value that can represent a duration for the timeout. This can be one of the following:
    ///
    ///   * `Duration` - a standard Rust `Duration` value.
    ///
    ///   * `Field` - an identifier for a specific field in the query, represented by an `Idiom` value.
    ///
    ///   * `Param` - a named parameter in the query, represented by a `Param` value.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let query = query.timeout(Duration::from_secs(30));
    ///
    /// assert_eq!(query.to_raw().to_string(), "30s");
    /// ```
    pub fn timeout(mut self, duration: impl Into<DurationLike>) -> Self {
        let duration: DurationLike = duration.into();
        self.timeout = Some(duration.to_raw().build());
        self
    }

    /// Indicates that the query should be executed in parallel.
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}
impl Display for SelectStatement {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.build())
    }
}

impl Buildable for SelectStatement {
    fn build(&self) -> String {
        let select = match self.selection_type {
            SelectionType::Select => "SELECT",
            SelectionType::SelectValue => "SELECT VALUE",
        };

        let mut query = format!(
            "{select} {} FROM {}",
            self.projections,
            self.targets.join(", ")
        );

        if let Some(condition) = &self.where_ {
            query = format!("{query} WHERE {condition}");
        }

        if !self.split.is_empty() {
            query = format!("{query} SPLIT {}", &self.split.join(", "));
        }

        if !self.group_by.is_empty() {
            query = format!("{query} GROUP BY {}", &self.group_by.join(", "));
        }

        if !self.order_by.is_empty() {
            query = format!(
                "{query} ORDER BY {}",
                &self
                    .order_by
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }

        if let Some(limit_value) = &self.limit {
            query = format!("{query} LIMIT {}", limit_value);
        }

        if let Some(start_value) = &self.start {
            query = format!("{query} START AT {}", start_value);
        }

        if !self.fetch.is_empty() {
            query = format!("{query} FETCH {}", &self.fetch.join(", "));
        }

        if let Some(timeout_value) = &self.timeout {
            query = format!("{query} TIMEOUT {}", timeout_value);
        }

        if self.parallel {
            query = format!("{query} PARALLEL");
        }

        format!("{query};")
    }
}

/// A mini version of the select statement used as Model convenience method for building select statements.
#[derive(Debug, Clone)]
pub struct SelectStatementMini<T: SurrealdbModel>(SelectStatement, PhantomData<T>);

impl<T: SurrealdbModel> SelectStatementMini<T> {
    /// Order the results by the given fields
    pub fn order_by(mut self, orderables: impl Into<Orderables>) -> Self {
        let orderables: Orderables = orderables.into();
        self.0.update_bindings(orderables.get_bindings());

        let orders: Vec<Order> = orderables.into();
        self.0.order_by.extend(orders);
        self
    }

    /// Starts the result at the offset
    pub fn start(mut self, start: impl Into<NumberLike>) -> Self {
        let start: NumberLike = start.into();
        self.0.start = Some(start.build());
        self.0.update_bindings(start.get_bindings());
        self
    }

    /// Limits the number of results returned
    pub fn limit(mut self, limit: impl Into<NumberLike>) -> Self {
        let limit: NumberLike = limit.into();
        self.0.limit = Some(limit.build());
        self.0.update_bindings(limit.get_bindings());
        self
    }

    /// Parallelizes the query
    pub fn parallel(mut self) -> Self {
        self.0.parallel = true;
        self
    }
}

impl<T> From<SelectStatement> for SelectStatementMini<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn from(value: SelectStatement) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Erroneous for SelectStatementMini<T> where T: Serialize + DeserializeOwned + SurrealdbModel {}

impl<T> Parametric for SelectStatementMini<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn get_bindings(&self) -> crate::BindingsList {
        self.0.get_bindings()
    }
}
impl<T> Buildable for SelectStatementMini<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn build(&self) -> String {
        self.0.build()
    }
}

impl<T> Queryable for SelectStatementMini<T> where T: Serialize + DeserializeOwned + SurrealdbModel {}

impl<T> ReturnableStandard<T> for SelectStatementMini<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel + Send + Sync,
{
    fn set_return_type(mut self, return_type: crate::ReturnType) -> Self {
        if let crate::ReturnType::Projections(projection) = return_type {
            self.0.projections = format!("{}, {}", self.0.projections, projection.build());
        }
        self
    }

    fn get_return_type(&self) -> crate::ReturnType {
        crate::ReturnType::After
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_statement_with_alias() {
        let canadian_cities = AliasName::new("legal_age");
        let age = Field::new("age");
        let city = Field::new("city");
        let fake_id = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
        let statement = select(All)
            .from(fake_id)
            .where_(
                cond(city.is("Prince Edward Island"))
                    .and(city.is("NewFoundland"))
                    .or(city.like("Toronto")),
            )
            // You can call directly on field
            .order_by(age.desc().numeric())
            // Or use order function
            .order_by(order(&city).asc())
            .limit(153)
            .start(10)
            .parallel();

        let statement_aliased = statement.__as__(canadian_cities);

        assert_eq!(
            statement_aliased.fine_tune_params(),
            "(SELECT * FROM $_param_00000001 WHERE (city IS $_param_00000002) \
                AND (city IS $_param_00000003) \
                OR (city ~ $_param_00000004) \
                ORDER BY age NUMERIC DESC, city ASC LIMIT \
                $_param_00000005 START AT $_param_00000006 PARALLEL) AS legal_age"
        );
        assert_eq!(
            statement_aliased.to_raw().to_string(),
            "(SELECT * FROM user:oyelowo WHERE (city IS 'Prince Edward Island') \
                    AND (city IS 'NewFoundland') OR (city ~ 'Toronto') \
                    ORDER BY age NUMERIC DESC, city ASC LIMIT 153 START AT 10 PARALLEL) AS legal_age"
        );
    }
}
