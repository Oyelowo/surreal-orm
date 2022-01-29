use std::time::Duration;

use actix_web::{guard, web, App, HttpServer};
use chrono::Utc;
use configs::{Configs};
// use configs::{index, index_playground, Configs, GraphQlApp};
pub mod configs;
// pub mod post;
// pub mod user;

use dotenv::dotenv;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, ConnectOptions};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let Configs {
        application_settings: application,
        database_settings: database,
    } = Configs::init();
    let app_url = &application.get_url();

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy_with(database.with_db());

    println!("Playground: {}", app_url);

    sqlx::migrate!("./migrations").run(&connection_pool).await?;

    // let schema = GraphQlApp::setup()
    //     .await
    //     .expect("Problem setting up graphql");

    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(schema.clone()))
            // .app_data(web::Data::new(connection_pool.clone()))
            // .service(web::resource("/").guard(guard::Post()).to(index))
            // .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("localhost:8000")?
    .run()
    .await?;

    Ok(())
}
