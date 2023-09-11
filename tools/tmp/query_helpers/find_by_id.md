# find_by_id

## 2. `find_by_id` Method

The find_by_id method provides a straightforward way to fetch a record from the
database using its unique ID.

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
