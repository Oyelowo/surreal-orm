/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    collections::HashMap,
    fmt::{format, Display},
    marker::PhantomData,
};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::{
    engine::local::Db,
    method::Query,
    opt::QueryResult,
    sql::{self, Operator},
    Response, Surreal,
};

use crate::{
    traits::{
        Binding, BindingsList, Buildable, Erroneous, Parametric, Queryable, Runnable,
        SurrealdbModel,
    },
    types::{expression::Expression, Updateables},
    RunnableDefault,
};

use super::SelectStatement;

pub struct InsertStatement<T: Serialize + DeserializeOwned + SurrealdbModel> {
    node_type: PhantomData<T>,
    on_duplicate_key_update: Vec<String>,
    bindings: BindingsList,
    // You can select values to copy data from an existing table into a new one
    select_query_string: Option<String>,
}

pub fn insert<T>(insertables: impl Into<Insertables<T>>) -> InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    let mut builder = InsertStatement::<T>::new();
    let insertables: Insertables<T> = insertables.into();
    builder.insert(insertables)
}

impl<T> Queryable for InsertStatement<T> where T: Serialize + DeserializeOwned + SurrealdbModel {}
impl<T> Erroneous for InsertStatement<T> where T: Serialize + DeserializeOwned + SurrealdbModel {}

impl<T> Display for InsertStatement<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

pub enum Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    Node(T),
    Nodes(Vec<T>),
    FromQuery(SelectStatement),
}

impl<T> From<Vec<T>> for Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn from(value: Vec<T>) -> Self {
        Self::Nodes(value)
    }
}

impl<T> From<T> for Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn from(value: T) -> Self {
        Self::Node(value)
    }
}

impl<T> From<SelectStatement> for Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn from(value: SelectStatement) -> Self {
        Self::FromQuery(value)
    }
}

impl<T> From<&SelectStatement> for Insertables<T>
where
    T: Serialize + DeserializeOwned + SurrealdbModel,
{
    fn from(value: &SelectStatement) -> Self {
        Self::FromQuery(value.to_owned())
    }
}

impl<T: SurrealdbModel + DeserializeOwned + Serialize> Parametric for T {
    fn get_bindings(&self) -> BindingsList {
        let value = self;
        // let fields_names = get_field_names(value);
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
}

impl<T: Serialize + DeserializeOwned + SurrealdbModel> Parametric for Insertables<T> {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Insertables::Node(node) => node.get_bindings(),
            Insertables::Nodes(nodes) => nodes
                .into_iter()
                .flat_map(|n| n.get_bindings())
                .collect::<Vec<_>>(),
            Insertables::FromQuery(query) => query.get_bindings(),
        }
    }
}

impl<T: Serialize + DeserializeOwned + SurrealdbModel> InsertStatement<T> {
    pub fn new() -> Self {
        Self {
            on_duplicate_key_update: vec![],
            bindings: vec![],
            node_type: PhantomData,
            select_query_string: None,
        }
    }

    pub fn insert<V: Into<Insertables<T>>>(mut self, value: V) -> Self {
        let value: Insertables<T> = value.into();
        if let Insertables::FromQuery(query_select) = &value {
            self.select_query_string = Some(format!("{query_select}"));
        }

        // I am handling deriving other values params later during actual query building
        // since we can derive that by chunking the bindings by the number of serialized fields
        // which I am able to derive at compile time. Call me zeus Oyelowo! haha!
        // Leaving this here for posteriy
        // let xx = match value {
        //     Insertables::Node(n) => [],
        //     Insertables::Nodes(_) => todo!(),
        //     Insertables::FromQuery(_) => todo!(),
        // };
        let bindings = value.get_bindings();
        self.bindings.extend(bindings);
        self
    }

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

impl<T: Serialize + DeserializeOwned + SurrealdbModel> Buildable for InsertStatement<T> {
    // fn build(&self) -> String {}
    fn build(&self) -> String {
        if self.bindings.is_empty() {
            return "".to_string();
        }

        let bindings = self.bindings.as_slice();
        let field_names = T::get_serializable_field_names();
        // let field_names = bindings
        //     .iter()
        //     .map(|b| b.get_original_name().to_owned())
        //     .collect::<Vec<_>>();

        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(&T::table_name());

        if let Some(query_select) = &self.select_query_string {
            query.push_str(" (");
            query.push_str(&query_select.trim_end_matches(";"));
            query.push_str(")");
        } else {
            query.push_str(" (");
            query.push_str(&field_names.join(", "));
            query.push_str(") ");

            query.push_str("VALUES ");

            let placeholders = self
                .bindings
                .iter()
                .map(|b| format!("${}", b.get_param()))
                .collect::<Vec<_>>()
                .chunks_exact(field_names.len())
                .map(|fields_values_params_list| {
                    format!("({})", fields_values_params_list.join(", "))
                })
                .collect::<Vec<_>>()
                .join(", ");
            // .join(", ");

            // query.push_str(" (");
            query.push_str(&placeholders);
            // query.push_str(") ");
        }

        if !&self.on_duplicate_key_update.is_empty() {
            let updates_str = self.on_duplicate_key_update.join(", ");

            query.push_str(" ON DUPLICATE KEY UPDATE ");
            query.push_str(&updates_str);
        }

        query.push_str(";");
        query
    }
}

impl<T: Serialize + DeserializeOwned + SurrealdbModel> Parametric for InsertStatement<T> {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl<T> RunnableDefault<T> for InsertStatement<T> where
    T: Serialize + DeserializeOwned + SurrealdbModel + Send + Sync
{
}

fn get_field_names<T>(value: &T) -> Vec<String>
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
        .unwrap()
        .as_object()
        .unwrap()
        .keys()
        .map(ToString::to_string)
        .collect()
}

fn get_field_value<T: Serialize>(
    value: &T,
    field_name: &str,
) -> Result<surrealdb::sql::Value, String>
where
    T: serde::Serialize,
{
    let whole_struct = json!(value);
    // TODO: Improve error handling
    Ok(sql::json(&whole_struct[field_name].to_string())?)
}
