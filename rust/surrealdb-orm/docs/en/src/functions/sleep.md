# Sleep function

## Table of Contents

- [sleep!()](#sleep-macro)

## sleep!() <a name="sleep-macro"></a>

The `sleep!()` macro suspends the current thread for the specified duration.

**Example:**

```rust
use std::time;
use surrealdb_orm::functions::sleep;

let result = sleep!(time::Duration::from_secs(55));
assert_eq!(result.to_raw().build(), "sleep(55s)");
```
