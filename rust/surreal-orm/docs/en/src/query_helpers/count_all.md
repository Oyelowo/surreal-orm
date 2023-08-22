# count_all

## 5. `count_all` Method

When you need a count of all records in a table, the count_all method is your
go-to.

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
