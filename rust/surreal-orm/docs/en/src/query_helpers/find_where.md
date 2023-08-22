# find_where

## 3. `find_where` Method

For more complex data retrieval needs, the find_where method allows you to
specify conditions to determine which records to fetch.

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
    let spaceship_schema::SpaceShip { name, id, .. } = SpaceShip::schema();

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
