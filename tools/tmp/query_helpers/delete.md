# delete

This method facilitates the deletion of a specific record instance

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
