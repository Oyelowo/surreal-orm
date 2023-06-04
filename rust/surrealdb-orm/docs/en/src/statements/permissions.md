# For Statement (Permissions)

The `for` statement is used to define permissions for a specific action or CRUD operation in SurrealDB. It allows you to specify the desired permissions and conditions for the action. This documentation provides an overview of the syntax and usage of the `for` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Define Permission for Single Action](#define-permission-for-single-action)
  - [Define Permissions for Multiple Actions](#define-permissions-for-multiple-actions)

## Syntax

The basic syntax of the `for` statement is as follows:

```rust
for_(permission_type)
    .where_(condition);
```

- `permission_type`: The type of permission or action for which you want to define permissions. It can be a single permission type or an array of permission types.
- `condition`: The condition or criteria for the permission. It specifies the conditions under which the permission should be granted.

The `for` statement supports the following methods:

- `.where_(condition)`: Specifies the condition or criteria for the permission.

## Examples

### Define Permission for Single Action

To define permissions for a single action, you can use the following code:

```rust
use CrudType::*;
let name = Field::new("name");

let for_res = for_(Create).where_(name.like("Oyelowo"));
assert_eq!(
    for_res.fine_tune_params(),
    "FOR create\n\tWHERE name ~ $_param_00000001"
);
assert_eq!(
    for_res.to_raw().build(),
    "FOR create\n\tWHERE name ~ 'Oyelowo'"
);
```

In the example above, the `for` statement defines permissions for the `Create` action. It specifies the condition that the field "name" should be matched with the pattern "Oyelowo". This means that the permission to create records will be granted only when the field "name" matches the pattern.

### Define Permissions for Multiple Actions

To define permissions for multiple actions, you can use the following code:

```rust
use CrudType::*;
let name = Field::new("name");

let for_res = for_(&[Create, Delete, Select, Update]).where_(name.is("Oyedayo"));
assert_eq!(
    for_res.fine_tune_params(),
    "FOR create, delete, select, update\n\tWHERE name IS $_param_00000001"
);

assert_eq!(
    for_res.to_raw().build(),
    "FOR create, delete, select, update\n\tWHERE name IS 'Oyedayo'"
);
```

In the example above, the `for` statement defines permissions for multiple actions: `Create`, `Delete`, `Select`, and `Update`. It specifies the condition that the field "name" should be equal to "Oyedayo". This means that the permissions for these actions will be granted only when the field "name" matches the specified value.

You have now learned how to define permissions using the `for` statement in SurrealDB. Use this statement to specify the desired access control for different actions or CRUD operations in your database.
