/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Statement syntax
// INSERT [ IGNORE ] INTO @what
// 	[ @value
// 	  | (@fields) VALUES (@values)
// 		[ ON DUPLICATE KEY UPDATE @field = @value ... ]
// 	]
// ;
use std::{fmt::Display, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    traits::{Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable, SurrealdbNode},
    types::Updateables,
    ErrorList, ReturnableDefault,
};

use super::SelectStatement;

/// Insert statement initialization builder
pub struct InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    on_duplicate_key_update: Vec<String>,
    // You can select values to copy data from an existing table into a new one
    select_query_string: Option<String>,
    bindings: BindingsList,
    field_names: Vec<String>,
    node_type: PhantomData<T>,
    errors: ErrorList,
}

/// Creates a new INSERT SQL statement for a given type.
///
/// The INSERT statement can be used to insert or update data into the database, using the same statement syntax as the traditional SQL Insert statement.
///
/// # Arguments
///
/// * `insertables` - a single or list of serializable surrealdb nodes or a select statement to
/// copy from another table.
/// # Examples
///
/// ```rust, ignore
/// // You can insert a single object
/// insert(User{
///         name: "Oyelowo".to_string(),
///         age: 192
///     });
///
/// // You can also insert a list of object
/// insert(vec![
///     User{
///         name: "Oyelowo".to_string(),
///         age: 192
///     },
///     User{
///         name: "Oyedayo".to_string(),
///         age: 192
///     },
/// ]);
///     
/// // You can also insert from another table. This is good for copying into a new table:
/// insert(select(All)
///         .from(Company::table_name())
///         .where_(age.greater(18))
/// );
/// ```
pub fn insert<T>(insertables: impl Into<Insertables<T>>) -> InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    let mut field_names = vec![];
    let mut errors = vec![];
    let insertables: Insertables<T> = insertables.into();
    let mut select_query = None;

    let bindings = match insertables {
        Insertables::Node(node) => {
            let node_bindings = create_bindings_for_node(&node);
            field_names = node_bindings.field_names;
            errors = node_bindings.errors;
            node_bindings.bindings
        }
        Insertables::Nodes(nodes) => nodes
            .into_iter()
            .flat_map(|n| {
                let node_bindings = create_bindings_for_node(&n);
                errors.extend(node_bindings.errors);
                field_names = node_bindings.field_names;
                node_bindings.bindings
            })
            .collect::<Vec<_>>(),
        Insertables::FromQuery(query) => {
            let bindings = query.get_bindings();
            select_query = Some(query.build());
            errors.extend(query.get_errors());
            bindings
        }
    };

    InsertStatement::<T> {
        bindings,
        select_query_string: select_query,
        on_duplicate_key_update: vec![],
        field_names,
        errors,
        node_type: PhantomData,
    }
}

impl<T> Queryable for InsertStatement<T> where T: Serialize + DeserializeOwned + SurrealdbNode {}
impl<T> Erroneous for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T> ReturnableDefault<T> for InsertStatement<T> where
    T: Serialize + DeserializeOwned + SurrealdbNode
{
}

impl<T> Display for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// Things that can be inserted including a single or list of surrealdb nodes or a select statement
/// when copying from a table to another table.
#[derive(Debug)]
pub enum Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    /// A single surrealdb node
    Node(T),
    /// A list of surrealdb node
    Nodes(Vec<T>),
    /// A select statement
    FromQuery(SelectStatement),
}

impl<T> From<Vec<T>> for Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(value: Vec<T>) -> Self {
        Self::Nodes(value)
    }
}

impl<T> From<T> for Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(value: T) -> Self {
        Self::Node(value)
    }
}

impl<T> From<SelectStatement> for Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(value: SelectStatement) -> Self {
        Self::FromQuery(value)
    }
}

impl<T> From<&SelectStatement> for Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn from(value: &SelectStatement) -> Self {
        Self::FromQuery(value.to_owned())
    }
}

struct NodeBindings {
    field_names: Vec<String>,
    bindings: BindingsList,
    errors: Vec<String>,
}
fn create_bindings_for_node<T>(node: &T) -> NodeBindings
where
    T: SurrealdbNode + DeserializeOwned + Serialize,
{
    let mut errors = vec![];
    let value = serde_json::to_value(node).ok().map_or_else(
        || {
            errors.push("Unable to convert node to json".to_string());
            serde_json::Value::Null
        },
        |v| v,
    );
    let object = value.as_object().map_or_else(
        || {
            errors.push("Unable to convert node to json object".to_string());
            serde_json::Map::new()
        },
        |v| v.to_owned(),
    );

    let (field_names, bindings): (Vec<String>, BindingsList) = object
        .iter()
        .map(|(key, value1)| {
            let value = value1.as_str().map_or_else(
                || {
                    errors.push(format!("Unable to convert value to string for key {}", key));
                    ""
                },
                |v| v,
            );

            let binding = Binding::new(value).with_name(key.into());
            (key.to_string(), binding)
        })
        .unzip();

    NodeBindings {
        field_names,
        bindings,
        errors,
    }
}

impl<T> InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    /// Generates ON DUPLICATE KEY UPDATE clause.
    /// This updates records which already exist by specifying an ON DUPLICATE KEY UPDATE clause.
    /// This clause also allows incrementing and decrementing numeric values, and adding or removing values from arrays.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// // increment a field number. Generates  +=
    /// updater(score).increment_by(5)
    /// // or alias
    /// updater(score).plus_equal(5)
    ///
    /// // decrement a field number. Generates  -=
    /// updater(score).decrement_by(5)
    /// // or alias
    /// updater(score).minus_equal(5)
    ///
    /// // add to an array. Generates  +=
    /// updater(friends_names).append("Oyelowo")
    /// // or alias
    /// updater(friends_names).plus_equal("Oyelowo")
    ///
    /// // remove value from an array. Generates  -=
    /// updater(friends_names).remove("Oyedayo")
    /// // or alias
    /// updater(friends_names).minus_equal("Oyedayo")
    /// ```
    pub fn on_duplicate_key_update(mut self, updateables: impl Into<Updateables>) -> Self {
        let updates: Updateables = updateables.into();
        self.bindings.extend(updates.get_bindings());
        let updater_query = match updates {
            Updateables::Updater(up) => vec![up.build()],
            Updateables::Updaters(ups) => ups.into_iter().map(|u| u.build()).collect::<Vec<_>>(),
        };
        self.on_duplicate_key_update.extend(updater_query);
        self
    }
}

impl<T> Buildable for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn build(&self) -> String {
        if self.bindings.is_empty() {
            return "".into();
        }

        let mut query = format!("INSERT INTO {}", &T::table_name());

        if let Some(query_select) = &self.select_query_string {
            query = format!("{query} ({})", &query_select.trim_end_matches(";"));
        } else {
            let field_names = self.field_names.clone();

            let placeholders = self
                .bindings
                .iter()
                .map(|b| b.get_param_dollarised())
                .collect::<Vec<_>>()
                .chunks_exact(field_names.len())
                .map(|fields_values_params_list| {
                    format!("({})", fields_values_params_list.join(", "))
                })
                .collect::<Vec<_>>()
                .join(", ");

            let field_names = &field_names.join(", ");
            query = format!("{query} ({field_names}) VALUES {placeholders}",);
        }

        if !&self.on_duplicate_key_update.is_empty() {
            let updates_str = self.on_duplicate_key_update.join(", ");
            query = format!("{query}  ON DUPLICATE KEY UPDATE {updates_str}",);
        }

        format!("{query};")
    }
}

impl<T> Parametric for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbNode,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}
