/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::{fmt::Display, ops::Deref};

use surrealdb::sql;

use crate::{BindingsList, Buildable, Erroneous, ErrorList, Parametric};

/// An alias for a table or a field/column or even a statement. You usually do not instantiate this
/// yourself.
#[derive(Debug, Clone)]
pub struct Alias {
    name: AliasName,
    bindings: BindingsList,
    errors: ErrorList,
    graph_string: String,
}

impl Erroneous for Alias {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
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
    /// Get the alias name.
    pub fn get_alias_name(self) -> AliasName {
        self.name
    }
}

/// The name of an alias.
#[derive(Debug, Clone)]
pub struct AliasName(sql::Ident);

impl From<&str> for AliasName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl AliasName {
    /// Create a new alias name.
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

/// A trait for aliasing a statement or a field/column.
pub trait Aliasable
where
    Self: Parametric + Buildable + Erroneous,
{
    /// Alias the current statement or field/column.
    ///
    /// # Arguments
    /// * `alias` - The alias name for the field/column or statement.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::*};
    /// # let name =  Field::new("name");
    /// let moniker =  AliasName::new("moniker");
    /// assert_eq!(name.__as__(moniker).build(), "name AS moniker");
    ///    
    /// # let name =  Field::new("name");
    /// assert_eq!(name.__as__("moniker").build(), "name AS moniker");
    ///
    /// # let users =  Table::new("users");
    /// # let selected_users =  AliasName::new("selected_users");
    /// let select = select(All).from(users).__as__(selected_users);
    /// assert_eq!(
    ///    select.build(),
    ///    "(SELECT * FROM users) AS selected_users"
    ///    );
    /// ```
    fn __as__(&self, alias: impl Into<AliasName>) -> Alias {
        let alias: AliasName = alias.into();
        let graph_string = format!("{} AS {}", self.build_aliasable(), &alias);

        Alias {
            name: alias,
            bindings: self.get_bindings(),
            errors: self.get_errors(),
            graph_string,
        }
    }

    /// builds current statement or field/column. This is useful for
    /// modifying the original build mostly slightly. e.g we can wrap select
    /// statement and trim the end semi-column.
    fn build_aliasable(&self) -> String {
        self.build()
    }
}
