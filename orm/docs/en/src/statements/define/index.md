# Define Index Statement

The `define_index` statement is used to define an index in SurrealDB. Indexes are used to improve the performance of queries by creating data structures that allow for efficient lookup and retrieval of data. This documentation provides an overview of the syntax and usage of the `define_index` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Index with Single Field](#define-index-with-single-field)
  - [Define Index with Single Column](#define-index-with-single-column)
  - [Define Index with Multiple Fields](#define-index-with-multiple-fields)
  - [Define Index with Multiple Columns](#define-index-with-multiple-columns)

## Syntax

The basic syntax of the `define_index` statement is as follows:

```rust
define_index(index_name: Index)
    .on_table(table: Table)
    .fields(arr![fields: Field])
    .columns(arr![columns: Field])
    .unique()
```

- `index_name`: The name of the index to define.
- `table`: The name of the table on which the index is defined.
- `fields`: An array of fields to include in the index.
- `columns`: An array of columns to include in the index.
- `unique`: Specifies that the index should enforce uniqueness.

The `define_index` statement supports the following features:

- Defining indexes with fields or columns.
- Specifying uniqueness for the index.

## Examples

### Define Index with Single Field

To define an index with a single field, you can use the following code:

```rust
let email = Field::new("email");

let query = define_index("userEmailIndex")
    .on_table("user")
    .fields(email)
    .unique();
```

In the example above, the `define_index` statement defines an index named "userEmailIndex" on the table "user" with the "email" field. The index is marked as unique.

This will generate the following SQL statement:

```sql
DEFINE INDEX userEmailIndex ON TABLE user FIELDS email UNIQUE;
```

### Define Index with Single Column

To define an index with a single column, you can use the following code:

```rust
let email = Field::new("email");

let query = define_index("userEmailIndex")
    .on_table("user")
    .columns(email)
    .unique();
```

In the example above, the `define_index` statement defines an index named "userEmailIndex" on the table "user" with the "email" column. The index is marked as unique.

This will generate the following SQL statement:

```sql
DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;
```

### Define Index with Multiple Fields

To define an index with multiple fields, you can use the following code:

```rust
let age = Field::new("age");
let name = Field::new("name");
let email = Field::new("email");
let dob = Field::new("dob");

let query = define_index("alien_index")
    .on_table("alien")
    .fields(arr![age, name, email, dob])
    .unique();
```

In the example above, the `define_index` statement defines an index named "alien_index" on the table "alien" with the "age", "name", "email", and "dob" fields. The index is marked as unique.

This will generate the following SQL statement:

```sql
DEFINE INDEX alien_index ON TABLE alien FIELDS age, name, email, dob UNIQUE;
```

### Define Index with Multiple Columns

To define an index with multiple columns, you can use the

following code:

```rust
let age = Field::new("age");
let name = Field::new("name");
let email = Field::new("email");
let dob = Field::new("dob");

let query = define_index("alien_index")
    .on_table("alien")
    .columns(arr![age, name, email, dob])
    .unique();
```

In the example above, the `define_index` statement defines an index named "alien_index" on the table "alien" with the "age", "name", "email", and "dob" columns. The index is marked as unique.

This will generate the following SQL statement:

```sql
DEFINE INDEX alien_index ON TABLE alien COLUMNS age, name, email, dob UNIQUE;
```

You have now learned how to define indexes using the `define_index` statement. Indexes improve query performance by creating data structures that allow for efficient lookup and retrieval of data. Use indexes strategically to optimize the performance of your database queries.
