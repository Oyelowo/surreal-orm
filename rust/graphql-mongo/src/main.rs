use std::process;

use common::{
    configurations::{
        application::{ApplicationConfigs, Environment},
        mongodb::MongodbConfigs,
        redis::RedisConfigs,
    },
    middleware,
};

use graphql_mongo::{
    app::sync_mongo_models,
    handlers::{
        healthcheck::{healthz, liveness},
        oauth::{oauth_login_authentication, oauth_login_initiator},
    },
    utils::graphql::{graphql_handler, graphql_handler_ws, graphql_playground, setup_graphql},
};

use poem::{get, listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};

#[tokio::main]
async fn main() {
    let application = ApplicationConfigs::default();
    let environment = application.clone().environment;
    let redis_config = RedisConfigs::default();

    let log_level = match &environment {
        Environment::Local => "debug",
        _ => "info",
    };

    std::env::set_var("RUST_LOG", log_level);
    // env_logger::init();
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "poem=debug");
    // }

    tracing_subscriber::fmt::init();

    let redis = redis_config.clone().get_client().unwrap_or_else(|e| {
        log::error!("Problem getting database. Error: {e:?}");
        process::exit(1)
    });
    let database = MongodbConfigs::default()
        .get_database()
        .unwrap_or_else(|e| {
            log::error!("Problem getting database. Error: {e:?}");
            process::exit(1)
        });

    sync_mongo_models(&database).await.expect("Problem syncing");

    let app_url = &application.get_url();

    let schema = setup_graphql(database.clone(), &environment);

    let session = middleware::get_session(redis_config.clone(), &environment)
        .await
        .unwrap_or_else(|e| {
            log::error!("{e:?}");
            process::exit(1)
        });

    let api_routes = Route::new()
        .at("/healthz", get(healthz))
        .at("/liveness", get(liveness))
        .at("/oauth/signin/:oauth_provider", get(oauth_login_initiator))
        .at("/oauth/callback", get(oauth_login_authentication))
        .at("/graphql", get(graphql_playground).post(graphql_handler))
        .at("/graphql/ws", get(graphql_handler_ws));

    let api = Route::new()
        .nest("/api", api_routes)
        .data(schema)
        .data(database)
        .data(redis)
        .data(redis_config)
        .with(session)
        .with(middleware::get_cors(environment))
        // .with(Logger)
        .with(Tracing);

    log::info!("Playground: {app_url}");

    Server::new(TcpListener::bind(app_url))
        .run(api)
        .await
        .unwrap_or_else(|e| {
            log::error!("Problem running server. Error: {e}");
            process::exit(1)
        });
}
