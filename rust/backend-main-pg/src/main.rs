use actix_web::{guard, web, App, HttpServer};
use chrono::Utc;
// use configs::{index, index_playground, Configs, GraphQlApp};
// pub mod configs;
// pub mod post;
// pub mod user;
// use sqlx::{postgres::PgRow, query, Executor, Row};
// use async_std::sync::RwLock;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::{postgres::PgPool, query, query_as};
use std::collections::hash_map::{Entry, HashMap};
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    //let Configs { application, .. } = Configs::init();
    // let app_url = &application.get_url();

    // println!("Playground: {}", app_url);

    dotenv().ok();
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");

    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let mut transaction = pool.begin().await?;

    let test_id = Uuid::new_v4();

    let k = query_as!(
        User,
        r#"INSERT INTO users (id, first_name, last_name, email) VALUES 
        ( $1, $2, $3, $4) returning id, first_name, last_name, email
        "#,
        test_id,
        "oyelowo",
        "oyedayo",
        "oyej2@gmail.com"
    )
    .fetch_one(&pool)
    .await?;

    // check that inserted todo can be fetched
    let n = query_as!(User, "SELECT * FROM users WHERE id = $1", test_id)
        .fetch_one(&mut transaction)
        .await?;

    println!("#TRE{:?}", n);

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

#[derive(sqlx::FromRow, Debug, Clone)]
struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
}
