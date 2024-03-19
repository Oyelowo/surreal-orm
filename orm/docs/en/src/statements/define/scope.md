# Define Scope Statement

The `define_scope` statement is used to define a scope in SurrealDB. Scopes
provide a way to encapsulate a set of operations within a specific context or
namespace. This documentation provides an overview of the syntax and usage of
the `define_scope` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Scope on Namespace](#define-scope-on-namespace)

## Syntax

The basic syntax of the `define_scope` statement is as follows:

```rust
define_scope(scope_name: &str) {
    // Scope definition
}
```

- `scope_name`: The name of the scope to define.

The `define_scope` statement supports the following features:

- Defining session duration for the scope.
- Defining operations for the scope, such as signup and signin.

## Examples

### Define Scope on Namespace

To define a scope on a namespace with signup and signin operations, you can use
the following code:

```rust
block! {
let user::Schema { email, pass } = &User::schema();
let email = "oyelowo@codebreather.com";
let password = "very-strong";

let token_def = define_scope("oyelowo_scope")
    .session(Duration::from_secs(45))
    .signup(
        create::<User>()
            .set(vec![
                email.equal_to(email),
                pass.equal_to(crypto::argon2::generate!(password)),
            ])
    )
    .signin(
        select(All).from(User::table()).where_(
            cond(email.equal(email))
                .and(crypto::argon2::compare!(pass, password)),
        ),
    );
}
```

In the example above, the `define_scope` statement defines a scope named
"oyelowo_scope" on the namespace. The scope includes a session duration of 45
seconds. It also defines signup and signin operations within the scope. The
signup operation uses the `create` statement with a non-raw query to create a
new user record. The `email` and `pass` fields are set using parameter
placeholders. The `pass` field is generated using the `crypto::argon2::generate`
function. The signin operation performs a select query with conditions.

This will generate the following SQL statement:

```sql
DEFINE SCOPE oyelowo_scope SESSION 45s
    SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($password) )
    SIGNIN ( SELECT * FROM user WHERE (email = email) AND (crypto::argon2::compare(pass, $password)) );
```

You can then use the defined scope in your queries by referencing the scope
name.

---

Now you have learned how to define a scope using the `define_scope` statement.
Scopes provide a way to encapsulate a set of operations within a specific
context or namespace. Refer to the SurrealDB documentation for more information
on scopes and their usage.
