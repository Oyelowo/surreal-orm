/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Statement syntax
// DEFINE INDEX @name ON [ TABLE ] @table [ FIELDS | COLUMNS ] @fields [ UNIQUE ]
// Example usage
// Below is an example showing how to create a unique index for the email address field on a user table.

use std::fmt::{self, Display};

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    types::{Field, Table, TableIndex},
};

/// Define a new database index.
/// Just like in other databases, SurrealDB uses indexes to help optimize query performance.
/// An index can consist of one or more fields in a table and can enforce a uniqueness constraint.
/// If you don't intend for your index to have a uniqueness constraint,
/// then the fields you select for your index should have a high degree of cardinality,
/// meaning that there is a high amount of diversity between the data in the indexed table records.
///
/// Requirements
/// You must be authenticated as a root, namespace, or database user before you can use the DEFINE INDEX statement.
/// You must select your namespace and database before you can use the DEFINE INDEX statement.
///
/// Example:
/// ```rust
/// # use surrealdb_query_builder as surrealdb_orm;
/// use surrealdb_orm::{*, CrudType::*, statements::{define_index, for_}};
/// # let alien = Table::from("alien");
/// # let name = Field::new("name");
/// # let age = Field::new("age");
/// # let email = Field::new("email");
/// # let dob = Field::new("dob");
///
/// let query = define_index("alien_index")
///                 .on_table(alien)
///                 .fields(&[age, name, email, dob])
///                 .unique();
///
/// assert_eq!(query.build(),
/// "DEFINE INDEX alien_index ON TABLE alien FIELDS age, name, email, dob UNIQUE;");
/// ```
pub fn define_index(index_name: impl Into<TableIndex>) -> DefineIndexStatement {
    let index_name: TableIndex = index_name.into();
    let index_name: String = index_name.to_string();

    DefineIndexStatement {
        index_name,
        table_name: None,
        fields: vec![],
        columns: vec![],
        unique: None,
    }
}

/// A statement for defining a database Index.
pub struct DefineIndexStatement {
    index_name: String,
    table_name: Option<String>,
    fields: Vec<String>,
    columns: Vec<String>,
    unique: Option<bool>,
}

pub enum Columns {
    Field(Field),
    Fields(Vec<Field>),
}

pub type Fields = Columns;

impl From<Field> for Columns {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

impl<const N: usize> From<&[Field; N]> for Columns {
    fn from(value: &[Field; N]) -> Self {
        Self::Fields(value.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>())
    }
}

impl Parametric for Columns {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Columns::Field(field) => field.get_bindings(),
            Columns::Fields(fields) => fields
                .into_iter()
                .flat_map(|f| f.get_bindings())
                .collect::<Vec<_>>(),
        }
    }
}

impl DefineIndexStatement {
    /// Set the table where the index is defined.
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        let table: Table = table.into();
        self.table_name = Some(table.to_string());
        self
    }

    /// Set the columns on the table where the index should be defined. This is alternative to
    /// fields just like in a relational database
    pub fn columns(mut self, columns: impl Into<Columns>) -> Self {
        let columns: Columns = columns.into();
        let columns = match columns {
            Columns::Field(f) => vec![f],
            Columns::Fields(fs) => fs,
        };
        self.columns
            .extend(columns.into_iter().map(|f| f.build()).collect::<Vec<_>>());
        self
    }

    /// Set the fields on the table where the index should be defined. This is alternative to
    /// columns
    pub fn fields(mut self, fields: impl Into<Fields>) -> Self {
        let fields: Fields = fields.into();
        let fields = match fields {
            Fields::Field(f) => vec![f],
            Fields::Fields(fs) => fs,
        };
        self.fields
            .extend(fields.into_iter().map(|f| f.build()).collect::<Vec<_>>());
        self
    }

    /// Set whether the field should be unique
    pub fn unique(mut self) -> Self {
        self.unique = Some(true);
        self
    }
}

impl Buildable for DefineIndexStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE INDEX {}", self.index_name);

        if let Some(table) = &self.table_name {
            query = format!("{query} ON TABLE {table}");
        }

        if !self.fields.is_empty() {
            let fields_str = self.fields.join(", ");
            query = format!("{query} FIELDS {fields_str}");
        }

        if !self.columns.is_empty() {
            let columns_str = self.columns.join(", ");
            query = format!("{query} COLUMNS {columns_str}");
        }

        if self.unique.unwrap_or(false) {
            query = format!("{query} UNIQUE");
        }
        query += ";";
        query
    }
}

impl Display for DefineIndexStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineIndexStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Queryable for DefineIndexStatement {}
impl Erroneous for DefineIndexStatement {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_index_statement_single_field() {
        let email = Field::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .fields(email)
            .unique();

        assert_eq!(
            query.build(),
            "DEFINE INDEX userEmailIndex ON TABLE user FIELDS email UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 0);
    }

    #[test]
    fn test_define_index_statement_single_column() {
        let email = Field::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .columns(email)
            .unique();

        assert_eq!(
            query.build(),
            "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 0);
    }

    #[test]
    fn test_define_index_statement_multiple_fields() {
        let age = Field::new("age");
        let name = Field::new("name");
        let email = Field::new("email");
        let dob = Field::new("dob");

        let query = define_index("alien_index")
            .on_table("alien")
            .fields(&[age, name, email, dob])
            .unique();

        assert_eq!(
            query.build(),
            "DEFINE INDEX alien_index ON TABLE alien FIELDS age, name, email, dob UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 0);
    }

    #[test]
    fn test_define_index_statement_multiple_columns() {
        let age = Field::new("age");
        let name = Field::new("name");
        let email = Field::new("email");
        let dob = Field::new("dob");

        let query = define_index("alien_index")
            .on_table("alien")
            .columns(&[age, name, email, dob])
            .unique();

        assert_eq!(
            query.build(),
            "DEFINE INDEX alien_index ON TABLE alien COLUMNS age, name, email, dob UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 0);
    }
}
