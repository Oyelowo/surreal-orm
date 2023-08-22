# save

## 1. `save` Method

The save method is versatile—it can either create a new record or update an
existing one, making it an "upsert" function.

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
