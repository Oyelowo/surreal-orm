## Define Token Statement

The `define_token` statement is used to define a token in SurrealDB. Tokens are used for authentication and authorization purposes, allowing users or applications to access protected resources. This documentation provides an overview of the syntax and usage of the `define_token` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Token on Namespace](#define-token-on-namespace)
  - [Define Token on Database](#define-token-on-database)
  - [Define Token on Scope](#define-token-on-scope)

## Syntax

The basic syntax of the `define_token` statement is as follows:

```rust
define_token(token_name: Token)
    .on_namespace()
    .type_(token_type: TokenType)
    .value(token_value: &str)

define_token(token_name: Token)
    .on_database()
    .type_(token_type: TokenType)
    .value(token_value: &str)

define_token(token_name: Token)
    .on_scope(scope_name: Scope)
    .type_(token_type: TokenType)
    .value(token_value: &str)
```

- `token_name`: The name of the token to define.
- `token_type`: The type of the token, specified using the `TokenType` enum.
- `token_value`: The value or secret associated with the token.

The `define_token` statement supports the following options:

- `on_namespace()`: Specifies that the token should be defined on the namespace level.
- `on_database()`: Specifies that the token should be defined on the database level.
- `on_scope(scope_name)`: Specifies that the token should be defined on a specific scope.

## Examples

### Define Token on Namespace

To define a token on the namespace level, you can use the following code:

```rust
let statement = define_token("oyelowo_token")
    .on_namespace()
    .type_(TokenType::PS512)
    .value("abrakradabra");
```

In the example above, the `define_token` statement defines a token named "oyelowo_token" on the namespace level. The token type is set to `TokenType::PS512` and the value is set to "abrakradabra".

This will generate the following SQL statement:

```sql
DEFINE TOKEN oyelowo_token ON NAMESPACE TYPE PS512 VALUE 'abrakradabra';
```

### Define Token on Database

To define a token on the database level, you can use the following code:

```rust
let statement = define_token("oyelowo_token")
    .on_database()
    .type_(TokenType::HS512)
    .value("anaksunamun");
```

In the example above, the `define_token` statement defines a token named "oyelowo_token" on the database level. The token type is set to `TokenType::HS512` and the value is set to "anaksunamun".

This will generate the following SQL statement:

```sql
DEFINE TOKEN oyelowo_token ON DATABASE TYPE HS512 VALUE 'anaksunamun';
```

### Define Token on Scope

To define a token on a specific scope, you can use the following code:

```rust
let statement = define_token("oyelowo_token")
    .on_scope("planet")
    .type_(TokenType::EDDSA)
    .value("abcde");
```

In the example above, the `define_token` statement defines a token named "oyelowo_token" on the scope "planet". The token type is set to `TokenType::EDDSA` and the value is set to "abcde".

This will generate the following SQL statement:

```sql


DEFINE TOKEN oyelowo_token ON SCOPE planet TYPE EDDSA VALUE 'abcde';
```

You have now learned how to define tokens using the `define_token` statement. Tokens are essential for authentication and authorization in SurrealDB, allowing you to secure your data and control access to resources. Define Token
