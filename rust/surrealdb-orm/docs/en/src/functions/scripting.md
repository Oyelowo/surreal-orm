# Scripting function

## Table of Contents

- [function!()](#function-macro)

## function!() <a name="function-macro"></a>

The `function!()` macro allows you to define JavaScript functions with different parameters and function bodies.

**Example:**

```rust
use surrealdb_orm::macros::function;
use surrealdb_orm::statements::let_;

let value = let_("value").equal_to("SurrealDB").get_param();
let words = let_("words").equal_to(vec!["awesome", "advanced", "cool"]).get_param();

let f2 = function!(
    (value, words),
    "{ return `${arguments[0]} is ${arguments[1]}`; }"
);

assert_eq!(
    f2.build(),
    "function($value, $words) { return `${arguments[0]} is ${arguments[1]}`; }"
);

assert_eq!(
    f2.to_raw().build(),
    "function($value, $words) { return `${arguments[0]} is ${arguments[1]}`; }"
);
```
