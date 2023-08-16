# Delete

In `surreal-orm`, data manipulation is a core feature, and deleting records is a
crucial part of that. This chapter provides an in-depth overview of the
different methods available for deleting records.

## Setup and Test Data Creation

Before diving into the deletion methods, let's set up the necessary environment
and generate some test data.

```rust
use pretty_assertions::assert_eq;
use surreal_models::{weapon_schema, Weapon};
use surreal_orm::{
    statements::{delete, insert},
    *,
};
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

async fn create_test_data(db: Surreal<Db>) -> Vec<Weapon> {
    let weapons = (0..1000)
        .map(|i| Weapon {
            name: format!("weapon-{}", i),
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<Weapon>>();
    insert(weapons).return_many(db.clone()).await.unwrap()
}
```

This setup creates a thousand `Weapon` records that we'll use in subsequent
tests.

## Delete by ID Using Helper Functions

The `surreal-orm` library provides helper functions on model instances for
common operations. Here's how you can delete a record using the `delete_by_id`
helper function:

```rust
# #[tokio::test]
async fn test_delete_by_id_helper_function() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapons = create_test_data(db.clone()).await;
    let weapon1 = weapons.first().unwrap();

    let weapon_schema::Weapon { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(&weapon1.id))
            .get(db.clone())
            .await
            .unwrap()
    };
    assert_eq!(deleted_weapon_count().await, 1);

    Weapon::delete_by_id(&weapon1.id).run(db.clone()).await?;

    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}
```

## Delete by ID

Another approach to delete a record is by directly using its ID. This method is
efficient for deleting a single record:

```rust
# #[tokio::test]
async fn test_delete_one_by_id() -> SurrealOrmResult<()> {
    // ... [Setup code]

    let weapon_schema::Weapon { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(&weapon1.id))
            .get(db.clone())
            .await
            .unwrap()
    };
    assert_eq!(deleted_weapon_count().await, 1);

    delete::<Weapon>(&weapon1.id).run(db.clone()).await?;

    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}
```

## Delete Using Model Instance

Deleting a record can also be achieved directly using a model instance. This
method is intuitive and leverages the object-oriented nature of Rust:

```rust
# #[tokio::test]
async fn test_delete_one_by_model_instance() -> SurrealOrmResult<()> {
    // ... [Setup code]

    let weapon_schema::Weapon { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(&weapon1.id))
            .get(db.clone())
            .await
            .unwrap()
    };
    assert_eq!(deleted_weapon_count().await, 1);

    weapon1.delete().run(db.clone()).await?;

    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}
```

## Delete Using Conditions with Model Helper Functions

Deleting multiple records based on certain conditions is a common requirement.
The `delete_where` model helper function facilitates this:

```rust
# #[tokio::test]
async fn test_delete_where_model_helper_function() -> SurrealOrmResult<()> {
    // ... [Setup code]

    let weapon_schema::Weapon { strength, .. } = &Weapon::schema();

    let weapons_count = || async { Weapon::count_all().get(db.clone()).await.unwrap() };
    assert_eq!(weapons_count().await, 1000);

    Weapon::delete_where(cond(strength.gte(500)).and(strength.lt(600)))
        .run(db.clone())
        .await?;

    assert_eq!(weapons_count().await, 900);

    Ok(())
}
```

## Delete Multiple Records Based on Conditions

Lastly, to delete multiple records based on a specific condition, you can use
the `delete` function:

```rust
# #[tokio::test]
async fn test_delete_many_query_by_condition() -> SurrealOrmResult<()> {
    // ... [Setup code]

    let weapon_schema::Weapon { strength, .. } = &Weapon::schema();

    let weapons_count = || async { Weapon::count_all().get(db.clone()).await.unwrap() };
    assert_eq!(weapons_count().await, 1000);

    delete::<Weapon>(Weapon::table_name())
        .where_(cond(strength.gte(500)).and(strength.lt(600)))
        .run(db.clone())
        .await?;

    assert_eq!(weapons_count().await, 900);

    Ok(())
}
```

## Conclusion

Deletion is a fundamental operation in any ORM. With `surreal-orm`, not only can
you perform standard deletions by ID or model instance, but you can also delete
records based on complex conditions, offering flexibility and power in data
manipulation.
