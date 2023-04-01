/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use serde::Serialize;
use surrealdb::sql;

#[derive(Debug, Clone, Serialize)]
pub struct Binding {
    param: String,
    value: sql::Value,
    original_inline_name: String,
    raw_string: String,
    description: Option<String>,
}

pub type BindingsList = Vec<Binding>;

impl Binding {
    pub fn new(value: impl Into<sql::Value>) -> Self {
        let value = value.into();
        let value_string = format!("{}", &value);
        let param_name = generate_param_name(&"param", value.clone());
        Binding {
            param: param_name.clone(),
            value,
            original_inline_name: param_name.clone(),
            raw_string: value_string,
            description: None,
        }
    }

    pub fn with_raw(mut self, raw_string: String) -> Self {
        self.raw_string = raw_string;
        self
    }

    pub fn with_name(mut self, original_name: String) -> Self {
        self.original_inline_name = original_name;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn get_raw_value(&self) -> &String {
        &self.raw_string
    }

    pub fn get_original_name(&self) -> &String {
        &self.original_inline_name
    }

    pub fn get_param(&self) -> &String {
        &self.param
    }

    pub fn get_param_dollarised(&self) -> String {
        format!("${}", &self.param)
    }

    pub fn get_description(&self) -> String {
        format!("{}", self.description.as_ref().unwrap_or(&"".into()))
    }

    pub fn get_value(&self) -> &sql::Value {
        &self.value
    }
}

impl From<sql::Value> for Binding {
    fn from(value: sql::Value) -> Self {
        Self::new(value)
    }
}

// impl From<(String, sql::Value)> for Binding {
//     fn from(value: (String, Value)) -> Self {
//         Self { field1: value }
//     }
// }

/// Can have parameters which can be bound
pub trait Parametric {
    fn get_bindings(&self) -> BindingsList;
}

fn generate_param_name(prefix: &str, value: impl Into<sql::Value>) -> String {
    let sanitized_uuid = uuid::Uuid::new_v4().simple();
    let mut param = format!("_{prefix}_{sanitized_uuid}");

    param.truncate(15);

    param
}
