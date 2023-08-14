# IfElse Statement

The `ifelse` statement is used to create conditional branching in SurrealDB. It allows you to execute different expressions or statements based on specified conditions. Here are some examples and usage scenarios for the `ifelse` statement.

## Table of Contents

- [Statement Syntax](#statement-syntax)
- [Creating an If Statement](#creating-an-if-statement)
- [Adding Else If Statements](#adding-else-if-statements)
- [Adding an Else Statement](#adding-an-else-statement)
- [Nested If Else Statements](#nested-if-else-statements)
- [Using Subqueries in If Else Statements](#using-subqueries-in-if-else-statements)

## Statement Syntax

The syntax for the `ifelse` statement is as follows:

```rust, ignore
if_(condition)
    .then(expression)
    .else_if(condition)
    .then(expression)
    .else_if(condition)
    .then(expression)
    .else_(expression)
    .end();
```

## Creating an If Statement

You can create a simple `if` statement using the `if_` function. Here's an example:

```rust
use surreal_orm::*;

let age = Field::new("age");

let if_statement = if_(age.greater_than_or_equal(18))
    .then("Valid".to_string())
    .end();
```

## Adding Else If Statements

You can add multiple `else if` statements to the `ifelse` statement. Here's an example:

```rust
let name = Field::new("name");
let age = Field::new("age");

let if_statement = if_(age.greater_than_or_equal(18))
    .then("Valid")
    .else_if(name.like("Oyelowo Oyedayo"))
    .then("The Alien!")
    .end();
```

## Adding an Else Statement

You can add an `else` statement to the `ifelse` statement to handle cases when none of the previous conditions are met. Here's an example:

```rust
let age = Field::new("age");

let if_statement = if_(age.greater_than_or_equal(18))
    .then("Valid")
    .else_("Invalid")
    .end();
```

## Nested If Else Statements

You can nest `ifelse` statements within each other to create complex conditional logic. Here's an example:

```rust
let name = Field::new("name");
let age = Field::new("age");
let country = Field::new("country");

let if_statement = if_(age.greater_than_or_equal(18))
    .then("Valid")
    .else_if(name.like("Oyelowo Oyedayo"))
    .then("The Alien!")
    .else_if(cond(country.is("Canada")).or(country.is("Norway")))
    .then("Cold")
    .else_("Hot")
    .end();
```

## Using Subqueries in If Else Statements

You can use subqueries in the `ifelse` statement to execute more complex expressions or statements. Here's an example:

```rust
let name = Field::new("name");
let age = Field::new("age");
let country = Field::new("country");
let city = Field::new("city");
let fake_id = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
let fake_id2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));

let statement1 = select(All)
    .from(fake_id)
    .where_(
        cond(city.is("Prince Edward Island"))
            .and(city.is("NewFoundland"))
            .or(city.like("Toronto")),
    )
    .order_by(order(&age).numeric())
    .limit(153)
    .start(10)
    .parallel();

let statement2 = select(All)
    .from(fake_id2)
    .where_(country.is("INDONESIA"))
    .order_by(order(&age).numeric())
    .limit(20)
    .start(5);

let if_statement = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
    .then(statement1)
    .else_if(name.like("Oyelowo Oyedayo"))
    .then(statement2)
    .else_if(cond(country.is("Canada"))
            .or(country.is("Norway")))
    .then("Cold")
    .else_("Hot")
    .end();
```
