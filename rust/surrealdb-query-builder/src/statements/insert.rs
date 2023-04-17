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
use serde_json::json;
use surrealdb::sql;

use crate::{
    traits::{Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable, SurrealdbNode},
    types::Updateables,
    ReturnableDefault,
};

use super::SelectStatement;

/// Insert statement initialization builder
pub struct InsertStatement<T: Serialize + DeserializeOwned + SurrealdbNode> {
    on_duplicate_key_update: Vec<String>,
    // You can select values to copy data from an existing table into a new one
    select_query_string: Option<String>,
    bindings: BindingsList,
    node_type: PhantomData<T>,
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
    let insertables: Insertables<T> = insertables.into();
    let mut select_query = None;

    let bindings = match insertables {
        Insertables::Node(node) => {
            let bindings = create_bindings_for_node(&node);
            bindings
        }
        Insertables::Nodes(nodes) => nodes
            .into_iter()
            .flat_map(|n| create_bindings_for_node(&n))
            .collect::<Vec<_>>(),
        Insertables::FromQuery(query) => {
            let bindings = query.get_bindings();
            select_query = Some(query.build());
            bindings
        }
    };

    InsertStatement::<T> {
        bindings,
        select_query_string: select_query,
        on_duplicate_key_update: vec![],
        node_type: PhantomData,
    }
}

impl<T> Queryable for InsertStatement<T> where T: Serialize + DeserializeOwned + SurrealdbNode {}
impl<T> Erroneous for InsertStatement<T> where T: Serialize + DeserializeOwned + SurrealdbNode {}

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

fn create_bindings_for_node<T: SurrealdbNode + DeserializeOwned + Serialize>(
    node: &T,
) -> BindingsList {
    let value = node;
    let field_names = T::get_serializable_field_names();

    field_names
        .into_iter()
        .map(|field_name| {
            let field_value = get_field_value(value, &field_name)
                .expect("Unable to get value name. This should never happen!");
            Binding::new(field_value).with_name(field_name.into())
        })
        .collect::<Vec<_>>()
}

impl<T: Serialize + DeserializeOwned + SurrealdbNode> InsertStatement<T> {
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

impl<T: Serialize + DeserializeOwned + SurrealdbNode> Buildable for InsertStatement<T> {
    fn build(&self) -> String {
        if self.bindings.is_empty() {
            return "".to_string();
        }

        let field_names = T::get_serializable_field_names();

        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(&T::table_name());

        if let Some(query_select) = &self.select_query_string {
            query = format!("{query} ({})", &query_select.trim_end_matches(";"));
        } else {
            let placeholders = self
                .bindings
                .iter()
                .map(|b| format!("{}", b.get_param_dollarised()))
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

        query.push_str(";");
        query
    }
}

impl<T: Serialize + DeserializeOwned + SurrealdbNode> Parametric for InsertStatement<T> {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

fn get_field_value<T: Serialize>(
    value: &T,
    field_name: &str,
) -> Result<surrealdb::sql::Value, String>
where
    T: serde::Serialize,
{
    let whole_struct = json!(value);
    Ok(sql::json(&whole_struct[field_name].to_string())?)
}
