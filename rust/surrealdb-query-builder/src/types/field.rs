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

use crate::traits::{BindingsList, Buildable, Operatable, Operation, Parametric, ToRaw};

use super::{
    binding::{Binding, Parametric},
    Idiomx,
    Operation::{Operatable, Operation},
};

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
}

impl Field {
    fn new(value: impl Into<Idiomx>) -> Operation {
        let value: sql::Idiom = value.into().into();
        let bindings = vec![Binding::new(sql::Value::from(value.clone()))];
        Self {
            name: value,
            bindings,
        }
    }
}

impl Operatable for Field {}

impl Parametric for Field {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for Field {
    fn build(&self) -> String {
        self.name.to_string()
    }
}

impl Parametric for Field {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl From<&Field> for Idiomx {
    fn from(value: &Field) -> Self {
        Self::new(value.name.clone().into())
    }
}

impl From<&mut Field> for sql::Value {
    fn from(value: &mut Field) -> Operation {
        Self::Idiom(value.name.to_string().into())
    }
}

impl Into<sql::Value> for &Field {
    fn into(self) -> Value {
        sql::Value::from(self.name).into()
    }
}

impl Into<sql::Idiom> for Field {
    fn into(self) -> sql::Idiom {
        self.name.into()
    }
}

impl From<Field> for sql::Value {
    fn from(val: Field) -> Operation {
        let idiom = sql::Idiom::from(val.name);
        sql::Value::from(idiom)
    }
}

impl<'a> From<Cow<'a, Self>> for Field {
    fn from(value: Cow<'a, Field>) -> Operation {
        match value {
            Cow::Borrowed(v) => v.clone(),
            Cow::Owned(v) => v,
        }
    }
}
impl<'a> From<&'a Field> for Cow<'a, Field> {
    fn from(value: &'a Field) -> Operation {
        Cow::Borrowed(value)
    }
}

impl From<Field> for Cow<'static, Field> {
    fn from(value: Field) -> Operation {
        Cow::Owned(value)
    }
}

impl From<String> for Field {
    fn from(value: String) -> Operation {
        Self::new(value)
    }
}
impl From<&Self> for Field {
    fn from(value: &Field) -> Operation {
        value.to_owned()
    }
}
impl From<&str> for Field {
    fn from(value: &str) -> Operation {
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
    let xx = Field::new("lowo");
    // let xx = Fielda::new(sql::Idiom::from("lowo".to_string()));
    let mm = xx.equal(34).less_than_or_equal(46);
    assert_eq!(mm.clone().to_raw().to_string(), "lowo = 34 <= 46");
    assert_eq!(mm.build(), "nawa");
}
