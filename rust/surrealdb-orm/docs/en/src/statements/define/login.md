## Define Login Statement

The `define_login` statement is used to define a login in SurrealDB. Logins are used for authentication purposes, allowing users to authenticate and access protected resources. This documentation provides an overview of the syntax and usage of the `define_login` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Login with Password](#define-login-with-password)
  - [Define Login with Passhash](#define-login-with-passhash)

## Syntax

The basic syntax of the `define_login` statement is as follows:

```rust
define_login(login_name: Login)
    .on_namespace()
    .password(password: &str)

define_login(login_name: Login)
    .on_database()
    .password(password: &str)

define_login(login_name: Login)
    .on_namespace()
    .passhash(passhash: &str)
```

- `login_name`: The name of the login to define.
- `password`: The password associated with the login.
- `passhash`: The password hash associated with the login.

The `define_login` statement supports the following options:

- `on_namespace()`: Specifies that the login should be defined on the namespace level.
- `on_database()`: Specifies that the login should be defined on the database level.

## Examples

### Define Login with Password

To define a login with a password, you can use the following code:

```rust
let username = Login::new("username");
let login_with_password = define_login(username)
    .on_database()
    .password("oyelowo");
```

In the example above, the `define_login` statement defines a login named "username" on the database level. The login is associated with a password "oyelowo".

This will generate the following SQL statement:

```sql
DEFINE LOGIN username ON DATABASE PASSWORD 'oyelowo';
```

### Define Login with Passhash

To define a login with a password hash, you can use the following code:

```rust
let login_with_hash = define_login("username")
    .on_namespace()
    .passhash("reiiereroyedayo");
```

In the example above, the `define_login` statement defines a login named "username" on the namespace level. The login is associated with a password hash "reiiereroyedayo".

This will generate the following SQL statement:

```sql
DEFINE LOGIN username ON NAMESPACE PASSHASH 'reiiereroyedayo';
```

You have now learned how to define logins using the `define_login` statement. Logins are essential for authentication in SurrealDB, allowing users to securely access protected resources.
