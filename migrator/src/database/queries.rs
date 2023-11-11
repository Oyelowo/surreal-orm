/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};
use surreal_query_builder::{
    statements::{
        remove_analyzer, remove_database, remove_event, remove_field, remove_function,
        remove_index, remove_model, remove_namespace, remove_param, remove_scope, remove_table,
        remove_token, remove_user,
    },
    *,
};

use crate::*;

#[derive(Debug, Clone)]
pub enum QueryType {
    Define(DefineStatementRaw),
    Remove(RemoveStatementRaw),
    Update(UpdateStatementRaw),
    NewLine,
}

impl Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = match self {
            QueryType::Define(def) => def.to_string(),
            QueryType::Remove(rem) => rem.to_string(),
            QueryType::Update(upd) => upd.to_string(),
            // TODO: Rethink new line handling
            QueryType::NewLine => "\n".to_string(),
        };
        let end = if let QueryType::NewLine = self {
            ""
        } else {
            ";"
        };
        write!(f, "{}{end}", query.trim_end_matches(';'))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Queries {
    pub(crate) up: Vec<QueryType>,
    pub(crate) down: Vec<QueryType>,
}

impl Queries {
    pub(crate) fn add_new_line_to_up(&mut self) {
        self.up.push(QueryType::NewLine);
    }

    pub(crate) fn add_new_line_to_down(&mut self) {
        self.down.push(QueryType::NewLine);
    }

    pub(crate) fn add_up(&mut self, query: QueryType) {
        self.up.push(query);
    }

    pub(crate) fn add_down(&mut self, query: QueryType) {
        self.down.push(query);
    }

    pub(crate) fn extend_up(&mut self, queries: &Self) {
        self.up.extend(queries.up.to_vec());
    }

    pub(crate) fn extend_down(&mut self, queries: &Self) {
        self.down.extend(queries.down.to_vec());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateStatementRaw(String);

impl Deref for UpdateStatementRaw {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for UpdateStatementRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for UpdateStatementRaw {
    fn from(value: T) -> Self {
        let str: String = value.into();
        Self(str)
    }
}

#[derive(Debug, Clone)]
pub struct RemoveStatementRaw(String);
impl Display for RemoveStatementRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct DefineStmtName(String);

impl Display for DefineStmtName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for DefineStmtName {
    fn from(value: T) -> Self {
        let str: String = value.into();
        Self(str)
    }
}

pub struct RemoveStmtName(String);

impl<T: Into<String>> From<T> for RemoveStmtName {
    fn from(value: T) -> Self {
        let str: String = value.into();
        Self(str)
    }
}

impl Deref for RemoveStmtName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for RemoveStatementRaw {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DefineStatementRaw {
    statement: String,
    fallback_resource_name: Option<String>,
}

impl From<DefineStatementRaw> for Raw {
    fn from(value: DefineStatementRaw) -> Self {
        Self::new(value.statement)
    }
}

impl DefineStatementRaw {
    /// Table name is only required/necessary when generating table resources such as fields, indexes, events
    pub fn as_remove_statement(&self) -> MigrationResult<RemoveStatementRaw> {
        use surreal_query_builder::sql::{statements::DefineStatement, Base, Statement};

        let query = surreal_query_builder::sql::parse(&self.to_string()).expect("Invalid statment");
        let stmt = query[0].clone();
        let get_resource_name = |name: String| self.fallback_resource_name.clone().unwrap_or(name);

        let stmt = match stmt {
            Statement::Define(define_stmt) => match define_stmt {
                DefineStatement::Namespace(ns) => {
                    remove_namespace(get_resource_name(ns.name.to_string()))
                        .to_raw()
                        .build()
                }
                DefineStatement::Database(db) => {
                    remove_database(get_resource_name(db.name.to_string()))
                        .to_raw()
                        .build()
                }
                DefineStatement::Function(fn_) => {
                    remove_function(get_resource_name(fn_.name.to_raw()))
                        .to_raw()
                        .build()
                }
                DefineStatement::Analyzer(analyzer) => {
                    remove_analyzer(get_resource_name(analyzer.name.to_raw()))
                        .to_raw()
                        .build()
                }
                DefineStatement::Token(tk) => {
                    let remove_init = remove_token(get_resource_name(tk.name.to_raw()));
                    let remove_stmt = match tk.base {
                        Base::Ns => remove_init.on_namespace(),
                        Base::Db => remove_init.on_database(),
                        Base::Root => remove_init.on_database(),
                        Base::Sc(sc_name) => remove_init.on_scope(sc_name.to_raw()),
                    };
                    remove_stmt.to_raw().build()
                }
                DefineStatement::Scope(sc) => remove_scope(get_resource_name(sc.name.to_raw()))
                    .to_raw()
                    .build(),
                DefineStatement::Param(param) => {
                    remove_param(get_resource_name(param.name.to_raw()))
                        .to_raw()
                        .build()
                }
                DefineStatement::Table(table) => {
                    remove_table(get_resource_name(table.name.to_raw()))
                        .to_raw()
                        .build()
                }
                DefineStatement::Event(ev) => remove_event(get_resource_name(ev.name.to_raw()))
                    .on_table(ev.what.to_raw())
                    .to_raw()
                    .build(),
                DefineStatement::Field(field) => {
                    remove_field(get_resource_name(field.name.to_string()))
                        .on_table(field.what.to_raw())
                        .to_raw()
                        .build()
                }
                DefineStatement::Index(index) => {
                    remove_index(get_resource_name(index.name.to_string()))
                        .on_table(index.what.to_raw())
                        .to_raw()
                        .build()
                }
                DefineStatement::User(user) => {
                    let remove_init = remove_user(get_resource_name(user.name.to_raw()));
                    let remove_stmt = match user.base {
                        Base::Ns => remove_init.on_namespace(),
                        Base::Db => remove_init.on_database(),
                        Base::Root => remove_init.on_database(),
                        Base::Sc(_sc_name) => {
                            return Err(MigrationError::InvalidDefineStatement(
                                "Users cannot be defined on scope in Define User statement".into(),
                            ))
                        }
                    };
                    remove_stmt.to_raw().build()
                }
                DefineStatement::MlModel(ml) => remove_model(get_resource_name(ml.name.to_raw()))
                    .version(ml.version)
                    .to_raw()
                    .build(),
            },
            _ => {
                return Err(MigrationError::InvalidDefineStatement(
                    "Not a define statement. Expected a define statement".into(),
                ))
            }
        };

        Ok(stmt.into())
    }

    pub fn with_override_resource_name(&mut self, name: String) -> &mut Self {
        self.fallback_resource_name = Some(name);
        self
    }

    pub fn trim(&self) -> &str {
        self.statement.trim()
    }
}

impl Display for DefineStatementRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.statement)
    }
}
