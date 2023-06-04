# Info Statement

The `INFO` statement in SurrealDB ORM is used to retrieve information about various elements in the database,
such as key-value pairs, namespaces, databases, scopes, and tables. This documentation covers
the usage and examples of the `INFO` statement for each of these elements.

## Table of Contents

- [Info for Key-Value (KV) Pairs](#info-for-key-value-kv-pairs)
- [Info for Namespaces](#info-for-namespaces)
- [Info for Databases](#info-for-databases)
- [Info for Scopes](#info-for-scopes)
- [Info for Tables](#info-for-tables)

## Info for Key-Value (KV) Pairs

The `INFO FOR KV` statement is used to retrieve information about key-value pairs in the SurrealDB. Here's an example:

```rust
let statement = info_for().kv().build();
assert_eq!(statement, "INFO FOR KV;");
```

The generated SQL query for this code block would be `INFO FOR KV;`.

## Info for Namespaces

The `INFO FOR NS` statement is used to retrieve information about namespaces in the SurrealDB. Here's an example:

```rust
let statement = info_for().namespace().build();
assert_eq!(statement, "INFO FOR NS;");
```

The generated SQL query for this code block would be `INFO FOR NS;`.

## Info for Databases

The `INFO FOR DB` statement is used to retrieve information about databases in the SurrealDB. Here's an example:

```rust
let statement = info_for().database().build();
assert_eq!(statement, "INFO FOR DB;");
```

The generated SQL query for this code block would be `INFO FOR DB;`.

## Info for Scopes

The `INFO FOR SCOPE` statement is used to retrieve information about a specific scope in the SurrealDB. Here's an example:

```rust
let statement = info_for().scope("test_scope").build();
assert_eq!(statement, "INFO FOR SCOPE test_scope;");
```

The generated SQL query for this code block would be `INFO FOR SCOPE test_scope;`.

## Info for Tables

The `INFO FOR TABLE` statement is used to retrieve information about a specific table in the SurrealDB. Here's an example:

```rust
let statement = info_for().table("test_table").build();
assert_eq!(statement, "INFO FOR TABLE test_table;");
```

The generated SQL query for this code block would be `INFO FOR TABLE test_table;`.

That concludes the documentation for the `INFO` statement in SurrealDB ORM. Use the examples and
explanations provided to retrieve information about key-value pairs, namespaces, databases, scopes, and tables effectively.
