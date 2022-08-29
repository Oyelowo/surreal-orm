use anyhow::Context;
use backoff::ExponentialBackoff;
use common::{
    configurations::{
        application::{ApplicationConfigs, Environment},
        mongodb::MongodbConfigs,
        oauth::{OauthGithubCredentials, OauthGoogleCredentials},
        redis::RedisConfigs,
    },
    middleware,
    oauth::{
        cache_storage::{self, RedisCache},
        github::GithubConfig,
        google::GoogleConfig,
    },
};

use backoff::future::retry;
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
async fn main() -> anyhow::Result<()> {
    let log_level = match &ApplicationConfigs::default().environment {
        Environment::Local => "debug",
        _ => "info",
    };

    std::env::set_var("RUST_LOG", log_level);
    // env_logger::init();
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "poem=debug");
    // }
    tracing_subscriber::fmt::init();
    let backoff = ExponentialBackoff::default();

    let operation = || async { Ok(run_app().await?) };

    retry(backoff, operation).await
}

async fn run_app() -> anyhow::Result<()> {
    let application = ApplicationConfigs::default();
    let environment = application.clone().environment;
    let redis_config = RedisConfigs::default();

    let redis = redis_config
        .clone()
        .get_client()
        .context("Problem getting redis")?;

    let oauth_client = get_oauth_client(redis.clone());
    let database = MongodbConfigs::default()
        .get_database()
        .context("Problem getting database")?;

    sync_mongo_models(&database).await?;

    let app_url = &application.get_url();

    let schema = setup_graphql(database.clone(), &environment);

    let session = middleware::get_session(redis_config.clone(), &environment)
        .await
        .context("Problem getting session")?;

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
        .data(oauth_client)
        .with(session)
        .with(middleware::get_cors(environment))
        // .with(Logger)
        .with(Tracing);

    log::info!("Playground: {app_url}");

    Server::new(TcpListener::bind(app_url))
        .run(api)
        .await
        .context("Problem running server")?;
    Ok(())
}

use common::oauth::cache_storage::CacheStorage;
use common::oauth::client::OauthClient;

fn get_oauth_client(redis_client: redis::Client) -> OauthClient<RedisCache> {
    let cache_storage = RedisCache::new(redis_client);
    let base_url = String::from("https://oyelowo.test");
    let github_creds = OauthGithubCredentials::default();

    let google_creds = OauthGoogleCredentials::default();

    let github = GithubConfig::new(&base_url, github_creds);
    let google = GoogleConfig::new(&base_url, google_creds);

    OauthClient::builder()
        .github(github)
        .google(google)
        .cache_storage(cache_storage)
        .build()
}
