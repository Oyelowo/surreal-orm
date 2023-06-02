# Quick Start

# SurrealDB ORM Documentation

## Introduction

SurrealDB ORM is an Object-Relational Mapping library for Rust that provides a high-level API for interacting with SurrealDB, a distributed document database. This documentation will guide you through the usage and features of the SurrealDB ORM library.

## Getting Started

To use SurrealDB ORM in your Rust project, you need to add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
surrealdb_orm = "0.1"
```

After adding the dependency, you can import the necessary modules in your Rust code:

```rust
use surrealdb_orm::*;
```

## Connecting to SurrealDB

Before interacting with SurrealDB, you need to establish a connection to the database. The following example demonstrates how to create a connection to a local SurrealDB instance:

```rust
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::main]
async fn main() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
}
```

In this example, we create a new SurrealDB instance using the `Surreal::new` function with the `local::Mem` engine. The `local::Mem` engine represents a local in-memory database. You can replace it with other engine types according to your setup.

## Defining a Model

A model in SurrealDB ORM represents a database table. You can define a model by creating a Rust struct and implementing the `SurrealdbModel` trait. Here's an example of defining a `SpaceShip` model:

```rust
use chrono::Utc;
use surrealdb_orm::SurrealdbModel;
use surrealdb_orm::types::DateTime;

#[derive(SurrealdbModel)]
#[surrealdb(table_name = "space_ship")]
struct SpaceShip {
    id: String,
    name: String,
    created: DateTime<Utc>,
}
```

In this example, we define a `SpaceShip` struct and annotate it with the `SurrealdbModel` derive macro. The `table_name` attribute specifies the name of the corresponding database table.

## Querying Data

SurrealDB ORM provides a fluent and expressive API for querying data from the database. You can use the `select` function to start a select statement and chain various methods to build the query. Here's an example:

```rust
use surrealdb_orm::statements::{select, All};

let statement = select(All)
    .from("space_ship")
    .where_("name".eq("Millennium Falcon"))
    .order_by("created".desc())
    .limit(10);
```

In this example, we start a select statement using the `select` function and pass the `All` argument to select all fields. We specify the table name using the `from` method and add a condition using the `where_` method. We can also use the `order_by` method to specify the sorting order and the `limit` method to limit the number of results.

## Inserting Data

To insert data into the database, you can use the `insert` function and provide the data as a vector of structs. Here's an example:

```rust
use surrealdb_orm::statements::insert;

let spaceships = vec![
    SpaceShip {
        id: "1".to_string(),
        name: "Millennium Falcon".to_string(),
        created: Utc::now(),
    },
    SpaceShip {
        id: "2".to_string(),
        name: "Starship Enterprise".to_string(),
        created: Utc::now(),


    },
];

insert(spaceships).run(db.clone()).await?;
```

In this example, we define a vector of `SpaceShip` structs and pass it to the `insert` function. We then call the `run` method to execute the insertion operation.

## Updating Data

To update data in the database, you can use the `update` function and provide the updated data as a struct. Here's an example:

```rust
use surrealdb_orm::statements::update;

let spaceship = SpaceShip {
    id: "1".to_string(),
    name: "Millennium Falcon".to_string(),
    created: Utc::now(),
};

update(spaceship).run(db.clone()).await?;
```

In this example, we define a `SpaceShip` struct with the updated data and pass it to the `update` function. We then call the `run` method to execute the update operation.

## Deleting Data

To delete data from the database, you can use the `delete` function and provide the condition for deletion. Here's an example:

```rust
use surrealdb_orm::statements::{delete, Field};

let condition = Field::new("name").eq("Millennium Falcon");

delete("space_ship")
    .where_(condition)
    .run(db.clone())
    .await?;
```

In this example, we use the `delete` function and specify the table name as a string. We add a condition using the `where_` method, and then call the `run` method to execute the deletion operation.

## Conclusion

This concludes the basic usage and features of the SurrealDB ORM library. You can explore more advanced features and methods in the API documentation. If you have any further questions or need assistance, feel free to reach out.
