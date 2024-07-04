use surreal_orm::{query, query_raw};
use surrealdb::{engine::local::Mem, Surreal};

#[tokio::test]
async fn test_query_macro_with_bindigns() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let _query = query!(db, "SELECT * FROM users").await;
    let _query = query!(db, "SELECT * FROM users", {}).await;
    let _query = query!(db, "SELECT * FROM users WHERE id = $id", {id : 1} ).await;
    let username = "Oyelowo";
    let _query = query!(db, "SELECT name, age FROM users WHERE id = $id AND name = $name", {
        id : 1,
        name : username
    })
    .await;

    // db.query("SELECT name, age, * FROM users WHERE id = $id AND name = $name")
    //     .bind(("id", 1))
    //     .bind(("name", "Oyelowo"));
}

#[tokio::test]
async fn test_complex_multiple_queries() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let _queries = query!(
        db,
        [
            "SELECT * FROM users WHERE id = $id",
            "CREATE user:oyelowo SET name = $name, planet = 'Codebreather', skills = $skills"
        ],
        {
            id: 1,
            name: "Oyelowo",
            skills: vec!["Rust", "python", "typescript"]
        }
    )
    .await;
}

#[test]
fn test_query_macro() {
    let query = query_raw!("SELECT name, age, * FROM users");
    assert_eq!(query, "SELECT name, age, * FROM users");
}

#[test]
fn test_query_macro_with_params() {
    let query = query_raw!("SELECT name, age, * FROM users WHERE name = $1 AND name = 'Oyelowo'");
    assert_eq!(
        query,
        "SELECT name, age, * FROM users WHERE name = $1 AND name = 'Oyelowo'"
    );
}

#[test]
fn test_query_macro_with_graph() {
    let query = query_raw!("SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL");
    assert_eq!(
        query,
        "SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL"
    );
}
