/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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
use surrealdb::sql;

use crate::{
    traits::{Binding, BindingsList, Buildable, Erroneous, Node, Parametric, Queryable},
    types::Updateables,
    ErrorList, ReturnType, ReturnableDefault, ReturnableStandard,
};

use super::{SelectStatement, Subquery};

/// Insert statement initialization builder
#[derive(Debug, Clone)]
pub struct InsertStatement<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    on_duplicate_key_update: Vec<String>,
    return_type: Option<ReturnType>,
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
///     copy from another table.
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
///         .from(Company::table())
///         .where_(age.greater(18))
/// );
/// ```
pub fn insert<T>(insertables: impl Into<Insertables<T>>) -> InsertStatement<T>
where
    T: Serialize + DeserializeOwned + Node,
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
        Insertables::FromSubQuery(query) => {
            let bindings = query.get_bindings();
            select_query = Some(query.build());
            errors.extend(query.get_errors());
            bindings
        }
    };

    InsertStatement::<T> {
        bindings,
        select_query_string: select_query,
        return_type: None,
        on_duplicate_key_update: vec![],
        field_names,
        errors,
        node_type: PhantomData,
    }
}

impl<T> Queryable for InsertStatement<T> where T: Serialize + DeserializeOwned + Node {}
impl<T> Erroneous for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T> ReturnableDefault<T> for InsertStatement<T> where T: Serialize + DeserializeOwned + Node {}

impl<T> ReturnableStandard<T> for InsertStatement<T>
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

impl<T> Display for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + Node,
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
    T: Serialize + DeserializeOwned + Node,
{
    /// A single surrealdb node
    Node(T),
    /// A list of surrealdb node
    Nodes(Vec<T>),
    /// A select statement
    FromSubQuery(Subquery),
}

impl<T> From<Vec<T>> for Insertables<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn from(value: Vec<T>) -> Self {
        Self::Nodes(value)
    }
}

impl<T> From<T> for Insertables<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn from(value: T) -> Self {
        Self::Node(value)
    }
}

impl<T> From<SelectStatement> for Insertables<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn from(value: SelectStatement) -> Self {
        Self::FromSubQuery(value.into())
    }
}

impl<T> From<&SelectStatement> for Insertables<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn from(value: &SelectStatement) -> Self {
        Self::FromSubQuery(value.to_owned().into())
    }
}

struct NodeBindings {
    field_names: Vec<String>,
    bindings: BindingsList,
    errors: Vec<String>,
}
fn create_bindings_for_node<T>(node: &T) -> NodeBindings
where
    T: Node + DeserializeOwned + Serialize,
{
    let mut errors = vec![];
    let mut serialized_field_names = T::get_serializable_fields();
    serialized_field_names.sort_by_key(|a| a.build());

    let value = sql::to_value(node).ok().map_or_else(
        || {
            errors.push("Unable to convert node to json".to_string());
            sql::Value::Null
        },
        |v| v,
    );

    let (field_names, bindings): (Vec<String>, BindingsList) = serialized_field_names
        .iter()
        .map(|key| {
            let key = &key.build();
            let value = value.pick(&[key.as_str().into()]);

            let binding = if value == sql::Value::None {
                Binding::new(sql::Value::Null).with_name(key.into())
            } else {
                Binding::new(value).with_name(key.into())
            };
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
    T: Serialize + DeserializeOwned + Node,
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
}

impl<T> Buildable for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn build(&self) -> String {
        // if self.bindings.is_empty() {
        //     return "".into();
        // }

        let mut query = format!("INSERT INTO {}", &T::table());

        if let Some(query_select) = &self.select_query_string {
            query = format!("{query} ({})", &query_select.trim_end_matches(';'));
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

        if let Some(return_type) = &self.return_type {
            query = format!("{query} {}", &return_type);
        }

        format!("{query};")
    }
}

impl<T> Parametric for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + Node,
{
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}
