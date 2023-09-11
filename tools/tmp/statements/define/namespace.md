# Define Namespace Statement

The `define_namespace` statement is used to define a namespace in SurrealDB. A namespace is a logical container for organizing database objects, such as tables, indexes, and functions. This documentation provides an overview of the syntax and usage of the `define_namespace` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define a Namespace](#define-a-namespace)

## Syntax

The syntax of the `define_namespace` statement is as follows:

```rust
define_namespace(namespace_name: &str)
```

- `namespace_name`: The name of the namespace to define.

## Examples

### Define a Namespace

To define a namespace, you can use the following code:

```rust
let statement = define_namespace("oyelowo");
```

In the example above, the `define_namespace` statement defines a namespace named "oyelowo".

This will generate the following SQL statement:

```sql
DEFINE NAMESPACE oyelowo;
```

You have now learned how to define a namespace using the `define_namespace` statement.
Namespaces provide a way to organize and structure your database objects within SurrealDB,
enabling better management and organization of your resources.
