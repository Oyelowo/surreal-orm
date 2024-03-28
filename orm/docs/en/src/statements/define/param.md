# Define Param Statement

The `define_param` statement is used to define a parameter in SurrealDB.
Parameters provide a way to store and reuse values within queries. This
documentation provides an overview of the syntax and usage of the `define_param` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Param Statement](#define-param-statement-usage)

## Syntax

The basic syntax of the `define_param` statement is as follows:

```rust
define_param(param_name: Param) {
    // Parameter definition
}
```

- `param_name`: The name of the parameter to define.

The `define_param` statement supports the following features:

- Assigning a value to the parameter.

## Examples

### Define Param Statement Usage

To define a parameter with a specific value, you can use the following code:

```rust
// Define the parameter
fn endpoint_base() -> Param {
    Param::new("endpoint_base")
}

// Define the param definition itself. This must be run against the database first to use the param.
let statement = define_param(endpoint_base()).value("https://dummyjson.com");
```

In the example above, the `define_param` statement defines a parameter named "endpoint_base" with a value of "https://dummyjson.com".

Before using the defined parameter, it is important to run the `define_param` statement to register the parameter with the database. You can do this by calling the `run` method on the statement object, passing the SurrealDB instance as an argument:

```rust
statement.run(db);
```

Once the `define_param` statement has been executed, you can use the defined parameter (`$endpoint_base`) across your codebase in queries and other operations.

You can then reference the parameter name in your queries to utilize the stored value.

```rust
let query = select(All).from(User::table()).where_(endpoint_base().equal_to("https://dummyjson.com"));
```

---

Now you have learned how to define a parameter using the `define_param` statement.
Parameters provide a way to store and reuse values within queries. Remember to
execute the `define_param` statement to register the parameter with the database
before using it in your codebase. Refer to the SurrealDB documentation for more
information on parameters and their usage.
