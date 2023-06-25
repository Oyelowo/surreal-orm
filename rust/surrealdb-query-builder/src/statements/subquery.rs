/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    Binding, BindingsList, Block, Buildable, Erroneous, ErrorList, Parametric, SurrealdbEdge,
    SurrealdbModel, SurrealdbNode, SurrealdbOrmError, ToRaw, Valuex,
};

use super::{
    CreateStatement, DeleteStatement, IfElseStatement, InsertStatement, RelateStatement,
    ReturnStatement, SelectStatement, UpdateStatement,
};

/// A subquery is a query that is nested inside another query.
#[derive(Debug, Clone)]
pub struct Subquery {
    query_string: String,
    bindings: BindingsList,
    errors: ErrorList,
}

impl Buildable for Subquery {
    fn build(&self) -> String {
        self.query_string.to_owned()
    }
}

impl Parametric for Subquery {
    fn get_bindings(&self) -> Vec<Binding> {
        self.bindings.to_owned()
    }
}

impl Erroneous for Subquery {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_owned()
    }
}

fn statement_str_to_subquery(
    statement: &str,
) -> std::result::Result<sql::Subquery, SurrealdbOrmError> {
    let query = sql::parse(statement).map_err(|err| {
        SurrealdbOrmError::InvalidSubquery(format!("Problem parsing subquery. Error: {err}"))
    })?;
    let parsed_statement = query
        .0
         .0
        .first()
        .ok_or(SurrealdbOrmError::InvalidSubquery(format!(
            "Problem parsing subquery. No statement found."
        )))?;

    let subquery = match parsed_statement {
        sql::Statement::Select(s) => sql::Subquery::Select(s.to_owned()),
        sql::Statement::Ifelse(s) => sql::Subquery::Ifelse(s.to_owned()),
        sql::Statement::Create(s) => sql::Subquery::Create(s.to_owned()),
        sql::Statement::Relate(s) => sql::Subquery::Relate(s.to_owned()),
        sql::Statement::Insert(s) => sql::Subquery::Insert(s.to_owned()),
        sql::Statement::Update(s) => sql::Subquery::Update(s.to_owned()),
        sql::Statement::Output(s) => sql::Subquery::Output(s.to_owned()),
        sql::Statement::Delete(s) => sql::Subquery::Delete(s.to_owned()),
        _ => return Err(SurrealdbOrmError::InvalidSubquery(statement.to_string())),
    };
    Ok(subquery)
}

fn statement_to_subquery(statement: impl Buildable + Erroneous + Parametric) -> Subquery {
    let mut errors = statement.get_errors();
    let binding = statement_str_to_subquery(&statement.to_raw().build())
        .map(|subquery| Binding::new(subquery))
        .map_err(|err| errors.push(err.to_string()))
        .unwrap_or(Binding::new(errors.join(", ")));

    Subquery {
        query_string: binding.get_param_dollarised(),
        bindings: vec![binding],
        errors: statement.get_errors(),
    }
}

impl From<SelectStatement> for Subquery {
    fn from(statement: SelectStatement) -> Self {
        statement_to_subquery(statement)
    }
}

impl From<Block> for Subquery {
    fn from(statement: Block) -> Self {
        Self {
            query_string: statement.build(),
            bindings: statement.get_bindings(),
            errors: statement.get_errors(),
        }
    }
}

impl<T> From<CreateStatement<T>> for Subquery
where
    T: SurrealdbNode + Serialize + DeserializeOwned,
{
    fn from(statement: CreateStatement<T>) -> Self {
        statement_to_subquery(statement)
    }
}

impl<T> From<UpdateStatement<T>> for Subquery
where
    T: SurrealdbModel + Serialize + DeserializeOwned,
{
    fn from(statement: UpdateStatement<T>) -> Self {
        statement_to_subquery(statement)
    }
}

impl<T> From<DeleteStatement<T>> for Subquery
where
    T: SurrealdbModel + Serialize + DeserializeOwned,
{
    fn from(statement: DeleteStatement<T>) -> Self {
        statement_to_subquery(statement)
    }
}

impl<T> From<RelateStatement<T>> for Subquery
where
    T: SurrealdbEdge + Serialize + DeserializeOwned,
{
    fn from(statement: RelateStatement<T>) -> Self {
        statement_to_subquery(statement)
    }
}

impl<T> From<InsertStatement<T>> for Subquery
where
    T: SurrealdbNode + Serialize + DeserializeOwned,
{
    fn from(statement: InsertStatement<T>) -> Self {
        statement_to_subquery(statement)
    }
}

impl From<IfElseStatement> for Subquery {
    fn from(statement: IfElseStatement) -> Self {
        statement_to_subquery(statement)
    }
}

impl From<ReturnStatement> for Subquery {
    fn from(statement: ReturnStatement) -> Self {
        statement_to_subquery(statement)
    }
}

impl From<Valuex> for Subquery {
    fn from(statement: Valuex) -> Self {
        Valuex::from(statement).into()
    }
}
