use actix_web::{guard, web, App, HttpServer};
use chrono::Utc;
// use configs::{index, index_playground, Configs, GraphQlApp};
// pub mod configs;
// pub mod post;
// pub mod user;
// use sqlx::{postgres::PgRow, query, Executor, Row};
// use async_std::sync::RwLock;
use dotenv::dotenv;
use ormx::{self, conditional_query_as, Insert, Patch, Table, Delete};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::{postgres::PgPool, query, query_as};
use validator::Validate;
use std::collections::hash_map::{Entry, HashMap};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;
use rand::Rng;

#[derive(Serialize, Deserialize, Table, Validate, Debug)]
#[ormx(table = "users", id = id, insertable, deletable)]
#[serde(rename_all = "camelCase")]
struct User {
    #[ormx(column = "id")]
    #[ormx(get_one)]
    // #[ormx(default, set)]
    id: Uuid,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    first_name: String,

    #[validate(length(min = 1), /*custom = "validate_unique_username"*/)]
    #[ormx(get_many)]
    last_name: String,

    #[validate(email)]
    #[ormx(get_optional(&str))]
    email: String,
}

// impl User {
//     /// Get the user's id.
//     fn set_id(&self) -> Uuid {
//         Uuid::new_v4()
//     }
// }

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let rand1: u8 = rng.gen();
    //let Configs { application, .. } = Configs::init();
    // let app_url = &application.get_url();

    // println!("Playground: {}", app_url);

    dotenv().ok();
    let conn_str =
        std::env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");

    let pool = sqlx::PgPool::connect(&conn_str).await?;
    let mut transaction = pool.begin().await?;

    let test_id = &Uuid::new_v4();

    let connection = &mut *pool.acquire().await?;

    let new_user = InsertUser {
        id: *test_id,
        first_name: "Ye".into(),
        last_name: "Blayz".into(),
        email: format!("blay{rand1}@gmail.com").into()
  
    }
    .insert(connection)
    .await?;

    println!("new userrr {:?}", new_user);


    println!("Getbyid{:?}", User::by_id(connection, &Uuid::from_str("528a8e4c-7f76-4e9d-b08a-198cc138cdd2")?).await?);

    
    // check that inserted todo can be fetched
    let n = User::by_id( &mut transaction, test_id).await?;

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
