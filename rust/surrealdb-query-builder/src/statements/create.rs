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
    traits::{
        Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable, ReturnableDefault,
        ReturnableStandard, SurrealdbNode,
    },
    types::{DurationLike, ReturnType},
    ErrorList,
};

/// Creates a new CREATE SQL statement for a given type.
///
/// Sets the content of the record to be created.
///
/// # Arguments
///
/// * `content` - a serializable surrealdb node model. Type must implement SurrealdbModel
/// # Examples
///
/// ```rust, ignore
/// create(User{
///         name: "Oylowo".to_string(),
///         age: 192
///     });
/// ```
pub fn create<T>(content: T) -> CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    let sql_value = sql::json(&serde_json::to_string(&content).unwrap()).unwrap();
    let binding = Binding::new(sql_value);

    CreateStatement::<T> {
        target: T::table_name().to_string(),
        content: binding.get_param_dollarised(),
        return_type: None,
        timeout: None,
        parallel: false,
        bindings: vec![binding],
        errors: vec![],
        __model_return_type: PhantomData,
    }
}

/// Represents a CREATE SQL statement that can be executed. It implements various traits such as
/// `Queryable`, `Buildable`, `Runnable`, and others to support its functionality.
pub struct CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    target: String,
    content: String,
    return_type: Option<ReturnType>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
    errors: ErrorList,
    __model_return_type: PhantomData<T>,
}

impl<T> Queryable for CreateStatement<T> where T: Serialize + DeserializeOwned + SurrealdbNode {}

impl<T> CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
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
    ///   * `Field` - an identifier for a specific field in the query, represented by an `Idiom` value.
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
        let duration: sql::Value = duration.into().into();
        self.timeout = Some(duration.to_string());
        self
    }

    /// Indicates that the query should be executed in parallel.
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}

impl<T> Buildable for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn build(&self) -> String {
        let mut query = String::new();

        query.push_str("CREATE ");
        query.push_str(&self.target);

        if !&self.content.is_empty() {
            query.push_str(" CONTENT ");
            query.push_str(&self.content);
        }

        if let Some(return_type) = &self.return_type {
            query += format!("{return_type}").as_str();
        }

        if let Some(timeout) = &self.timeout {
            query.push_str(" TIMEOUT ");
            query.push_str(&timeout);
        }

        if self.parallel {
            query.push_str(" PARALLEL");
        }

        query.push_str(";");

        query
    }
}

impl<T> std::fmt::Display for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.build()))
    }
}

impl<T> Parametric for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> Erroneous for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T> ReturnableDefault<T> for CreateStatement<T>
where
    Self: Parametric + Buildable,
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
}

impl<T> ReturnableStandard<T> for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }
}
