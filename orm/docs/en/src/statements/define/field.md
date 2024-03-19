# Define Field Statement

The `define_field` statement is used to define a field in SurrealDB. It allows you to specify various options and permissions for the field. This documentation provides an overview of the syntax and usage of the `define_field` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Field with Full Configuration](#define-field-with-full-configuration)
  - [Define Field with Simple Configuration](#define-field-with-simple-configuration)
- [Field Types](#field-types)
- [Geometry Types](#geometry-types)
- [Permission Types](#permission-types)

## Syntax

The basic syntax of the `define_field` statement is as follows:

```rust
define_field(field_name)
    .on_table(table)
    .type_(field_type)
    .value(default_value)
    .assert(assertion)
    .permissions(permission_statements);
```

- `field_name`: The name of the field to define.
- `table`: The name of the table where the field belongs.
- `field_type`: The type of the field.
- `default_value` (optional): The default value for the field.
- `assertion` (optional): An assertion condition for the field.
- `permission_statements` (optional): The permissions for the field.

The `define_field` statement supports the following methods:

- `.on_table(table)`: Specifies the table where the field belongs.
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
    .permissions(for_permission(Permission::Select).where_(age.greater_than_or_equal(18))) // Single permission
    .permissions(for_permission(&[Permission::Create, Permission::Update]).where_(name.is("Oyedayo"))) // Multiple permissions
    .permissions(&[
        for_permission(&[Permission::Create, Permission::Delete]).where_(name.is("Oyedayo")),
        for_permission(Permission::Update).where_(age.less_than_or_equal(130)),
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

In the example above, the `define_field` statement defines a field named "email" on the "user" table. It specifies the field type as `String`, sets a default value of `'example@codebreather.com'`, and adds an

assertion condition. It also sets different permissions for the field based on conditions.

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

## Field Types

The `define_field` statement supports various field types in SurrealDB. The available field types are:

- `any`: Allows any data type supported by SurrealDB.
- `array`: Represents a list.
- `bool`: Represents true or false values.
- `datetime`: Represents an ISO 8601 compliant date with time and time zone.
- `decimal`: Represents any real number with arbitrary precision.
- `duration`: Represents a length of time that can be added or subtracted from datetimes or other durations.
- `float`: Represents a value stored in a 64-bit float.
- `int`: Represents a value stored in a 64-bit integer.
- `number`: Represents numbers without specifying the type, allowing SurrealDB to detect and store the number based on its minimal representation.
- `object`: Represents formatted objects containing values of any supported type.
- `string`: Represents a string value.
- `record`: Represents a reference to another record in any table.
- `geometry`: Represents a geometry type conforming to the GeoJSON format.

## Geometry Types

The `geometry` field type allows you to define geometric fields in SurrealDB. The available geometry types are:

- `feature`: Represents any geometric type.
- `point`: Represents a point.
- `line`: Represents a line.
- `polygon`: Represents a polygon.
- `multipoint`: Represents a multipoint.
- `multiline`: Represents a multiline.
- `multipolygon`: Represents a multipolygon.
- `collection`: Represents a collection of geometry types.

## Permission Types

The `define_field` statement allows you to define permissions for the field using permission types. The available permission types are:

- `Create`: Allows creating new records with the field.
- `Read`: Allows reading the field value from existing records.
- `Update`: Allows updating the field value in existing records.
- `Delete`: Allows deleting records that have the field.

These permission types can be used in the `permissions` method to define the desired access control for the field.

You have now learned how to define fields using the `define_field` statement, including different configuration options, field types, geometry types, and permission types. Use this statement to define fields in SurrealDB and specify their configurations and permissions.
