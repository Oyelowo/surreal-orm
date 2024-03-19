# Record Ids

The `SurrealId` is a wrapper struct that extends the capabilities of
`surrealdb::sql::Thing` and provides a more ergonomic interface. It's a static
type representing the id of a model in the Surreal ORM and is a combination of
the model's table name and the id, where the id can be anything that can be
converted into a `surrealdb::sql::Id`.

Let's explore how to utilize these ID types both implicitly (through
auto-generation via the Default trait) and explicitly (by creating them
manually).

1. **SurrealSimpleId<Self>:**

This ID type auto-generates a unique identifier when a new instance of the
struct is created, thanks to the implementation of the Default trait. But you
can also manually generate it using the `create_simple_id()` function directly
on the struct.

Example struct:

```rust
#[derive(Node, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    // other fields
}
```

Creating an instance of Alien with an auto-generated ID (implicit):

```rust
let alien = Alien {
    // other fields
    ..Default::default()
};
```

Creating an instance of Alien with a manually generated ID (explicit):

```rust
let alien = Alien {
    id: Alien::create_simple_id(),
    // other fields
};
```

2. **SurrealUuid<Self>:**

`SurrealUuid<Self>` auto-generates a UUID when a new instance of the struct is
created. You can also manually generate it using the `create_uuid()` function on
the struct.

Example struct:

```rust
#[derive(Node, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = "account")]
pub struct Account {
    pub id: SurrealUuid<Self>,
    // other fields
}
```

Creating an instance of Account with an auto-generated UUID (implicit):

```rust
let account = Account {
    // other fields
    ..Default::default()
};
```

Creating an instance of Account with a manually generated UUID (explicit):

```rust
let account = Account {
    id: Account::create_uuid(),
    // other fields
};
```

3. **SurrealUlid<Self>:**

`SurrealUlid<Self>` auto-generates a ULID when a new instance of the struct is
created. You can also manually generate it using the `create_ulid()` function on
the struct.

Example struct:

```rust
#[derive(Node, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = "spaceship")]
pub struct SpaceShip {
    pub id: SurrealUlid<Self>,
    // other fields
}
```

Creating an instance of SpaceShip with an auto-generated ULID (implicit):

```rust
let spaceship = SpaceShip {
    // other fields
    ..Default::default()
};
```

Creating an instance of SpaceShip with a manually generated ULID (explicit):

```rust
let spaceship = SpaceShip {
    id: SpaceShip::create_ulid(),
    // other fields
};
```

4. **SurrealId<Self, T>:**

This is the most flexible ID type, allowing for any arbitrary serializable type
`T` as the ID. However, it doesn't implement the Default trait, which means you
must manually create instances of this type using the `create_id()` function.

Example struct:

```rust
#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = "weapon")]
pub struct Weapon {
    pub id: SurrealId<Self, String>,
    // other fields
}
```

Creating an

instance of Weapon with a manually created ID (explicit):

```rust
let weapon = Weapon {
    id: Weapon::create_id("sword".into()),
    // other fields
};
```

These ID types provide various options for users to meet the needs of different
scenarios when working with entities in SurrealDB. Whether you want
auto-generated identifiers or prefer to create them manually, there's an ID type
to suit your requirements.

The SurrealID types in SurrealDB are designed to be flexible and accommodating
to various needs for entity identification and linking within the database.
