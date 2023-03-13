/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    ops::Deref,
};

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql::{self, Table, Value};

use crate::{
    db_field::{Binding, BindingsList, DbFilter, Parametric},
    query_insert::Buildable,
    value_type_wrappers::SurrealId,
    DbField, Queryable, SurrealdbModel, SurrealdbNode,
};

/// Creates a new `Order` instance with the specified database field.
///
/// # Arguments
///
/// * `field` - A reference to a `DbField` instance to be used as the ordering field.
///
/// # Example
///
/// ```
/// use my_crate::{Order, DbField};
///
/// let id_field = DbField::new("id");
/// let order = Order::new(&id_field);
/// ```
pub fn order(field: impl Into<DbField>) -> Order {
    let field: DbField = field.into();
    Order::new(&field)
}

/// Represents an ordering field, direction, and options for a database query.
#[derive(Debug, Clone)]
pub struct Order {
    field: DbField,
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

pub enum Orderables {
    Order(Order),
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
    /// Creates a new `Order` instance with the specified database field.
    ///
    /// # Arguments
    ///
    /// * `field` - A reference to a `DbField` instance to be used as the ordering field.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField};
    ///
    /// let id_field = DbField::new("id");
    /// let order = Order::new(&id_field);
    /// ```
    pub fn new(field: &DbField) -> Self {
        Order {
            field: field.clone(),
            direction: None,
            option: None,
        }
    }

    /// Sets the direction of the ordering to ascending.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField, OrderDirection};
    ///
    /// let id_field = DbField::new("id");
    /// let order = Order::new(&id_field).asc();
    /// assert_eq!(order.direction, Some(OrderDirection::Asc));
    /// ```
    pub fn asc(mut self) -> Self {
        self.direction = Some(OrderDirection::Asc);
        self
    }

    /// Sets the direction of the ordering to descending.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField, OrderDirection};
    ///
    /// let id_field = DbField::new("id");
    /// let order = Order::new(&id_field).desc();
    /// assert_eq!(order.direction, Some(OrderDirection::Desc));
    /// ```
    pub fn desc(mut self) -> Self {
        self.direction = Some(OrderDirection::Desc);
        self
    }

    /// Sets the ordering option to random.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField, OrderOption};
    ///
    /// let id_field = DbField::new("id");
    /// let order = Order::new(&id_field).rand();
    /// assert_eq!(order.option, Some(OrderOption::Rand));
    /// ```
    pub fn rand(mut self) -> Self {
        self.option = Some(OrderOption::Rand);
        self
    }

    /// Sets the ordering option to collate.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, DbField, OrderOption};
    ///
    /// let name_field = DbField::new("name");
    /// let order = Order::new(&name_field).collate();
    /// assert_eq!(order.option, Some(OrderOption::Collate));
    /// ```
    pub fn collate(mut self) -> Self {
        self.option = Some(OrderOption::Collate);
        self
    }

    /// Sets the ordering option to sort the values numerically instead of as strings.
    ///
    /// # Example
    ///
    /// ```
    /// use my_cool_database::query::{Order, DbField};
    ///
    /// let field = DbField::new("age", "users");
    /// let order = Order::new(&field).numeric();
    ///
    /// assert_eq!(order.field.name(), "age");
    /// assert_eq!(order.option.unwrap(), OrderOption::Numeric);
    /// ```
    pub fn numeric(mut self) -> Self {
        self.option = Some(OrderOption::Numeric);
        self
    }
}

impl Display for &Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            self.field,
            self.option.map_or("".into(), |op| op.to_string()),
            self.direction.unwrap_or(OrderDirection::Asc)
        ))
    }
}

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
pub enum Targettables {
    Table(sql::Table),
    Tables(Vec<sql::Table>),
    SurrealId(SurrealId),
    SurrealIds(Vec<SurrealId>),
    // Should already be bound
    SubQuery(SelectStatement),
}

impl From<Vec<sql::Table>> for Targettables {
    fn from(value: Vec<sql::Table>) -> Self {
        Self::Tables(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}
// impl<'a> From<sql::Tables> for Targettables<'a> {
//     fn from(value: sql::Tables) -> Self {
//         Self::Tables(value)
//     }
// }

impl From<Vec<sql::Thing>> for Targettables {
    fn from(value: Vec<sql::Thing>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

impl From<&sql::Table> for Targettables {
    fn from(value: &sql::Table) -> Self {
        Self::Table(value.to_owned())
    }
}
impl From<&sql::Thing> for Targettables {
    fn from(value: &sql::Thing) -> Self {
        Self::SurrealId(value.to_owned().into())
    }
}

impl From<sql::Thing> for Targettables {
    fn from(value: sql::Thing) -> Self {
        Self::SurrealId(value.into())
    }
}

impl From<Vec<&sql::Table>> for Targettables {
    fn from(value: Vec<&sql::Table>) -> Self {
        Self::Tables(value.into_iter().map(|t| t.to_owned()).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<&[&sql::Table; N]> for Targettables {
    fn from(value: &[&sql::Table; N]) -> Self {
        Self::Tables(value.into_iter().map(|&t| t.to_owned()).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<&[sql::Table; N]> for Targettables {
    fn from(value: &[sql::Table; N]) -> Self {
        Self::Tables(value.to_vec())
    }
}

impl From<&SurrealId> for Targettables {
    fn from(value: &SurrealId) -> Self {
        Self::SurrealId(value.to_owned())
    }
}

impl<const N: usize> From<&[SurrealId; N]> for Targettables {
    fn from(value: &[SurrealId; N]) -> Self {
        Self::SurrealIds(value.to_vec())
    }
}

impl From<Vec<&SurrealId>> for Targettables {
    fn from(value: Vec<&SurrealId>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_owned()).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<&[&SurrealId; N]> for Targettables {
    fn from(value: &[&SurrealId; N]) -> Self {
        Self::SurrealIds(value.into_iter().map(|&t| t.to_owned()).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<&[sql::Thing; N]> for Targettables {
    fn from(value: &[sql::Thing; N]) -> Self {
        Self::SurrealIds(
            value
                .into_iter()
                .map(|t| t.to_owned().into())
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Vec<SurrealId>> for Targettables {
    fn from(value: Vec<SurrealId>) -> Self {
        Self::SurrealIds(value)
    }
}

impl From<SurrealId> for Targettables {
    fn from(value: SurrealId) -> Self {
        Self::SurrealId(value)
    }
}

impl From<sql::Table> for Targettables {
    fn from(value: Table) -> Self {
        Self::Table(value)
    }
}

impl From<&mut SelectStatement> for Targettables {
    fn from(value: &mut SelectStatement) -> Self {
        Self::SubQuery(value.clone())
    }
}

impl From<SelectStatement> for Targettables {
    fn from(value: SelectStatement) -> Self {
        Self::SubQuery(value.clone())
    }
}

impl Parametric for Targettables {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Targettables::Table(table) => {
                // Table binding does not seem to work right now. skip it first
                let binding = Binding::new(table.to_owned());
                vec![binding]
            }
            Targettables::Tables(tables) => {
                // Table binding does not seem to work right now. skip it first
                let bindings = tables
                    .to_vec()
                    .into_iter()
                    .map(Binding::new)
                    .collect::<Vec<_>>();
                bindings
            }
            // Should already be bound
            Targettables::SubQuery(query) => query.get_bindings(),
            Targettables::SurrealId(id) => vec![Binding::new(id.to_owned())],

            Targettables::SurrealIds(ids) => {
                let bindings = ids
                    .into_iter()
                    .map(|id| Binding::new(id.to_owned()))
                    .collect::<Vec<_>>();
                bindings
            }
        }
    }
}

#[derive(Clone)]
pub enum Splittables {
    Field(DbField),
    Fields(Vec<DbField>),
}

impl From<DbField> for Splittables {
    fn from(value: DbField) -> Self {
        Self::Field(value.into())
    }
}

impl From<&DbField> for Splittables {
    fn from(value: &DbField) -> Self {
        Self::Field(value.into())
    }
}

impl<'a, const N: usize> From<&[&DbField; N]> for Splittables {
    fn from(value: &[&DbField; N]) -> Self {
        Self::Fields(value.map(Into::into).to_vec())
    }
}

impl<'a, const N: usize> From<&[DbField; N]> for Splittables {
    fn from(value: &[DbField; N]) -> Self {
        Self::Fields(value.to_vec())
    }
}

impl From<Vec<DbField>> for Splittables {
    fn from(value: Vec<DbField>) -> Self {
        Self::Fields(value)
    }
}

impl From<Vec<&DbField>> for Splittables {
    fn from(value: Vec<&DbField>) -> Self {
        Self::Fields(value.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl Parametric for Splittables {
    fn get_bindings(&self) -> BindingsList {
        // match self {
        // Splittables::Split(s) => vec![Binding::new(s)],
        // Splittables::Splits(splits) => {
        //     let bindings = splits
        //         .into_iter()
        //         .map(|id| Binding::new(id.to_owned()))
        //         .collect::<Vec<_>>();
        //     bindings
        // }
        // }
        vec![]
    }
}
type Groupables = Splittables;
type Fetchables = Groupables;

pub struct Duration(sql::Duration);

impl From<self::Duration> for sql::Duration {
    fn from(value: self::Duration) -> Self {
        value.0
    }
}

impl From<Duration> for sql::Value {
    fn from(value: self::Duration) -> Self {
        value.0.into()
    }
}
impl From<sql::Duration> for self::Duration {
    fn from(value: sql::Duration) -> Self {
        Self(value)
    }
}

impl From<&std::time::Duration> for Duration {
    fn from(value: &std::time::Duration) -> Self {
        Self(value.to_owned().into())
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self(value.into())
    }
}

impl Deref for Duration {
    type Target = sql::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum Selectables {
    All,
    AllWithRelations,
    Field(DbField),
    Fields(Vec<DbField>),
}

#[derive(Debug, Clone, Copy)]
pub struct All;

impl AsRef<Selectables> for All {
    fn as_ref(&self) -> &Selectables {
        todo!()
    }
}
impl AsRef<All> for All {
    fn as_ref(&self) -> &All {
        todo!()
    }
}

impl AsRef<Selectables> for Selectables {
    fn as_ref(&self) -> &Selectables {
        todo!()
    }
}

impl From<&All> for Selectables {
    fn from(_value: &All) -> Self {
        Self::All
    }
}

impl From<All> for Selectables {
    fn from(_value: All) -> Self {
        Self::All
    }
}

impl<'a, const N: usize> From<&[&DbField; N]> for Selectables {
    fn from(value: &[&DbField; N]) -> Self {
        Self::Fields(value.map(Into::into).to_vec())
    }
}

impl From<Vec<&DbField>> for Selectables {
    fn from(value: Vec<&DbField>) -> Self {
        Self::Fields(value.into_iter().map(ToOwned::to_owned).collect())
    }
}
impl From<Vec<DbField>> for Selectables {
    fn from(value: Vec<DbField>) -> Self {
        Self::Fields(value)
    }
}

impl From<DbField> for Selectables {
    fn from(value: DbField) -> Self {
        Self::Field(value)
    }
}

impl From<&DbField> for Selectables {
    fn from(value: &DbField) -> Self {
        Self::Field(value.to_owned())
    }
}

impl Parametric for Selectables {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Selectables::All => vec![],
            Selectables::AllWithRelations => vec![],
            Selectables::Field(f) => f.get_bindings(),
            Selectables::Fields(fields) => {
                fields.into_iter().flat_map(|f| f.get_bindings()).collect()
            }
        }
    }
}

/// The query builder struct used to construct complex database queries.
#[derive(Debug, Clone)]
pub struct SelectStatement {
    projections: Vec<String>,
    targets: Vec<String>,
    where_: Option<String>,
    split: Vec<String>,
    group_by: Vec<String>,
    order_by: Vec<Order>,
    limit: Option<u64>,
    start: Option<u64>,
    fetch: Vec<String>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
}

impl Queryable for SelectStatement {}

impl Parametric for SelectStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl From<Selectables> for SelectStatement {
    fn from(value: Selectables) -> Self {
        value.into()
    }
}

pub fn select(selectables: impl Into<Selectables>) -> SelectStatement {
    let builder = SelectStatement::new();
    let selectables: Selectables = selectables.into();
    builder.select(selectables)
}

impl SelectStatement {
    /// Create a new instance of QueryBuilder.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::QueryBuilder;
    ///
    /// let query_builder = QueryBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            projections: vec![],
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
            bindings: vec![],
        }
    }

    /// Add a wildcard projection to the query.
    ///
    /// # Example
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to select from.
    ///
    /// ```
    /// use surrealdb::QueryBuilder;
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.select(selectables);
    /// ```
    pub fn select(mut self, selectables: impl Into<Selectables>) -> Self {
        let selectables: Selectables = selectables.into();
        let fields = match selectables {
            Selectables::All => vec!["*".to_string()],
            // TODO: include all relations, graph strings automatically. To be generated by the
            // macro system.
            Selectables::AllWithRelations => vec!["*".into()],
            Selectables::Field(f) => vec![format!("${f}")],
            Selectables::Fields(fields) => {
                fields.iter().map(|f| format!("${f}")).collect::<Vec<_>>()
            }
        };
        self.projections.extend(fields);
        self
    }

    /// Specifies the table to select from.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to select from.
    ///
    /// # Example
    ///
    /// ```
    /// use query_builder::{QueryBuilder, DbField};
    ///
    /// let mut builder = QueryBuilder::select();
    /// builder.from("users");
    ///
    /// assert_eq!(builder.to_string(), "SELECT * FROM users");
    /// ```
    pub fn from(mut self, targettables: impl Into<Targettables>) -> Self {
        let targets: Targettables = targettables.into();
        let targets_bindings = targets.get_bindings();

        // When we have either one or many table names or record ids, we want to use placeholders
        // as the targets which would be bound later but for a subquery in from, that must have
        // already been done by the Subquery(in this case, select query) builder itself
        let target_names = match targets {
            Targettables::Table(table) => vec![table.to_string()],
            Targettables::Tables(tbs) => tbs.iter().map(|t| t.to_string()).collect::<Vec<_>>(),
            Targettables::SurrealId(_) | Targettables::SurrealIds(_) => targets_bindings
                .iter()
                .map(|b| format!("${}", b.get_param()))
                .collect::<Vec<_>>(),
            // Subquery must have be built and interpolated, so no need for rebinding
            Targettables::SubQuery(subquery) => vec![format!("({subquery})")],
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
    /// ```
    /// use query_builder::{QueryBuilder, DbField, DbFilter};
    ///
    /// let mut builder = QueryBuilder::select();
    /// let condition = DbFilter::from(("age", ">", 18));
    /// builder._(condition);
    ///
    /// assert_eq!(builder.to_string(), "SELECT *  age > 18");
    /// ```
    pub fn where_(mut self, condition: impl Into<DbFilter> + Parametric + Clone) -> Self {
        self.update_bindings(condition.get_bindings());
        let condition: DbFilter = condition.into();
        self.where_ = Some(condition.to_string());
        self
    }

    fn update_bindings(&mut self, bindings: BindingsList) -> &mut Self {
        // let mut updated_params = vec![];
        // updated_params.extend(self.________params_accumulator.to_vec());
        // updated_params.extend(parametric_value.get_bindings());
        self.bindings.extend(bindings);
        self
    }

    /// Adds a field or multiple fields to the `SPLIT BY` clause of the SQL query.
    ///
    /// # Arguments
    ///
    /// * `splittables` - The name of the field or array or vector of fields to add to the `SPLIT BY` clause.
    ///
    /// # Example: For single field
    ///
    /// ```
    /// use query_builder::{QueryBuilder, DbField};
    ///
    /// let mut builder = QueryBuilder::select();
    /// let country = DbField::new("country");
    /// builder.split(country);
    ///
    /// assert_eq!(builder.to_string(), "SELECT * SPLIT BY country");
    ///
    /// ```
    ///
    /// # Examples: For multiple fields
    ///
    /// ```
    ///
    /// let age = DbField::new("age");
    /// let gender = DbField::new("gender");
    /// query = query.split(&[age, gender]);
    ///
    /// assert_eq!(query.build(), "SELECT *, age, gender FROM table SPLIT age, gender");
    /// ```
    pub fn split(mut self, splittables: impl Into<Splittables>) -> Self {
        let fields: Splittables = splittables.into();
        self.update_bindings(fields.get_bindings());

        let fields = match fields {
            Splittables::Field(one_field) => vec![one_field],
            Splittables::Fields(many_fields) => many_fields,
        };

        // self.split
        //     .extend(fields.iter().map(ToString::to_string).collect::<Vec<_>>());
        fields.iter().for_each(|f| {
            self.split.push(f.to_string());
        });
        self
    }

    /// Sets the GROUP BY clause for the query.
    ///
    /// # Arguments
    ///
    /// * `field(s)` - The name or names of the field to group by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::{QueryBuilder, DbField};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.group_by(DbField::new("age"));
    /// ```
    ///
    ///
    /// # Examples: For multiple fields
    ///
    /// ```
    ///
    /// let age = DbField::new("age");
    /// let gender = DbField::new("gender");
    /// query = query.group_by(&[age, gender]);
    ///
    /// assert_eq!(query.build(), "SELECT *, age, gender FROM table GROUP BY age, gender");
    /// ```
    pub fn group_by(mut self, groupables: impl Into<Groupables>) -> Self {
        let fields: Groupables = groupables.into();
        self.update_bindings(fields.get_bindings());

        let fields = match fields {
            Groupables::Field(one_field) => vec![one_field],
            Groupables::Fields(many_fields) => many_fields,
        };

        // self.split
        //     .extend(fields.iter().map(ToString::to_string).collect::<Vec<_>>());
        fields.iter().for_each(|f| {
            self.group_by.push(f.to_string());
        });
        self
    }

    /// Sets the ORDER BY clause for the query. Multiple values can also be set within same call.
    /// Repeated calls are accumulated
    ///
    /// # Arguments
    ///
    /// * `orderables` - The field and direction to order by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::{QueryBuilder, Order, Direction, DbField};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.order_by(Order::new(DbField::new("age"), Direction::Ascending));
    ///
    /// query_builder.order(&[
    ///     Order::new(DbField::new("age"), Direction::Ascending),
    ///     Order::new(DbField::new("name"), Direction::Descending),
    /// ]);
    /// ```
    pub fn order_by(mut self, orderables: impl Into<Orderables>) -> Self {
        let orderables: Orderables = orderables.into();
        self.update_bindings(orderables.get_bindings());

        let orders: Vec<Order> = orderables.into();
        self.order_by.extend(orders);
        self
    }

    /// Sets the LIMIT clause for the query.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of rows to return.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::QueryBuilder;
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.limit(10);
    /// ```
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
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
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .start(50)
    ///     .fetch("id")
    ///     .fetch("name")
    ///     .from("users")
    ///     .build();
    /// ```
    ///
    /// # Output
    ///
    /// The `start` method returns a mutable reference to the QueryBuilder instance it was called on,
    /// allowing further method chaining.
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .start(50)
    ///     .fetch("id")
    ///     .fetch("name")
    ///     .from("users")
    ///     .build();
    ///
    /// assert_eq!(query, "SELECT id, name FROM users OFFSET 50");
    /// ```
    pub fn start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }

    /// Adds a field or many fields to the list of fields to fetch in the current query.
    ///
    /// # Arguments
    ///
    /// * `fetchables` - A reference to a field/fields to be fetched in the query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb_macros::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch("friend")
    ///     .fetch(&["friend", "book"])
    ///     .from(vec!["fiend", "book"])
    ///     .build();
    ///
    /// assert_eq!(query, "FETCH friend, book");
    /// ```
    pub fn fetch(mut self, fetchables: impl Into<Fetchables>) -> Self {
        let fields: Fetchables = fetchables.into();
        self.update_bindings(fields.get_bindings());

        let fields = match fields {
            Fetchables::Field(one_field) => vec![one_field],
            Fetchables::Fields(many_fields) => many_fields,
        };

        fields.iter().for_each(|f| {
            self.group_by.push(f.to_string());
        });
        self
    }

    /// Sets the timeout duration for the query.
    ///
    /// # Arguments
    ///
    /// * `duration` - a string slice that specifies the timeout duration. It can be expressed in any format that the database driver supports.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_db_client::{Query, QueryBuilder};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.timeout("5s");
    /// ```
    ///
    /// ---
    ///
    /// Indicates that the query should be executed in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_db_client::{Query, QueryBuilder};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.parallel();
    /// ```
    pub fn timeout(mut self, duration: impl Into<Duration>) -> Self {
        let duration: Duration = duration.into();
        let duration = sql::Duration::from(duration);
        self.timeout = Some(duration.to_string());
        self
    }

    /// Indicates that the query should be executed in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_db_client::{Query, QueryBuilder};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.parallel();
    /// ```
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}
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
impl Display for SelectStatement {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.build())
    }
}

// impl  Runnable  for SelectStatement  T: Serialize + DeserializeOwned {}

impl Buildable for SelectStatement {
    fn build(&self) -> String {
        let mut query = String::new();

        query.push_str("SELECT ");
        query.push_str(&self.projections.join(", "));
        query.push_str(" FROM ");
        query.push_str(&self.targets.join(", "));

        if let Some(condition) = &self.where_ {
            query.push_str(" WHERE ");
            query.push_str(&condition);
        }

        if !self.split.is_empty() {
            query.push_str(" SPLIT ");
            query.push_str(&self.split.join(", "));
        }

        if !self.group_by.is_empty() {
            query.push_str(" GROUP BY ");
            query.push_str(&self.group_by.join(", "));
        }

        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            query.push_str(
                &self
                    .order_by
                    .iter()
                    .map(|o| format!("{o}"))
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        }

        if let Some(limit_value) = self.limit {
            query.push_str(" LIMIT ");
            query.push_str(&limit_value.to_string());
        }

        if let Some(start_value) = self.start {
            query.push_str(" START AT ");
            query.push_str(&start_value.to_string());
        }

        if !self.fetch.is_empty() {
            query.push_str(" FETCH ");
            query.push_str(&self.fetch.join(", "));
        }

        if let Some(timeout_value) = &self.timeout {
            query.push_str(" TIMEOUT ");
            query.push_str(&timeout_value.to_string());
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        query.push(';');
        query
    }
}

#[async_trait::async_trait]
pub trait RunnableSelect
where
    Self: Parametric + Buildable,
{
    async fn return_one<T: Serialize + DeserializeOwned>(
        &self,
        db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    ) -> surrealdb::Result<T> {
        let query = self.build();
        println!("XXXX {query}");
        let mut response = self
            .get_bindings()
            .iter()
            .fold(db.query(query), |acc, val| {
                acc.bind((val.get_param(), val.get_value()))
            })
            .await?;

        // If it errors, try to check if multiple entries have been inputed, hence, suurealdb
        // trying to return Vec  rather than Option , then pick the first of the returned
        // Ok .
        let mut returned_val = match response.take::<Option<T>>(0) {
            Ok(one) => vec![one.unwrap()],
            Err(err) => response.take::<Vec<T>>(0)?,
        };

        // TODO:: Handle error if nothing is returned
        let only_or_last = returned_val.pop().unwrap();
        Ok(only_or_last)
    }

    async fn return_many<T: Serialize + DeserializeOwned>(
        &self,
        db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    ) -> surrealdb::Result<Vec<T>> {
        let query = self.build();
        println!("XXXX {query}");
        let mut response = self
            .get_bindings()
            .iter()
            .fold(db.query(query), |acc, val| {
                acc.bind((val.get_param(), val.get_value()))
            })
            .await?;

        println!("mmmmm {response:?}");
        // This does the reverse of get_one
        // If it errors, try to check if only single entry has been inputed, hence, suurealdb
        // trying to return Option , then pick the return the only item as Vec .
        let mut returned_val = match response.take::<Vec<T>>(0) {
            Ok(many) => many,
            Err(err) => vec![response.take::<Option<T>>(0)?.unwrap()],
        };

        // TODO:: Handle error if nothing is returned
        Ok(returned_val)
    }
}

impl RunnableSelect for SelectStatement {}
