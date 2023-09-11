# delete_by_id

## 7. `delete_by_id` Method

If you know the ID of a record you wish to delete, the delete_by_id method makes
the task straightforward.

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
