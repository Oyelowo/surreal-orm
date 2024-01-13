/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use surrealdb::sql;

use crate::{
    BindingsList, Buildable, Clause, Erroneous, Index, Operatable, Parametric, SchemaGetter,
    ValueLike,
};

/// Represents a surrogate parameter
#[derive(Debug, Clone)]
pub struct Param {
    param: sql::Param,
    bindings: BindingsList,
}

impl<T> From<T> for Param
where
    T: Into<sql::Param>,
{
    fn from(value: T) -> Self {
        let param: sql::Param = value.into();
        Self {
            param,
            bindings: vec![],
        }
    }
}

impl Erroneous for Param {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Buildable for Param {
    fn build(&self) -> String {
        format!("${}", self.param.to_raw().trim_start_matches('$'))
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for Param {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Param {
    /// Creates a new instance of `Param`
    pub fn new(param: impl Into<sql::Param>) -> Self {
        let param: sql::Param = param.into();

        Self {
            param,
            bindings: vec![],
        }
    }

    /// For traversing from the param
    pub fn with_path<T: SchemaGetter>(&self, clause: impl Into<Clause>) -> T::Schema {
        let clause: Clause = clause.into();
        let value = ValueLike {
            string: format!("{}{}", self.build(), clause.build()),
            bindings: self
                .get_bindings()
                .into_iter()
                .chain(clause.get_bindings())
                .collect::<Vec<_>>(),
            errors: self.get_errors(),
        };

        T::schema_prefixed(value)
    }

    /// For accessing an object in a list.
    pub fn index(self, index: impl Into<Index>) -> Self {
        let index: Index = index.into();
        let value = ValueLike {
            string: format!("{}{}", self.build(), index.build()),
            bindings: self
                .get_bindings()
                .into_iter()
                .chain(index.get_bindings())
                .collect::<Vec<_>>(),
            errors: self.get_errors(),
        };

        Self {
            param: sql::Param::from(value.build()),
            bindings: self.get_bindings(),
        }
    }
}

impl Operatable for Param {}
