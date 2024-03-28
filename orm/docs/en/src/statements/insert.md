# Surreal ORM - Insertion Operations

Surreal ORM provides various options to perform data insertion operations in
your database. This guide focuses on three main operations:

**Table of Contents**

1. [Inserting Single Record](#Inserting-Single-Record)
2. [Inserting Multiple Records](#Inserting-Multiple-Records)
3. [Inserting from Another Table](#Inserting-from-Another-Table)

---

<a name="Inserting-Single-Record"></a>

## Inserting Single Record

The ORM allows for inserting a single record into a database table. Below is an
example of this:

```rust
// Required imports
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use surreal_models::Weapon;
use surreal_orm::statements::insert;
use chrono::Utc;

// Initialize SurrealDB with the in-memory engine
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

// Define a single weapon
let weapon = Weapon {
    name: String::from("Excalibur"),
    created: Utc::now(),
    strength: 1000,
    ..Default::default()
};

// Insert the weapon into the database
let created_weapon = insert(weapon).return_one(db.clone()).await.unwrap();

// Verify the inserted record
assert_eq!(created_weapon.name, "Excalibur");
assert_eq!(created_weapon.strength, 1000);
```

This code creates a single `Weapon` record with the name "Excalibur" and a
strength of 1000.

---

<a name="Inserting-Multiple-Records"></a>

## Inserting Multiple Records

In addition to inserting single records, Surreal ORM also supports inserting
multiple records at once. Here is an example:

```rust
// Required imports
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use surreal_models::Weapon;
use surreal_orm::statements::insert;
use chrono::Utc;

// Initialize SurrealDB with the in-memory engine
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

// Define a list of weapons
let weapons = (0..1000)
    .into_iter()
    .map(|i| Weapon {
        name: format!("Weapon{}", i),
        created: Utc::now(),
        strength: i,
        ..Default::default()
    })
    .collect::<Vec<_>>();

// Insert the weapons into the database
let created_weapons = insert(weapons).return_many(db.clone()).await.unwrap();

// Verify the inserted records
assert_eq!(created_weapons.len(), 1000);
assert_eq!(created_weapons[0].name, "Weapon0");
assert_eq!(created_weapons[0].strength, 0);
```

This code creates 1000 `Weapon` records with sequential names and strength
values.

---

<a name="Inserting-from-Another-Table"></a>

## Inserting from Another Table

Surreal ORM allows you to copy data from one table to another using the `insert`
statement. This is similar to creating a view in PostgreSQL, but instead of just
a projection, it's copying the data to a new table.

```rust
// Required imports
use surrealdb::Surreal;
use surrealdb::engine::local::Mem;
use surreal_models::{Weapon, StrongWeapon};
use surreal_orm::statements::{insert, select, All, cond, order};
use chrono::Utc;

// Initialize SurrealDB with the in-memory engine
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

// Define a list of weapons
let weapons = (

0..1000)
    .into_iter()
    .map(|i| Weapon {
        name: format!("Weapon{}", i),
        created: Utc::now(),
        strength: i,
        ..Default::default()
    })
    .collect::<Vec<_>>();

// Insert the weapons into the database
let created_weapons = insert(weapons).return_many(db.clone()).await.unwrap();

// Define a SELECT statement for weapons with strength values between 800 and 950
let weapon::Schema { strength, .. } = Weapon::schema();
let select_statement = select(All)
    .from(Weapon::table())
    .where_(cond(strength.greater_than_or_equal(800)).and(strength.less_than(950)));

// Insert the selected weapons into the StrongWeapon table
let strong_weapons = insert::<StrongWeapon>(select_statement)
    .return_many(db.clone())
    .await
    .unwrap();

// Verify the copied records
assert_eq!(strong_weapons.len(), 150);
assert_eq!(strong_weapons[0].strength, 800);
```

This script inserts 1000 `Weapon` records, selects those with strength values
between 800 and 950, and copies them into the `StrongWeapon` table.
