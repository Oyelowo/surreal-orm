/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    statements::SelectStatement,
    traits::{BindingsList, Conditional, Erroneous, Parametric},
};

use super::{surreal_id::SurrealId, Filter, SurrealId};

pub fn where_(condition: impl Conditional) -> Filter {
    if condition.get_errors().is_empty() {
        // TODO: Maybe pass to DB filter and check and return Result<Filter> in relate_query
    }
    Filter::new(condition)
}

#[derive(Debug, Clone)]
pub enum Clause {
    All,
    Last,
    Index(u128),
    Empty,
    Where(Filter),
    Query(SelectStatement),
    Id(SurrealId),
}

impl From<&Self> for Clause {
    fn from(value: &Self) -> Self {
        value.clone()
    }
}

impl Parametric for Clause {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Clause::Empty => vec![],
            Clause::Where(filter) => filter.get_bindings(),
            Clause::Query(select_statement) => select_statement.get_bindings(),
            Clause::Id(id) => id.get_bindings(),
            Clause::All => vec![],
            Clause::Last => vec![],
            Clause::Index(_) => vec![],
        }
    }
}

impl Clause {
    pub fn get_errors(&self, table_name: &'static str) -> Vec<String> {
        let mut errors = vec![];
        if let Clause::Id(id) = self {
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
        match self {
            Clause::Query(q) => self.to_string(),
            _ => format!("{table_name}{self}"),
        }
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let clause = match self {
            Clause::Empty => "".into(),
            Clause::Where(filter) => {
                format!("[WHERE {filter}]")
            }
            Clause::Id(surreal_id) => {
                // The Table name component of the Id comes from the macro. e.g For student:5, the Schema which this is wrapped into provide. So all we need here is the id component, student
                format!(":{}", surreal_id.id)
            }
            Clause::Query(select_statement) => {
                format!("({})", select_statement.to_string().trim_end_matches(";"))
            }
            Clause::All => format!("[*]"),
            Clause::Last => format!("[$]"),
            Clause::Index(i) => format!("[{i}]"),
        };

        write!(f, "{}", clause)
    }
}

impl From<SurrealId> for Clause {
    fn from(value: SurrealId) -> Self {
        Self::Id(value)
    }
}

impl From<&SurrealId> for Clause {
    fn from(value: &SurrealId) -> Self {
        Self::Id(value.to_owned())
    }
}

impl From<Field> for Clause {
    fn from(value: Field) -> Self {
        Self::Where(Filter::new(value))
    }
}

impl From<&Field> for Clause {
    fn from(value: &Field) -> Self {
        Self::Where(Filter::new(value.clone()))
    }
}

impl From<Filter> for Clause {
    fn from(value: Filter) -> Self {
        Self::Where(value)
    }
}

impl From<&Filter> for Clause {
    fn from(value: &Filter) -> Self {
        Self::Where(value.to_owned())
    }
}

impl From<Empty> for Clause {
    fn from(value: Empty) -> Self {
        Self::Empty
    }
}

impl From<SelectStatement> for Clause {
    fn from(value: SelectStatement) -> Self {
        Self::Query(value.into())
    }
}

impl From<&SelectStatement> for Clause {
    fn from(value: &SelectStatement) -> Self {
        Self::Query(value.to_owned().into())
    }
}

pub struct Empty;

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
        Self::All
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

pub struct Index(u128);

pub fn index(index: u128) -> Index {
    Index(index)
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_clause() {
        // test empty clause
        let empty_clause = Clause::Empty;
        assert_eq!(format!("{}", empty_clause), "");

        // test where clause
        let filter = cond(Field::new("age").equal(18));
        let where_clause = Clause::Where(filter);
        assert_eq!(
            format!("{}", where_clause),
            "[WHERE age = $_param_00000000]"
        );

        // test id clause
        let id_clause = Clause::Id(SurrealId::try_from("student:5").unwrap());
        assert_eq!(format!("{}", id_clause), ":5");

        // test query clause
        let table = Table::new("students");
        let select_statement = select(All).from(table);
        let query_clause = Clause::Query(select_statement);
        assert_eq!(format!("{}", query_clause), "(SELECT * FROM students)");

        // test all clause
        let all_clause = Clause::All;
        assert_eq!(format!("{}", all_clause), "[*]");

        // test last clause
        let last_clause = Clause::Last;
        assert_eq!(format!("{}", last_clause), "[$]");

        // test index clause
        let index_clause = Clause::Index(42);
        assert_eq!(format!("{}", index_clause), "[42]");
    }
}
