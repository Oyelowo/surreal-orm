/*
 * Author: Oyelowo Oyedayo
 * Email: Oyelowo Oyedayo
 * */

use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::{db_field::DbQuery, DbField, SurrealdbNode};

pub fn order(field: &DbField) -> Order {
    Order::new(field)
}
#[derive(Debug, Clone, Copy)]
pub struct Order<'a> {
    field: &'a DbField,
    direction: Option<OrderDirection>,
    option: Option<OrderOption>,
}

impl<'a> Order<'a> {
    // pub fn new(field: &'a str) -> Self {
    pub fn new(field: &'a DbField) -> Self {
        Order {
            field,
            direction: None,
            option: None,
        }
    }

    pub fn asc(mut self) -> Self {
        self.direction = Some(OrderDirection::Asc);
        self
    }

    pub fn desc(mut self) -> Self {
        self.direction = Some(OrderDirection::Desc);
        self
    }
    pub fn rand(mut self) -> Self {
        self.option = Some(OrderOption::Rand);
        self
    }

    pub fn collate(mut self) -> Self {
        self.option = Some(OrderOption::Collate);
        self
    }

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

pub struct Select<'a> {
    projections: Vec<&'a str>,
    // targets: Vec<&'a str>,
    targets: Vec<String>,
    where_: Option<String>,
    // where_: Option<&'a str>,
    split: Option<Vec<&'a str>>,
    // group_by: Option<Vec<&'a str>>,
    group_by: Vec<String>,
    order_by: Vec<Order<'a>>,
    limit: Option<u64>,
    start: Option<u64>,
    fetch: Option<Vec<&'a str>>,
    timeout: Option<&'a str>,
    parallel: bool,
}

impl<'a> Select<'a> {
    pub fn new() -> Select<'a> {
        Select {
            projections: vec![],
            targets: vec![],
            where_: None,
            split: None,
            group_by: vec![],
            order_by: vec![],
            limit: None,
            start: None,
            fetch: None,
            timeout: None,
            parallel: false,
        }
    }

    pub fn projection(&mut self, projection: &'a str) -> &mut Self {
        self.projections.push(projection);
        self
    }

    // pub fn from(&mut self, table_name: impl std::borrow::Borrow<str>) -> &mut Self {
    pub fn from(&'a mut self, table_name: impl std::borrow::Borrow<str> + 'a) -> &'a mut Self {
        self.targets.push(table_name.borrow().to_string());
        self
    }

    // pub fn where_(&mut self, condition: &'a str) -> &mut Self {
    pub fn where_(&mut self, condition: impl Into<DbQuery>) -> &mut Self {
        let condition: DbQuery = condition.into();
        self.where_ = Some(condition.to_string());
        self
    }

    pub fn split(&mut self, fields: &[&'a str]) -> &mut Self {
        self.split = Some(fields.to_vec());
        self
    }

    pub fn group_by<'field, T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Cow<'field, DbField>>,
    {
        let field: &DbField = &field.into();
        self.group_by.push(field.to_string());
        self
    }

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

    pub fn order_by(&mut self, order: Order<'a>) -> &mut Self {
        self.order_by.push(order);
        self
    }

    pub fn order_by_many(&mut self, orders: &[Order<'a>]) -> &mut Self {
        self.order_by.extend_from_slice(orders.to_vec().as_slice());
        self
    }

    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn start(&mut self, start: u64) -> &mut Self {
        self.start = Some(start);
        self
    }

    pub fn fetch(&mut self, fields: &[&'a str]) -> &mut Self {
        self.fetch = Some(fields.to_vec());
        self
    }

    pub fn timeout(&mut self, duration: &'a str) -> &mut Self {
        self.timeout = Some(duration);
        self
    }

    pub fn parallel(&mut self) -> &mut Self {
        self.parallel = true;
        self
    }
}

impl<'a> Display for Select<'a> {
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

        if let Some(split) = &self.split {
            query.push_str(" SPLIT ");
            query.push_str(&split.join(", "));
        }

        // if let Some(group) = &self.group_by {
        if !self.group_by.is_empty() {
            query.push_str(" GROUP BY ");
            query.push_str(&self.group_by.join(", "));
        }

        // if let Some(order) = &self.order_by {
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

        if let Some(fetch) = &self.fetch {
            query.push_str(" FETCH ");
            query.push_str(&fetch.join(", "));
        }

        if let Some(timeout_value) = self.timeout {
            query.push_str(" TIMEOUT ");
            query.push_str(&timeout_value.to_string());
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        query.push(';');

        write!(f, "{}", query)
    }
}
