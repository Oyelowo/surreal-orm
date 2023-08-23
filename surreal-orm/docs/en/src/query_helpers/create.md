# create

## 9. `create` Method

While the save method is versatile, the create method is specialized for
creating new records. It's specifically for nodes. Unlike the `save` method,
rather than updating the existing record by its `id`, it throws and error when a
record already exists.

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
