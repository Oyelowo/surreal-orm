/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use bigdecimal::BigDecimal;
use serde::{
    de::{value, DeserializeOwned},
    Serialize,
};
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    fmt::{format, Display},
    ops::Deref,
    rc::Rc,
};

use surrealdb::{
    engine::local::Db,
    sql::{self, Number, Value},
};

use crate::traits::{
    Binding, BindingsList, Buildable, Conditional, Erroneous, Operatable, Operation, Parametric,
    ToRaw,
};

use super::Idiomx;

/// Represents a field in the database. This type wraps a `String` and
/// provides a convenient way to refer to a database fields.
///
/// # Examples
///
/// Creating a `Field`:
///
/// ```
/// use crate::query::field::Field;
///
/// let field = Field::new("name");
///
/// assert_eq!(field.to_string(), "name");
/// ```
#[derive(Debug, Clone)]
pub struct Field {
    name: sql::Idiom,
    bindings: BindingsList,
    graph_string: String,
}

impl Field {
    pub fn new(value: impl Into<Idiomx>) -> Self {
        let value: sql::Idiom = value.into().into();
        // let binding = Binding::new(sql::Value::from(value.clone()));
        // let graph_string = format!("{}", &binding.get_param_dollarised());
        // let bindings = vec![binding];
        // TODO: Check if surrealdb drive supports binding field param idiom. IF so, I can just
        // parametize everything. Otherwise, I can leave fields out of parametization
        // Update: This is checked and seems true
        Self {
            name: value.clone(),
            bindings: vec![],
            graph_string: value.to_string(),
        }
    }

    pub fn set_graph_string(mut self, connection_string: String) -> Self {
        self.graph_string = connection_string;
        // self.graph_string.push_str(&self.name.to_string());
        self
    }

    pub fn ____________update_many_bindings<'bi>(
        &self,
        bindings: impl Into<&'bi [Binding]>,
    ) -> Self {
        let bindings: &'bi [Binding] = bindings.into();
        // println!("bindingszz {bindings:?}");
        // updated_params.extend_from_slice(&self.bindings[..]);
        // updated_params.extend_from_slice(&bindings[..]);
        let updated_params = [&self.get_bindings().as_slice(), bindings].concat();
        Self {
            graph_string: self.graph_string.to_string(),
            bindings: updated_params,
            name: self.name.clone(),
        }
    }
}

impl Conditional for Field {}

impl Operatable for Field {}
impl Erroneous for Field {}

impl Parametric for Field {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for Field {
    fn build(&self) -> String {
        self.graph_string.to_string()
    }
}

impl From<&Field> for Idiomx {
    fn from(value: &Field) -> Self {
        Self::new(value.name.clone().into())
    }
}

impl From<&mut Field> for sql::Value {
    fn from(value: &mut Field) -> Self {
        Self::Idiom(value.name.to_string().into())
    }
}

impl Into<sql::Value> for &Field {
    fn into(self) -> Value {
        sql::Value::from(self.name.clone()).into()
    }
}

impl Into<sql::Idiom> for Field {
    fn into(self) -> sql::Idiom {
        self.name.into()
    }
}

impl From<Field> for sql::Value {
    fn from(val: Field) -> Self {
        let idiom = sql::Idiom::from(val.name);
        sql::Value::from(idiom)
    }
}

impl<'a> From<Cow<'a, Self>> for Field {
    fn from(value: Cow<'a, Field>) -> Self {
        match value {
            Cow::Borrowed(v) => v.clone(),
            Cow::Owned(v) => v,
        }
    }
}
impl<'a> From<&'a Field> for Cow<'a, Field> {
    fn from(value: &'a Field) -> Self {
        Cow::Borrowed(value)
    }
}

impl From<Field> for Cow<'static, Field> {
    fn from(value: Field) -> Self {
        Cow::Owned(value)
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
impl From<&Self> for Field {
    fn from(value: &Field) -> Self {
        value.to_owned()
    }
}
impl From<&str> for Field {
    fn from(value: &str) -> Self {
        let value: sql::Idiom = value.to_string().into();
        Self::new(Idiomx::new(value))
    }
}

impl From<Field> for String {
    fn from(value: Field) -> Self {
        value.build()
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            self.build() // self.condition_query_string.trim_start_matches("`")
        ))
    }
}

#[test]
fn test_field() {
    let age = Field::new("age");
    let operation = age.greater_than_or_equal(18).less_than_or_equal(56);

    assert_eq!(
        operation.fine_tune_params(),
        "age >= $_param_00000001 <= $_param_00000002"
    );
    assert_eq!(operation.clone().to_raw().to_string(), "age >= 18 <= 56");
}
