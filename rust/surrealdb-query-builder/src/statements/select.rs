/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    borrow::{Borrow, Cow},
    env,
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    ops::Deref,
};

use regex::Replacer;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql::{self, Value};

use crate::{
    cond,
    traits::{Binding, BindingsList, Buildable, Conditional, Erroneous, Parametric, Queryable},
    types::{All, DurationLike, Field, Filter, SurrealId, Table},
    AliasName, Aliasable, Operatable, ReturnableSelect, ToRaw,
};

/// Creates a new `Order` instance with the specified database field.
///
/// # Arguments
///
/// * `field` - A reference to a `Field` instance to be used as the ordering field.
///
/// # Example
///
/// ```
/// use my_crate::{Order, Field};
///
/// let id_field = Field::new("id");
/// let order = Order::new(&id_field);
/// ```
pub fn order(field: impl Into<Field>) -> Order {
    let field: Field = field.into();
    Order::new(&field)
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
    /// * `field` - A reference to a `Field` instance to be used as the ordering field.
    ///
    /// # Example
    ///
    /// ```
    /// use my_crate::{Order, Field};
    ///
    /// let id_field = Field::new("id");
    /// let order = Order::new(&id_field);
    /// ```
    pub fn new(field: &Field) -> Self {
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
    /// use my_crate::{Order, Field, OrderDirection};
    ///
    /// let id_field = Field::new("id");
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
    /// use my_crate::{Order, Field, OrderDirection};
    ///
    /// let id_field = Field::new("id");
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
    /// use my_crate::{Order, Field, OrderOption};
    ///
    /// let id_field = Field::new("id");
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
    /// use my_crate::{Order, Field, OrderOption};
    ///
    /// let name_field = Field::new("name");
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
    /// use my_cool_database::query::{Order, Field};
    ///
    /// let field = Field::new("age", "users");
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
pub enum TargettablesForSelect {
    Table(sql::Table),
    Tables(Vec<sql::Table>),
    SurrealId(SurrealId),
    SurrealIds(Vec<SurrealId>),
    // Should already be bound
    SubQuery(SelectStatement),
}

impl From<Vec<sql::Table>> for TargettablesForSelect {
    fn from(value: Vec<sql::Table>) -> Self {
        Self::Tables(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}
// impl<'a> From<sql::Tables> for Targettables<'a> {
//     fn from(value: sql::Tables) -> Self {
//         Self::Tables(value)
//     }
// }

impl From<Vec<sql::Thing>> for TargettablesForSelect {
    fn from(value: Vec<sql::Thing>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
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

impl From<&SurrealId> for TargettablesForSelect {
    fn from(value: &SurrealId) -> Self {
        Self::SurrealId(value.to_owned())
    }
}

impl<const N: usize> From<&[SurrealId; N]> for TargettablesForSelect {
    fn from(value: &[SurrealId; N]) -> Self {
        Self::SurrealIds(value.to_vec())
    }
}

impl From<Vec<&SurrealId>> for TargettablesForSelect {
    fn from(value: Vec<&SurrealId>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.to_owned()).collect::<Vec<_>>())
    }
}

impl<const N: usize> From<&[&SurrealId; N]> for TargettablesForSelect {
    fn from(value: &[&SurrealId; N]) -> Self {
        Self::SurrealIds(value.into_iter().map(|&t| t.to_owned()).collect::<Vec<_>>())
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

impl From<Vec<SurrealId>> for TargettablesForSelect {
    fn from(value: Vec<SurrealId>) -> Self {
        Self::SurrealIds(value)
    }
}

impl From<SurrealId> for TargettablesForSelect {
    fn from(value: SurrealId) -> Self {
        Self::SurrealId(value)
    }
}

impl From<sql::Table> for TargettablesForSelect {
    fn from(value: sql::Table) -> Self {
        Self::Table(value)
    }
}

impl From<&mut SelectStatement> for TargettablesForSelect {
    fn from(value: &mut SelectStatement) -> Self {
        Self::SubQuery(value.clone())
    }
}

impl From<SelectStatement> for TargettablesForSelect {
    fn from(value: SelectStatement) -> Self {
        Self::SubQuery(value.clone())
    }
}

#[derive(Clone)]
pub enum Splittables {
    Field(Field),
    Fields(Vec<Field>),
}

impl From<Field> for Splittables {
    fn from(value: Field) -> Self {
        Self::Field(value.into())
    }
}

impl From<&Field> for Splittables {
    fn from(value: &Field) -> Self {
        Self::Field(value.into())
    }
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

type Groupables = Splittables;
type Fetchables = Groupables;

#[derive(Debug, Clone)]
pub enum Selectables {
    All,
    AllWithRelations,
    Field(Field),
    Fields(Vec<Field>),
}

impl Buildable for Selectables {
    fn build(&self) -> String {
        let fields = match self {
            Selectables::All => vec!["*".to_string()],
            // TODO: include all relations, graph strings automatically. To be generated by the
            // macro system.
            Selectables::AllWithRelations => vec!["*".into()],
            Selectables::Field(f) => vec![format!("{f}")],
            Selectables::Fields(fields) => {
                fields.iter().map(|f| format!("{f}")).collect::<Vec<_>>()
            }
        };
        fields.join(", ")
    }
}

// #[derive(Debug, Clone, Copy)]
// pub struct All;

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

impl<'a, const N: usize> From<&[&Field; N]> for Selectables {
    fn from(value: &[&Field; N]) -> Self {
        Self::Fields(value.map(Into::into).to_vec())
    }
}

impl From<Vec<&Field>> for Selectables {
    fn from(value: Vec<&Field>) -> Self {
        Self::Fields(value.into_iter().map(ToOwned::to_owned).collect())
    }
}
impl From<Vec<Field>> for Selectables {
    fn from(value: Vec<Field>) -> Self {
        Self::Fields(value)
    }
}

impl From<Field> for Selectables {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

impl From<&Field> for Selectables {
    fn from(value: &Field) -> Self {
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
        vec![]
    }
}

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
pub fn select(selectables: impl Into<Selectables>) -> SelectStatement {
    let selectables: Selectables = selectables.into();

    SelectStatement {
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
    }
}

impl SelectStatement {
    /// Specifies the table to select from.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to select from.
    ///
    /// # Example
    ///
    /// ```
    /// use query_builder::{QueryBuilder, Field};
    ///
    /// let mut builder = QueryBuilder::select();
    /// builder.from("users");
    ///
    /// assert_eq!(builder.to_string(), "SELECT * FROM users");
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
            TargettablesForSelect::SubQuery(query) => {
                let params = query.get_bindings();
                vec![query.to_string()]
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
    /// use query_builder::{QueryBuilder, Field, Filter};
    ///
    /// let mut builder = QueryBuilder::select();
    /// let condition = Filter::from(("age", ">", 18));
    /// builder._(condition);
    ///
    /// assert_eq!(builder.to_string(), "SELECT *  age > 18");
    /// ```
    pub fn where_(mut self, condition: impl Conditional + Clone) -> Self {
        self.update_bindings(condition.get_bindings());
        let condition = Filter::new(condition);
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
    /// use query_builder::{QueryBuilder, Field};
    ///
    /// let mut builder = QueryBuilder::select();
    /// let country = Field::new("country");
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
    /// let age = Field::new("age");
    /// let gender = Field::new("gender");
    /// query = query.split(&[age, gender]);
    ///
    /// assert_eq!(query.build(), "SELECT *, age, gender FROM table SPLIT age, gender");
    /// ```
    pub fn split(mut self, splittables: impl Into<Splittables>) -> Self {
        let fields: Splittables = splittables.into();
        // self.update_bindings(fields.get_bindings());

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
    /// # use query_builder::{QueryBuilder, Field};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.group_by(Field::new("age"));
    /// ```
    ///
    ///
    /// # Examples: For multiple fields
    ///
    /// ```
    ///
    /// let age = Field::new("age");
    /// let gender = Field::new("gender");
    /// query = query.group_by(&[age, gender]);
    ///
    /// assert_eq!(query.build(), "SELECT *, age, gender FROM table GROUP BY age, gender");
    /// ```
    pub fn group_by(mut self, groupables: impl Into<Groupables>) -> Self {
        let fields: Groupables = groupables.into();
        // self.update_bindings(fields.get_bindings());

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
    /// # use query_builder::{QueryBuilder, Order, Direction, Field};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.order_by(Order::new(Field::new("age"), Direction::Ascending));
    ///
    /// query_builder.order(&[
    ///     Order::new(Field::new("age"), Direction::Ascending),
    ///     Order::new(Field::new("name"), Direction::Descending),
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
    pub fn limit(mut self, limit: impl Into<crate::NumberLike>) -> Self {
        let limit: crate::NumberLike = limit.into();
        match limit {
            crate::NumberLike::Number(n) => {
                let binding = Binding::new(n);
                self.limit = Some(binding.get_param_dollarised());
                self.update_bindings(vec![binding]);
            }
            crate::NumberLike::Field(_) | crate::NumberLike::Param(_) => {
                self.limit = Some(limit.to_value().to_raw_string());
            } // crate::NumberLike::Param(_) => todo!(),
        };
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
    pub fn start(mut self, start: impl Into<crate::NumberLike>) -> Self {
        let start: crate::NumberLike = start.into();
        match start {
            crate::NumberLike::Number(n) => {
                let binding = Binding::new(n);
                self.start = Some(binding.get_param_dollarised());
                self.update_bindings(vec![binding]);
            }
            crate::NumberLike::Field(_) | crate::NumberLike::Param(_) => {
                self.start = Some(start.to_value().to_raw_string());
            }
        };
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
        // self.update_bindings(fields.get_bindings());

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
    pub fn timeout(mut self, duration: impl Into<DurationLike>) -> Self {
        let duration: sql::Value = duration.into().into();
        // let duration = sql::Duration::from(duration);
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

        query = format!(
            "SELECT {} FROM {}",
            self.projections,
            self.targets.join(", ")
        );

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

        if let Some(limit_value) = &self.limit {
            query.push_str(" LIMIT ");
            query.push_str(&limit_value.to_string());
        }

        if let Some(start_value) = &self.start {
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

#[test]
fn test_statement_with_alias() {
    let canadian_cities = AliasName::new("legal_age");
    let age = Field::new("age");
    let city = Field::new("city");
    let fake_id = SurrealId::try_from("user:oyelowo").unwrap();
    let statement = select(All)
        .from(fake_id)
        .where_(
            cond(city.is("Prince Edward Island"))
                .and(city.is("NewFoundland"))
                .or(city.like("Toronto")),
        )
        .order_by(order(&age).numeric())
        .limit(153)
        .start(10)
        .parallel();

    let statement_aliased = statement.__as__(canadian_cities);

    assert_eq!(
        statement_aliased.fine_tune_params(),
        "(SELECT * FROM $_param_00000001 WHERE (city IS $_param_00000002) AND (city IS $_param_00000003) OR (city ~ $_param_00000004) ORDER BY age NUMERIC ASC LIMIT $_param_00000005 START AT $_param_00000006 PARALLEL) AS legal_age"
    );
    assert_eq!(
        statement_aliased.to_raw().to_string(),
        "(SELECT * FROM user:oyelowo WHERE (city IS 'Prince Edward Island') AND (city IS 'NewFoundland') OR (city ~ 'Toronto') ORDER BY age NUMERIC ASC LIMIT 153 START AT 10 PARALLEL) AS legal_age"
    );
}
