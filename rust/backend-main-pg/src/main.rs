use actix_web::{guard, web, App, HttpServer};
use chrono::Utc;
// use configs::{index, index_playground, Configs, GraphQlApp};
// pub mod configs;
// pub mod post;
// pub mod user;
use sqlx::{postgres::PgRow, query, Row, Executor};
use uuid::{self, Uuid};

/*
// #[actix_web::main]
async fn _main() -> anyhow::Result<()> {
     //let Configs { application, .. } = Configs::init();
    // let app_url = &application.get_url();

    // println!("Playground: {}", app_url);

     let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");

    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let mut transaction = pool.begin().await?;

        let test_id = 1;

//    let k = query!(
//         r#"INSERT INTO users (id, created_at, updated_at, deleted_at, first_name, last_name, email, role, disabled, last_login)
//         VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10 )
//         "#,
//         "95022733-f013-301a-0ada-abc18f151006", Utc::now(), Utc::now(), Utc::now(), "oyelowo", "oyedayo", "oye@gmail.com", "admin", "maybe", Utc::now()
//     )
//     .execute(&mut transaction)
//     .await?;

    // check that inserted todo can be fetched
    let n = query!("SELEfCT content FROM poos WHERE id = $1", 1)
        .fetch_one(&mut transaction)
        .await?;


    transaction.rollback();

    // check that inserted todo is now gone
    // let inserted_todo = query!(r#"SELECT FROM todos WHERE id = $1"#, test_id)
    //     .fetch_one(&pool)
    //     .await;


    // let schema = GraphQlApp::setup()
    //     .await
    //     .expect("Problem setting up graphql");

    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(schema.clone()))
            // .service(web::resource("/").guard(guard::Post()).to(index))
            // .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("localhost:8000")?
    .run()
    .await?;


    println!("gfg");
    Ok(())
}

 */
use sqlx::postgres::PgPoolOptions;
use futures::TryStreamExt;
use futures::StreamExt;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:1234@localhost/my_db")
        .await?;
    // .connect("postgres://postgres:password@localhost/test").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    let user: User = sqlx::query_as!(User,
        "SELECT id, first_name, last_name FROM users where first_name = ?",
    )
    .bind("oyelowo".to_string())
    .fetch(&pool).await?;

    println!("rterweqtter{:?}", user);
    // countries[0].country
    // countries[0].count

    Ok(())
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
}
