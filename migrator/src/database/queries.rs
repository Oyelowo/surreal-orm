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
pub struct DefineStatementRaw(String);

impl From<DefineStatementRaw> for Raw {
    fn from(value: DefineStatementRaw) -> Self {
        Self::new(value.0)
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

impl DefineStatementRaw {
    /// Table name is only required/necessary when generating table resources such as fields, indexes, events
    pub fn as_remove_statement(
        &self,
        remove_stmt_name: RemoveStmtName,
        table: Option<&Table>,
    ) -> MigrationResult<RemoveStatementRaw> {
        use surreal_query_builder::sql::{statements::DefineStatement, Base, Statement};

        let query = surreal_query_builder::sql::parse(&self.to_string()).expect("Invalid statment");
        let stmt = query[0].clone();
        let get_error = |_resource_name: String| {
            // I gave this a  second thought because there is a scenario
            // whereby we could use a different Define statement to generaate
            // a remove statement for another field. The first and only example
            // in mind for now is a rename field case. We could use a new
            // field name define statement to want to create a remove statement for
            // the old field. And since this validation is not totally
            // necessary, I am commenting it out for now.
            // if resource_name != define_statement_name.to_string() {
            //     panic!("Resource name - {} - in define statement does not match name - {} - in removal statement", resource_name, define_statement_name);
            // }
        };

        let stmt = match stmt {
            Statement::Define(define_stmt) => match define_stmt {
                DefineStatement::Namespace(ns) => {
                    get_error(ns.name.to_raw());
                    remove_namespace(remove_stmt_name.to_string())
                        .to_raw()
                        .build()
                }
                DefineStatement::Database(db) => {
                    get_error(db.name.to_raw());
                    remove_database(remove_stmt_name.to_string())
                        .to_raw()
                        .build()
                }
                DefineStatement::Function(fn_) => {
                    get_error(fn_.name.to_raw());
                    remove_function(remove_stmt_name.to_string())
                        .to_raw()
                        .build()
                }
                DefineStatement::Analyzer(analyzer) => {
                    get_error(analyzer.name.to_raw());
                    remove_analyzer(remove_stmt_name.to_string())
                        .to_raw()
                        .build()
                }
                DefineStatement::Token(tk) => {
                    get_error(tk.name.to_raw());

                    let remove_init = remove_token(remove_stmt_name.to_string());
                    let remove_stmt = match tk.base {
                        Base::Ns => remove_init.on_namespace(),
                        Base::Db => remove_init.on_database(),
                        Base::Root => remove_init.on_database(),
                        Base::Sc(sc_name) => remove_init.on_scope(sc_name.to_raw()),
                    };
                    remove_stmt.to_raw().build()
                }
                DefineStatement::Scope(sc) => {
                    get_error(sc.name.to_raw());
                    remove_scope(remove_stmt_name.to_string()).to_raw().build()
                }
                DefineStatement::Param(_) => {
                    get_error(remove_stmt_name.to_string());
                    remove_param(remove_stmt_name.to_string()).to_raw().build()
                }
                DefineStatement::Table(table) => {
                    get_error(table.name.to_raw());
                    remove_table(remove_stmt_name.to_string()).to_raw().build()
                }
                DefineStatement::Event(ev) => {
                    get_error(ev.name.to_raw());
                    remove_event(remove_stmt_name.to_string())
                        .on_table(table.expect("Invalid event. Event must be attached to a table."))
                        .to_raw()
                        .build()
                }
                DefineStatement::Field(field) => {
                    get_error(field.name.to_string());
                    remove_field(remove_stmt_name.to_string())
                        .on_table(table.expect("Invalid field. Field must be attached to a table."))
                        .to_raw()
                        .build()
                }
                DefineStatement::Index(index) => {
                    get_error(index.name.to_string());
                    remove_index(remove_stmt_name.to_string())
                        .on_table(table.expect("Invalid index. Index must be attached to a table."))
                        .to_raw()
                        .build()
                }
                DefineStatement::User(user) => {
                    get_error(user.name.to_raw());
                    let remove_init = remove_user(remove_stmt_name.to_string());
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
                DefineStatement::MlModel(ml) => remove_model(remove_stmt_name.to_string())
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

    pub fn trim(&self) -> &str {
        self.0.trim()
    }
}

impl Display for DefineStatementRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.0)
    }
}
