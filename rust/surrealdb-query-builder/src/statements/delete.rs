/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{marker::PhantomData, time::Duration};

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable, SurrealdbModel},
    types::{DurationLike, Filter, ReturnType},
    RunnableStandard,
};

use super::update::TargettablesForUpdate;

/*
Statement syntax
DELETE @targets
    [ WHERE @condition ]
    [ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
    [ TIMEOUT @duration ]
    [ PARALLEL ]
;
*/

pub fn delete<T>(targettables: impl Into<TargettablesForUpdate>) -> DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    // TODO: Pass this to DeleteStatement constructor and gather the errors to be handled when
    // query is run using one of the run methods.
    let table_name = T::table_name();
    let targettables: TargettablesForUpdate = targettables.into();
    if !targettables
        .get_bindings()
        .first()
        .unwrap()
        .get_value()
        .to_raw_string()
        .starts_with(&table_name.to_string())
    {
        panic!("You're trying to update into the wrong table");
    }
    // let errors: String = connection.get_errors();
    DeleteStatement::<T>::new(targettables)
}

#[derive(Debug)]
pub struct DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    target: String,
    where_: Option<String>,
    return_type: Option<ReturnType>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
    __model_return_type: PhantomData<T>,
}

impl<T> Queryable for DeleteStatement<T> where T: Serialize + DeserializeOwned + SurrealdbModel {}
impl<T> Erroneous for DeleteStatement<T> where T: Serialize + DeserializeOwned + SurrealdbModel {}

impl<T> DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    pub fn new(targettables: impl Into<TargettablesForUpdate>) -> Self {
        let targets: TargettablesForUpdate = targettables.into();
        let targets_bindings = targets.get_bindings();

        let mut target_names = match targets {
            TargettablesForUpdate::Table(table) => vec![table.to_string()],
            TargettablesForUpdate::SurrealId(_) => targets_bindings
                .iter()
                .map(|b| format!("${}", b.get_param()))
                .collect::<Vec<_>>(),
        };

        Self {
            target: target_names
                .pop()
                .expect("Table or record id must exist here. this is a bug"),
            where_: None,
            return_type: None,
            timeout: None,
            parallel: false,
            bindings: targets_bindings,
            __model_return_type: PhantomData,
        }
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
    pub fn where_(mut self, condition: impl Into<Filter> + Parametric + Clone) -> Self {
        self.update_bindings(condition.get_bindings());
        let condition: Filter = condition.into();
        self.where_ = Some(condition.to_string());
        self
    }

    fn update_bindings(&mut self, bindings: BindingsList) -> &mut Self {
        self.bindings.extend(bindings);
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
    /// query_builder.timeout();
    /// ```
    pub fn timeout(mut self, duration: impl Into<DurationLike>) -> Self {
        let duration: sql::Value = duration.into().into();
        // let duration = sql::Duration::from(duration);
        self.timeout = Some(duration.to_string());
        self
    }

    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}

impl<T> Buildable for DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn build(&self) -> String {
        let mut query = format!("DELETE {};", self.target);
        if let Some(condition) = &self.where_ {
            query += format!("{} WHERE {};", query, condition).as_str();
        }
        if let Some(return_type) = &self.return_type {
            query += format!("{return_type}").as_str();
        }
        if let Some(timeout) = &self.timeout {
            query.push_str(" TIMEOUT ");
            query.push_str(timeout);
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }
        query
    }
}

impl<T> std::fmt::Display for DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.build()))
    }
}

impl<T> Parametric for DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> RunnableStandard<T> for DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel + Send + Sync,
{
    fn set_return_type(&self, return_type: ReturnType) {
        self.return_type = Some(return_type);
    }
}

#[test]
fn test_query_builder() {
    assert_eq!(2, 2);
}
