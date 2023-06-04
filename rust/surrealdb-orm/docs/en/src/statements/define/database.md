# Define Database Statement

The `define_database` statement is used to define a database in SurrealDB. A database is a logical container for storing related data and organizing resources. This documentation provides an overview of the syntax and usage of the `define_database` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define a Database](#define-a-database)

## Syntax

The syntax of the `define_database` statement is as follows:

```rust
define_database(database_name: Database)
```

- `database_name`: The name of the database to define.

## Examples

### Define a Database

To define a database, you can use the following code:

```rust
let statement = define_database("oyelowo");
```

In the example above, the `define_database` statement defines a database named "oyelowo".

This will generate the following SQL statement:

```sql
DEFINE DATABASE oyelowo;
```

You have now learned how to define a database using the `define_database` statement. Databases provide a way to organize and manage data within SurrealDB, allowing you to create distinct containers for your data resources.
