/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::ops::Deref;

use surrealdb::sql;

use crate::{
    array,
    statements::{
        select::{select, Selectables},
        SelectStatement,
    },
    traits::{
        Binding, BindingsList, Buildable, Conditional, Erroneous, Operatable, Parametric, ToRaw,
    },
    types::{cond, Param, Table},
    ErrorList, Operation, Tables,
};

use super::{Field, Filter, NumberLike, SurrealId};

pub fn where_(condition: impl Conditional) -> Filter {
    if condition.get_errors().is_empty() {
        // TODO: Maybe pass to DB filter and check and return Result<Filter> in relate_query
    }
    Filter::new(condition)
}

#[derive(Debug, Clone)]
pub enum ClauseType {
    All,
    Last,
    Index(Index),
    Empty,
    Where(Filter),
    Query(SelectStatement),
    Id(SurrealId),
    AnyEdgeFilter(AnyEdgeFilter),
}

#[derive(Debug, Clone)]
enum ModelOrFieldName {
    Model(String),
    Field(String),
}

#[derive(Debug, Clone)]
struct Clause {
    kind: ClauseType,
    // edge_table_name: Option<String>,
    arrow: Option<String>,
    model_or_field_name: Option<ModelOrFieldName>,
    // query_string: String,
    bindings: BindingsList,
    errors: ErrorList,
}

impl Buildable for Clause {
    fn build(&self) -> String {
        // let edge_table_name = self.clone().edge_table_name.unwrap_or_default();
        let connection_name = match self.model_or_field_name {
            Some(name) => match name {
                ModelOrFieldName::Model(m) => m,
                ModelOrFieldName::Field(f) => f,
            },
            None => "".to_string(),
        };

        let clause = match self.kind.clone() {
            ClauseType::Query(q) => self.build(),
            ClauseType::AnyEdgeFilter(edge_filters) => {
                format!(
                    "{}({}, {})",
                    self.arrow.as_ref().unwrap_or(&"".to_string()),
                    connection_name,
                    edge_filters.build(),
                    // self.arrow.as_ref().unwrap_or(&"".to_string()),
                )
            }
            ClauseType::Id(id) => self
                .get_bindings()
                .pop()
                .expect("Id must have only one binding. Has to be an error. Please report.")
                .get_param_dollarised(),
            _ => format!("{}{}", connection_name, self),
        };

        let connection = self.arrow.map_or(clause, |a| format!("{a}{clause}"));
        connection
    }
}

struct NodeClause(Clause);

impl<T> From<T> for NodeClause
where
    T: Into<Clause>,
{
    fn from(value: T) -> Self {
        let clause: Clause = value.into();
        Self(clause)
    }
}

struct EdgeClause(Clause);

// impl EdgeClause {
//     pub fn with_arrow(mut self, arrow: impl Into<String>) -> Self {
//         self.0.arrow = Some(arrow.into());
//         self
//     }
// }

impl<T> From<T> for EdgeClause
where
    T: Into<Clause>,
{
    fn from(value: T) -> Self {
        let clause: Clause = value.into();
        Self(clause)
    }
}

impl From<Operation> for Clause {
    fn from(value: Operation) -> Self {
        let filter = Filter::new(value);
        Self::new(ClauseType::Where(filter))
    }
}

impl From<&Self> for Clause {
    fn from(value: &Self) -> Self {
        value.clone()
    }
}

impl Parametric for Clause {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Erroneous for Clause {
    fn get_errors(&self) -> ErrorList {
        vec![]
    }
}

impl Clause {
    pub fn new(kind: ClauseType) -> Self {
        use ClauseType::*;
        let mut bindings = vec![];

        let query_string = match &kind {
            Empty => "".into(),
            Where(filter) => {
                // bindings.extend(filter.get_bindings());
                bindings = filter.get_bindings();
                format!("[WHERE {filter}]")
            }
            Id(surreal_id) => {
                // The Table name component of the Id comes from the macro. e.g For student:5, the Schema which this is wrapped into provide. So all we need here is the id component, student
                let id_bindings = Binding::new(surreal_id.clone());
                let param_string = format!("{}", id_bindings.get_param_dollarised());
                bindings = vec![id_bindings];
                param_string
            }
            Query(select_statement) => {
                bindings = select_statement.get_bindings();
                format!("({})", select_statement.build().trim_end_matches(";"))
            }
            All => format!("[*]"),
            Last => format!("[$]"),
            Index(index) => {
                let index_bindings = Binding::new(index.clone().0.to_value());
                let param_string = format!("{}", index_bindings.get_param_dollarised());
                bindings = vec![index_bindings];
                format!("[{param_string}]")
            }
            AnyEdgeFilter(edge_tables) => {
                bindings = edge_tables.get_bindings();
                let build = format!("{}", edge_tables.build());
                format!("({build})")
            }
        };

        Self {
            kind,
            // query_string,
            bindings,
            arrow: None,
            model_or_field_name: None,
            // edge_table_name: None,
            errors: todo!(),
        }
    }

    pub fn with_arrow(mut self, arrow: impl Into<String>) -> Self {
        self.arrow = Some(arrow.into());
        self
    }

    pub fn with_table_name(mut self, table_name: impl Into<String>) -> Self {
        let table_name: String = table_name.into();
        let mut updated_clause = self.update_errors(&table_name);
        updated_clause.model_or_field_name = Some(ModelOrFieldName::Model(table_name));
        updated_clause
    }

    pub fn with_field_name(mut self, field_name: impl Into<String>) -> Self {
        let field_name: String = field_name.into();
        let mut updated_clause = self.update_errors(&field_name);
        updated_clause.model_or_field_name = Some(ModelOrFieldName::Field(field_name));
        updated_clause
    }

    fn update_errors(mut self, table_name: &str) -> Self {
        let mut errors = vec![];
        if let ClauseType::Id(id) = &self.kind {
            if !id
                .to_string()
                .starts_with(format!("{table_name}:").as_str())
            {
                errors.push(format!(
                    "invalid id {id}. Id does not belong to table {table_name}"
                ))
            }
        }
        self.errors = errors;
        self
    }

    pub fn format_with_model(mut self, table_name: &'static str) -> String {
        match self.kind.clone() {
            ClauseType::Query(q) => self.build(),
            ClauseType::AnyEdgeFilter(edge_filters) => {
                self.bindings.extend(edge_filters.get_bindings());

                format!(
                    "{}({}, {}){}",
                    self.arrow.as_ref().unwrap_or(&"".to_string()),
                    table_name,
                    edge_filters.build(),
                    self.arrow.as_ref().unwrap_or(&"".to_string()),
                )
            }
            ClauseType::Id(id) => self
                .get_bindings()
                .pop()
                .expect("Id must have only one binding. Has to be an error. Please report.")
                .get_param_dollarised(),
            _ => format!("{table_name}{self}"),
        }
    }

    // pub fn format_with_object(&self) -> String {
    //     match self.kind.clone() {
    //         // ClauseType::Query(q) => self.to_string(),
    //         ClauseType::Id(q) => self.to_string(),
    //         _ => self.build(),
    //     }
    // }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl From<SurrealId> for Clause {
    fn from(value: SurrealId) -> Self {
        Self::new(ClauseType::Id(value.clone()))
    }
}

impl From<&SurrealId> for Clause {
    fn from(value: &SurrealId) -> Self {
        Self::new(ClauseType::Id(value.clone()))
    }
}

// impl From<Field> for Clause {
//     fn from(value: Field) -> Self {
//         Self::new(ClauseType::Where(value))
//     }
// }

// impl From<&Field> for Clause {
//     fn from(value: &Field) -> Self {
//         Self::new(ClauseType::(value.clone()))
//     }
// }

impl From<Filter> for Clause {
    fn from(value: Filter) -> Self {
        Self::new(ClauseType::Where(value))
    }
}

impl From<&Filter> for Clause {
    fn from(value: &Filter) -> Self {
        Self::new(ClauseType::Where(value.clone()))
    }
}

impl From<Empty> for Clause {
    fn from(value: Empty) -> Self {
        Self::new(ClauseType::Empty)
    }
}

impl From<SelectStatement> for Clause {
    fn from(value: SelectStatement) -> Self {
        Self::new(ClauseType::Query(value))
    }
}

impl From<&SelectStatement> for Clause {
    fn from(value: &SelectStatement) -> Self {
        // Self::Query(value.to_owned().into())
        Self::new(ClauseType::Query(value.clone()))
    }
}

pub struct Empty;

impl Operatable for Empty {}

impl Buildable for Empty {
    fn build(&self) -> String {
        "".to_string()
    }
}

impl Conditional for Empty {
    fn get_condition_query_string(&self) -> String {
        "".to_string()
    }
}

impl Erroneous for Empty {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Parametric for Empty {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

pub struct All;

impl From<All> for Clause {
    fn from(value: All) -> Self {
        Self::new(ClauseType::All)
    }
}

impl From<Last> for Clause {
    fn from(value: Last) -> Self {
        Self::new(ClauseType::Last)
    }
}

impl std::fmt::Display for All {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("*"))
    }
}

pub struct Last;

impl std::fmt::Display for Last {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("*"))
    }
}

// pub struct Index(u128);
#[derive(Debug, Clone)]
pub struct Index(NumberLike);

impl From<Index> for Clause {
    fn from(value: Index) -> Self {
        Self::new(ClauseType::Index(value))
    }
}
pub fn index(index: impl Into<NumberLike>) -> Index {
    // Index(index)
    Index(index.into())
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0.clone().to_value()))
    }
}

#[derive(Debug, Clone)]
pub struct AnyEdgeFilter {
    edge_tables: Vec<Table>,
    where_: Option<String>,
    bindings: BindingsList,
}

impl AnyEdgeFilter {
    pub fn where_(mut self, condition: impl Conditional + Clone) -> Self {
        self.bindings.extend(condition.get_bindings());
        let condition = Filter::new(condition);
        self.where_ = Some(condition.build());
        self
    }
}

impl Buildable for AnyEdgeFilter {
    fn build(&self) -> String {
        let mut query = format!(
            "{} ",
            self.edge_tables
                .to_vec()
                .into_iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        if let Some(where_) = &self.where_ {
            query = format!("{} WHERE {}", query, where_);
        }

        query
    }
}

impl Parametric for AnyEdgeFilter {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl From<AnyEdgeFilter> for Clause {
    fn from(value: AnyEdgeFilter) -> Self {
        Self::new(ClauseType::AnyEdgeFilter(value))
    }
}

pub fn any_edge(edges: impl Into<crate::Tables>) -> AnyEdgeFilter {
    AnyEdgeFilter {
        edge_tables: edges.into().into(),
        where_: None,
        bindings: vec![],
    }
}
#[test]
fn test_display_clause_with_empty() {
    // test empty clause
    let empty_clause = Clause::from(Empty);
    assert_eq!(format!("{}", empty_clause), "");
    assert_eq!(format!("{}", empty_clause.to_raw()), "");
}

#[test]
fn test_display_clause_with_where_filter() {
    // test where clause
    let filter = cond(Field::new("age").equal(18));
    // let where_clause = ClauseType::Where(filter);
    let where_clause = Clause::from(filter);
    assert_eq!(
        format!("{}", where_clause.fine_tune_params()),
        "[WHERE age = $_param_00000001]"
    );
    assert_eq!(format!("{}", where_clause.to_raw()), "[WHERE age = 18]");
}

#[test]
fn test_display_clause_with_id() {
    // test id clause
    let id_clause = Clause::from(SurrealId::try_from("student:5").unwrap());
    // assert_eq!(format!("{:?}", id_clause), ":5");
    assert_eq!(id_clause.fine_tune_params(), "$_param_00000001");
    assert_eq!(format!("{}", id_clause.to_raw()), "student:5");
}

#[test]
fn test_display_clause_with_query() {
    // test query clause
    let table = Table::new("students");
    let select_statement = select(All).from(table);
    let query_clause = Clause::from(select_statement);
    assert_eq!(format!("{}", query_clause), "(SELECT * FROM students)");
    assert_eq!(
        query_clause.to_raw().to_string(),
        "(SELECT * FROM students)"
    );
}

#[test]
fn test_display_clause_with_all() {
    // test all clause
    let all_clause = Clause::from(All);
    assert_eq!(format!("{}", all_clause), "[*]");
    assert_eq!(format!("{}", all_clause.to_raw()), "[*]");
}

#[test]
fn test_display_clause_with_last() {
    // test last clause
    let last_clause = Clause::from(Last);
    assert_eq!(format!("{}", last_clause), "[$]");
    assert_eq!(format!("{}", last_clause.to_raw()), "[$]");
}

#[test]
fn test_display_clause_with_index() {
    // test index clause
    let index_clause = Clause::from(index(42));
    assert_eq!(
        format!("{}", index_clause.fine_tune_params()),
        "[$_param_00000001]"
    );
    assert_eq!(format!("{}", index_clause.to_raw()), "[42]");
}

#[test]
fn test_display_clause_with_index_field() {
    // test index clause
    let position = Field::new("position");
    let index_clause = Clause::from(index(position));
    assert_eq!(index_clause.fine_tune_params(), "[$_param_00000001]");
    assert_eq!(format!("{}", index_clause.to_raw()), "[position]");
}

#[test]
fn test_display_clause_with_index_param() {
    // test index clause
    let position = Param::new("position");
    let index_clause = Clause::from(index(position));
    assert_eq!(index_clause.fine_tune_params(), "[$_param_00000001]");
    assert_eq!(format!("{}", index_clause.to_raw()), "[$position]");
}

#[test]
fn test_display_clause_with_any_edge_condition_simple() {
    let writes = Table::new("writes");
    let reads = Table::new("reads");
    let purchased = Table::new("purchased");
    let amount = Field::new("amount");

    let age_edge_condition =
        any_edge(vec![writes, reads, purchased]).where_(amount.less_than_or_equal(120));

    assert_eq!(
        age_edge_condition.fine_tune_params(),
        "writes, reads, purchased  WHERE amount <= $_param_00000001"
    );
    assert_eq!(
        format!("{}", age_edge_condition.to_raw()),
        "writes, reads, purchased  WHERE amount <= 120"
    );
}

#[test]
fn test_display_clause_with_any_edge_condition_complex() {
    let writes = Table::new("writes");
    let reads = Table::new("reads");
    let purchased = Table::new("purchased");
    let city = Field::new("city");

    let age_edge_condition = any_edge(vec![writes, reads, purchased]).where_(
        cond(city.is("Prince Edward Island"))
            .and(city.is("NewFoundland"))
            .or(city.like("Toronto")),
    );

    assert_eq!(
        age_edge_condition.fine_tune_params(),
        "writes, reads, purchased  WHERE (city IS $_param_00000001) AND (city IS $_param_00000002) OR (city ~ $_param_00000003)"
    );
    assert_eq!(
        format!("{}", age_edge_condition.to_raw()),
        "writes, reads, purchased  WHERE (city IS 'Prince Edward Island') AND (city IS 'NewFoundland') OR (city ~ 'Toronto')"
    );
}
