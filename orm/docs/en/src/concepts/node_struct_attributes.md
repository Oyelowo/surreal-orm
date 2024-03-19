# Node Attributes on Struct

In Surreal ORM, node attributes provide a convenient mechanism to dictate the
behavior and structure of database tables and their associated fields. These
attributes are not only powerful tools for developers but also help in
maintaining a consistent and clear database schema. This chapter will delve into
the intricacies of node attributes, their application, and best practices for
their usage.

## Table of Contents

1. [Introduction to Node Attributes](#introduction-to-node-attributes)
2. [Working with Node Attributes](#working-with-node-attributes)
   - [Supported Table Attributes](#supported-table-attributes)
3. [Node Attributes: Examples](#node-attributes-examples)
   - [Auto-Inferred Table Name](#auto-inferred-table-name)
   - [Explicit Table Name](#explicit-table-name)
   - [Using `define` for Inline Table Definition](#using-define-for-inline-table-definition)
   - [Using `define_fn` for External Function Definition](#using-define_fn-for-external-function-definition)
   - [Specifying Permissions](#specifying-permissions)
4. [Ensuring Valid Usage of Node Attributes](#ensuring-valid-usage-of-node-attributes)
   - [Conflicting Definitions](#conflicting-definitions)
   - [Avoid Excessive Attributes with `define` or `define_fn`](#avoid-excessive-attributes-with-define-or-define_fn)
   - [Consistent Table Naming](#consistent-table-naming)
   - [Using Functions for Attributes](#using-functions-for-attributes)
5. [Conclusion](#conclusion)

---

## Introduction to Node Attributes

Node attributes in Surreal ORM allow developers to:

- Rename fields of a struct according to a naming convention.
- Explicitly set or infer the table name.
- Enforce schema structures.
- Handle table drops and recreations.
- Create table projections or views.
- Set granular permissions for CRUD operations on tables.
- Define the table structure either inline or through external functions.

## Working with Node Attributes

### Supported table attributes

{{#include ../../../../src/docs/node_struct_attributes.md}}

## Node Attributes: Examples

### Auto-Inferred Table Name

By default, the ORM auto-infers the table name from the struct's name. For a
struct named `Alien`, the table name would be inferred as `alien`.

```rust
#[derive(Node, Serialize, Deserialize)]
pub struct Alien {
    id: SurrealSimpleId<Self>,
}
```

The corresponding table definition would be:

```
DEFINE TABLE alien;
```

### Explicit Table Name

You can explicitly set the table name using the `table` attribute. By
default, the table name should be the snake case of the struct name. This is to
ensure consistency and uniqueness of table model struct. If you want a name
other than the snake case version, you need to add the attribute -
`relax_table`:

```rust
#[derive(Node, Serialize, Deserialize)]
#[surreal_orm(table = "student_test")]
pub struct StudentTest {
    id: SurrealSimpleId<Self>,
}
```

The corresponding table definition would be:

```
DEFINE TABLE student_test;
```

### Using `define` for Inline Table Definition

The `define` attribute allows for inline table definitions, either through an
inline expression or an invoked external function.

```rust
#[derive(Node, Serialize, Deserialize)]
#[surreal_orm(table = "student_test_4", as_ = "select(All).from(Student::table())", define = "define_student()")]
pub struct StudentTest4 {
    id: SurrealSimpleId<Self>,
}
```

### Using `define_fn` for External Function Definition

Alternatively, the `define_fn` attribute points to an external function to
define the table:

```rust
#[derive(Node, Serialize, Deserialize)]
#[surreal_orm(table = "student_test_7", define_fn = "define_student")]
pub struct StudentTest7 {
    id: SurrealSimpleId<Self>,
}
```

### Specifying Permissions

The `permissions` attribute allows you to set granular permissions for CRUD
operations. This takes `Permissions` struct. Therefore, if you are using an
external function, it has to return `Permissions` which is then invoked and
passed in:

```rust
#[derive(Node, Serialize, Deserialize)]
#[surreal_orm(table = "student_test_5", permissions = "student_permissions()")]
pub struct StudentTest5 {
    id: SurrealSimpleId<Self>,
}
```

In the example above, the `student_permissions()` function would define
permissions using the `for` statement from Surreal orm. `for` returns
`Permissions`.

## Ensuring Valid Usage of Node Attributes

While node attributes are powerful and flexible, their misuse can lead to
unexpected behaviors. Thankfully, the ORM actively checks for invalid usages and
ensures that developers don't misuse these attributes. Here are some guidelines
and checks enforced by the ORM to avoid pitfalls:

### **Conflicting Definitions**:

- **`define` vs `define_fn`**: Using both `define` and `define_fn` attributes on
  the same struct is not allowed . Only one should be present to define the
  table.

```rust
#[derive(Node, Serialize, Deserialize)]
#[surreal_orm(table = "student_test_6", define_fn = "define_student", define = "define_student()")]
pub struct StudentTest6 {
    id: SurrealSimpleId<Self>,
}
```

The ORM will raise an error for such definitions, ensuring clarity and
preventing conflicts.

---

- **`as` vs `as_fn`**: Only one of these should be used to define projections or
  views.

- **`permissions` vs `permissions_fn`**: These attributes shouldn't coexist on
  the same struct, choose one based on your need.

- **`value` vs `value_fn`** and **`assert` vs `assert_fn`**: Similar to the
  above, only one of these pairs should be present on a struct.

### **Avoid Excessive Attributes with `define` or `define_fn`**:

When using `define` or `define_fn`, ensure no other attributes are present
except `table` and `relax_table`.

### **Consistent Table Naming**:

By default, the table name should be the snake case of the struct name. This is
to ensure consistency and uniqueness of table model struct. If you want a name
other than the snake case version, you need to add the attribute -
`relax_table`.

### **Using Functions for Attributes**:

When using attributes that invoke functions, such as
`define = "define_student()"`, ensure that the invoked function returns the
appropriate type. For instance, `define_student()` should return a
`DefineStatement` struct, and `student_permissions()` should return
`Permissions`.

### Conclusion

By following these guidelines and the checks enforced by the ORM, developers can
ensure a smooth and error-free database definition process. Remember, while the
ORM provides these checks, it's always a good practice for developers to
validate and review their implementations to guarantee best practices and avoid
potential pitfalls.
