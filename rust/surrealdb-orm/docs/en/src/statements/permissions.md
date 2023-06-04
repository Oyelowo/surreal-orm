# For Statement (Permissions)

The `for` statement is used to define permissions for a specific action or CRUD operation in SurrealDB. It allows you to specify the desired permissions and conditions for the action. This statement is commonly used when defining tables or fields in SurrealDB, but it may also be used for access control for other objects in the future. This documentation provides an overview of the syntax and usage of the `for` statement.

## Table of Contents

- [Syntax](#syntax)
- [Permission Types](#permission-types)
- [Examples](#examples)
  - [Define Permission for Single Action](#define-permission-for-single-action)
  - [Define Permissions for Multiple Actions (Individual)](#define-permissions-for-multiple-actions-individual)
  - [Define Permissions for Multiple Actions (Array)](#define-permissions-for-multiple-actions-array)
  - [Define Permissions for Multiple Actions (Mixed)](#define-permissions-for-multiple-actions-mixed)

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

## Permission Types

SurrealDB uses permission types to define different actions or CRUD operations that can be performed on tables or fields. Here are the available permission types:

- `Create`: Grants permission to create new records or objects.
- `Read` (or `Select`): Grants permission to read or retrieve data from records or objects.
- `Update`: Grants permission to modify or update existing records or objects.
- `Delete`: Grants permission to delete records or objects.

These permission types allow you to define fine-grained access control for different actions in your database.

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

### Define Permissions for Multiple Actions (Individual)

To define permissions for multiple actions individually, you can use the following code:

```rust
use CrudType::*;
let name = Field::new("name");

let for_res = for_(Select).where_(age.greater_than_or_equal(18))
    .permissions(for_(Create).where_(name.is("Oyedayo")))
    .permissions(for_(Update).where_(age.less_than_or_equal(130)));
assert_eq!(
    for_res.fine_tune_params(),
    "FOR select\n\tWHERE age >= 18\nPERMISSIONS\n\tFOR create\n\t\tWHERE name IS $_param

_00000002\n\tFOR update\n\t\tWHERE age <= 130"
);

assert_eq!(
    for_res.to_raw().build(),
    "FOR select\n\tWHERE age >= 18\nPERMISSIONS\n\tFOR create\n\t\tWHERE name IS 'Oyedayo'\n\tFOR update\n\t\tWHERE age <= 130"
);
```

In the example above, the `for` statement defines permissions for the `Select` action, as well as individual permissions for the `Create` and `Update` actions. It specifies different conditions for each action. This means that the permissions for these actions will be granted only when the specified conditions are met.

### Define Permissions for Multiple Actions (Array)

To define permissions for multiple actions using an array, you can use the following code:

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

In the example above, the `for` statement defines permissions for multiple actions (`Create`, `Delete`, `Select`, and `Update`) using an array. It specifies a common condition for all the actions. This means that the permissions for these actions will be granted only when the field "name" is equal to "Oyedayo".

### Define Permissions for Multiple Actions (Mixed)

To define permissions for multiple actions using a mix of individual permissions and an array, you can use the following code:

```rust
use CrudType::*;
let name = Field::new("name");

let for_res = for_(&[Create, Delete]).where_(name.is("Oyedayo"))
    .permissions(for_(Update).where_(age.less_than_or_equal(130)));
assert_eq!(
    for_res.fine_tune_params(),
    "FOR create, delete\n\tWHERE name IS $_param_00000001\nPERMISSIONS\n\tFOR update\n\t\tWHERE age <= 130"
);

assert_eq!(
    for_res.to_raw().build(),
    "FOR create, delete\n\tWHERE name IS 'Oyedayo'\nPERMISSIONS\n\tFOR update\n\t\tWHERE age <= 130"
);
```

In the example above, the `for` statement defines individual permissions for the `Create` and `Delete` actions, and an array of permissions for the `Update` action. It specifies different conditions for each action. This means that the permissions for these actions will be granted only when the specified conditions are met.

You have now learned how to define permissions using the `for` statement in SurrealDB. Use this statement to specify the desired access control for different actions or CRUD operations in your database. While it is commonly used when defining tables or fields, it may also be utilized for access control for other objects in the future.
