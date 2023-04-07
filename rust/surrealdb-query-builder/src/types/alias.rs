/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{fmt::Display, ops::Deref};

use surrealdb::sql;

use crate::{BindingsList, Buildable, Parametric};

#[derive(Debug, Clone)]
pub struct Alias {
    name: AliasName,
    aliased: sql::Ident,
    bindings: BindingsList,
    graph_string: String,
}

impl Parametric for Alias {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for Alias {
    fn build(&self) -> String {
        self.graph_string.to_string()
    }
}

impl Alias {
    // pub fn new(
    //     name: AliasName,
    //     aliased: sql::Ident,
    //     bindings: BindingsList,
    //     graph_string: String,
    // ) -> Self {
    //     Self {
    //         name,
    //         aliased,
    //         bindings,
    //         graph_string,
    //     }
    // }

    pub fn get_alias_name(self) -> AliasName {
        self.name
    }
}

#[derive(Debug, Clone)]
pub struct AliasName(sql::Ident);

impl AliasName {
    pub fn new(name: impl Into<sql::Ident>) -> Self {
        Self(name.into())
    }
}

impl Deref for AliasName {
    type Target = sql::Ident;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for AliasName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Aliasable
where
    Self: Parametric + Buildable,
{
    fn __as__(&self, alias: impl Into<AliasName>) -> Alias {
        let alias: AliasName = alias.into();
        let graph_string = format!("{} AS {}", self.build().trim_end_matches(";"), &alias);

        Alias {
            name: alias,
            aliased: self.build().into(),
            bindings: self.get_bindings(),
            graph_string,
        }
    }
}
