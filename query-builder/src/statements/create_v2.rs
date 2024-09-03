/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    traits::{
        Binding, BindingsList, Buildable, Erroneous, Node, Parametric, Queryable,
        ReturnableDefault, ReturnableStandard,
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
    T: Serialize + DeserializeOwned + Node,
{
    fn from(content: T) -> Self {
        ContentOrSets::Content(content)
    }
}

impl<T> From<Vec<Setter>> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn from(sets: Vec<Setter>) -> Self {
        ContentOrSets::Sets(sets)
    }
}

impl<T> From<Setter> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn from(set: Setter) -> Self {
        ContentOrSets::Sets(vec![set])
    }
}

impl<const N: usize, T> From<[Setter; N]> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn from(sets: [Setter; N]) -> Self {
        ContentOrSets::Sets(sets.to_vec())
    }
}

impl<T> From<&[Setter]> for ContentOrSets<T>
where
    T: Serialize + DeserializeOwned + Node,
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
/// * `content` - a serializable surrealdb node model. Type must implement Model
/// # Examples
///
/// ```rust, ignore
/// create(User{
///         name: "Oylowo".to_string(),
///         age: 192
///     });
/// ```
pub fn create_v2<T>(content: impl Into<ContentOrSets<T>>) -> CreateStatementV2<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    let content: ContentOrSets<T> = content.into();
    let mut errors = vec![];
    let mut bindings = vec![];
    let content_or_set;

    match content {
        ContentOrSets::Content(content) => {
            let sql_value = sql::to_value(&content).unwrap();
            let binding = Binding::new(sql_value);
            content_or_set = ContentOrSetString::Content(binding.get_param_dollarised());
            bindings.push(binding);
        }
        ContentOrSets::Sets(sets) => {
            bindings.extend(sets.get_bindings());
            errors.extend(sets.get_errors());
            content_or_set = ContentOrSetString::Sets(sets.build());
        }
    };

    CreateStatementV2::<T> {
        target: T::table().to_string(),
        content: content_or_set,
        return_type: None,
        timeout: None,
        parallel: false,
        bindings,
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
pub struct CreateStatementV2<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    target: String,
    content: ContentOrSetString,
    return_type: Option<ReturnType>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
    errors: ErrorList,
    __model_return_type: PhantomData<T>,
}

impl<T> Queryable for CreateStatementV2<T> where T: Serialize + DeserializeOwned + Node {}

impl<T> CreateStatementV2<T>
where
    T: Serialize + DeserializeOwned + Node,
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

impl<T> Buildable for CreateStatementV2<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn build(&self) -> String {
        let mut query = format!("CREATE {}", &self.target);

        match &self.content {
            ContentOrSetString::Content(content) => {
                query = format!("{query} CONTENT {content}");
            }
            ContentOrSetString::Sets(sets) => {
                query = format!("{query} SET {sets}");
            }
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

impl<T> std::fmt::Display for CreateStatementV2<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.build()))
    }
}

impl<T> Parametric for CreateStatementV2<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> Erroneous for CreateStatementV2<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T> ReturnableDefault<T> for CreateStatementV2<T>
where
    Self: Parametric + Buildable,
    T: Serialize + DeserializeOwned + Node,
{
}

impl<T> ReturnableStandard<T> for CreateStatementV2<T>
where
    T: Serialize + DeserializeOwned + Node + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.return_type.clone().unwrap_or(ReturnType::None)
    }
}
