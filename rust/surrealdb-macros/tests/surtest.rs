use serde::Deserialize;
use serde::Serialize;
use surrealdb::engine::local::Mem;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio::time;

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct User {
    id: String,
    name: String,
    company: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // let db = Surreal::new::<File>("localhost:8001").await?;
    // let db = Surreal::new::<File>("lowona").await?;
    let db = Surreal::new::<Mem>(()).await.unwrap();
    println!("lwowo1");

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .map_err(|e| eprintln!("Error: {}", e))
    .expect("derere");

    println!("lwowo2");

    // db.use_ns("namespace").use_db("database").await?;
    db.use_ns("test").use_db("test").await?;

    println!("lwowo3");
    let sql = "CREATE user SET name = $name, company = $company";

    let mut results = db
        .query(sql)
        .bind(User {
            id: "john".to_owned(),
            name: "John Doe".to_owned(),
            company: "ACME Corporation".to_owned(),
        })
        .await?;

    println!("lwowo4");
    // print the created user:
    let user: Option<User> = results.take(0)?;
    println!("{user:?}");

    let mut response = db
        .query("SELECT * FROM user WHERE name.first = 'John'")
        .await?;

    // print all users:
    println!("lwowo5");
    let users: Vec<User> = response.take(0)?;
    println!("{users:?}");

    Ok(())
}
