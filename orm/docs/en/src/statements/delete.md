# Delete Operations

## Table of Contents

1. [Setup and Test Data Creation](#setup-and-test-data-creation)
2. [Delete by ID Using Helper Functions](#delete-by-id-using-helper-functions)
3. [Delete by ID](#delete-by-id)
4. [Delete Using Model Instance](#delete-using-model-instance)
5. [Delete Using Conditions with Model Helper Functions](#delete-using-conditions-with-model-helper-functions)
6. [Delete Multiple Records Based on Conditions](#delete-multiple-records-based-on-conditions)
7. [Conclusion](#conclusion)

---

## Setup and Test Data Creation

Before diving into the deletion methods, let's set up the necessary environment
and generate some test data.

```rust
# use pretty_assertions::assert_eq;
# use surreal_models::{weapon, Weapon};
# use surreal_orm::{
#     statements::{delete, insert},
#     *,
# };
# use surrealdb::{
#     engine::local::{Db, Mem},
#     Surreal,
# };

async fn create_test_data(db: Surreal<Db>) -> Vec<Weapon> {
    let space_ships = (0..1000)
        .map(|i| Weapon {
            name: format!("weapon-{}", i),
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<Weapon>>();
    insert(space_ships).return_many(db.clone()).await.unwrap()
}
```

---

## Delete by ID Using Helper Functions

The `surreal-orm` library provides helper functions on model instances for
common operations. Here's how you can delete a record using the `delete_by_id`
helper function:

```rust
#[tokio::test]
async fn test_delete_by_id_helper_function() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapons = create_test_data(db.clone()).await;
    let weapon1 = weapons.first().unwrap();
    let ref weapon1_id = weapon1.id.clone();

    let weapon::Schema { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(weapon1_id))
            .get(db.clone())
            .await
            .unwrap()
    };
    assert_eq!(deleted_weapon_count().await, 1);

    Weapon::delete_by_id(weapon1_id).run(db.clone()).await?;

    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}
```

---

## Delete by ID

Another approach to delete a record is by directly using its ID. This method is
efficient for deleting a single record:

```rust
#[tokio::test]
async fn test_delete_one_by_id() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapons = create_test_data(db.clone()).await;
    let weapon1 = weapons.first().unwrap();
    let ref weapon1_id = weapon1.id.clone();

    let weapon::Schema { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(weapon1_id))
            .get(db.clone())
            .await
            .unwrap()
    };
    assert_eq!(deleted_weapon_count().await, 1);

    delete::<Weapon>(weapon1_id).run(db.clone()).await?;

    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}
```

## Delete Using Model Instance

Rather than specifying an ID or condition, `surreal-orm` allows developers to
delete records directly using a model instance. This approach can be useful when
the developer already has a reference to the model instance they want to delete:

```rust
#[tokio::test]
async fn test_delete_one_by_model_instance() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapons = create_test_data(db.clone()).await;
    let weapon1 = weapons.first().unwrap();
    let ref weapon1_id = weapon1.id.clone();

    let weapon::Schema { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(weapon1_id))
            .get(db.clone())
            .await
            .unwrap()
    };
    let deleted_weapon = || async {
        Weapon::find_by_id(weapon1_id)
            .return_one(db.clone())
            .await
            .unwrap()
    };

    assert_eq!(deleted_weapon().await.is_some(), true);
    assert_eq!(deleted_weapon_count().await, 1);

    weapon1.delete().run(db.clone()).await?;

    assert_eq!(deleted_weapon().await.is_some(), false);
    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}
```

---

## Delete Using Conditions with Model Helper Functions

Sometimes, developers may need to delete a group of records based on a
particular condition. Model helper functions can also facilitate such
operations:

```rust
#[tokio::test]
async fn test_delete_where_model_helper_function() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;

    let weapon::Schema { strength, .. } = &Weapon::schema();

    let weapons_count = || async { Weapon::count_all().get(db.clone()).await.unwrap() };
    assert_eq!(weapons_count().await, 1000);

    Weapon::delete_where(cond(strength.gte(500)).and(strength.lt(600)))
        .run(db.clone())
        .await?;

    assert_eq!(weapons_count().await, 900);

    Ok(())
}
```

---

## Delete Multiple Records Based on Conditions

The ORM also provides direct deletion methods for multiple records based on
specific conditions. This is particularly useful when the developer knows the
exact criteria they want to match for the deletion:

```rust
#[tokio::test]
async fn test_delete_many_query_by_condition() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;

    let weapon::Schema { strength, .. } = &Weapon::schema();

    let weapons_count = || async { Weapon::count_all().get(db.clone()).await.unwrap() };
    assert_eq!(weapons_count().await, 1000);

    delete::<Weapon>(Weapon::table())
        .where_(cond(strength.gte(500)).and(strength.lt(600)))
        .run(db.clone())
        .await?;

    assert_eq!(weapons_count().await, 900);

    Ok(())
}
```

---

## Conclusion

The delete operations in `surreal-orm` offer a flexible and comprehensive
mechanism to remove records from the `surrealdb` database. Whether it's deleting
a single record using its ID, removing multiple records based on conditions, or
even utilizing model instances for deletions, the ORM provides an arsenal of
tools to help developers manage their data efficiently.
