use std::process;

use anyhow::Context;
use common::{
    configurations::{application::ApplicationConfigs, redis::RedisConfigs},
    middleware,
};

use graphql_mongo::{
    handlers::oauth::{oauth_login_authentication, oauth_login_initiator},
    utils::graphql::{graphql_handler, graphql_handler_ws, graphql_playground, setup_graphql},
};
use log::info;
use poem::{
    get,
    listener::TcpListener,
    middleware::{AddData, Tracing},
    EndpointExt, Route, Server,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    // let application = ApplicationConfigs::get();
    let redis_config = RedisConfigs {
        username: "".into(),
        password: "1234".into(),
        host: "localhost".into(),
        port: 6379,
    };
    // let app_url = &application.get_url();

    // let schema = setup_graphql()
    //     .await
    //     .with_context(|| "Problem setting up graphql")
    //     .unwrap_or_else(|e| {
    //         log::error!("{e:?}");
    //         process::exit(1)
    //     });

    // let session = middleware::get_session(
    //     redis_config.clone(),
    //     common::configurations::application::Environment::Local,
    // )
    // .await
    // .unwrap_or_else(|e| {
    //     log::error!("{e:?}");
    //     process::exit(1)
    // });

    let app = Route::new()
        .at(
            "/api/oauth/signin/:oauth_provider",
            get(oauth_login_initiator),
        )
        .at("/api/oauth/callback", get(oauth_login_authentication))
        // .at(
        //     "/api/graphql",
        //     get(graphql_playground).post(graphql_handler),
        // )
        // .at("/api/graphql/ws", get(graphql_handler_ws))
        // .data(schema)
        // .with(AddData::new(con))
        .with(AddData::new(redis_config))
        // .with(session)
        .with(middleware::get_cors())
        // .with(Logger)
        .with(Tracing);

    info!("Playground:");

    Server::new(TcpListener::bind("localhost:8000"))
        .run(app)
        .await
        .unwrap_or_else(|e| {
            log::error!("Problem running server. Error: {e}");
            process::exit(-1)
        });
}
