# Define Field Statement

The `define_field` statement is used to define a field in SurrealDB. It allows you to specify various options and permissions for the field. This documentation provides an overview of the syntax and usage of the `define_field` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Field with Full Configuration](#define-field-with-full-configuration)
  - [Define Field with Simple Configuration](#define-field-with-simple-configuration)

## Syntax

The basic syntax of the `define_field` statement is as follows:

```rust
define_field(field_name)
    .on_table(table_name)
    .type_(field_type)
    .value(default_value)
    .assert(assertion)
    .permissions(permission_statements);
```

- `field_name`: The name of the field to define.
- `table_name`: The name of the table where the field belongs.
- `field_type`: The type of the field.
- `default_value` (optional): The default value for the field.
- `assertion` (optional): An assertion condition for the field.
- `permission_statements` (optional): The permissions for the field.

The `define_field` statement supports the following methods:

- `.on_table(table_name)`: Specifies the table where the field belongs.
- `.type_(field_type)`: Specifies the type of the field.
- `.value(default_value)`: Specifies the default value for the field.
- `.assert(assertion)`: Specifies an assertion condition for the field.
- `.permissions(permission_statements)`: Specifies the permissions for the field.

## Examples

### Define Field with Full Configuration

To define a field with full configuration, including a default value, assertion condition, and permissions, you can use the following code:

```rust
let email = Field::new("email");
let user_table = Table::from("user");
let age = Field::new("age");
let statement = define_field(email)
    .on_table(user_table)
    .type_(String)
    .value("example@codebreather.com")
    .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
    .permissions(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
    .permissions(for_(&[Create, Update]).where_(name.is("Oyedayo"))) // Multiple
    .permissions(&[
        for_(&[Create, Delete]).where_(name.is("Oyedayo")),
        for_(Update).where_(age.less_than_or_equal(130)),
    ]);
```

This will generate the following SQL statement:

```sql
DEFINE FIELD email ON TABLE user TYPE string VALUE 'example@codebreather.com' \
    ASSERT ($value IS NOT NONE) AND ($value ~ 'is_email')
PERMISSIONS
    FOR select
        WHERE age >= 18
    FOR create, update
        WHERE name IS 'Oyedayo'
    FOR create, delete
        WHERE name IS 'Oyedayo'
    FOR update
        WHERE age <= 130;
```

In the example above, the `define_field` statement defines a field named "email" on the "user" table. It specifies the field type as `String`, sets a default value of `'example@codebreather.com'`, and adds an assertion condition. It also sets different permissions for the field based on conditions.

### Define Field with Simple Configuration

To define a field with a simple configuration, you can use the following code:

```rust
use FieldType::*;

let email = Field::new("email");
let user_table = Table::from("user");
let statement = define_field(email).on_table(user_table).type_(String);
```

This will generate the following SQL statement:

```sql
DEFINE FIELD email ON TABLE user TYPE string;
```

In the example above, the `define_field` statement defines a field named "email" on the "user" table. It specifies the field type as `String` without setting a default value, assertion condition, or permissions.

This concludes the documentation for the `define_field` statement. Use this statement to define fields in SurrealDB and specify their configurations and permissions.
