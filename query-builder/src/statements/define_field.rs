/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::{self, Display};

use crate::{
    BindingsList, Buildable, Conditional, Erroneous, Field, FieldType, Filter, Parametric,
    Queryable, Table, ValueLike,
};

use super::for_permission::Permissions;

// DEFINE FIELD statement
// The DEFINE FIELD statement allows you to instantiate a named field on a table, enabling you to set the field's data type, set a default value, apply assertions to protect data consistency, and set permissions specifying what operations can be performed on the field.
//
// Requirements
// You must be authenticated as a root, namespace, or database user before you can use the DEFINE FIELD statement.
// You must select your namespace and database before you can use the DEFINE FIELD statement.
// Statement syntax
// DEFINE FIELD @name ON [ TABLE ] @table
// 	[ TYPE @type ]
// 	[ VALUE @expression ]
// 	[ ASSERT @expression ]
// 	[ PERMISSIONS [ NONE | FULL
// 		| FOR select @expression
// 		| FOR create @expression
// 		| FOR update @expression
// 		| FOR delete @expression
// 	] ]

/// A statement for defining a Field.
#[derive(Clone, Debug)]
pub struct DefineFieldStatement {
    field_name: String,
    table: Option<String>,
    type_: Option<String>,
    value: Option<String>,
    assert: Option<String>,
    permissions_none: Option<bool>,
    permissions_full: Option<bool>,
    permissions_for: Vec<String>,
    bindings: BindingsList,
}

/// Define a new field.
/// The DEFINE FIELD statement allows you to instantiate a named field on a table, enabling you to set the field's data type, set a default value, apply assertions to protect data consistency, and set permissions specifying what operations can be performed on the field.
///
/// Requirements
/// You must be authenticated as a root, namespace, or database user before you can use the DEFINE FIELD statement.
/// You must select your namespace and database before you can use the DEFINE FIELD statement.
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, CrudType::*, statements::{define_field, for_permission}};
///
/// # let name = Field::new("name");
/// # let user_table = Table::from("user");
/// # let age = Field::new("age");
/// # let email = Field::new("email");
///
/// let statement = define_field(email)
///     .on_table(user_table)
///     .type_(FieldType::String)
///     .value("example@codebreather.com")
///     .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
///     // Additional permission chaining accumulates
///     .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
///     .permissions(for_permission([Create, Update]).where_(name.is("Oyedayo"))) // Multiple
///     // Multiples multples
///     .permissions([
///         for_permission([Create, Delete]).where_(name.is("Oyedayo")),
///         for_permission(Update).where_(age.less_than_or_equal(130)),
///     ]);
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_field(fieldable: impl Into<Field>) -> DefineFieldStatement {
    let field: Field = fieldable.into();
    DefineFieldStatement {
        field_name: field.to_string(),
        table: None,
        type_: None,
        value: None,
        assert: None,
        permissions_none: None,
        permissions_full: None,
        permissions_for: vec![],
        bindings: vec![],
    }
}

impl DefineFieldStatement {
    /// Set the table where the field is defined.
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        let table: Table = table.into();
        self.table = Some(table.to_string());
        self
    }

    /// Set the data type of the field.
    pub fn type_(mut self, field_type: impl Into<FieldType>) -> Self {
        let field_type: FieldType = field_type.into();
        self.type_ = Some(field_type.to_string());
        self
    }

    /// Set the default value for the field.
    pub fn value(mut self, default_value: impl Into<ValueLike>) -> Self {
        // pub fn value<T, U>(mut self, default_value: U) where T: Deref<Target = U>, T: Into<U>, U: Into<T>, T: Into<ValueLike>, T: Into<sql::Value> -> Self {
        let value: ValueLike = default_value.into();
        self.value = Some(value.build());
        self.bindings.extend(value.get_bindings());
        self
    }

    /// assert constraint on the field.
    ///  
    ///  Examples:
    ///  
    /// ```rust
    ///     # use surreal_query_builder as surreal_orm;
    ///     use surreal_orm::{*, CrudType::*, statements::{define_field}};
    ///
    ///     # let name = Field::new("name");
    ///     # let user_table = Table::from("user");
    ///     # let age = Field::new("age");
    ///     # let email = Field::new("email");
    ///     
    ///     # let statement = define_field(email)
    ///     #    .on_table(user_table)
    ///     #    .type_(FieldType::String)
    ///     #    .value("example@codebreather.com");
    ///     
    /// // For simple single condition
    /// let statement = statement.assert(value().is_not(NONE));
    ///
    /// // For multiple conditions
    /// let statement = statement.assert(cond(value().is_not(NONE))
    ///                                      .and(value().like("is_email"))
    ///                                 );
    pub fn assert(mut self, assertion: impl Conditional) -> Self {
        let assertion = Filter::new(assertion);
        self.bindings.extend(assertion.get_bindings());
        self.assert = Some(assertion.build());
        self
    }

    /// set no permission.
    pub fn permissions_none(mut self) -> Self {
        self.permissions_none = Some(true);
        self
    }

    /// set full permission.
    pub fn permissions_full(mut self) -> Self {
        self.permissions_full = Some(true);
        self
    }

    /// set specific permissions for specific event type inluding CREATE, UPDATE, SELECT and DELETE.
    /// Additional permission chaining accumulates
    ///  Examples:
    ///  
    /// ```rust
    ///     # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, CrudType::*, statements::{define_field, for_permission}};
    ///
    ///     # let name = Field::new("name");
    ///     # let user_table = Table::from("user");
    ///     # let age = Field::new("age");
    ///     # let email = Field::new("email");
    ///     # let statement = define_field(email)
    ///     #    .on_table(user_table)
    ///     #    .type_(FieldType::String)
    ///     #    .value("example@codebreather.com")
    ///     #    .assert(cond(value().is_not(NONE)).and(value().like("is_email")));
    ///
    /// // You can create perimssion for a single event
    /// let statement = statement.permissions(for_permission(Select).where_(age.greater_than_or_equal(18)));
    ///
    /// // Even multiple
    /// let statement = statement.permissions(for_permission([Create, Update]).where_(name.is("Oyedayo")));
    ///
    /// // Multiples multples
    /// let statement = statement.permissions([
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
                self.permissions_for.push(one.build());
                self.bindings.extend(one.get_bindings());
            }
            Fors(many) => many.iter().for_each(|f| {
                self.permissions_for.push(f.build());
                self.bindings.extend(f.get_bindings());
            }),
            RawStatement(raw) => {
                self.permissions_for.push(raw.build());
                self.bindings.extend(raw.get_bindings());
            }
            RawStatementList(raw_list) => {
                self.permissions_for.extend(
                    raw_list
                        .into_iter()
                        .map(|r| {
                            self.bindings.extend(r.get_bindings());
                            r.build()
                        })
                        .collect::<Vec<_>>(),
                );
            }
        }
        self
    }
}

impl Queryable for DefineFieldStatement {}
impl Erroneous for DefineFieldStatement {}

impl Parametric for DefineFieldStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for DefineFieldStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE FIELD {}", &self.field_name);

        if let Some(table) = &self.table {
            query = format!("{query} ON TABLE {table}");
        }

        if let Some(field_type) = &self.type_ {
            query = format!("{query} TYPE {field_type}");
        }

        if let Some(value) = &self.value {
            query = format!("{query} VALUE $value OR {value}");
        }

        if let Some(assertion) = &self.assert {
            query = format!("{query} ASSERT {assertion}");
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

impl Display for DefineFieldStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cond, value, Operatable, ToRaw, NONE};
    use crate::{statements::for_permission, CrudType::*};

    #[test]
    fn test_define_field_statement_full() {
        let name = Field::new("name");
        let user_table = Table::from("user");
        let age = Field::new("age");
        let email = Field::new("email");
        use FieldType::*;

        let statement = define_field(email)
            .on_table(user_table)
            .type_(String)
            .value("example@codebreather.com")
            .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
            .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
            .permissions(for_permission([Create, Update]).where_(name.is("Oyedayo"))) //Multiple
            .permissions([
                for_permission([Create, Delete]).where_(name.is("Oyedayo")),
                for_permission(Update).where_(age.less_than_or_equal(130)),
            ]);

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE FIELD email ON TABLE user TYPE string VALUE $value OR $_param_00000001 \
                ASSERT ($value IS NOT NONE) AND ($value ~ $_param_00000002)\n\
                PERMISSIONS\n\
                FOR select\n\tWHERE age >= $_param_00000003\n\
                FOR create, update\n\tWHERE name IS $_param_00000004\n\
                FOR create, delete\n\tWHERE name IS $_param_00000005\n\
                FOR update\n\tWHERE age <= $_param_00000006;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE FIELD email ON TABLE user TYPE string VALUE $value OR 'example@codebreather.com' \
                ASSERT ($value IS NOT NONE) AND ($value ~ 'is_email')\n\
                PERMISSIONS\n\
                FOR select\n\tWHERE age >= 18\n\
                FOR create, update\n\tWHERE name IS 'Oyedayo'\n\
                FOR create, delete\n\tWHERE name IS 'Oyedayo'\n\
                FOR update\n\tWHERE age <= 130;"
        );
        insta::assert_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 6);
    }

    #[test]
    fn test_define_field_statement_simple() {
        use FieldType::*;

        let email = Field::new("email");
        let user_table = Table::from("user");
        let statement = define_field(email).on_table(user_table).type_(String);

        assert_eq!(
            statement.build(),
            "DEFINE FIELD email ON TABLE user TYPE string;"
        );
        insta::assert_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 0);
    }
}
