/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{self, Display},
    ops::Deref,
};

use insta::{assert_debug_snapshot, assert_display_snapshot};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql::{self, statements::DefineStatement};

use crate::{
    db_field::{cond, Binding},
    query_create::CreateStatement,
    query_define_token::{Name, Scope},
    query_delete::DeleteStatement,
    query_ifelse::Expression,
    query_insert::{Buildable, InsertStatement},
    query_relate::RelateStatement,
    query_remove::{Event, RemoveScopeStatement, Runnable, Table},
    query_select::{Duration, SelectStatement},
    query_update::UpdateStatement,
    BindingsList, DbField, DbFilter, Parametric, Queryable,
};

// DEFINE EVENT statement
// Events can be triggered after any change or modification to the data in a record. Each trigger is able to see the $before and $after value of the record, enabling advanced custom logic with each trigger.
//
// Requirements
// You must be authenticated as a root, namespace, or database user before you can use the DEFINE EVENT statement.
// You must select your namespace and database before you can use the DEFINE EVENT statement.
// Statement syntax
// DEFINE EVENT @name ON [ TABLE ] @table WHEN @expression THEN @expression
// Example usage
// Below is an example showing how to create an event which upon updating a user's email address will create an entry recording the change on an event table.
//
// -- Create a new event whenever a user changes their email address
// DEFINE EVENT email ON TABLE user WHEN $before.email != $after.email THEN (
// 	CREATE event SET user = $this, time = time::now(), value = $after.email, action = 'email_changed'
// );

pub struct EventBuilder {
    event: String,
    on_table: Option<String>,
    when: Option<String>,
    then_string: Option<String>,
    bindings: BindingsList,
}

pub fn define_event(event_name: impl Into<Event>) -> EventBuilder {
    EventBuilder::new(event_name)
}

impl EventBuilder {
    // Set the event name
    fn new(event_name: impl Into<Event>) -> Self {
        Self {
            event: event_name.into().to_string(),
            on_table: None,
            when: None,
            then_string: None,
            bindings: vec![],
        }
    }

    // Set the event table
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.on_table = Some(table.into().to_string());
        self
    }

    // Set the event trigger
    pub fn when(mut self, condition: impl Into<DbFilter>) -> Then {
        let cond: DbFilter = condition.into();
        self.when = Some(format!("{}", &cond));
        self.bindings.extend(cond.get_bindings());
        Then(self)
    }
}

pub struct Then(EventBuilder);

impl Then {
    pub fn then(mut self, query: impl Queryable + Parametric + Display) -> DefineEventStatement {
        self.0.then_string = Some(format!("{}", &query));
        self.0.bindings.extend(query.get_bindings());
        DefineEventStatement(self.0)
    }
}

pub struct DefineEventStatement(EventBuilder);

// DEFINE EVENT @name ON [ TABLE ] @table WHEN @expression THEN @expression
impl Buildable for DefineEventStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE EVENT {}", &self.0.event);
        if let Some(table_name) = &self.0.on_table {
            query = format!("{query} ON TABLE {table_name}");
        }

        if let Some(condition) = &self.0.when {
            query = format!("{query} WHEN {condition}");
        }

        if let Some(expression) = &self.0.then_string {
            query = format!("{query} THEN {}", expression.trim_end_matches(";"));
        }

        query += ";";
        query
    }
}

impl Display for DefineEventStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for DefineEventStatement {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Runnable for DefineEventStatement {}
impl Queryable for DefineEventStatement {}

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use crate::{
        query_remove::Table,
        query_select::{select, All},
        value_type_wrappers::SurrealId,
        Operatable,
    };

    use super::*;

    #[test]
    fn test_define_event_statement_state_machine() {
        let age = DbField::new("age");
        let city = DbField::new("city");
        let fake_id = SurrealId::try_from("user:oyelowo").unwrap();
        // TODO: Probably use try_from_converter for String<->Table and for other types
        let user_table = Table::new("user");
        let email_event = Event::new("email");

        let query = define_event(email_event)
            .on_table(user_table)
            .when(cond(age.greater_than_or_equal(18)))
            .then(
                select(All)
                    .from(fake_id)
                    .where_(
                        cond(city.is("Prince Edward Island"))
                            .and(city.is("NewFoundland"))
                            .or(city.like("Toronto")),
                    )
                    .limit(153)
                    .start(10)
                    .parallel(),
            );

        assert_eq!(
            query.to_string(),
            "DEFINE EVENT email ON TABLE user WHEN age >= $_param_00000000 THEN SELECT * FROM $_param_00000000 WHERE (city IS $_param_00000000) AND (city IS $_param_00000000) OR (city ~ $_param_00000000) LIMIT 153 START AT 10 PARALLEL;",
        );
        insta::assert_debug_snapshot!(query.get_bindings());
    }
}
