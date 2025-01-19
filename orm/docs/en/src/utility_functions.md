# Helper Methods in `surreal_orm`

The `surreal_orm` library offers a set of utility functions encapsulated in the
`SurrealCrud` and `SurrealCrudNode` traits. These methods provide a high-level
abstraction over raw database statements, simplifying CRUD operations.

## Preparations

Before we dive into the helper methods, let's set up our environment:

```rust
use surreal_models::{space_ship, weapon, SpaceShip, Weapon};
use surreal_orm::{
    statements::{insert, select, select_value},
    *,
};
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

async fn create_test_data(db: Surreal<Db>) {
    let space_ships = (0..1000)
        .map(|i| Weapon {
            name: format!("weapon-{}", i),
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<Weapon>>();
    insert(space_ships).run(db.clone()).await.unwrap();
}
```

---

## 1. `save` Method

The `save` method can either create a new record or update an existing one in
the database. You can think of it as an upsert method.

```rust
#[tokio::test]
async fn test_save() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ss_id = SpaceShip::create_id("num-1".into());
    let spaceship = SpaceShip {
        id: ss_id.clone(),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    let spaceship = spaceship.save().get_one(db.clone()).await?;

    let saved_spaceship = SpaceShip::find_by_id(ss_id.clone())
        .get_one(db.clone())
        .await?;

    assert_eq!(spaceship.id.to_thing(), saved_spaceship.id.to_thing());
    assert_eq!(spaceship.name, saved_spaceship.name);
    Ok(())
}
```

---

## 2. `find_by_id` Method

Retrieve a record by its ID:

```rust
#[tokio::test]
async fn test_find_by_id() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    spaceship.clone().save().run(db.clone()).await?;

    let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
        .get_one(db.clone())
        .await?;

    assert_eq!(spaceship.id.to_thing(), found_spaceship.id.to_thing());
    Ok(())
}
```

---

## 3. `find_where` Method

Retrieve records based on specific conditions:

```rust
#[tokio::test]
async fn test_find_where() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };
    let _spaceship2 = SpaceShip {
        id: SpaceShip::create_id("num-2".into()),
        name: "spaceship-2".into(),
        created: chrono::Utc::now(),
    }
    .save()
    .run(db.clone())
    .await?;

    let _spaceschip = spaceship.clone().save().get_one(db.clone()).await?;
    let space_ship::Schema { name, id, .. } = SpaceShip::schema();

    let found_spaceships = SpaceShip::find_where(id.is_not(NULL))
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 2);

    let found_spaceships = SpaceShip::find_where(name.equal("spaceship-1"))
        .return_many(db.clone())
        .await?;

    assert_eq!(found_spaceships.len(), 1);
    assert_eq!(found_spaceships[0].id.to_thing(), spaceship.id.to_thing());

    let found_spaceship = SpaceShip::find_where(name.equal("spaceship-1"))
        .get_one(db.clone())
        .await?;

    assert_eq!(found_spaceship.id.to_thing(), spaceship.id.to_thing());
    Ok(())
}
```

---

## 4. `count_where` Method

Count records based on specific conditions:

```rust
#[tokio::test]
async fn test_count_where() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;
    let weapon::Schema { strength, .. } = &Weapon::schema();

    let weapons_query = Weapon::count_where(strength.gte(500));
    let weapons_count = weapons_query.get(db.clone()).await?;

    assert_eq!(
        weapons_query.to_raw().build(),
        "SELECT VALUE count FROM (SELECT count(strength >= 500) FROM weapon GROUP ALL);"
    );

    assert_eq!(weapons_count, 500);

    Ok(())
}
```

---

## 5. `count_all` Method

Count all records:

```rust
#[tokio::test]
async fn test_count_all() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;

    let weapons_query = Weapon::count_all();
    let weapons_count = weapons_query.get(db.clone()).await?;

    assert_eq!(
        weapons_query.to_raw().build(),
        "SELECT VALUE count FROM (SELECT count() FROM weapon GROUP ALL);"
    );

    assert_eq!(weapons_count, 1000);

    Ok(())
}
```

## 6. `delete` Method

This method deletes the current record instance from the database.

```rust
#[tokio::test]
async fn test_delete() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    let spaceship = spaceship.save().get_one(db.clone()).await?;

    let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceship.len(), 1);

    spaceship.clone().delete().run(db.clone()).await?;

    let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
        .return_many(db.clone())
        .await?;

    assert!(found_spaceship.is_empty());
    assert_eq!(found_spaceship.len(), 0);
    Ok(())
}
```

---

## 7. `delete_by_id` Method

This method deletes a record by its ID.

```rust
#[tokio::test]
async fn test_delete_by_id() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    let spaceship = spaceship.save().get_one(db.clone()).await?;
    let found_spaceships = SpaceShip::find_by_id(spaceship.id.clone())
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 1);

    SpaceShip::delete_by_id(spaceship.id.clone())
        .run(db.clone())
        .await?;

    let found_spaceships = SpaceShip::find_by_id(spaceship.id.clone())
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 0);
    Ok(())
}
```

---

## 8. `delete_where` Method

This method deletes records based on a specific condition.

```rust
#[tokio::test]
async fn test_delete_where() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    spaceship.save().run(db.clone()).await.unwrap();
    let space_ship::Schema { name, .. } = SpaceShip::schema();

    let found_spaceships = SpaceShip::find_where(name.like("spaceship-1"))
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 1);

    SpaceShip::delete_where(name.like("spaceship"))
        .run(db.clone())
        .await?;

    let found_spaceships = SpaceShip::find_where(name.like("spaceship-1"))
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 0);
    Ok(())
}
```

---

## 9. `create` Method

This method creates a new record in the database. It's specifically for nodes.

```rust
#[tokio::test]
async fn test_create() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ss_id = SpaceShip::create_id(format!("num-{}", 1));
    let spaceship = SpaceShip {
        id: ss_id.clone(),
        name: format!("spaceship-{}", 1),
        created: chrono::Utc::now(),
    };

    let spaceship = spaceship.create().get_one(db.clone()).await?;
    // Second attempt should fail since it will be duplicate.
    spaceship
        .clone()
        .create()
        .get_one(db.clone())
        .await
        .expect_err("should fail");

    let saved_spaceship = SpaceShip::find_by_id(ss_id.clone())
        .get_one(db.clone())
        .await?;

    assert_eq!(spaceship.id.to_thing(), saved_spaceship.id.to_thing());
    assert_eq!(spaceship.name, saved_spaceship.name);
    Ok(())
}
```

---

This wraps up the explanations and demonstrations for all the helper methods in
surreal_orm.
