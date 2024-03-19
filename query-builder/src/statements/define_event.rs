/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use crate::{
    BindingsList, Buildable, Conditional, Erroneous, Event, Filter, Parametric, Queryable, Table,
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

/**
Define a new event.
Events can be triggered after any change or modification to the data in a record. Each trigger is able to see the $before and $after value of the record, enabling advanced custom logic with each trigger.

Requirements
You must be authenticated as a root, namespace, or database user before you can use the DEFINE EVENT statement.
You must select your namespace and database before you can use the DEFINE EVENT statement.

# Example
```rust
 # use surreal_query_builder as surreal_orm;
use surreal_orm::{*, statements::{define_event, select}};

# let age = Field::new("age");
# let city = Field::new("city");
# let fake_id = TestUser::create_id("oyelowo");
# let user_table = Table::new("user");
# let email_event = Event::new("email");
 let query = define_event(email_event)
    .on_table(user_table)
     .when(age.greater_than_or_equal(18))
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

 assert!(!query.build().is_empty());
 assert_eq!(
     query.fine_tune_params(),
     "DEFINE EVENT email ON TABLE user WHEN age >= $_param_00000001 \
THEN SELECT * FROM $_param_00000002 WHERE (city IS $_param_00000003) AND (city IS $_param_00000004) OR (city ~ $_param_00000005) LIMIT $_param_00000006 START AT $_param_00000007 PARALLEL;",
);

 assert_eq!(
     query.to_raw().build(),
     "DEFINE EVENT email ON TABLE user WHEN age >= 18 THEN SELECT * FROM user:oyelowo WHERE (city IS 'Prince Edward Island') AND (city IS 'NewFoundland') OR \
(city ~ 'Toronto') LIMIT 153 START AT 10 PARALLEL;",
);
```
*/
pub fn define_event(event_name: impl Into<Event>) -> EventBuilder {
    EventBuilder::new(event_name)
}

impl EventBuilder {
    /// Set the event name
    fn new(event_name: impl Into<Event>) -> Self {
        Self {
            event: event_name.into().to_string(),
            on_table: None,
            when: None,
            then_string: None,
            bindings: vec![],
        }
    }

    /// Set the event table
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.on_table = Some(table.into().to_string());
        self
    }

    /// Set the event trigger
    pub fn when(mut self, condition: impl Conditional) -> Then {
        let cond = Filter::new(condition);
        self.when = Some(cond.build());
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

/// A statement for defining an event.
pub struct DefineEventStatement(EventBuilder);

// DEFINE EVENT @name ON [ TABLE ] @table WHEN @expression THEN @expression
impl Buildable for DefineEventStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE EVENT {}", &self.0.event);
        if let Some(table) = &self.0.on_table {
            query = format!("{query} ON TABLE {table}");
        }

        if let Some(condition) = &self.0.when {
            query = format!("{query} WHEN {condition}");
        }

        if let Some(expression) = &self.0.then_string {
            query = format!("{query} THEN {}", expression.trim_end_matches(';'));
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

impl Queryable for DefineEventStatement {}

impl Erroneous for DefineEventStatement {}

#[cfg(test)]
mod tests {
    use surrealdb::sql;

    use super::*;
    use crate::{statements::select, *};

    #[test]
    fn test_define_event_statement_state_machine() {
        let age = Field::new("age");
        let city = Field::new("city");
        let fake_id = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
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
            query.fine_tune_params(),
            "DEFINE EVENT email ON TABLE user WHEN age >= $_param_00000001 THEN SELECT * FROM $_param_00000002 WHERE (city IS $_param_00000003) AND (city IS $_param_00000004) OR (city ~ $_param_00000005) LIMIT $_param_00000006 START AT $_param_00000007 PARALLEL;",
        );

        assert_eq!(
            query.to_raw().build(),
            "DEFINE EVENT email ON TABLE user \
                WHEN age >= 18 THEN \
                SELECT * FROM user:oyelowo WHERE (city IS 'Prince Edward Island') AND \
                (city IS 'NewFoundland') OR (city ~ 'Toronto') LIMIT 153 START AT 10 PARALLEL;",
        );
    }
}
