# Update Statement

The `update` statement in SurrealDB ORM allows you to modify existing records in your database.
It provides various operations to update fields and perform incremental changes to the data.
This documentation provides an overview of the syntax and usage of the `update` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Updating a Single Object](#updating-a-single-object)
    - [Using the Update Content](#using-the-update-content)
    - [Using the Set Operation](#using-the-set-operation)
    - [Using the Merge Operation](#using-the-merge-operation)
    - [Using the Replace Operation](#using-the-replace-operation)
    - [Using the Patch Operation](#using-the-patch-operation)
      - [Using the Patch Add Operation](#using-the-patch-add-operation)
      - [Using the Patch Replace Operation](#using-the-patch-replace-operation)
      - [Using the Patch Remove Operation](#using-the-patch-remove-operation)
      - [Using the Patch Change Operation](#using-the-patch-change-operation)
  - [Updating Multiple Objects](#updating-multiple-objects)

## Syntax

The basic syntax of the `update` statement is as follows:

```rust
update::<Type>(id)
    .content(content)
    .merge(merge)
    .replace(replace)
    .set(settables)
    .patch(patch_op)
    .where_(condition)
    .return_type(return_type)
    .timeout(duration)
    .parallel();
```

The `update` statement supports the following methods:

- `.content(content)`: Sets the content of the update statement.
- `.merge(merge)`: Performs a merge operation to update specific fields.
- `.replace(replace)`: Replaces the entire object with a new one.
- `.set(settables)`: Sets the values of the fields to be updated.
- `.patch(patch_op)`: Applies patch operations to the record.
- `.where_(condition)`: Adds a condition to the update statement.
- `.return_type(return_type)`: Specifies the desired return type for the query.
- `.timeout(duration)`: Sets the timeout duration for the query.
- `.parallel()`: Executes the query in parallel.

Note: Only one of the .content(), .merge(), .replace(), .set(), or .patch() methods can be used at a time.

## Examples

### Updating a Single Object

#### Using the Update Content

The `update` statement also supports the `content` method, which allows you to specify the updated fields using a separate object.
This provides a convenient way to define the fields to be updated.

```rust
let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();

let weapon_to_update = Weapon {
    name: "Oyelowo".to_string(),
    created: Utc::now(),
    strength: 1000,
    ..Default::default()
};

let updated_weapon = update::<Weapon>(created_weapon.clone().id)
    .content(weapon_to_update)
    .get_one(db.clone())
    .await?;
```

In the above example, the `content` method is used to specify the fields to be updated in the `created_weapon` object using the `weapon_to_update` object.

#### Using the Set Operation

The `update` statement also supports the `set` method, which allows you to perform 3 major kinds of updates
including, overwriting a field with an `equal_to` method, `increment` and `decrement` method operations for
numbers, `append` and `remove` methods for arrays. All the arguments to these methods are type-checked at compile-
time to make sure they are valid for the respective fields

1. Use set method for a single field

```rust
let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();

let weapon_to_update = Weapon {
    name: "Oyelowo".to_string(),
    created: Utc::now(),
    strength: 1000,
    ..Default::default()
};

update::<Weapon>(weapon_to_update.id)
    .set(strength.increment_by(5u64))
    .run(db.clone())
    .await?;

// You can even pass the entire model instance as an argument
update::<Weapon>(weapon_to_update)
    .set(strength.increment_by(5u64))
    .run(db.clone())
    .await?;
```

2. Use set methods for updating multiple fields

```rust
update::<Weapon>(id)
    .set([
        strength.increment_by(5u64),
        name.equal("Oyedayo"),
    ])
    .run(db.clone())
    .await?;

// In addition to array const `[T]`,you can also use a `vec!`.
update::<Weapon>(id)
    .set(vec![
        strength.increment_by(5u64),
        name.equal("Oyedayo"),
    ])
    .run(db.clone())
    .await?;
```

In the above example, the `set` method is used to specify the fields to be updated in the `created_weapon` object using the `weapon_to_update` object.

#### Using the Merge Operation

The `merge` operation allows you to update a single object by merging new values into the existing object.
The new values overwrite the old ones, while fields not present in the new object are unaffected.

```rust
let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();

let weapon_to_update = Weapon {
    name: "Oyelowo".to_string(),
    created: Utc::now(),
    strength: 1000,
    ..Default::default()
};

let updated_weapon = update::<Weapon>(created_weapon.clone().id)
    .merge(weapon_to_update)
    .get_one(db.clone())
    .await?;

```

In the above example, the `merge` operation is used to update the `created_weapon` object with the
fields from `weapon_to_update`. The result is stored in `updated_weapon`.

#### Using the Replace Operation

The `replace` operation allows you to replace an existing object entirely with a new one.
This operation removes all fields not present in the new object.

```rust
let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();

let weapon_to_replace = Weapon {
    name: "Oyelowo".to_string(),
    created: Utc::now(),
    strength: 823,
    ..Default::default()
};

let updated_weapon = update::<Weapon>(created_weapon.clone().id)
    .replace(weapon_to_replace)
    .get_one(db.clone())
    .await?;
```

In the above example, the `replace` operation replaces the `created_weapon` object with the fields
from `weapon_to_replace`. The result is stored in `updated_weapon`.

#### Using the Patch Operation

The `patch` operation allows you to perform detailed modifications on fields using methods
such as `patch_change`, `patch_replace`, `patch_remove`, and `patch_add`. It enables incremental
changes to string fields, replacing field values, removing fields, or adding new fields.

##### Using the Patch Add Operation

The `patch_add` operation adds a new field to the object. It allows you to include additional fields during the update.

1. Applying single patch

```rust
let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();

let updated_weapon = update::<Weapon>(created_weapon.clone().id)
    .patch(nice.patch_add(true))
    .get_one(db.clone())
    .await?;
```

2. Applying multiple patches

```rust
let ref _updated_weapon = update::<WeaponOld>(old_weapon.clone().id)
    .patch([nice.patch_add(true), bunchOfOtherFields.patch_add(56)])
    .return_one(db.clone())
    .await;

```

In the above example, the `patch_add` operation adds the `nice` field with the value `true` to the `created_weapon` object.

##### Using the Patch Replace Operation

The `patch_replace` operation replaces the value of a field with a new value. It allows you to update a field to a different value.

```rust
let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();

let updated_weapon = update::<Weapon>(created_weapon.clone().id)
    .patch(strength.patch_replace(34u64))
    .get_one(db.clone())
    .await?;
```

In the above example, the `patch_replace` operation replaces the value of the `strength` field in the `created_weapon` object with the specified value.

##### Using the Patch Remove Operation

The `patch_remove` operation removes a field from the object entirely. This operation is destructive,
and the field will no longer be available after the update. Make sure that the struct used here does not
require that field to be present. You can create a copy of the existing struct but without the new field.

```rust
let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();

let updated_weapon = update::<Weapon>(created_weapon.clone().id)
    .patch(bunchOfOtherFields.patch_remove())
    .get_one(db.clone())
    .await?;
```

In the above example, the `patch_remove` operation removes the `bunchOfOtherFields` field from the `created_weapon` object.

##### Using the Patch Change Operation

The `patch_change` operation modifies part of a string field using the diff format. It allows you to specify the changes to be applied to the field.

```rust
let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();

let updated_weapon = update::<Weapon>(created_weapon.clone().id)
    .patch(name.patch_change("@@ -1,4 +1,4 @@\n te\n-s\n+x\n t\n"))
    .get_one(db.clone())
    .await?;
```

In the above example, the `patch_change` operation modifies the `name` field of the `created_weapon` object by changing "test" to "text".

### Updating Multiple Objects

To update multiple objects, you can use the `update` statement with a filter to select the objects to update.

```rust
let filter = cond(strength.greater_than(5)).and(strength.less_than_or_equal(15));

let update_weapons_with_filter = update::<Weapon>(Weapon::table_name())
    .content(Weapon {
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        ..Default::default()
    })
    .where_(filter)
    .return_many(db.clone())
    .await?;
```

In the above example, the `update` statement updates all `Weapon` objects that meet the specified filter condition with the new values.

Please note that the above code snippets are for illustration purposes and may need to be adapted to your specific use case.

You have now learned how to use the `update` statement to modify existing records in your SurrealDB database.
Use the various operations and methods provided by the `update` statement to perform precise updates and incremental changes to your data.
