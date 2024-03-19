# Define Event Statement

The `define_event` statement is used to define an event in SurrealDB. It allows
you to specify the conditions and actions associated with the event. This
documentation provides an overview of the syntax and usage of the `define_event`
statement.

## Table of Contents

- [Syntax](#syntax)
- [Using the `cond!` Macro](#using-the-cond!-macro)
- [Examples](#examples)
  - [Define Event with State Machine](#define-event-with-state-machine)

## Syntax

The basic syntax of the `define_event` statement is as follows:

```rust
define_event(event_name)
    .on_table(table)
    .when(condition)
    .then(action);
```

- `event_name`: The name of the event to define.
- `table`: The name of the table where the event occurs.
- `condition`: The condition that triggers the event.
- `action`: The action to perform when the event is triggered.

The `define_event` statement supports the following methods:

- `.on_table(table)`: Specifies the table where the event occurs.
- `.when(condition)`: Specifies the condition that triggers the event.
- `.then(action)`: Specifies the action to perform when the event is triggered.

## Using the `cond!` Macro

The `cond!` macro is a handy tool when defining conditions for the `WHEN` clause
in the `DEFINE EVENT` statement. It provides a concise way to define conditions,
enhancing readability while ensuring type safety.

Example:

```rust
let filter = cond!((strength > 5) && (strength < 15));
```

By using the `cond!` macro, you can effectively and expressively define
conditions for the `DEFINE EVENT` statement.

For a more in-depth explanation and advanced usage of the `cond!` macro,
[refer to the dedicated chapter on helper macros](#helper-macros).

## Examples

### Define Event with State Machine

To define an event with a state machine-like behavior, you can use the following
code:

```rust
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
```

This will generate the following SQL statement:

```sql
DEFINE EVENT email ON TABLE user WHEN age >= 18 THEN SELECT * FROM user:oyelowo WHERE (city IS 'Prince Edward Island') AND (city IS 'NewFoundland') OR (city ~ 'Toronto') LIMIT 153 START AT 10 PARALLEL;
```

In the example above, the `define_event` statement defines an event named
"email" on the "user" table. It specifies that the event is triggered when the
age is greater than or equal to 18. The action associated with the event is to
perform a `SELECT` query on the "user:oyelowo" table with certain conditions and
settings.

This concludes the documentation for the `define_event` statement. Use this
statement to define events in SurrealDB and specify their conditions and
actions.
