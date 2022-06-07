use std::process;

use actix_web::{guard, web, App, HttpServer};
use graphql_postgres::utils::{
    configuration,
    graphql::{index, index_playground, setup_graphql_schema},
};
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_url = configuration::get_app_config().get_url();

    info!("Playground: {:?}", app_url);

    let schema = setup_graphql_schema().await.map_err(|e| {
        log::error!("Problem setting up graphql. Error: {e}");
        process::exit(-1)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_playground),
            )
    })
    .bind(app_url)?
    .run()
    .await?;

    Ok(())
}
