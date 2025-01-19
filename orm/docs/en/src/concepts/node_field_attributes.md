# Chapter: Node Field Attributes

## Table of Contents

1. [Introduction](#introduction)
   - [Attributes Table](#attributes-table)
2. [Basic Annotations](#basic-annotations)
3. [Granular Attributes](#granular-attributes)
4. [Defining Attributes with Functions](#defining-attributes-with-functions)
5. [Field Definitions](#field-definitions)
6. [Links and Relationships](#links-and-relationships)
7. [Customizing Behavior with Inline Expressions](#customizing-behavior-with-inline-expressions)
8. [Invalid Usages](#invalid-usages)
9. [Summary and Conclusion](#summary-and-conclusion)

---

## 1. Introduction <a name="introduction"></a>

Field attributes in Surreal orm allow developers to fine-tune the behavior and
characteristics of each field within a database node. As you've already seen in
the table of attributes, each attribute serves a specific purpose. In this
chapter, we'll delve deeper into each attribute, providing examples and
clarifying common misconceptions.

---

### Attributes Table <a name="attributes-table"></a>

{{#include ../../../../src/docs/node_field_attributes.md}}

## 2. Basic Annotations <a name="basic-annotations"></a>

Let's begin with a basic example. The `Student` struct below uses minimal
annotations:

```rust
#[orm(table = "student")]
pub struct Student {
    id: SurrealId<Student, String>,
    first_name: String,
    last_name: String,
    age: u8,
}
```

Here:

- `table` determines the name of the table in the database that corresponds
  to this struct.

---

## 3. Granular Attributes <a name="granular-attributes"></a>

For a more detailed configuration of a field, you can use granular attributes.
The `Student` struct provides various usages:

```rust
#[orm(
    table = "student",
    permissions = "student_permissions()",
)]
pub struct Student {
    id: SurrealId<Student, String>,
    first_name: String,
    last_name: String,
    #[orm(
        type_ = "int",
        value = "18",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "age_permissions()"
    )]
    age_inline_expr: u8,
    // ... other fields ...
}
```

Here:

- `type` specifies the data type of the field in the database.
- `value` sets a default value for the field.
- `assert` provides a condition that the field value must satisfy.
- `permissions` specifies what operations can be performed on the field and
  under what conditions.

---

## 4. Defining Attributes with Functions <a name="defining-attributes-with-functions"></a>

You can externalize the logic for defining attributes by using external
functions. This aids in reusability and cleaner code:

```rust
#[orm(
    table = "student_with_define_fn_attr",
    define_fn = "define_student_with_define_attr"
)]
pub struct StudentWithDefineFnAttr {
    // ... fields ...
    #[orm(type_ = "int", define_fn = "age_define_external_fn_path")]
    age_define_external_fn_path: u8,
}
```

Here:

- `define_fn` allows you to specify an external function that returns the
  definition of the table or field.

---

## 5. Field Definitions <a name="field-definitions"></a>

Fields can be defined in multiple ways using `surreal_orm`:

### Inline Definitions:

```rust
#[orm(type_ = "int", value = "18")]
age: u8,
```

### External Function Invoked:

```rust
#[orm(type_ = "int", value = "get_age_default_value()")]
age_default_external_function_invoked_expr: u8,
```

### Using External Function Attributes:

```rust
#[orm(type_ = "int", value_fn = "get_age_default_value")]
age_external_fn_attrs: u8,
```

### Mixing and Matching:

```rust
#[orm(type_ = "int", value = "get_age_default_value()", assert_fn = "get_age_assertion")]
age_mix_and_match_external_fn_inline_attrs: u8,
```

---

## 6. Links and Relationships <a name="links-and-relationships"></a>

You can define relationships between different structs (representing tables in
the database). Relationships can be `one-to-one`, `one-to-many`, or
`many-to-many`.

For instance:

```rust
#[orm(link_one = "Book")]
fav_book: LinkOne<Book>,
```

This indicates a one-to-one relationship between a student and a book.

---

## 7. Customizing Behavior with Inline Expressions <a name="customizing-behavior-with-inline-expressions"></a>

In `surreal_orm`, you can use inline expressions to add custom behavior:

```rust
#[orm(
    type_ = "int",
    value = "get_age_by_group_default_value(AgeGroup::Teen)",
    assert = "get_age_assertion()",
)]
age_teen_external_function_invoked_expr: u8,
```

Here, the default value of `age_teen_external_function_invoked_expr` is
determined by the `get_age_by_group_default_value` function with
`AgeGroup::Teen` as a parameter.

---

## 8. Invalid Usages <a name="invalid-usages"></a>

When using `surreal_orm`, it's essential to be cautious about the attributes you
combine. Certain combinations are considered invalid and will result in
compilation errors.

### 1. Mixing `value` and `value_fn`:

These two attributes are mutually exclusive. You can't define a default value
using both a direct expression and a function at the same time.

```rust
#[orm(
    type_ = "int",
    value = "get_age_default_value()",
    value_fn = "get_age_default_value"
)]
age: u8,
```

### 2. Mixing `assert` and `assert_fn`:

Similarly, you can't use both an inline assertion and an external function for
the same purpose.

```rust
#[orm(
    type_ = "int",
    assert = "get_age_assertion()",
    assert_fn = "get_age_assertion"
)]
age: u8,
```

### 3. Mixing `permissions` and `permissions_fn`:

Permissions should be defined either inline or through an external function, but
not both.

```rust
#[orm(
    type_ = "int",
    permissions = "age_permissions()",
    permissions_fn = "age_permissions"
)]
age: u8,
```

### 4. Combining `define` and `define_fn`:

These attributes are also mutually exclusive. When specifying a custom
definition, you should use either an inline expression or an external function.

```rust
#[orm(
    type_ = "int",
    define = "define_age()",
    define_fn = "define_age"
)]
age: u8,
```

### 5. Using other attributes with `define` or `define_fn`:

When you use either the `define` or `define_fn` attribute, you cannot use any
other attributes (except for `type`). This is because the definition provided
should be comprehensive and not require additional modifiers.

For example, the following combinations are invalid:

```rust
#[orm(
    type_ = "int",
    value = "18",
    define = "define_age()"
)]
age: u8,
```

```rust
#[orm(
    type_ = "int",
    assert = "cond(value().is_not(NONE)).and(value().gte(18))",
    define = "define_age()"
)]
age: u8,
```

```rust
#[orm(
    type_ = "int",
    permissions = "for_permission([CrudType::Create, CrudType::Delete]).where_(StudentTest3::schema().firstName.is(\"Oyelowo\"))",
    define = "define_age()"
)]
age: u8,
```

By being aware of these restrictions and avoiding the invalid combinations, you
can ensure that your code remains consistent, clear, and free from compilation
errors.

---

## 9. Summary and Conclusion

With `surreal_orm`, you can easily map Rust structs to database tables,
customize field properties, define relationships, and more. This provides a
powerful way to interact with databases in a type-safe manner while keeping the
codebase clean and maintainable.

For a hands-on illustration, consider the following code snippet which provides
a comprehensive overview of the various annotations:

```rust
#[derive(Node, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(
    table = "student_with_granular_attributes",
    drop,
    schemafull,
    as_ = "select(All).from(Student::table())",
    permissions = "student_permissions()",
)]
pub struct StudentWithGranularAttributes {
    id: SurrealId<StudentWithGranularAttributes, String>,
    first_name: String,
    last_name: String,
    #[orm(
        type_ = "int",
        value = "18",
        assert = "cond(value().is_not(NONE)).and(value().gte(18))",
        permissions = "for_permission([CrudType::Create, CrudType::Delete]).where_(StudentWithGranularAttributes::schema().firstName.is(\"Oyelowo\"))"
    )]
    age_inline_expr: u8,
    // ... other fields ...
}
```

This chapter is a starting point to dive deeper into `surreal_orm`. With this
foundation, you can explore more advanced features and best practices to make
the most of this powerful ORM crate in Rust.
