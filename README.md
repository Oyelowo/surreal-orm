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

---

#### Surreal ORM Migrations CLI
### Fully Automated Database Schema Migration

Surreal ORM provides a powerful command-line interface (CLI) for automatically diffing and managing database migrations in a SurrealDB environment. This tool offers functionalities ranging from initializing migrations, generating migration files, applying migrations up or down, and various other tasks to manage your database schema effectively.

#### Installation

Gather and prepare codebase resources. These include 
To install and use the CLI tool, ensure you have Rust and Cargo installed, then follow these steps:

#### Setting Up And Gathering Codebase Resources

```rust
use surreal_orm::*;

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "animal", schemafull)]
pub struct Animal {
    pub id: SurrealSimpleId<Self>,
    pub species: String,
    // #[surreal_orm(old_name = "field_old_name")] // Comment this line out to carry out a renaming operation
    pub attributes: Vec<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub velocity: u64,
}

impl TableResources for Animal {
    fn events_definitions() -> Vec<Raw> {
        let animal::Schema { species, velocity, .. } = Self::schema();

        let event1 = define_event("event1".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Erectus")).and(velocity.gt(545)))
            .then(select(All).from(Crop::table_name()))
            .to_raw();

        vec![event1]
    }

    fn indexes_definitions() -> Vec<Raw> {
        let animal::Schema { species, velocity, .. } = Self::schema();

        let idx1 = define_index("species_speed_idx".to_string())
            .on_table(Self::table_name())
            .fields(arr![species, velocity])
            .unique()
            .to_raw();

        vec![idx1]
    }
}

#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "eats", schemafull)]
pub struct Eats<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: In,
    pub out: Out,
    pub place: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub type AnimalEatsCrop = Eats<Animal, Crop>;
impl TableResources for AnimalEatsCrop {}

#[derive(Debug, Clone)]
pub struct Resources;

impl DbResources for Resources {
    create_table_resources!(
        Animal,
        Crop,
        AnimalEatsCrop,
    );

    // Define other database resources here. They default to empty vecs
    fn analyzers(&self) -> Vec<Raw> {
        vec![]
    }

    fn functions(&self) -> Vec<Raw> {
        vec![]
    }

    fn params(&self) -> Vec<Raw> {
        vec![]
    }

    fn scopes(&self) -> Vec<Raw> {
        vec![]
    }

    fn tokens(&self) -> Vec<Raw> {
        vec![]
    }

    fn users(&self) -> Vec<Raw> {
        vec![]
    }
}


use surreal_orm::migrator::Migrator;

#[tokio::main]
async fn main() {
    Migrator::run(Resources).await;
}
```

#### Basic Usage

The CLI tool offers a range of commands, each with specific options and flags. Here's a quick overview:

1. **Initialize Migrations:**
   ```bash
   cargo run -- init --name "initial_migration" -r
   ```
   This initializes the migrations directory with a reversible migration named "initial_migration".
   Omit the `-r` flag if you want up only migrations.

2. **Generate Migrations:**
   ```bash
   cargo run -- gen --name "add_users_table"
   ```
   Generates a new migration file named "add_users_table".
   Notice that we do not need to include the `-r` or `--reversiable` flag. 
   Because we specified whether we want a reversible or non-reversible migration when we initialized, 
   the migration type is automatically detected subsequently.

3. **Apply Migrations Up:**
   ```bash
   # Applies all pending till latest by default
   cargo run -- up
   
   # Applies all pending till latest
   cargo run -- up -l
   
   # Applies by the number specified
   cargo run -- up -n 5
   cargo run -- up --number 5
   
   # Applies till specified migration
   cargo run -- up -t "20240107015727114_create_first.up.surql"
   cargo run -- up --till "20240107015727114_create_first.up.surql"
   ```
   Applies pending migrations forward using various strategies: till latest, by number count and till a specified migration.

4. **Rollback Migrations:**
   ```bash
   # Rollback migration to previous by default
   cargo run -- down
   
   # Rollback all pending till previous
   cargo run -- down --previous
   
   # Rollback by the number specified
   cargo run -- down -n 5
   cargo run -- down --number 5
   
   # Rollback till specified migration
   cargo run -- down -t "20240107015727114_create_first.up.surql"
   cargo run -- down --till "20240107015727114_create_first.up.surql"

   # In addition, you can use the --prune flag to delete local migration 
   # files after rolling back. This can be useful in development for rapid changes.
   cargo run -- down -n 5 --prune
   ```
   Rolls back the last applied migration.

5. **Reset Migrations:**
   ```bash
   cargo run -- reset --name "initial_migration" -r
   ```
   Resets all migrations and initializes a new reversible migration named "initial_migration".
   Skip the `-r` or `--reversible` flag if you want up only migrations,

6. **List Migrations:**
   ```bash
   # List pending migrations by default
   cargo run -- ls
   cargo run -- list

   # List all migrations
   cargo run -- list --status all

   # List pending migrations
   cargo run -- list --status pending
   
   # List applied migrations
   cargo run -- list --status applied
   ```
   Lists migrations by their statuses i.e, `all`, `pending` and `applied`.

#### Advanced Migration CLI Usage

Advanced usage involves specifying additional flags and options to tailor the migration process to your specific needs. Here's how you can use these advanced features:

1. **Custom Migration Directory:**
   ```bash
   cargo run -- init --name "initial_migration" --dir "custom_migrations" -r
   ```
   Initializes migrations in a custom directory named "custom_migrations".

2. **Verbose Output:**
   ```bash
   cargo run -- up -vvv
   ```
   Runs migrations with 3 levels verbose output.

3. **Database Connection Configuration:**
   - URL: `ws://localhost:8000`
   - Database Name: `test`
   - Namespace: `test`
   - User: `root`
   - Password: `root`

   ```bash
   cargo run -- up --url "ws://localhost:8000" --db "mydb" --ns "myns" --user "username" --pass "password"
   ```
   Connects to the specified SurrealDB instance with custom credentials and applies migrations.

   Other supported urls types include:
    ```bash
    - ws://localhost:8000
    - wss://cloud.example.com
    - http://localhost:8000
    - https://cloud.example.com
    - mem:// 
    # or simply
    - memory
    - file://temp.db
    - indxdb://MyDatabase
    - tikv://localhost:2379
    - fdb://fdb.cluster
    ```

#### Database Connection Configuration

The `DatabaseConnection` struct allows you to specify various parameters for connecting to the database:

```rust
#[derive(Args, Debug, Clone, TypedBuilder)]
pub struct DatabaseConnection {
    // URL, Database, Namespace, User, Password configurations
}
```

This configuration enables the CLI to connect to different database backends including WebSocket, HTTP(S), In-Memory, File-Backend, and more.

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
