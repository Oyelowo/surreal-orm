/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable, SurrealdbModel},
    types::{DurationLike, Filter, ReturnType},
    Binding, Conditional, ErrorList, ReturnableDefault, ReturnableStandard, ToRaw,
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

/// Creates a new DELETE statement.
/// The DELETE statement can be used to delete records from the database.
///
/// # Argument: `table or id`
///
/// # Examples
///
/// ```rust, ignore
/// delete(user)
/// .where_(age.less_than(18)); // simple filtering on a field
///
/// delete(user)
/// .where_(cond(age.greater_than(18)) // Or more complex filtering with `cond` helper
///         .and(age.less_than(80)));
/// ```
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

/// Define the API for delete Statement
#[derive(Debug, Clone)]
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
    /// Adds a condition to the delete statement.
    ///
    /// # Arguments
    ///
    /// * `condition` - conditional statement.
    ///
    /// Examples:
    ///
    /// ```rust, ignore
    /// # delete(user)
    /// // simple filtering on a field
    /// .where_(age.less_than(18));
    ///
    /// // Or more complex filtering with `cond` helper
    /// # delete(user)
    /// .where_(cond(age.greater_than(18))
    ///         .and(age.less_than(80)));
    /// ```
    pub fn where_(mut self, condition: impl Conditional + Clone) -> Self {
        self.update_bindings(condition.get_bindings());
        let condition = Filter::new(condition);
        self.where_ = Some(condition.build());
        self
    }

    fn update_bindings(&mut self, bindings: BindingsList) -> &mut Self {
        self.bindings.extend(bindings);
        self
    }

    /// Sets the return type for the query.
    ///
    /// # Arguments
    ///
    /// * `return_type` - The type of return to set.
    ///
    /// # Examples
    ///
    /// Set the return type to `None`:
    ///
    /// ```rust,ignore
    /// query.return_type(ReturnType::None);
    /// ```
    ///
    /// Set the return type to `Before`:
    ///
    /// ```rust,ignore
    /// query.return_type(ReturnType::Before);
    /// ```
    ///
    /// Set the return type to `After`:
    ///
    /// ```rust,ignore
    /// query.return_type(ReturnType::After);
    /// ```
    ///
    /// Set the return type to `Diff`:
    ///
    /// ```rust,ignore
    /// query.return_type(ReturnType::Diff);
    /// ```
    ///
    /// Set the return type to a projection of specific fields:
    ///
    /// ```rust,ignore
    /// query.return_type(ReturnType::Projections(vec![...]));
    /// ```
    pub fn return_type(mut self, return_type: impl Into<ReturnType>) -> Self {
        let return_type = return_type.into();
        self.return_type = Some(return_type);
        self
    }

    /// Sets the timeout duration for the query.
    ///
    /// # Arguments
    ///
    /// * `duration` - a value that can represent a duration for the timeout. This can be one of the following:
    ///
    ///   * `Duration` - a standard Rust `Duration` value.
    ///
    ///   * `Field` - table field.
    ///
    ///   * `Param` - a named parameter in the query, represented by a `Param` value.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let query = query.timeout(Duration::from_secs(30));
    ///
    /// assert_eq!(query.to_raw().to_string(), "30s");
    /// ```
    pub fn timeout(mut self, duration: impl Into<DurationLike>) -> Self {
        let duration: DurationLike = duration.into();
        self.timeout = Some(duration.to_raw().build());
        self
    }

    /// Indicates that the query should be executed in parallel.
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
        let mut query = format!("DELETE {}", self.target);

        if let Some(condition) = &self.where_ {
            query = format!("{query} WHERE {}", condition);
        }

        if let Some(return_type) = &self.return_type {
            query = format!("{query} {return_type}");
        }

        if let Some(timeout) = &self.timeout {
            query = format!("{query} TIMEOUT {timeout}");
        }

        if self.parallel {
            query = format!("{query} PARALLEL");
        }

        format!("{query};")
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
fn test_query_builder() {}
