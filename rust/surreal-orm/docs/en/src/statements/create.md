# Create Statement

The `create` statement is used to add new entries to the SurrealDB database. It
allows you to create records with specified content and set additional
properties for the query. This documentation provides an overview of the syntax
and usage of the `create` statement.

## Table of Contents

- [Syntax](#syntax)
- [Examples](#examples)
  - [Basic Create Statement with Content Method](#basic-create-statement-with-content-method)
  - [Creating Linked Entities](#creating-linked-entities)
  - [Using the object! Macro](#using-the-object-macro)
  - [Create with Set Method](#create-with-set-method)

## Syntax

The basic syntax of the `create` statement is as follows:

```rust
create()
    .content(record_content)
    .set(set_statements)
    .return_type(return_types)
    .timeout(seconds)
    .parallel();
```

The `create` statement supports the following methods:

- `.content(record_content)`: Specifies the content of the record to be created.
- `.set(set_statements)`: Sets the values of the fields to be updated in the
  record.
- `.return_type(return_types)`: Specifies the return type for the query.
- `.timeout(seconds)`: Sets the timeout duration for the query.
- `.parallel()`: Indicates whether the query should be executed in parallel.

## Examples

### Basic Create Statement with Content Method

To create a basic record using the `create` statement, you can use the following
code:

```rust
let space_ship1 = create()
    .content(space_ship1.clone())
    .get_one(db.clone())
    .await?;
```

This code will create a new entry for `space_ship1` in the database.

### Creating Linked Entities

The `create` statement allows you to create entries that have links to other
entities. Here's an example of creating a linked entity:

```rust
let unsaved_alien = Alien {
    ...
    space_ships: LinkMany::from(vec![
        created_spaceship1.clone(),
        created_spaceship2.clone(),
        space_ship3.clone(),
    ]),
    ...
};

let created_alien_with_fetched_links = create()
    .content(unsaved_alien.clone())
    .load_link_manys()?
    .return_one(db.clone())
    .await?;
```

In this example, `unsaved_alien` is being created with links to three different
spaceships. The `.load_link_manys()` method loads the linked entities in a
single statement.

### Create Using the `object!` Macro

The `object!` macro provides a concise and type-safe way to specify values when
creating or updating records when using the `set` method. It acts as syntactic
sugar for an array of setters but provides stricter field checking compared to
using a basic struct.

Here's an example showcasing its usage:

```rust
let spaceship_id_1 = SpaceShip::create_id("spaceship1".to_string());

let space_ship1 = create::<SpaceShip>()
    .set(object!(SpaceShip {
        id: spaceship_id_1,
        name: "SpaceShip1",
        created: Utc::now(),
    }))
    .get_one(db.clone())
    .await?;
assert_eq!(space_ship1.name, "SpaceShip1");
```

Using the `object!` macro ensures all fields are provided and belong to the
specified struct. It also allows for using `parameter` or `field` as values.
This is recommended over using array of setters as shown next because of the
extra checks this provides.

Additionally, the `object_partial!` macro functions similarly but allows for
omitting some fields. This is particularly useful for update statements where
only a subset of fields need to be changed.

```rust
let updated = update::<Weapon>(id)
    .set(object_partial!(Weapon { strength: 923u64 }))
    .return_one(db.clone())
    .await?;
```

### Create with Set Method

You can use the `set` method with the `create` statement to set specific fields
of the record being created. The `set` method supports multiple approaches for
specifying the setter statements:

1. Using an array const (`&[T]`):

```rust
let space_ship2 = create::<SpaceShip>()
    .set([
        id.equal_to(spaceship_id_2),
        name.equal_to("SpaceShip2".to_string()),
        created.equal_to(Utc::now()),
    ])
    .get_one(db.clone())
    .await?;
```

2. Using a `Vec` of setter statements:

```rust
let space_ship1 = create::<SpaceShip>()
    .set(vec![
        id.equal_to(spaceship_id_1),
        name.equal_to("SpaceShip1".to_string()),
        created.equal_to(Utc::now()),
    ])
    .get_one(db.clone())
    .await?;
```

In these examples, we demonstrate different ways to use the `set` method. You
can use an array const (`[T]` or `&[T]`) or a `Vec` to provide a list of setter
statements.

This concludes the documentation for the `create` statement. Use this statement
to add new entries to the SurrealDB database with desired content and additional
properties.
