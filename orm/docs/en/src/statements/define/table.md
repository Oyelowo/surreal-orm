# Define Table Statement

The `define_table` statement is used to define a table in SurrealDB. It allows you to specify various options and permissions for the table. This documentation provides an overview of the syntax and usage of the `define_table` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Schemaless Table](#schemaless-table)
  - [Schemaless Table with Permissions](#schemaless-table-with-permissions)
  - [Define Table with Projection](#define-table-with-projection)
  - [Define Table with Multiple Permissions](#define-table-with-multiple-permissions)

## Syntax

The basic syntax of the `define_table` statement is as follows:

```rust
define_table(table)
    .drop()
    .as_(select_statement)
    .schemafull()
    .permissions(permission_statements);
```

- `table`: The name of the table to define.

The `define_table` statement supports the following methods:

- `.drop()`: Drops the existing table before defining it.
- `.as_(select_statement)`: Specifies a `SELECT` statement to populate the table.
- `.schemafull()`: Defines the table with a schema.
- `.permissions(permission_statements)`: Specifies the permissions for the table.

## Examples

### Schemaless Table

To define a schemaless table with no permissions, you can use the following code:

```rust
let user = Table::from("user");
let statement = define_table(user).schemaless().permissions_none();
```

This will generate the following SQL statement:

```sql
DEFINE TABLE user SCHEMALESS PERMISSIONS NONE;
```

### Schemaless Table with Permissions

To define a schemaless table with full permissions, you can use the following code:

```rust
let user = Table::from("user");
let statement = define_table(user).schemaless().permissions_full();
```

This will generate the following SQL statement:

```sql
DEFINE TABLE user SCHEMALESS PERMISSIONS FULL;
```

### Define Table with Projection

A projection allows you to define a table based on a subset of columns from another table. It is similar to creating a view in a relational database. You can specify a projection using the `as_` method and provide a `SELECT` statement as the projection definition. The selected columns and rows will be used to populate the defined table.

Here's an example that demonstrates how to define a table with a projection:

```rust
let user_table = Table::from("user");
let projection_statement = select(All).from(user_table).where_(age.greater_than(18));
let statement = define_table(user_table).as_(projection_statement);
```

This will generate the following SQL statement:

```sql
DEFINE TABLE user AS SELECT * FROM user WHERE age > 18;
```

In the example above, the `define_table` statement defines a table named "user" with a projection based on a `SELECT` statement. Only the rows that satisfy the condition `age > 18` will be included in the table.

### Define Table with Multiple Permissions

You can define a table with multiple permissions using the `permissions` method. The following example demonstrates various permission configurations:

```rust
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
            .order_by(order(&age).numeric().desc())
            .limit(20)
            .start(5),
    )
    .schemafull()
    .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
    .permissions(for_permission([Create, Delete]).where_(name.is("Oyedayo"))) // Multiple
    .permissions([
        for_permission([Create, Delete]).where_(name.is("Oyedayo")),
        for_permission(Update).where_(age.less_than_or_equal(130)),
    ]);
```

This will generate the following SQL statement:

```sql
DEFINE TABLE user DROP SCHEMAFULL AS
    SELECT * FROM user:oyedayo
    WHERE country IS 'INDONESIA' ORDER BY age NUMERIC DESC
    LIMIT 20 START AT 5
PERMISSIONS
    FOR select
        WHERE age >= 18
    FOR create, delete
        WHERE name IS 'Oyedayo'
    FOR create, delete
        WHERE name IS 'Oyedayo'
    FOR update
        WHERE age <= 130;
```

In the example above, the `define_table` statement defines a table named "user". It drops the existing table, populates it with data from a `SELECT` statement, and sets various permissions based on conditions.

This concludes the documentation for the `define_table` statement. Use this statement to define tables in SurrealDB and specify the desired permissions, configurations, and projections.
