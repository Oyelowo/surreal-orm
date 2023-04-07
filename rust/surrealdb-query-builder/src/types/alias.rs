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

#[derive(Debug, Clone)]
pub struct AliasName(sql::Ident);

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

trait Aliasable
where
    Self: Parametric + Buildable,
{
    fn __as__(&self, alias: impl Into<AliasName>) -> Alias {
        let alias: AliasName = alias.into();
        let graph_string = format!("{} AS {}", self.build(), &alias);

        Alias {
            name: alias,
            aliased: self.build().into(),
            bindings: self.get_bindings(),
            graph_string,
        }
    }
}
