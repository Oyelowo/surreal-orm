# Record Ids

The `SurrealId` is a wrapper struct that extends the capabilities of `surrealdb::sql::Thing` and provides a more ergonomic interface. It's a static type representing the id of a model in the SurrealDB ORM and is a combination of the model's table name and the id, where the id can be anything that can be converted into a `surrealdb::sql::Id`.

1. **SurrealSimpleId<Self>:**

This ID type is perfect when you need a unique identifier without any
special requirements for its structure. `SurrealSimpleId<Self>`
automatically generates an ID when a new instance of the struct is created,
thanks to the implementation of the Default trait.

However, to manually generate a simple ID, you'd invoke the `create_simple_id`
method directly on the struct. This returns an instance of `SurrealSimpleId<Self>`
that can be used as an ID.

Example struct:

```rust
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    // other fields
}
```

Creating an instance of Alien with a simple ID:

```rust
let alien = Alien {
    id: Alien::create_simple_id(),
    // other fields
};
```

2. **SurrealUuid<Self>:**

`SurrealUuid<Self>` provides a high level of uniqueness given the nature of UUIDs. It automatically generates a UUID when a new instance of the struct is created, by implementing the Default trait.

To manually generate a UUID, you can invoke the `create_uuid` method directly on the struct, which returns an instance of `SurrealUuid<Self>`.

Example struct:

```rust
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "account")]
pub struct Account {
    pub id: SurrealUuid<Self>,
    // other fields
}
```

Creating an instance of Account with a UUID:

```rust
let account = Account {
    id: Account::create_uuid(),
    // other fields
};
```

3. **SurrealUlid<Self>:**

`SurrealUlid<Self>` provides a high level of uniqueness given the nature of ULIDs. It automatically generates a ULID when a new instance of the struct is created, by implementing the Default trait.

To manually generate a ULID, you can invoke the `create_ulid` method directly on the struct, which returns an instance of `SurrealUlid<Self>`.

Example struct:

```rust
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "account")]
pub struct Account {
    pub id: SurrealUlid<Self>,
    // other fields
}
```

Creating an instance of Account with a UUID:

```rust
let account = Account {
    id: Account::create_uuid(),
    // other fields
};
```

4. **SurrealId<Self, T>:**

This is the most flexible ID type, allowing for any arbitrary serializable type `T` as the ID. It's useful when your ID needs to include specific information or adhere to a certain format. This ID type doesn't implement the Default trait, thus instances of this type must be manually created using the `create_id` method.

Example struct:

```rust
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealId<Self, String>,
    // other fields
}
```

Creating an instance of Weapon with a specific ID:

```rust
let weapon = Weapon {
    id: Weapon::create_id("laser_weapon"),
    // other fields
};
```

The SurrealID types in SurrealDB are designed to be flexible and accommodating to various needs for entity identification and linking within the database. The `create_id` methods offer a convenient way to create and manage these identifiers.
