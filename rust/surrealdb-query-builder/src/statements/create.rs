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
    ErrorList, Setter, ToRaw,
};

#[derive(Debug, Clone)]
pub enum ContentOrSets<T> {
    /// serializable surrealdb node struct used with CONTENT in create statement.
    Content(T),
    /// serializable surrealdb node struct used with SET in create statement.
    Sets(Vec<Setter>),
}

impl<T> From<T> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(content: T) -> Self {
        ContentOrSets::Content(content)
    }
}

impl<T> From<Vec<Setter>> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(sets: Vec<Setter>) -> Self {
        ContentOrSets::Sets(sets)
    }
}

impl<T> From<Setter> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(set: Setter) -> Self {
        ContentOrSets::Sets(vec![set])
    }
}

impl<const N: usize, T> From<[Setter; N]> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(sets: [Setter; N]) -> Self {
        ContentOrSets::Sets(sets.to_vec())
    }
}

impl<T> From<&[Setter]> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(sets: &[Setter]) -> Self {
        ContentOrSets::Sets(sets.to_vec())
    }
}

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
pub fn create<T>(content: impl Into<ContentOrSets<T>>) -> CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    CreateStatement::<T> {
        target: T::table_name().to_string(),
        content: "".to_string(),
        set: vec![],
        return_type: None,
        timeout: None,
        parallel: false,
        bindings: vec![],
        errors: vec![],
        __model_return_type: PhantomData,
    }
}

#[derive(Debug, Clone)]
enum ContentOrSetString {
    Content(String),
    Sets(String),
}

/// Represents a CREATE SQL statement that can be executed. It implements various traits such as
/// `Queryable`, `Buildable`, `Runnable`, and others to support its functionality.
#[derive(Debug, Clone)]
pub struct CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    target: String,
    content: String,
    set: Vec<String>,
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
    /// Sets the content of the record to be created.
    /// When using this, the type can be automatically inferred unlike the `set` method.
    ///
    /// # Arguments
    ///
    /// * `content` - a serializable surrealdb node model.
    /// # Examples
    ///
    /// ```rust, ignore
    /// create().content(User{
    ///         name: "Oylowo".to_string(),
    ///         age: 192
    ///     });
    /// ```
    pub fn content(mut self, content: T) -> CreateStatement<T> {
        let sql_value = sql::json(&serde_json::to_string(&content).unwrap()).unwrap();
        let binding = Binding::new(sql_value);
        self.content = binding.get_param().to_owned();
        self.bindings.push(binding);
        self
    }

    /// Sets the values of the fields to be updated in the record.
    ///
    /// # Arguments
    ///
    /// * `settables` - an instance of `Setter` trait. This can be created using a single
    /// `equal_to` helper method on a field or a list of `equal_to` methods for multiple fields
    ///
    /// # Examples
    ///
    /// Setting single field
    /// ```rust, ignore
    /// assert_eq!(create::<User>().set(name.equal("Oyelowo")).to_raw().build(), "CREATE user SET name='Oyelowo'")
    /// ```
    ///
    /// Setting multiple fields by chaining `set` method
    /// ```rust, ignore
    /// assert_eq!(create::<User>()
    ///             .set(name.equal_to("Oyelowo"))
    ///             .set(age.equal_to(192))
    ///         ).to_raw().build(), "Create user SET name='Oyelowo', age=192")
    /// ```
    ///
    /// Setting multiple fields by using a list of updaters in a single `set` method
    /// ```rust, ignore
    /// assert_eq!(create::<User>()
    ///             .set(vec![
    ///                     name.equal_to("Oyelowo"),
    ///                     age.equal_to(192)
    ///                 ],
    ///         ).to_raw().build(), "CREATE user SET name='Oyelowo', age=192")
    /// ```

    pub fn set(mut self, settables: impl Into<Vec<Setter>>) -> Self {
        let settable: Vec<Setter> = settables.into();

        let (settable, bindings, errors) = settable.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut settable, mut bindings, mut errors), s| {
                settable.push(s.build());
                bindings.extend(s.get_bindings());
                errors.extend(s.get_errors());
                (settable, bindings, errors)
            },
        );

        self.bindings.extend(bindings);
        self.errors.extend(errors);
        self.set.extend(settable);

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
    /// statement.return_type(ReturnType::None);
    /// ```
    ///
    /// Set the return type to `Before`:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::Before);
    /// ```
    ///
    /// Set the return type to `After`:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::After);
    /// ```
    ///
    /// Set the return type to `Diff`:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::Diff);
    /// ```
    ///
    /// Set the return type to a projection of specific fields:
    ///
    /// ```rust,ignore
    /// statement.return_type(ReturnType::Projections(vec![...]));
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

impl<T> Buildable for CreateStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn build(&self) -> String {
        let mut query = format!("CREATE {}", &self.target);

        if !self.content.is_empty() {
            query = format!("{query} CONTENT {content}", content = &self.content);
        } else if !self.set.is_empty() {
            query = format!("{query} SET {set}", set = &self.set.join(", "));
        }

        if let Some(return_type) = &self.return_type {
            query = format!("{query} {}", &return_type);
        }

        if let Some(timeout) = &self.timeout {
            query = format!("{query} TIMEOUT {}", &timeout);
        }

        if self.parallel {
            query = format!("{query} PARALLEL");
        }

        format!("{query};")
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

    fn get_return_type(&self) -> ReturnType {
        self.return_type.clone().unwrap_or(ReturnType::None)
    }
}
