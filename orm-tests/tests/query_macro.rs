use surreal_orm::{query, surql};
use surrealdb::{engine::local::Mem, Surreal};

#[test]
fn test_query_macro() {
    let query = query!("SELECT name, age, * FROM users");
    assert_eq!(query, "SELECT name, age, * FROM users");
}

#[test]
fn test_query_macro_with_params() {
    let query = query!("SELECT name, age, * FROM users WHERE name = $1 AND name = 'Oyelowo'");
    assert_eq!(
        query,
        "SELECT name, age, * FROM users WHERE name = $1 AND name = 'Oyelowo'"
    );
}

#[test]
fn test_query_macro_with_graph() {
    let query = query!("SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL");
    assert_eq!(
        query,
        "SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL"
    );
}

#[tokio::test]
async fn test_query_simple() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_db("test").await;
    db.use_ns("test").await;

    let query = surql!(db, "SELECT * FROM users", {});
    let query = surql!(db, "SELECT * FROM users WHERE id = $id", {id : 1});
    let username = "Oyelowo";
    let query = surql!(db, "SELECT name, age FROM users WHERE id = $id AND name = $name", {
        id : 1,
        name : username
    });

    // db.query("SELECT name, age, * FROM users WHERE id = $id AND name = $name")
    //     .bind(("id", 1))
    //     .bind(("name", "Oyelowo"));
    // assert_eq!(query, "SELECT * FROM users");
}
