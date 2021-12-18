#![warn(unused_imports)]
use actix_web::{guard, web, App, HttpServer};

mod configs;
use configs::{index, index_playground, Configs, GraphQlApp};

use crate::configs::ApplicationConfigs;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let Configs {
        application: ApplicationConfigs { domain, .. },
        ..
    } = Configs::init();

    println!("Playground: {}", domain);

    let schema = GraphQlApp::setup().expect("Problem setting up graphql");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(domain)?
    .run()
    .await?;

    Ok(())
}
