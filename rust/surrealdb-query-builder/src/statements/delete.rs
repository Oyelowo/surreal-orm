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
    Binding, ErrorList, ReturnableDefault, ReturnableStandard,
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
    let table_name = T::table_name();
    let targettables: TargettablesForUpdate = targettables.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    let param = match targettables {
        TargettablesForUpdate::Table(table) => {
            let table = table.to_string();
            if &table != &table_name.to_string() {
                errors.push(format!(
                    "table name -{table} does not match the surreal model struct type which belongs to {table_name} table"
                ));
            }
            table
        }
        TargettablesForUpdate::SurrealId(id) => {
            if !id
                .to_string()
                .starts_with(format!("{table_name}:").as_str())
            {
                errors.push(format!(
                    "id - {id} does not belong to {table_name} table from the surreal model struct provided"
                ));
            }
            let binding = Binding::new(id);
            let param = binding.get_param_dollarised();
            bindings.push(binding);
            param
        }
    };

    DeleteStatement::<T> {
        target: param,
        where_: None,
        return_type: None,
        timeout: None,
        parallel: false,
        bindings,
        errors,
        __model_return_type: PhantomData,
    }
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
    errors: ErrorList,
    __model_return_type: PhantomData<T>,
}

impl<T> Queryable for DeleteStatement<T> where T: Serialize + DeserializeOwned + SurrealdbModel {}

impl<T> Erroneous for DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T> DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
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

impl<T> ReturnableDefault<T> for DeleteStatement<T> where
    T: Serialize + DeserializeOwned + SurrealdbModel
{
}

impl<T> ReturnableStandard<T> for DeleteStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }
}

#[test]
fn test_query_builder() {
    assert_eq!(2, 2);
}
