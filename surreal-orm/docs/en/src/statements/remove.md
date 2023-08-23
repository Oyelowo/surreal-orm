# Remove Statement

The `REMOVE` statement in SurrealDB ORM is used to remove various elements from the database, such as databases, events, fields, indexes, logins, scopes, namespaces, tables, and tokens. This documentation covers the usage and examples of the `REMOVE` statement for each of these elements.

## Table of Contents

- [Remove Database](#remove-database)
- [Remove Event](#remove-event)
- [Remove Field](#remove-field)
- [Remove Index](#remove-index)
- [Remove Login](#remove-login)
- [Remove Scope](#remove-scope)
- [Remove Namespace](#remove-namespace)
- [Remove Table](#remove-table)
- [Remove Token](#remove-token)

## Remove Database

The `REMOVE DATABASE` statement is used to remove a database from the SurrealDB. Here's an example:

```rust
assert_eq!(
    remove_database("oyelowo").build(),
    "REMOVE DATABASE oyelowo;"
);
```

The generated SQL query for this code block would be `REMOVE DATABASE oyelowo;`.

## Remove Event

The `REMOVE EVENT` statement is used to remove an event from a table. Here's an example:

```rust
let user = Table::new("user");
let party = Event::new("party");

let statement = remove_event(party).on_table(user);
assert_eq!(statement.build(), "REMOVE EVENT party ON TABLE user;");
```

The generated SQL query for this code block would be `REMOVE EVENT party ON TABLE user;`.

## Remove Field

The `REMOVE FIELD` statement is used to remove a field from a table. Here's an example:

```rust
let user = Table::new("user");
let name = Field::new("name");

let statement = remove_field(name).on_table(user);
assert_eq!(statement.build(), "REMOVE FIELD name ON TABLE user;");
```

The generated SQL query for this code block would be `REMOVE FIELD name ON TABLE user;`.

## Remove Index

The `REMOVE INDEX` statement is used to remove an index from a table. Here's an example:

```rust
let user = Table::new("user");
let party = TableIndex::new("party");

let statement = remove_index(party).on_table(user);
assert_eq!(statement.build(), "REMOVE INDEX party ON TABLE user;");
```

The generated SQL query for this code block would be `REMOVE INDEX party ON TABLE user;`.

## Remove Login

The `REMOVE LOGIN` statement is used to remove a login from either a namespace or a database. Here are examples for removing a login on a namespace and a database:

```rust
let login = Login::new("login");

// Remove login on a namespace
let statement = remove_login(login).on_namespace();
assert_eq!(statement.build(), "REMOVE LOGIN login ON NAMESPACE;");

// Remove login on a database
let statement = remove_login(login).on_database();
assert_eq!(statement.build(), "REMOVE LOGIN login ON DATABASE;");
```

The generated SQL queries for these code blocks would be `REMOVE LOGIN login ON NAMESPACE;` and `REMOVE LOGIN login ON DATABASE;` respectively.

## Remove Scope

The `REMOVE SCOPE` statement is used to remove a scope from the SurrealDB. Here's an example:

```rust
let scope = Scope::new("scope");
let statement = remove_scope(scope);
assert_eq!(statement.build(), "REMOVE SCOPE scope;");
```

The generated SQL query for this code block would be `REMOVE SCOPE scope;`.

## Remove Namespace

The `REMOVE NAMESPACE` statement is used to

remove a namespace from the SurrealDB. Here's an example:

```rust
let namespace = Namespace::new("namespace");
let statement = remove_namespace(namespace);
assert_eq!(statement.build(), "REMOVE NAMESPACE namespace;");
```

The generated SQL query for this code block would be `REMOVE NAMESPACE namespace;`.

## Remove Table

The `REMOVE TABLE` statement is used to remove a table from the SurrealDB. Here's an example:

```rust
let table = Table::new("table");
let statement = remove_table(table);
assert_eq!(statement.build(), "REMOVE TABLE table;");
```

The generated SQL query for this code block would be `REMOVE TABLE table;`.

## Remove Token

The `REMOVE TOKEN` statement is used to remove a token from either a namespace or a database. Here are examples for removing a token on a namespace and a database:

```rust
let token = Token::new("token");

// Remove token on a namespace
let statement = remove_token(token).on_namespace();
assert_eq!(statement.build(), "REMOVE TOKEN token ON NAMESPACE;");

// Remove token on a database
let statement = remove_token(token).on_database();
assert_eq!(statement.build(), "REMOVE TOKEN token ON DATABASE;");
```

The generated SQL queries for these code blocks would be `REMOVE TOKEN token ON NAMESPACE;` and `REMOVE TOKEN token ON DATABASE;` respectively.

That concludes the documentation for the `REMOVE` statement in SurrealDB ORM. Use the examples and explanations provided to effectively remove various elements from the database.
