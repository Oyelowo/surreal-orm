# Define Function Statement

The `define_function!` statement is used to define a custom function in SurrealDB. It allows you to define reusable logic that can be used within queries. This documentation provides an overview of the syntax and usage of the `define_function!` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Function with Parameters and Logic](#define-function-with-parameters-and-logic)
  - [Define Function with Complex Logic](#define-function-with-complex-logic)

## Syntax

The basic syntax of the `define_function!` statement is as follows:

```rust
define_function!(function_name(parameter1: type1, parameter2: type2, ...) {
    // Function logic
});
```

- `function_name`: The name of the function to define.
- `parameter1`, `parameter2`, ...: The parameters of the function, along with their types.
- `function logic`: The logic or operations to be performed by the function.

The `define_function!` statement supports the following features:

- Defining function parameters and their types.
- Writing custom logic or operations within the function body.
- Returning values from the function.

## Examples

### Define Function with Parameters and Logic

To define a function with parameters and custom logic, you can use the following code:

```rust
define_function!(get_it(first: bool, last: string, birthday: string) {
    let person = "43";
    return person;
});
```

In the example above, the `define_function!` statement defines a function named "get_it" with three parameters: `first`, `last`, and `birthday`. The function body consists of assigning a value to the `person` variable and returning it.

This will generate the following SQL statement:

```sql
DEFINE FUNCTION get_it($first: bool, $last: string, $birthday: string) {
    LET $person = '43';

    RETURN $person;
};
```

You can then use the defined function in queries by calling it with the appropriate arguments.

### Define Function with Complex Logic

Here's an example of defining a function with more complex logic and operations:

```rust
use surrealdb_models::SpaceShip;
use surrealdb_orm::{
    cond, index,
    statements::{create, define_function, if_, select},
    All, Buildable, Operatable, SchemaGetter, SetterAssignable, SurrealdbModel, ToRaw, NONE,
};

define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
    let person = select(All)
        .from(SpaceShip::table_name())
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

In the example above, the `define_function!` statement defines a function named "get_person" with three parameters: `first_arg`, `last_arg`, and `birthday_arg`. The function body consists of a complex logic that includes a SELECT statement, conditional checks,

and the creation of a new record if the condition is not met.

This will generate the following SQL statement:

```sql
DEFINE FUNCTION get_person($first_arg: string, $last_arg: string, $birthday_arg: string) {
    LET $person = (SELECT * FROM space_ship WHERE (id = $first_arg) AND (name = $last_arg) AND (created = $birthday_arg));

    RETURN IF $person[0].id != NONE THEN $person[0] ELSE (CREATE space_ship SET id = $first_arg, name = $last_arg, created = $birthday_arg) END;
};
```

You can then use the defined function in queries by calling it with the appropriate arguments.

This concludes the documentation for the `define_function!` statement. Use this statement to define custom functions in SurrealDB with reusable logic.
