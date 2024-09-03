/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// Statement syntax
// RELATE @from -> @table -> @with
// 	[ CONTENT @value
// 	  | SET @field = @value ...
// 	]
// 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
// 	[ TIMEOUT @duration ]
// 	[ PARALLEL ]
// ;
use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    derive_binding_and_errors_from_value,
    traits::{BindingsList, Buildable, Edge, Erroneous, ErrorList, Parametric, Queryable},
    types::{DurationLike, ReturnType},
    ReturnableDefault, ReturnableStandard, Setter, ToRaw,
};

/// Creates a new RELATE statement.
///
/// # Arguments
///
/// * `connection` - built using `with` method on a node. e.g `Student::with(..).writes(..).book(..)`
/// # Examples
///
/// ```rust, ignore
/// // Add a graph edge between two specific records setting a field on the edge
/// relate(Student::with(student_id).writes__(Empty).book(book_id))
///     .set(updater(score).equals(5));
///     
/// // Instead of specifying record data using the SET clause,
/// // it is also possible to use the CONTENT keyword to specify the record data using a SurrealQL object.
/// relate(Student::with(student_id).writes__(Empty).book(book_id))
///     .content(write);
/// // Generates e.g RELATE student:1->writes->book:2 CONTENT {...}
///
/// // Add a graph edge between multiple specific students and books
/// relate(
///     Student::with(select(All).from(Student::table()))
///         .writes__(Empty)
///         .book(select(All).from(Book::table()))
/// ).content(write)
/// // Generates e.g RELATE (select * from student)->writes->(select * from book) CONTENT {...}
/// ```
pub fn relate<T>(connection: impl Buildable + Parametric + Erroneous) -> RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge,
{
    let errors = connection.get_errors();
    RelateStatement {
        relation: connection.build(),
        is_only: false,
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

/// Creates a new RELATE ONLY statement.
///
/// # Arguments
///
/// * `connection` - built using `with` method on a node. e.g `Student::with(..).writes(..).book(..)`
/// # Examples
///
/// ```rust, ignore
/// // Add a graph edge between two specific records setting a field on the edge
/// relate_only(Student::with(student_id).writes__(Empty).book(book_id))
///     .set(updater(score).equals(5));
///     
/// // Instead of specifying record data using the SET clause,
/// // it is also possible to use the CONTENT keyword to specify the record data using a SurrealQL object.
/// relate_only(Student::with(student_id).writes__(Empty).book(book_id))
///     .content(write);
/// // Generates e.g RELATE student:1->writes->book:2 CONTENT {...}
///
/// // Add a graph edge between multiple specific students and books
/// relate_only(
///     Student::with(select(All).from(Student::table()))
///         .writes__(Empty)
///         .book(select(All).from(Book::table()))
/// ).content(write)
/// // Generates e.g RELATE (select * from student)->writes->(select * from book) CONTENT {...}
/// ```
pub fn relate_only<T>(connection: impl Buildable + Parametric + Erroneous) -> RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge,
{
    let mut relate_statement = relate(connection);
    relate_statement.is_only = true;
    relate_statement
}

/// Relate statement initialization builder
#[derive(Debug, Clone)]
pub struct RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge,
{
    relation: String,
    is_only: bool,
    content_param: Option<String>,
    set: Vec<String>,
    return_type: Option<ReturnType>,
    timeout: Option<String>,
    parallel: bool,
    bindings: BindingsList,
    errors: ErrorList,
    __return_model_type: PhantomData<T>,
}

impl<T> RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge,
{
    /// Set a serailizable surrealdb edge model. It must implement the Edge trait.
    pub fn content(mut self, content: T) -> Self {
        let (binding, errors) = derive_binding_and_errors_from_value(&content);
        self.content_param = Some(binding.get_param_dollarised().to_owned());
        self.bindings.push(binding);
        self.errors.extend(errors);
        self
    }

    /// This updates records on the edge field.
    /// This clause also allows setting, incrementing and decrementing numeric values, and adding or removing values from arrays.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// // Set fields using a helper macro function:
    /// .set(object_partial!(Weapon {
    ///     id: weapon_id.clone(),
    ///     name: "Laser".to_string()
    /// }))
    ///
    /// // Set multiple fields as an array or vector:
    /// .set([name.equal_to("Laser"), damage.increment_by(100)]);
    ///
    /// // set a single field number. Generates  =
    /// .set(score.equal_to(5))
    ///
    /// // increment a field number. Generates  +=
    /// .set(score.increment_by(5))
    ///
    /// // decrement a field number. Generates  -=
    /// .set(score.decrement_by(5))
    ///
    /// // add to an array. Generates  +=
    /// .set(friends_names.append("Oyelowo"))
    ///
    /// // remove value from an array. Generates  -=
    /// .set(friends_names.remove("Oyedayo"))
    /// ```
    pub fn set(mut self, settables: impl Into<Vec<Setter>>) -> Self {
        let settable: Vec<Setter> = settables.into();

        let (settable, bindings) = settable.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut settable, mut bindings), s| {
                settable.push(s.build());
                bindings.extend(s.get_bindings());
                (settable, bindings)
            },
        );

        self.bindings.extend(bindings);
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

impl<T> Queryable for RelateStatement<T> where T: Serialize + DeserializeOwned + Edge {}

impl<T> Erroneous for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T> std::fmt::Display for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.build()))
    }
}

impl<T> Parametric for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> Buildable for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge,
{
    fn build(&self) -> String {
        let mut query = format!("RELATE {}", self.relation);

        if self.is_only {
            query = format!("{query} ONLY");
        }

        if let Some(param) = &self.content_param {
            query = format!("{query} CONTENT {param} ");
        }

        if !&self.set.is_empty() {
            let set_vec = self.set.join(", ");
            query = format!("{query} SET {set_vec} ");
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

impl<T> ReturnableDefault<T> for RelateStatement<T> where
    T: Serialize + DeserializeOwned + Edge + Send + Sync
{
}

impl<T> ReturnableStandard<T> for RelateStatement<T>
where
    T: Serialize + DeserializeOwned + Edge + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.return_type.clone().unwrap_or(ReturnType::None)
    }
}

#[test]
fn test_query_builder() {}
