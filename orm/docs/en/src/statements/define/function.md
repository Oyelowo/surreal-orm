# Define Function Statement

The `define_function!` statement is used to define a custom function in
SurrealDB. It allows you to define reusable logic that can be used within
queries. This documentation provides an overview of the syntax and usage of the
`define_function!` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Function with Parameters and Logic](#define-function-with-parameters-and-logic)
  - [Define Function with Complex Logic](#define-function-with-complex-logic)
- [Using the Generated Function](#using-the-generated-function)

## Syntax

The basic syntax of the `define_function!` statement is as follows:

```rust
define_function!(function_name(parameter1: type1, parameter2: type2, ...) {
    // Function logic
});
```

- `function_name`: The name of the function to define.
- `parameter1`, `parameter2`, ...: The parameters of the function, along with
  their types.
- `function logic`: The logic or operations to be performed by the function.

The `define_function!` statement supports the following features:

- Defining function parameters and their types.
- Writing custom logic or operations within the function body.
- Returning values from the function.

## Examples

### Define Function with Parameters and Logic

To define a function with parameters and custom logic, you can use the following
code:

```rust
define_function!(get_it(first: bool, last: string, birthday: string) {
    let person = "43";
    return person;
});
```

In the example above, the `define_function!` statement defines a function named
"get_it" with three parameters: `first`, `last`, and `birthday`. The function
body consists of assigning a value to the `person` variable and returning it.

This will generate the following SQL statement:

```sql
DEFINE FUNCTION get_it($first: bool, $last: string, $birthday: string) {
    LET $person = '43';

    RETURN $person;
};
```

You can then use the defined function in queries by calling it with the
appropriate arguments.

### Define Function with Complex Logic

Here's an example of defining a function with more complex logic and operations:

```rust
use surreal_models::SpaceShip;
use surreal_orm::{
    cond, index,
    statements::{create, define_function, if_, select},
    All, Buildable, Operatable, SchemaGetter, SetterAssignable, Model, ToRaw, NONE,
};

define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
    let person = select(All)
        .from(SpaceShip::table())
        .where_(
            cond(SpaceShip::schema().id.equal(&first_arg))
                .and(SpaceShip::schema().name.equal(&last_arg))
                .and(SpaceShip::schema().created.equal(&birthday_arg)),
        );

    return if_(person.with_path::<SpaceShip>(index(0)).id.is_not(NONE))
                .then(person.with_path::<SpaceShip>(index(0)))
            .else_(
                create::<SpaceShip>().set(
                    vec![
                        SpaceShip::schema().id.equal_to(&first_arg),


 SpaceShip::schema().name.equal_to(&last_arg),
                        SpaceShip::schema().created.equal_to(&birthday_arg),
                    ]
                )
            ).end();
});
```

In the example above, the `define_function!` statement defines a function named
"get_person" with three parameters: `first_arg`, `last_arg`, and `birthday_arg`.
The function body consists of a complex logic that includes a SELECT statement,
conditional checks, and the creation of a new record if the condition is not
met.

This will generate the following SQL statement:

```sql
DEFINE FUNCTION get_person($first_arg: string, $last_arg: string, $birthday_arg: string) {
    LET $person = (SELECT * FROM space_ship WHERE (id = $first_arg) AND (name = $last_arg) AND (created = $birthday_arg));

    RETURN IF $person[0].id != NONE THEN $person[0] ELSE (CREATE space_ship SET id = $first_arg, name = $last_arg, created = $birthday_arg) END;
};
```

You can then use the defined function in queries by calling it with the
appropriate arguments.

## Using the Generated Function

To use the function defined using `define_function!`, you need to execute the
generated statement before you can use the function in your queries. The
generated statement is suffixed by `_statement` and contains the actual function
definition. After executing the statement, you can use the function without the
`_statement` suffix.

Here's an example of how to use the defined function:

```rust
// Define the function statement
let fn_statement = get_it_statement();

// Execute the statement to define the function
// This statement needs to be executed before the function can be used
fn_statement.run(db);

// Use the defined function in a query
let get_it_function = get_it(false, "3".to_string(), "3".to_string());

// Verify the generated function can be used in a query
assert_eq!(get_it_function.to_raw().build(), "get_it(false, '3', '3')");
assert_eq!(
    get_it_function.fine_tune_params(),
    "et_it($_param_00000001, $_param_00000002, $_param_00000003)"
);
```

In this example, we first define the function statement using the
`get_it_statement()` macro. Then, we execute the generated statement using
`surreal_orm::execute()` to define the function in SurrealDB. After that, we can
use the defined function `get_it()` in our queries by calling it with the
appropriate arguments.

Make sure to execute the statement to define the function before using it in
your queries.

---

Now you have learned how to define custom functions using the `define_function!`
macro, how to execute the generated statement to define the function, and how to
use the defined function in your queries. Refer to the SurrealDB documentation
for more information on custom functions and their usage.
