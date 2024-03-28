# Parameters

Parameters in SurrealDB serve as essential tools for storing and manipulating
data within queries. The ORM simplifies this process, making it intuitive and
streamlined.

## Table of Contents

- [Parameters in Query Creation and Execution](#query-creation-and-execution)
- [Native ORM Parameters](#native-orm-parameters)
- [Advanced Parameter Name Creation](#advanced-parameter-name-creation)

## Query Creation and Execution

The ORM abstracts away much of the complexity involved in crafting queries. To
calculate the average strength of weapons, for instance:

```rust
# let db = Surreal::new::<Mem>(()).await.unwrap();
# db.use_ns("test").use_db("test").await.unwrap();
#
# let ref weapon = Weapon::table();
# let weapon::Schema { ref strength, .. } = Weapon::schema();
# let weapon_stats::Schema {
#     averageStrength, ..
# } = WeaponStats::schema();
#
# let generated_weapons = (0..=14)
#     .map(|i| Weapon {
#         name: format!("weapon_{}", i),
#         strength: i,
#         ..Default::default()
#     })
#     .collect::<Vec<_>>();
#
# insert(generated_weapons).return_many(db.clone()).await?;


let created_stats_statement = create::<WeaponStats>().set(averageStrength.equal_to(block! {
    LET strengths = select_value(strength).from(weapon);
    LET total = math::sum!(strengths);
    LET count = count!(strengths);
    LET distance = 65;
    RETURN math::ceil!((((total / count) * (count * total)) / (total + 4)) * 100);
}));


assert_eq!(
    created_stats_statement.to_raw().build(),
    "CREATE weapon_stats SET averageStrength = {\n\
            LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
            LET $total = math::sum($strengths);\n\n\
            LET $count = count($strengths);\n\n\
            RETURN math::ceil(((($total / $count) * ($count * $total)) / ($total + 4)) * 100);\n\
            };"
);

assert_eq!(
    created_stats_statement.fine_tune_params(),
    "CREATE weapon_stats SET averageStrength = {\n\
            LET $strengths = $_param_00000001;\n\n\
            LET $total = math::sum($strengths);\n\n\
            LET $count = count($strengths);\n\n\
            RETURN math::ceil(((($total / $count) * ($count * $total)) / ($total + $_param_00000002)) * $_param_00000003);\n\
            };"
);
```

This block of code demonstrates the ORM's ability to define and utilize
parameters within queries.

## Native ORM Parameters

SurrealDB provides a set of predefined variables designed to simplify query
development. While these predefined parameters can be utilized directly within
your queries, it's crucial to note that you cannot declare new parameters with
these specific names. The ORM is equipped with built-in functions that represent
these standard SurrealDB parameters. A function like `after()` corresponds to
the `$after` parameter in raw queries. These functions allow developers to
interact with the database at a high level, abstracting away the complexity of
raw queries.

To bridge this system with the ORM, these predefined variables are represented
by functions in the ORM, each mimicking the name of the corresponding parameter:

Here's a list of some of the prominent parameters and their descriptions:

| Function    | Parameter  | Description                                                                                                 |
| ----------- | ---------- | ----------------------------------------------------------------------------------------------------------- |
| `auth()`    | `$auth`    | Represents the currently authenticated scope user.                                                          |
| `token()`   | `$token`   | Represents values held inside the JWT token used for the current session.                                   |
| `session()` | `$session` | Values from session functions as an object.                                                                 |
| `before()`  | `$before`  | Value before a field mutation.                                                                              |
| `after()`   | `$after`   | Value post field mutation.                                                                                  |
| `value()`   | `$value`   | Post mutation value (identical to `$after` for events).                                                     |
| `input()`   | `$input`   | Initially inputted value in a field definition; the value clause might have modified the `$value` variable. |
| `parent()`  | `$parent`  | Parent record in a subquery.                                                                                |
| `event()`   | `$event`   | Type of table event triggered on an event.                                                                  |

These native functions simplify the query-writing process, enabling developers
to focus on the logic of their application without getting bogged down by the
intricacies of the database language.

## Advanced Parameter Name Creation

For those requiring further customization, the `create_param_name_fn!()` macro
is available. This macro not only aids in generating custom parameter names but
also supports field traversal using parameter paths. Typically though, you will
use this with the `define_param` statement when you want to define a constant
global variable. However, in a typical `let statement` (e.g used within the
`block!` macro), this is automatically handled.

Suppose you want to create a custom parameter name for a user's age. Using the
macro:

```rust
create_param_name_fn!(user_age);
```

If you would like to add a rust doc comment, you can do so as shown below:

```rust
create_param_name_fn!(
    /// $user_age represents the age of a user
    => userAge
);
```

To use the param name created above, you can invoke it as `user_age`

This means that any parameter name created with this macro can be used for field
traversal. For more information on field traversal, refer to the
[Field Traversal chapter](./concepts/field_traversal.md).
