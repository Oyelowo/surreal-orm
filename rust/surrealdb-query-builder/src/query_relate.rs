/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{fmt::Display, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql::{self, Operator};

use crate::{
    binding::{Binding, BindingsList},
    sql::{Buildable, Duration, Queryable, Return, Runnable, Updateables},
    Clause, Erroneous, ErrorList, Field, Parametric, SurrealdbEdge,
};

// RELATE @from -> @table -> @with
// 	[ CONTENT @value
// 	  | SET @field = @value ...
// 	]
// 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
// 	[ TIMEOUT @duration ]
// 	[ PARALLEL ]
// ;

pub fn relate<T>(connection: impl std::fmt::Display + Parametric + Erroneous) -> RelateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbEdge,
{
    let errors = connection.get_errors();
    RelateStatement {
        relation: connection.to_string(),
        content_param: None,
        set: vec![],
        return_type: None,
        timeout: None,
        parallel: false,
        __return_model_type: PhantomData,
        bindings: connection.get_bindings(),
        errors,
    }
}

pub struct RelateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbEdge,
{
    relation: String,
    content_param: Option<String>,
    set: Vec<String>,
    return_type: Option<Return>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
    errors: ErrorList,
    __return_model_type: PhantomData<T>,
}

impl<T> RelateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbEdge,
{
    pub fn content(mut self, content: T) -> Self {
        let sql_value = sql::json(&serde_json::to_string(&content).unwrap()).unwrap();
        let binding = Binding::new(sql_value);
        self.content_param = Some(binding.get_param_dollarised().to_owned());
        self.bindings.push(binding);
        self
    }

    pub fn set(mut self, settables: impl Into<Updateables>) -> Self {
        let settable: Updateables = settables.into();
        self.bindings.extend(settable.get_bindings());

        let setter_query = match settable {
            Updateables::Updater(up) => vec![up.get_updater_string()],
            Updateables::Updaters(ups) => ups
                .into_iter()
                .map(|u| u.get_updater_string())
                .collect::<Vec<_>>(),
        };
        self.set.extend(setter_query);
        self
    }

    pub fn return_(mut self, return_type: impl Into<Return>) -> Self {
        let return_type = return_type.into();
        self.return_type = Some(return_type);
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
    /// use surrealdb_orm::select;
    ///
    /// let mut query_builder = SelectStatement::new();
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
    /// use surrealdb_orm::select;
    ///
    /// let mut query_builder = SelectStatement::new();
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
    /// use surrealdb_orm::select;
    ///
    /// select(All).parallel();
    /// ```
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}

impl<T> Queryable for RelateStatement<T> where T: Serialize + DeserializeOwned + SurrealdbEdge {}

impl<T> Erroneous for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbEdge,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T> std::fmt::Display for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbEdge,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.build()))
    }
}

impl<T> Parametric for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbEdge,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> Buildable for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbEdge,
{
    fn build(&self) -> String {
        let mut query = String::new();

        if !&self.relation.is_empty() {
            query += &format!("RELATE {} ", self.relation);
        }

        if let Some(param) = &self.content_param {
            query += &format!("CONTENT {} ", param);
        }

        if !&self.set.is_empty() {
            query += "SET ";
            let set_vec = self.set.join(", ");
            query += &set_vec;
            query += " ";
        }

        if let Some(return_type) = &self.return_type {
            query += format!("{return_type}").as_str();
        }

        if let Some(timeout) = &self.timeout {
            query += &format!("TIMEOUT {} ", timeout);
        }

        if self.parallel {
            query += "PARALLEL ";
        }

        query += ";";

        query
    }
}

impl<T> Runnable<T> for RelateStatement<T> where T: Serialize + DeserializeOwned + SurrealdbEdge {}

#[test]
#[cfg(feature = "mock")]
fn test_query_builder() {
    // let query = RelateStatement::new()
    //     // .from("from")
    //     // .table("table")
    //     // .with("with")
    //     // .content("content")
    //     // .set("field1", "value1")
    //     // .set("field2", "value2")
    //     // .return_(Return::Projections(vec!["projection1", "projection2"]))
    //     // .timeout(Duration::from_secs(30))
    //     .parallel()
    //     .build();
    //
    // assert_eq!(
    //     query,
    //     "RELATE from -> table -> with CONTENT content SET field1 = value1, field2 = value2 RETURN projection1, projection2 TIMEOUT 30000 PARALLEL ;"
    // );
}
