[![surreal-orm](https://github.com/Oyelowo/surreal-orm/actions/workflows/rust.yaml/badge.svg)](https://github.com/Oyelowo/surreal-orm/actions/workflows/rust.yaml)
[![cleanup old images](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/delete-old-images.yaml/badge.svg)](https://github.com/Oyelowo/modern-distributed-app-template/actions/workflows/delete-old-images.yaml)

## Introduction

Surreal ORM is an Object-Relational Mapping and query-building library for Rust
that provides a high-level API for interacting with SurrealDB, a distributed
graph database. This documentation will guide you through the usage and features
of the Surreal ORM library.

## Getting Started

To use Surreal ORM in your Rust project, you need to add it as a dependency in
your `Cargo.toml` file:

```toml
[dependencies]
surreal_orm = "0.1"
```

After adding the dependency, you can import the necessary modules in your Rust
code:

```rust
use surreal_orm::*;
```

## Connecting to SurrealDB

Before interacting with SurrealDB, you need to establish a connection to the
database. The following example demonstrates how to create a connection to a
local SurrealDB instance:

```rust
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::main]
async fn main() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
}
```

In this example, we create a new SurrealDB instance using the `Surreal::new`
function with the `local::Mem` engine. The `local::Mem` engine represents a
local in-memory database. You can replace it with other engine types according
to your setup.

## Defining a Node

A model in Surreal ORM represents a database table. You can define a node by
creating a Rust struct and implementing the `Node` or `Edge` trait. Here's an
example of defining a `SpaceShip` model:

```rust
use surreal_orm::*;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub age: u8,
}
```

In this example, we define a `SpaceShip` struct and annotate it with the `Node`
derive macro. The `table_name` attribute specifies the name of the corresponding
database table.

## Querying Data

Surreal ORM provides a fluent and expressive API for querying data from the
database. You can use the `select` function to start a select statement and
chain various methods to build the query. Here's an example:

```rust
use surreal_orm::statements::{select, All};

let space_ship::Schema { name, age, .. } = SpaceShip::schema();
let space_ship = SpaceShip::table_name();

let statement = select(All)
    .from(space_ship)
    .where_(name.equal("Millennium Falcon"))
    .order_by(age.desc())
    .limit(10);
```

In this example, we start a select statement using the `select` function and
pass the `All` argument to select all fields. We specify the table name using
the `from` method and add a condition using the `where_` method. We can also use
the `order_by` method to specify the sorting order and the `limit` method to
limit the number of results.

## Inserting Data

To insert data into the database, you can use the `insert` function and provide
the data as a vector of structs. Here's an example:

```rust
use surreal_orm::statements::insert;

let spaceships = vec![
    SpaceShip {
        id: SpaceShip::create_simple_id(),
        name: "Millennium Falcon".to_string(),
        age: 79,
    },
    SpaceShip {
        name: "Starship Enterprise".to_string(),
        age: 15,
        ..Default::default()
    },
];

insert(spaceships).return_many(db.clone()).await?;
```

In this example, we define a vector of `SpaceShip` structs and pass it to the
`insert` function. We then call the `run` method to execute the insertion
operation.

We can also use `object!` and `object_partial!` macros for a much more flexbible
and robust insertion supporting fields and parameter assignment e.g:

```rust
let weapon = &Weapon::table_name();
let weapon::Schema { ref strength, .. } = Weapon::schema();

let created_stats_statement = create::<WeaponStats>().set(object_partial!(WeaponStats {
    averageStrength: block! {
        let strengths = select_value(strength).from(weapon);
        let total = math::sum!(strengths);
        let count = array::len!(strengths);
        return math::ceil!((((total / count) * (count * total)) / (total + 4)) * 100);
    }
}));
```

## Updating Data

To update data in the database, you can use the `update` function and provide
the updated data as a struct. Here's an example:

```rust
use surreal_orm::statements::update;

let space_ship = SpaceShip::table_name();

update::<SpaceShip>(space_ship)
    .content(SpaceShip {
        name: "Oyelowo".to_string(),
        age: 90,
        ..Default::default()
    })
    .where_(cond(strength.greater_than(5)).and(strength.less_than_or_equal(15)))
    .return_many(db.clone())
    .await?;
```

In this example, we define a `SpaceShip` struct with the updated data and pass
it to the `update` function to update records within the table based on the
condition. We then call the `return_many` method to execute the update
operation.

## Deleting Data

To delete data from the database, you can use the `delete` function and provide
the condition for deletion. Here's an example:

```rust
use surreal_orm::{*, statements::{delete}};

let space_ship::Schema { name, age, .. } = SpaceShip::schema();
let space_ship = SpaceShip::table_name();

delete(space_ship)
    .where_(cond(name.equal("Millennium Falcon")).and(age.less_then(50)))
    .run(db.clone())
    .await?;
```

In this example, we use the `delete` function and specify the table name. We add
a condition using the `where_` method, and then call the `run` method to execute
the deletion operation.

## Complex queries with `query_turbo!`, `transaction!` and `block!`.

With these macros, you can create extremely complex queries with native-like
syntax. `query_turbo!` macro creates a query chain by default which comprises
several statements separated by `;`. In order to create a transaction, you have
to begin the queries with `BEGIN TRANSACTION;` and end with
`COMMIT TRANSACTION;` or `CANCEL TRANSACTION;`. You can also just use the
dedicated `transaction!` macro which also enforces beginning and ending with the
necessary transaction statements. Lastly, to create a block, you have to return
an expression typically at the end. You can return a normal value, an if else
expression or a or a variable. You can also enforce a block by using the
dedicated `block!` macro. A block is surrounded by curly braces and typically
returns.

```rust
let id1 = &Account::create_id("one".to_string());
let id2 = &Account::create_id("two".to_string());
let amount_to_transfer = 300.00;

let acc = Account::schema();
let account::Schema { balance, .. } = Account::schema();

let query_chain = query_turbo! {
    begin transaction;
    let balance1 = create_only().content(Balance {
            id: Balance::create_id("balance1".to_string()),
            amount: amount_to_transfer,
        });

    create_only().content(Balance {
            id: Balance::create_id("balance2".to_string()),
            amount: amount_to_transfer,
        });

    if balance.greater_than(100) {
        let first_name = "Oyelowo";
        let score = 100;
        select(All).from(Account::table_name()).where_(acc.balance.eq(5));
    } else if balance.less_than(100) {
        let first_name = "Oyelowo";
        let score = 100;
        select(All).from(Account::table_name()).where_(acc.balance.eq(5));
    } else if balance.gte(100) {
        let first_name = "Oyelowo";
        let score = 100;
        select(All).from(Account::table_name()).where_(acc.balance.eq(5));
    } else {
        let first_name = "Oyelowo";
        let score = 100;
        select(All).from(Account::table_name()).where_(acc.balance.eq(5));
    };

    for name in vec!["Oyelowo", "Oyedayo"] {
        let first = "Oyelowo";
        select(All).from(Account::table_name()).where_(acc.balance.eq(5));

        let good_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(64));

        if balance.gt(50) {
            let first_name = "Oyelowo";
        };

        select(All).from(Account::table_name()).where_(acc.balance.eq(34));

        let numbers = vec![23, 98];

        for age in numbers {
          let score = 100;
          let first_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(5));

          let second_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(25));
          select(All).from(Account::table_name()).where_(acc.balance.eq(923));

        };
    };

     let  balance3 = create().content(Balance {
            id: Balance::create_id("balance3".into()),
            amount: amount_to_transfer,
        });

    let accounts = select(All)
        .from(id1..=id2);


    // You can reference the balance object by using the $balance variable and pass the amount
    // as a parameter to the decrement_by function. i.e $balance.amount
    let updated1 = update::<Account>(id1).set(acc.balance.increment_by(balance1.with_path::<Balance>(E).amount));
    update::<Account>(id1).set(acc.balance.increment_by(balance1.with_path::<Balance>(E).amount));
    update::<Account>(id1).set(acc.balance.increment_by(45.3));

    // You can also pass the amount directly to the decrement_by function. i.e 300.00
    update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
    update::<Account>(id2).set(acc.balance.decrement_by(50));

    commit transaction;
};
```

## Transactions

Surreal ORM supports transactions, which are a series of operations that are
treated as a single unit of work. Here's an example of a transaction that
involves creating two accounts, updating their balances, and then committing the
transaction:

```rust
use surreal_orm::{
    statements::{begin_transaction, create, update},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "account")]
pub struct Account {
    pub id: SurrealId<Self, String>,
    pub balance: f64,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "balance")]
pub struct Balance {
    pub id: SurrealId<Self, String>,
    pub amount: f64,
}


let id1 = &Account::create_id("one".into());
let id2 = &Account::create_id("two".into());
let amount_to_transfer = 300.00;

let acc = Account::schema();

transaction! {
    BEGIN TRANSACTION;

    let balance = create_only().content(Balance {
            id: Balance::create_id("balance1".into()),
            amount: amount_to_transfer,
        });

    create_only().content(Account {
        id: id1.clone(),
        balance: 135_605.16,
    });

    create_only().content(Account {
        id: id2.clone(),
        balance: 91_031.31,
    });

    // You can reference the balance object by using the $balance variable and pass the amount
    // as a parameter to the decrement_by function. i.e $balance.amount
    update::<Account>(id1).set(acc.balance.increment_by(balance.with_path::<Balance>(E).amount));

    // You can also pass the amount directly to the decrement_by function. i.e 300.00
    update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));

    COMMIT TRANSACTION;
}
.run(db.clone())
.await?;

let accounts = select(All)
    .from(id1..=id2)
    .return_many::<Account>(db.clone())
    .await?;
```

In this example, we begin a transaction and then create two accounts with
initial balances. We then increment the balance of the first account and
decrement the balance of the second account by the same amount. Finally, we
commit the transaction and then verify that the balances were updated correctly.

## `query!` Macro

The `query!` macro allows for writing SQL queries directly in Rust, providing
automatic handling of placeholders and bindings, ensuring type safety, and
reducing repetitive boilerplate code.

### Usage

With the `query!` macro, you can easily construct and execute SQL queries:

- **Basic Query**:

  ```rust
  let query = query!(db, "SELECT * FROM users").await;
  let query = query!(db, "SELECT * FROM users", {}).await;
  ```

- **With Placeholders**:

  ```rust
  let username = "Oyelowo";
  let query = query!(db, "SELECT name, age FROM users WHERE age > $age AND name = $ame", {
      age : 102,
      ame : username
  })
  .await;
  ```

- **Multiple Queries**:
  ```rust
  let queries = query!(
      db,
      [
          "SELECT * FROM users WHERE score = $score",
          "CREATE user:oyelowo SET name = $name, company = $company_name, skills = $skills"
      ],
      {
          score: 100,
          name: "Oyelowo",
          skills: vec!["Rust", "python", "typescript"],
          company_name: "Codebreather"
      }
  )
  .await;
  ```

### Benefits

The `query!` macro offers several benefits, including:

- **Developer Efficiency**: Reduce the time spent writing and debugging SQL
  queries.
- **Code Clarity**: Achieve clearer and more maintainable code with direct SQL
  queries in Rust.
- **Error Reduction**: Minimize the potential for runtime errors and SQL
  injection vulnerabilities.

## Conclusion

This concludes the basic usage and features of the Surreal ORM library. You can
explore more advanced features and methods in the API documentation. If you have
any further questions or need assistance, feel free to reach out.

## Development: Convention

To carry out certain tasks in any directory, these are the standard commands:

| Commands       | Purpose                                          |
| -------------- | :----------------------------------------------- |
| `make setup`   | To setup the codebase for development            |
| `make install` | install packages                                 |
| `make upgrade` | upgrade packages                                 |
| `make sync`    | synchronize/generate local code etc              |
| `make dev`     | start cluster/app locally in live reloading mode |
| `make format`  | format code                                      |
| `make check`   | check that code aligns with standard             |
| `make test`    | run automated tests                              |
