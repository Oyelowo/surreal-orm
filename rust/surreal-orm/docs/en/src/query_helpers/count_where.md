# count_where

## 4. `count_where` Method

To count records based on a condition, you can use the count_where method.

```rust
#[tokio::test]
async fn test_count_where() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;
    let weapon_schema::Weapon { strength, .. } = &Weapon::schema();

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
