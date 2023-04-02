/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    statements::{select, SelectStatement},
    traits::{
        Binding, BindingsList, Buildable, Conditional, Erroneous, Operatable, Parametric, ToRaw,
    },
    types::{cond, Table},
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
}

#[derive(Debug, Clone)]
pub struct Clause {
    kind: ClauseType,
    query_string: String,
    bindings: BindingsList,
}

impl Buildable for Clause {
    fn build(&self) -> String {
        self.query_string.to_string()
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
        // match self.kind.clone() {
        //     ClauseType::Empty => vec![],
        //     ClauseType::Where(filter) => filter.get_bindings(),
        //     ClauseType::Query(select_statement) => select_statement.get_bindings(),
        //     ClauseType::Id(id) => id.get_bindings(),
        //     ClauseType::All => vec![],
        //     ClauseType::Last => vec![],
        //     ClauseType::Index(_) => vec![],
        // }
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
            Index(i) => format!("[{i}]"),
        };
        Self {
            kind,
            query_string,
            bindings,
        }
    }

    pub fn get_errors(&self, table_name: &'static str) -> Vec<String> {
        let mut errors = vec![];
        if let ClauseType::Id(id) = self.kind.clone() {
            if !id
                .to_string()
                .starts_with(format!("{table_name}:").as_str())
            {
                errors.push(format!(
                    "invalid id {id}. Id does not belong to table {table_name}"
                ))
            }
        }
        errors
    }

    pub fn format_with_model(&self, table_name: &'static str) -> String {
        match self.kind.clone() {
            ClauseType::Query(q) => self.to_string(),
            _ => format!("{table_name}{self}"),
        }
    }
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
pub fn index(index: Index) -> Index {
    // Index(index)
    index
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0.clone().to_value()))
    }
}

#[test]
fn test_display_clause_with_empty() {
    // test empty clause
    let empty_clause = Clause::from(Empty);
    assert_eq!(format!("{}", empty_clause), "");
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
}

#[test]
fn test_display_clause_with_all() {
    // test all clause
    let all_clause = Clause::from(All);
    assert_eq!(format!("{}", all_clause), "[*]");
}

#[test]
fn test_display_clause_with_last() {
    // test last clause
    let last_clause = Clause::from(Last);
    assert_eq!(format!("{}", last_clause), "[$]");
}

#[test]
fn test_display_clause_with_index() {
    // test index clause
    let index_clause = Clause::from(Index(42.into()));
    assert_eq!(format!("{}", index_clause), "[42]");
}
// }
