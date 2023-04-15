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
    types::{DurationLike, ReturnType, Updateables},
    ErrorList,
};

use super::update::TargettablesForUpdate;

/// Creates a new CREATE SQL statement for a given type. This function returns a CreateStatement.
///
/// # Arguments
///
/// * `targettables` - a table or surrealdb record id. This can be one of the following:
///
///   * `Table` - a surrealdb table.
///
///   * `SurrealdbId` - a surrealdb id.
///
/// # Panics
///
/// Panics when executed via `run` or `return_many`, `return_many` from generated error if `targettables` argument points to a wrong table.
pub fn create_by_setting<T>(
    targettables: impl Into<TargettablesForUpdate>,
) -> CreateBySettingStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    let table_name = T::table_name();
    let targettables: TargettablesForUpdate = targettables.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    let targets = match targettables {
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

    CreateBySettingStatement::<T> {
        target: targets,
        content: None,
        set: Vec::new(),
        return_type: None,
        timeout: None,
        parallel: false,
        bindings,
        errors,
        __model_return_type: PhantomData,
    }
}

/// Represents a CREATE SQL statement that can be executed. It implements various traits such as
/// `Queryable`, `Buildable`, `Runnable`, and others to support its functionality.
pub struct CreateBySettingStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    target: String,
    content: Option<String>,
    set: Vec<String>,
    return_type: Option<ReturnType>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
    errors: ErrorList,
    __model_return_type: PhantomData<T>,
}

impl<T> Queryable for CreateBySettingStatement<T> where
    T: Serialize + DeserializeOwned + SurrealdbNode
{
}

impl<T> CreateBySettingStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    /// Sets the content of the record to be created.
    ///
    /// # Arguments
    ///
    /// * `content` - a serializable surrealdb node model.
    /// # Examples
    ///
    /// ```rust, ignore
    /// create(user_table).content(User{
    ///         name: "Oylowo".to_string(),
    ///         age: 192
    ///     });
    /// ```
    pub fn content(mut self, content: T) -> Self {
        let sql_value = sql::json(&serde_json::to_string(&content).unwrap()).unwrap();
        let binding = Binding::new(sql_value);
        self.content = Some(binding.get_param().to_owned());
        self.bindings.push(binding);
        self
    }

    /// Sets the values of the fields to be updated in the record.
    ///
    /// # Arguments
    ///
    /// * `settables` - an instance of `Updateables` trait. This can be created using a single
    /// `updater`helper function or a list of updater functions for multiple fields
    ///
    /// # Examples
    ///
    /// Setting single field
    /// ```rust, ignore
    /// assert_eq!(query.set(updater(name).equal("Oyelowo")).to_raw().build(), "SET name='Oyelowo'")
    /// ```
    ///
    /// Setting multiple fields by chaining `set` method
    /// ```rust, ignore
    /// assert_eq!(query.
    ///             set(updater(name).equal("Oyelowo"))
    ///             set(updater(age).equal(192))
    ///         ).to_raw().build(), "SET name='Oyelowo', age=192")
    /// ```
    ///
    /// Setting multiple fields by using a list of updaters in a single `set` method
    /// ```rust, ignore
    /// assert_eq!(query.set(vec![
    ///             updater(name).equal("Oyelowo"),
    ///             updater(age).equal(192)],
    ///         ).to_raw().build(), "SET name='Oyelowo', age=192")
    /// ```
    pub fn set(mut self, settables: impl Into<Updateables>) -> Self {
        let settable: Updateables = settables.into();
        self.bindings.extend(settable.get_bindings());

        let setter_query = match settable {
            Updateables::Updater(up) => vec![up.build()],
            Updateables::Updaters(ups) => ups.into_iter().map(|u| u.build()).collect::<Vec<_>>(),
        };
        self.set.extend(setter_query);
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
    ///   * `Field` - an identifier for a specific field in the query, represented by an `Idiom` value.
    ///
    ///   * `Param` - a named parameter in the query, represented by a `Param` value.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let query = query.timeout(Duration::from_secs(30));
    ///
    /// assert_eq!(query.to_raw().to_string(), "30000".to_string());
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

impl<T> Buildable for CreateBySettingStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn build(&self) -> String {
        let mut query = String::new();

        query.push_str("CREATE ");
        query.push_str(&self.target);

        if let Some(content) = &self.content {
            query.push_str(" CONTENT ");
            query.push_str(&content);
        } else if !&self.set.is_empty() {
            query += "SET ";
            let set_vec = self.set.join(", ");
            query += &set_vec;
            query += " ";
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

impl<T> std::fmt::Display for CreateBySettingStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.build()))
    }
}

impl<T> Parametric for CreateBySettingStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> Erroneous for CreateBySettingStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T> ReturnableDefault<T> for CreateBySettingStatement<T>
where
    Self: Parametric + Buildable,
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
}

impl<T> ReturnableStandard<T> for CreateBySettingStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }
}
