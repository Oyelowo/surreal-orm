# Count Function

This chapter introduces the `count` macros provided by SurrealDB ORM. The `count` macros are used to generate SQL queries for counting records in a database table.

## Table of Contents

- [count!()](#count)
- [count!().\_\_as\_\_(alias)](#count-alias)
- [count!(field)](#count-field)
- [count!(field.operation(value))](#count-field-operation)
- [count!(condition1.and(condition2))](#count-condition-and)
- [count!(array)](#count-array)

## count!()

The `count!()` macro counts all records in a table. It generates the SQL query `count()`.

```rust
use surreal_orm::{count, *};

let result = count!();
```

Generated SQL query:

```sql
count()
```

The `count!()` macro provides the following functionality:

- `to_raw().build()`: Converts the `count` macro into a raw SQL query string. In this case, it would be `"count()"`.

## count!().\_\_as\_\_(alias)

The `count!().__as__(alias)` macro allows you to specify an alias for the count result. It generates the SQL query `count() AS alias`.

```rust
use surreal_orm::{count, AliasName};

let head_count = AliasName::new("head_count");
let result = count!().__as__(head_count);
```

Generated SQL query:

```sql
count() AS head_count
```

The `count!().__as__(alias)` macro provides the same functionality as `count!()`, but with an additional `AS` clause to specify the alias for the count result.

## count!(field)

The `count!(field)` macro counts records in a table based on a specific field. It generates the SQL query `count(field)`.

```rust
use surreal_orm::{count, Field};

let email = Field::new("email");
let result = count!(email);
```

Generated SQL query:

```sql
count(email)
```

The `count!(field)` macro provides the same functionality as `count!()`, but with a specific field to count records on.

## count!(field.operation(value))

The `count!(field.operation(value))` macro allows you to perform filter operations on the count. It generates the SQL query `count(field.operation(value))`.

```rust
use surreal_orm::{count, Field};

let email = Field::new("email");
let result = count!(email.greater_than(15));
```

Generated SQL query:

```sql
count(email > 15)
```

The `count!(field.operation(value))` macro provides the same functionality as `count!(field)`, but with a filter operation applied to the field.

## count!(condition1.and(condition2))

The `count!(condition1.and(condition2))` macro allows you to apply multiple conditions to the count. It generates the SQL query `count(condition1 AND condition2)`.

```rust
use surreal_orm::{count, Field, cond};

let email = Field::new("email");
let age = Field::new("age");
let result = count!(cond(age.greater_than(15)).and(email.like("oyelowo@example.com")));
```

Generated SQL query:

```sql
count((age > 15) AND (email ~ 'oyelowo@example.com'))
```

The `count!(condition1.and(condition2))

`macro provides the same functionality as`count!(field.operation(value))`, but with multiple conditions combined using the `AND` operator.

## count!(array)

The `count!(array)` macro counts the number of elements in an array. It generates the SQL query `count(array)`.

```rust
use surreal_orm::{count, array};

let result = count!(array![1, 2, 3, 4, 5]);
```

Generated SQL query:

```sql
count([1, 2, 3, 4, 5])
```

The `count!(array)` macro provides the same functionality as `count!()`, but with an array as the input for counting.
