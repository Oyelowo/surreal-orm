# Define Statement

The `define` statement in SurrealDB is a powerful tool that allows you to define various objects and configurations within the database. It provides a flexible and expressive way to create and manage entities such as tables, indexes, namespaces, tokens, logins, and more. This documentation provides an overview of the `define` statement and its usage.

## Table of Contents

- [Introduction](#introduction)
- [Syntax](#syntax)
- [Supported Objects](#supported-objects)
- [Examples](#examples)

## Introduction

The `define` statement serves as a declarative mechanism for defining and configuring various elements in SurrealDB. It enables you to specify the properties and characteristics of different objects, helping you define the structure, behavior, and access controls of your database components.

By using the `define` statement, you can create and manage objects such as tables, indexes, namespaces, tokens, logins, and more, all within a single comprehensive syntax. This provides a unified approach to defining and organizing your database entities, making it easier to maintain and modify them over time.

## Syntax

The general syntax of the `define` statement is as follows:

```rust
define(object_name)
    .property1(value1)
    .property2(value2)
    .property3(value3)
    // ...
```

The specific properties and values depend on the type of object being defined. Each object may have different properties that can be set, such as names, types, constraints, configurations, and more. The `define` statement provides a fluent and chainable API to set these properties in a concise and readable manner.

## Supported Objects

The `define` statement supports a variety of objects that can be defined within SurrealDB. Some of the commonly used objects include:

- Tables: Define the structure and schema of tables within the database.
- Indexes: Define indexes on tables to optimize data retrieval and querying.
- Namespaces: Define logical containers to organize database objects.
- Tokens: Define authentication and authorization tokens for access control.
- Logins: Define user logins for authentication purposes.
- Scopes: Define scopes to encapsulate and manage query execution environments.

These are just a few examples of the objects that can be defined using the `define` statement. SurrealDB provides a rich set of features and options for each object type, allowing you to customize and tailor the behavior of your database entities according to your specific requirements.

## Examples

Here are a few examples of using the `define` statement to define different objects:

- Defining a table:

```rust
let user = Table::from("user");
let statement = define_table(user).schemaless().permissions_full();
```

- Defining an index:

```rust
let query = define_index("userEmailIndex")
    .on_table(User::table())
    .fields(email)
    .unique();
```

- Defining a namespace:

```rust
let namespace_def = define_namespace("myapp");
```

- Defining a token:

```rust
let token_def = define_token("access_token")
    .on_namespace()
    .type_(TokenType::HS256)
    .value("mysecretpassword");
```

These examples showcase the versatility and power of the `define` statement in SurrealDB. You can define and configure a wide range of objects using a consistent and intuitive syntax, enabling you to shape your database according to your desired structure and requirements.

This concludes the overview of the `define` statement in SurrealDB. You can now leverage its capabilities to define and manage various objects within your database, providing a solid foundation

for building robust and scalable applications.
