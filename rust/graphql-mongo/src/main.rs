use std::process;

use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Context;
use graphql_mongo::utils::{
    configuration, cors,
    graphql::{gql_playground, index, index_ws, setup_graphql},
    session,
};
use log::info;
use tracing_actix_web::TracingLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let application = configuration::get_app_config();
    let redis = configuration::get_redis_config();
    let app_url = &application.get_url();

    info!("Playground: {}", app_url);

    let schema = setup_graphql()
        .await
        .with_context(|| "Problem setting up graphql")
        .unwrap_or_else(|e| {
            log::error!("{e:?}");
            process::exit(1)
        });

    HttpServer::new(move || {
        App::new()
            .wrap(cors::get_cors())
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(session::get_session_middleware(&redis, &application))
            .app_data(web::Data::new(schema.clone()))
            .service(gql_playground)
            .service(index)
            .service(web::resource("/graphql/ws").to(index_ws))
    })
    .bind(app_url)?
    .run()
    .await?;

    Ok(())
}
