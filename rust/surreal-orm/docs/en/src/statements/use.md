# Use Statement

The `use` statement in SurrealDB ORM is used to switch the active namespace and database.
This documentation provides an overview of the `use` statement and its usage.

## Table of Contents

- [Introduction](#introduction)
- [Syntax](#syntax)
- [Examples](#examples)
  - [Using the `use` Statement with Namespace](#using-the-use-statement-with-namespace)
  - [Using the `use` Statement with Database](#using-the-use-statement-with-database)
  - [Using the `use` Statement with Namespace and Database](#using-the-use-statement-with-namespace-and-database)

## Introduction

The `use` statement in SurrealDB ORM allows you to switch the active namespace and database.
By specifying the desired namespace and/or database, you can focus your queries and operations on specific areas of your database.

## Syntax

The basic syntax of the `use` statement is as follows:

```rust
use_()
    .namespace(namespace)
    .database(database);
```

The `use` statement supports the following methods:

- `.namespace(namespace)`: Specifies the namespace to use.
- `.database(database)`: Specifies the database to use.
- `.build()`: Builds the `use` statement.

## Examples

### Using the `use` Statement with Namespace

To switch the active namespace using the `use` statement, you can use the following code:

```rust
use surreal_orm::statements::use_;
use surreal_orm::models::Namespace;

let use_statement = use_()
    .namespace(Namespace::from("mars".to_string()));

assert_eq!(use_statement, "USE NS mars;");
```

In the above example, the `use` statement is used to switch the active namespace to "mars". The resulting use statement is "USE NS mars;".

### Using the `use` Statement with Database

To switch the active database using the `use` statement, you can use the following code:

```rust
use surreal_orm::statements::use_;
use surreal_orm::models::Database;

let use_statement = use_()
    .database(Database::from("root".to_string()));

assert_eq!(use_statement, "USE DB root;");
```

In the above example, the `use` statement is used to switch the active database to "root". The resulting use statement is "USE DB root;".

### Using the `use` Statement with Namespace and Database

You can also switch both the active namespace and database using the `use` statement. Here's an example:

```rust
use surreal_orm::statements::use_;
use surreal_orm::models::{Namespace, Database};

let use_statement = use_()
    .namespace(Namespace::from("mars".to_string()))
    .database(Database::from("root".to_string()));

assert_eq!(use_statement, "USE DB root NS mars;");
```

In the above example, the `use` statement is used to switch the active namespace to "mars"
and the active database to "root". The resulting use statement is "USE DB root NS mars;".

You have now learned how to use the `use` statement in SurrealDB ORM to switch the active
namespace and database. This allows you to focus your queries and operations on specific areas of your database.
