/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use crate::{
    statements::{for_permission::Permissions, select::SelectStatement},
    BindingsList, Buildable, Erroneous, Parametric, Queryable, Table,
};

// DEFINE TABLE statement
// The DEFINE TABLE statement allows you to declare your table by name, enabling you to apply strict controls to a table's schema by making it SCHEMAFULL, create a foreign table view, and set permissions specifying what operations can be performed on the field.
//
// Requirements
// You must be authenticated as a root, namespace, or database user before you can use the DEFINE TABLE statement.
// You must select your namespace and database before you can use the DEFINE TABLE statement.
// Statement syntax
// DEFINE TABLE @name
// 	[ DROP ]
// 	[ SCHEMAFULL | SCHEMALESS ]
// 	[ AS SELECT @projections
// 		FROM @tables
// 		[ WHERE @condition ]
// 		[ GROUP [ BY ] @groups ]
// 	]
// 	[ PERMISSIONS [ NONE | FULL
// 		| FOR select @expression
// 		| FOR create @expression
// 		| FOR update @expression
// 		| FOR delete @expression
// 	] ]

/// Define the API for the Table builder
pub struct DefineTableStatement {
    table: String,
    drop: Option<bool>,
    flexible: Option<bool>,
    schema_type: Option<SchemaType>,
    as_: Option<String>,
    permissions_none: Option<bool>,
    permissions_full: Option<bool>,
    permissions_for: Vec<String>,
    bindings: BindingsList,
}

/// Define a new table.
///
/// The DEFINE TABLE statement allows you to declare your table by name,
/// enabling you to apply strict controls to a table's schema by making it SCHEMAFULL,
/// create a foreign table view, and set permissions specifying what operations can be performed on the field.
///
/// Requirements
/// You must be authenticated as a root, namespace, or database user before you can use the DEFINE TABLE statement.
/// You must select your namespace and database before you can use the DEFINE TABLE statement.
///
/// Examples:
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::crypto, statements::{define_table, for_permission, order, select}};
/// use CrudType::*;
/// use std::time::Duration;
///
/// let name = Field::new("name");
/// let user = Table::from("user");
/// let age = Field::new("age");
/// let country = Field::new("country");
/// let fake_id = TestUser::create_id("oyelowo");
/// let statement = define_table(user)
///     .drop()
///     .as_(
///         select(All)
///             .from(fake_id)
///             .where_(country.is("INDONESIA"))
///             .order_by(order(&age).numeric().desc())
///             .limit(20)
///             .start(5),
///     )
///     .schemafull()
///     .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
///     .permissions(for_permission(&[Create, Delete]).where_(name.is("Oyedayo"))) //Multiple
///     .permissions(&[
///         for_permission(&[Create, Delete]).where_(name.is("Oyedayo")),
///         for_permission(Update).where_(age.less_than_or_equal(130)),
///     ]);
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_table(table: impl Into<Table>) -> DefineTableStatement {
    let table: Table = table.into();
    DefineTableStatement {
        table: table.to_string(),
        drop: None,
        flexible: None,
        schema_type: None,
        as_: None,
        permissions_none: None,
        permissions_full: None,
        permissions_for: vec![],
        bindings: vec![],
    }
}

impl DefineTableStatement {
    /// Drop the table if it exists and create a new one with the same name.
    pub fn drop(mut self) -> Self {
        self.drop = Some(true);
        self
    }

    /// Make table flexible
    pub fn flexible(mut self) -> Self {
        self.flexible = Some(true);
        self
    }

    /// Make table scehmafull
    pub fn schemafull(mut self) -> Self {
        self.schema_type = Some(SchemaType::Schemafull);
        self
    }

    /// Make table scehmaless.
    pub fn schemaless(mut self) -> Self {
        self.schema_type = Some(SchemaType::Schemaless);
        self
    }

    /// Select from existing table.
    ///
    /// Examples:
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::{*, functions::crypto, statements::{define_table, order, select}};
    /// # use CrudType::*;
    /// # use std::time::Duration;
    ///
    /// # let name = Field::new("name");
    /// # let user = Table::from("user");
    /// # let age = Field::new("age");
    /// # let country = Field::new("country");
    /// # let fake_id = TestUser::create_id("oyedayo");
    ///   let statement = define_table(user)
    ///     .as_(
    ///         select(All)
    ///             .from(fake_id)
    ///             .where_(country.is("INDONESIA"))
    ///             .order_by(order(&age).numeric().desc())
    ///             .limit(20)
    ///             .start(5),
    ///     );
    ///
    /// assert!(!statement.build().is_empty());
    /// ```
    pub fn as_(mut self, select_statement: impl Into<SelectStatement>) -> Self {
        let statement: SelectStatement = select_statement.into();
        self.as_ = Some(statement.to_string());
        self.bindings.extend(statement.get_bindings());
        self
    }

    /// Set permission as NONE
    pub fn permissions_none(mut self) -> Self {
        self.permissions_none = Some(true);
        self
    }

    /// Set permission as FULL
    pub fn permissions_full(mut self) -> Self {
        self.permissions_full = Some(true);
        self
    }

    /// set specific permissions for the table with constraint for certain table access types.
    /// Events include type inluding CREATE, UPDATE, SELECT and DELETE.
    /// Additional permission chaining accumulates
    ///
    ///  Examples:
    ///  
    /// ```rust
    ///     # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::{*, CrudType::*, statements::{define_table, for_permission}};
    /// # use CrudType::*;
    /// # use std::time::Duration;
    ///
    /// # let name = Field::new("name");
    /// # let user = Table::from("user");
    /// # let age = Field::new("age");
    ///
    /// # let statement = define_table(user);
    /// // You can create perimssion for a single event
    /// let statement = statement.permissions(for_permission(Select).where_(age.greater_than_or_equal(18)));
    ///
    /// // Even multiple
    /// let statement = statement.permissions(for_permission([Create, Update]).where_(name.is("Oyedayo")));
    ///
    /// // Multiples multples
    /// let statement = statement.permissions(&[
    ///     for_permission([Create, Delete]).where_(name.is("Oyedayo")),
    ///     for_permission(Update).where_(age.less_than_or_equal(130)),
    /// ]);
    ///
    /// ```
    pub fn permissions(mut self, fors: impl Into<Permissions>) -> Self {
        use Permissions::*;
        let fors: Permissions = fors.into();
        match fors {
            For(one) => {
                self.permissions_for.push(one.to_string());
                self.bindings.extend(one.get_bindings());
            }
            Fors(many) => many.iter().for_each(|f| {
                self.permissions_for.push(f.to_string());
                self.bindings.extend(f.get_bindings());
            }),
            RawStatement(raw) => {
                self.permissions_for.push(raw.to_string());
            }
            RawStatementList(raw_list) => {
                self.permissions_for.extend(
                    raw_list
                        .into_iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<_>>(),
                );
            }
        }
        self
    }
}

// Statement syntax
// DEFINE TABLE @name
// 	[ DROP ]
// 	[ SCHEMAFULL | SCHEMALESS ]
// 	[ AS SELECT @projections
// 		FROM @tables
// 		[ WHERE @condition ]
// 		[ GROUP [ BY ] @groups ]
// 	]
// 	[ PERMISSIONS [ NONE | FULL
// 		| FOR select @expression
// 		| FOR create @expression
// 		| FOR update @expression
// 		| FOR delete @expression
// 	] ]
impl Buildable for DefineTableStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE TABLE {}", &self.table);

        if self.drop.unwrap_or_default() {
            query = format!("{query} DROP");
        }

        if self.flexible.unwrap_or_default() {
            query = format!("{query} FLEXIBLE");
        }

        match self.schema_type {
            Some(SchemaType::Schemafull) => {
                query = format!("{query} SCHEMAFULL");
            }
            Some(SchemaType::Schemaless) => {
                query = format!("{query} SCHEMALESS");
            }
            None => {}
        };

        if let Some(select_statement) = &self.as_ {
            query = format!("{query} AS \n\t{}", select_statement.trim_end_matches(';'));
        }

        if let Some(true) = self.permissions_none {
            query = format!("{query} PERMISSIONS NONE");
        } else if let Some(true) = self.permissions_full {
            query = format!("{query} PERMISSIONS FULL");
        } else if !&self.permissions_for.is_empty() {
            query = format!("{query}\nPERMISSIONS\n{}", self.permissions_for.join("\n"));
        }
        query.push(';');

        query
    }
}

impl Display for DefineTableStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Queryable for DefineTableStatement {}
impl Erroneous for DefineTableStatement {}

impl Parametric for DefineTableStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

enum SchemaType {
    Schemafull,
    Schemaless,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        statements::{for_permission, select},
        *,
    };
    use surrealdb::sql;
    use CrudType::*;

    #[test]
    fn test_define_statement_schemaless_permissions_none() {
        let user = Table::from("user");
        let statement = define_table(user).schemaless().permissions_none();

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TABLE user SCHEMALESS PERMISSIONS NONE;"
        );
        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TABLE user SCHEMALESS PERMISSIONS NONE;"
        );
        assert_eq!(statement.get_bindings().len(), 0);
    }

    #[test]
    fn test_define_statement_schemaless() {
        let user = Table::from("user");
        let statement = define_table(user).schemaless().permissions_full();

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TABLE user SCHEMALESS PERMISSIONS FULL;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TABLE user SCHEMALESS PERMISSIONS FULL;"
        );

        assert_eq!(statement.get_bindings().len(), 0);
    }

    #[test]
    fn test_define_statement_multiple() {
        let name = Field::new("name");
        let user_table = Table::from("user");
        let age = Field::new("age");
        let country = Field::new("country");
        let fake_id2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));

        let statement = define_table(user_table)
            .drop()
            .as_(
                select(All)
                    .from(fake_id2)
                    .where_(country.is("INDONESIA"))
                    .order_by(age.numeric().desc())
                    .limit(20)
                    .start(5),
            )
            .schemafull()
            .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
            .permissions(for_permission([Create, Delete]).where_(name.is("Oyedayo"))) //Multiple
            .permissions([
                for_permission([Create, Delete]).where_(name.is("Oyedayo")),
                for_permission(Update).where_(age.less_than_or_equal(130)),
            ]);

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TABLE user DROP SCHEMAFULL AS \n\tSELECT * FROM $_param_00000001 \
                WHERE country IS $_param_00000002 ORDER BY age NUMERIC DESC \
                LIMIT $_param_00000003 START AT $_param_00000004\nPERMISSIONS\n\
                FOR select\n\tWHERE age >= $_param_00000005\nFOR create, delete\n\tWHERE name IS $_param_00000006\n\
                FOR create, delete\n\tWHERE name IS $_param_00000007\nFOR update\n\t\
                WHERE age <= $_param_00000008;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TABLE user DROP SCHEMAFULL AS \n\tSELECT * FROM user:oyedayo \
                WHERE country IS 'INDONESIA' ORDER BY age NUMERIC DESC LIMIT 20 \
                START AT 5\nPERMISSIONS\nFOR select\n\tWHERE age >= 18\n\
                FOR create, delete\n\tWHERE name IS 'Oyedayo'\n\
                FOR create, delete\n\tWHERE name IS 'Oyedayo'\nFOR update\n\tWHERE age <= 130;"
        );
        assert_eq!(statement.get_bindings().len(), 8);
    }
}
