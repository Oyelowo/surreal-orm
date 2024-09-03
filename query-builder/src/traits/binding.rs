/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use serde::Serialize;
use surrealdb::sql;

#[doc(hidden)]
#[derive(Debug, Clone, Serialize)]
pub struct Binding {
    param: String,
    value: sql::Value,
    original_inline_name: String,
    raw_string: String,
    description: Option<String>,
}

#[doc(hidden)]
pub type BindingsList = Vec<Binding>;

impl Binding {
    pub fn new(value: impl Into<sql::Value>) -> Self {
        let value = value.into();
        let value_string = format!("{}", &value);
        let param_name = Self::generate_param_name();

        Binding {
            param: param_name.clone(),
            value,
            original_inline_name: param_name.clone(),
            raw_string: value_string,
            description: None,
        }
    }

    // as value as raw
    pub fn as_raw(mut self) -> Self {
        self.raw_string = self.value.to_raw_string();
        self
    }

    fn generate_param_name() -> String {
        let sanitized_uuid = uuid::Uuid::new_v4().simple();
        let mut param = format!("_param_{sanitized_uuid}");

        param.truncate(15);

        param
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
        self.description.as_ref().unwrap_or(&"".into()).to_string()
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

/// Can have parameters which can be bound. Includes the param and corresponding value
pub trait Parametric {
    /// Get the bindings
    fn get_bindings(&self) -> BindingsList;
}
