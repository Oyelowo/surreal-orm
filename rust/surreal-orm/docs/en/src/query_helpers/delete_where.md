# delete_where

## 8. `delete_where` Method

For scenarios where you need to delete multiple records based on a condition,
the delete_where method comes in handy.

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
    let spaceship_schema::SpaceShip { name, .. } = SpaceShip::schema();

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
