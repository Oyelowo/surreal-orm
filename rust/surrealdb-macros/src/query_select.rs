/*
 * Author: Oyelowo Oyedayo
 * Email: Oyelowo Oyedayo
 * */

use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter, Result as FmtResult},
};

use surrealdb::sql::{self, Table, Value};

use crate::{
    db_field::{Binding, BindingsList, DbFilter, Parametric},
    value_type_wrappers::SurrealId,
    DbField, SurrealdbNode,
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
pub fn order(field: &DbField) -> Order {
    Order::new(field)
}

/// Represents an ordering field, direction, and options for a database query.
#[derive(Debug, Clone, Copy)]
pub struct Order<'a> {
    field: &'a DbField,
    direction: Option<OrderDirection>,
    option: Option<OrderOption>,
}

impl<'a> Order<'a> {
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
    pub fn new(field: &'a DbField) -> Self {
        Order {
            field,
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

impl<'a> Display for &Order<'a> {
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
pub enum FromAbles<'a> {
    Table(Table),
    Tables(sql::Tables),
    SurrealId(SurrealId),
    SurrealIds(Vec<SurrealId>),
    // Should already be bound
    SubQuery(QueryBuilder<'a>),
}

impl<'a> From<sql::Tables> for FromAbles<'a> {
    fn from(value: sql::Tables) -> Self {
        Self::Tables(value)
    }
}

impl<'a> From<Vec<sql::Thing>> for FromAbles<'a> {
    fn from(value: Vec<sql::Thing>) -> Self {
        Self::SurrealIds(value.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}

impl<'a> From<sql::Thing> for FromAbles<'a> {
    fn from(value: sql::Thing) -> Self {
        Self::SurrealId(value.into())
    }
}

impl<'a> From<Vec<SurrealId>> for FromAbles<'a> {
    fn from(value: Vec<SurrealId>) -> Self {
        Self::SurrealIds(value)
    }
}

impl<'a> From<SurrealId> for FromAbles<'a> {
    fn from(value: SurrealId) -> Self {
        Self::SurrealId(value)
    }
}

impl<'a> From<Table> for FromAbles<'a> {
    fn from(value: Table) -> Self {
        Self::Table(value)
    }
}

impl<'a> Parametric for FromAbles<'a> {
    fn get_bindings(&self) -> BindingsList {
        match self {
            FromAbles::Table(table) => {
                let binding = Binding::new(table.to_owned());
                vec![binding]
            }
            FromAbles::Tables(tables) => {
                let bindings = tables
                    .to_vec()
                    .into_iter()
                    .map(|t| Binding::new(t))
                    .collect::<Vec<_>>();
                bindings
            }
            // Should already be bound
            FromAbles::SubQuery(_query) => vec![],
            FromAbles::SurrealId(id) => vec![Binding::new(id.to_owned())],

            FromAbles::SurrealIds(ids) => {
                let bindings = ids
                    .into_iter()
                    .map(|id| Binding::new(id.to_owned()))
                    .collect::<Vec<_>>();
                bindings
            }
        }
    }
}

/// The query builder struct used to construct complex database queries.
#[derive(Debug, Clone)]
pub struct QueryBuilder<'a> {
    // projections: Vec<&'a str>,
    projections: Vec<String>,
    /// The list of target tables for the query.
    // targets: Vec<&'a str>,
    targets: Vec<String>,
    where_: Option<String>,
    // where_: Option<&'a str>,
    // split: Option<Vec<&'a str>>,
    split: Vec<String>,
    // group_by: Option<Vec<&'a str>>,
    group_by: Vec<String>,
    order_by: Vec<Order<'a>>,
    limit: Option<u64>,
    start: Option<u64>,
    // fetch: Option<Vec<&'a str>>,
    fetch: Vec<String>,
    timeout: Option<&'a str>,
    parallel: bool,
    ________params_accumulator: BindingsList,
}

impl<'a> Parametric for QueryBuilder<'a> {
    fn get_bindings(&self) -> BindingsList {
        self.________params_accumulator.to_vec()
    }
}

impl<'a> QueryBuilder<'a> {
    /// Create a new instance of QueryBuilder.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::QueryBuilder;
    ///
    /// let query_builder = QueryBuilder::new();
    /// ```
    pub fn new() -> QueryBuilder<'a> {
        QueryBuilder {
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
            ________params_accumulator: vec![],
        }
    }

    /// Add a wildcard projection to the query.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::QueryBuilder;
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.select_all();
    /// ```
    pub fn select_all(&mut self) -> &mut Self {
        self.projections.push("*".to_string());
        self
    }

    /// Add a projection to the query for a single field.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field to project.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::{QueryBuilder, DbField};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.select(DbField("my_field".to_string()));
    /// ```
    pub fn select<'field, T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Cow<'field, DbField>>,
    {
        let field: &DbField = &field.into();
        self.projections.push(field.to_string());
        self
    }

    /// Add projections to the query for multiple fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of field names to project.
    ///
    /// # Example
    ///
    /// ```
    /// use surrealdb::{QueryBuilder, DbField};
    ///
    /// let mut query_builder = QueryBuilder::new();
    /// let fields = &[DbField("field_1".to_string()), DbField("field_2".to_string())];
    /// query_builder.select_many(fields);
    /// ```
    pub fn select_many<'field, T>(&mut self, fields: &[T]) -> &mut Self
    where
        T: Into<Cow<'field, DbField>> + Clone + Display,
    {
        self.projections.extend_from_slice(
            fields
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .as_slice(),
        );
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
    pub fn from(&'a mut self, targets: impl Into<FromAbles<'a>>) -> &'a mut Self {
        let targets: FromAbles = targets.into();
        let targets_bindings = targets.get_bindings();

        // When we have either one or many table names or record ids, we want to use placeholders
        // as the targets which would be bound later but for a subquery in from, that must have
        // already been done by the Subquery(in this case, select query) builder itself
        let target_names = match targets {
            FromAbles::Table(_)
            | FromAbles::Tables(_)
            | FromAbles::SurrealId(_)
            | FromAbles::SurrealIds(_) => targets_bindings
                .iter()
                .map(|b| b.get_param().to_string())
                .collect::<Vec<_>>(),
            // Subquery must have be built and interpolated, so no need for rebinding
            FromAbles::SubQuery(subquery) => vec![subquery.to_string()],
        };
        self.update_bindings(targets_bindings);
        // self.________params_accumulator.extend(targets_bindings);
        self.targets.extend(target_names);
        self
    }

    /// Adds a condition to the `WHERE` clause of the SQL query.
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
    /// builder.where_(condition);
    ///
    /// assert_eq!(builder.to_string(), "SELECT * WHERE age > 18");
    /// ```
    pub fn where_(&mut self, condition: impl Into<DbFilter> + Parametric) -> &mut Self {
        self.update_bindings(condition.get_bindings());
        let condition: DbFilter = condition.into();
        self.where_ = Some(condition.to_string());
        self
    }

    fn update_bindings(&mut self, bindings: BindingsList) -> &mut Self {
        // let mut updated_params = vec![];
        // updated_params.extend(self.________params_accumulator.to_vec());
        // updated_params.extend(parametric_value.get_bindings());
        self.________params_accumulator.extend(bindings);
        self
    }

    /// Adds a field to the `SPLIT BY` clause of the SQL query.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field to add to the `SPLIT BY` clause.
    ///
    /// # Example
    ///
    /// ```
    /// use query_builder::{QueryBuilder, DbField};
    ///
    /// let mut builder = QueryBuilder::select();
    /// builder.split(DbField::new("country"));
    ///
    /// assert_eq!(builder.to_string(), "SELECT * SPLIT BY country");
    /// ```
    pub fn split<'field, T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Cow<'field, DbField>>,
    {
        let field: &DbField = &field.into();
        self.split.push(field.to_string());
        self
    }

    /// Adds multiple fields to split the query result into multiple groups.
    ///
    /// # Arguments
    ///
    /// * `fields` - The names of the fields to split the result by.
    ///
    /// # Example
    ///
    /// ```
    /// use my_db::{QueryBuilder, DbField};
    ///
    /// let mut query = QueryBuilder::select();
    /// let fields = vec![DbField::from("age"), DbField::from("gender")];
    /// query = query.split_many(&fields);
    /// assert_eq!(query.build(), "SELECT *, age, gender FROM table GROUP BY age, gender");
    /// ```
    pub fn split_many<'field, T>(&mut self, fields: &[T]) -> &mut Self
    where
        T: Into<Cow<'field, DbField>> + Clone + Display,
    {
        self.split.extend_from_slice(
            fields
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        self
    }

    /// Sets the GROUP BY clause for the query.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field to group by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::{QueryBuilder, DbField};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.group_by(DbField::new("age"));
    /// ```
    pub fn group_by<'field, T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Cow<'field, DbField>>,
    {
        let field: &DbField = &field.into();
        self.group_by.push(field.to_string());
        self
    }

    /// Sets multiple fields to GROUP BY in the query.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of field names to group by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::{QueryBuilder, DbField};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.group_by_many(&[DbField::new("age"), DbField::new("name")]);
    /// ```
    pub fn group_by_many<'field, T>(&mut self, fields: &[T]) -> &mut Self
    where
        T: Into<Cow<'field, DbField>> + Clone + Display,
    {
        self.group_by.extend_from_slice(
            fields
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        self
    }

    /// Sets the ORDER BY clause for the query.
    ///
    /// # Arguments
    ///
    /// * `order` - The field and direction to order by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::{QueryBuilder, Order, Direction, DbField};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.order_by(Order::new(DbField::new("age"), Direction::Ascending));
    /// ```
    pub fn order_by(&mut self, order: Order<'a>) -> &mut Self {
        self.order_by.push(order);
        self
    }

    /// Sets multiple fields to ORDER BY in the query.
    ///
    /// # Arguments
    ///
    /// * `orders` - A slice of fields and directions to order by.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use query_builder::{QueryBuilder, Order, Direction, DbField};
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.order_by_many(&[
    ///     Order::new(DbField::new("age"), Direction::Ascending),
    ///     Order::new(DbField::new("name"), Direction::Descending),
    /// ]);
    /// ```
    pub fn order_by_many(&mut self, orders: &[Order<'a>]) -> &mut Self {
        self.order_by.extend_from_slice(orders.to_vec().as_slice());
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
    pub fn limit(&mut self, limit: u64) -> &mut Self {
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
    pub fn start(&mut self, start: u64) -> &mut Self {
        self.start = Some(start);
        self
    }

    /// Adds a field to the list of fields to fetch in the current query.
    ///
    /// # Arguments
    ///
    /// * `field` - A reference to a field to be fetched in the query.
    ///
    /// # Example
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch("id")
    ///     .fetch("name")
    ///     .from("users")
    ///     .build();
    /// ```
    ///
    /// # Output
    ///
    /// The `fetch` method returns a mutable reference to the QueryBuilder instance it was called on,
    /// allowing further method chaining.
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch("id")
    ///     .fetch("name")
    ///     .from("users")
    ///     .build();
    ///
    /// assert_eq!(query, "SELECT id, name FROM users");
    /// ```
    pub fn fetch<'field, T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Cow<'field, DbField>>,
    {
        let field: &DbField = &field.into();
        self.fetch.push(field.to_string());
        self
    }

    /// Adds multiple fields to the list of fields to fetch in the current query.
    ///
    /// # Arguments
    ///
    /// * `fields` - A slice of references to fields to be fetched in the query.
    ///
    /// # Example
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let fields = ["id", "name"];
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch_many(&fields)
    ///     .from("users")
    ///     .build();
    /// ```
    ///
    /// # Output
    ///
    /// The `fetch_many` method returns a mutable reference to the QueryBuilder instance it was called on,
    /// allowing further method chaining.
    ///
    /// ```
    /// use my_cool_library::QueryBuilder;
    ///
    /// let fields = ["id", "name"];
    ///
    /// let query = QueryBuilder::new()
    ///     .fetch_many(&fields)
    ///     .from("users")
    ///     .build();
    ///
    /// assert_eq!(query, "SELECT id, name FROM users");
    /// ```
    pub fn fetch_many<'field, T>(&mut self, fields: &[T]) -> &mut Self
    where
        T: Into<Cow<'field, DbField>> + Clone + Display,
    {
        self.fetch.extend_from_slice(
            fields
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .as_slice(),
        );
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
    pub fn timeout(&mut self, duration: &'a str) -> &mut Self {
        self.timeout = Some(duration);
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
    pub fn parallel(&mut self) -> &mut Self {
        self.parallel = true;
        self
    }
}
/*
 * Syntax from specs:https://surrealdb.com/docs/surrealql/statements/select
 * SELECT @projections
    FROM @targets
    [ WHERE @condition ]
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
impl<'a> Display for QueryBuilder<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
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

        if let Some(timeout_value) = self.timeout {
            query.push_str(" TIMEOUT ");
            query.push_str(&timeout_value.to_string());
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        query.push(';');
        // Idea
        println!("VOOOOVOOO ",);
        self.________params_accumulator
            .clone()
            .into_iter()
            .map(|x| {
                let yy = (format!("{}", x.get_param()), format!("{}", x.get_value()));
                dbg!(yy)
            })
            .collect::<Vec<_>>();
        write!(f, "{}", query)
    }
}
