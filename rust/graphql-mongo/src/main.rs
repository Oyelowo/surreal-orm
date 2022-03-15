use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_web::{
    cookie::{self, time::Duration, Key},
    http,
    middleware::Logger,
    web::{self, scope},
    App, HttpServer,
};
use graphql_mongo::configs::{gql_playground, index, index_ws, Configs, GraphQlApp};
use log::info;
use secrecy::{ExposeSecret, Secret};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let Configs {
        application, redis, ..
    } = Configs::init();
    let app_url = &application.get_url();

    info!("Playground: {}", app_url);

    let schema = GraphQlApp::setup()
        .await
        .expect("Problem setting up graphql");

    // https://javascript.info/fetch-crossorigin#cors-for-safe-requests
    // https://docs.rs/actix-cors/0.5.4/actix_cors/index.html
    // http://www.ruanyifeng.com/blog/2016/04/cors.html
    // Cors short for Cross-Origin Resource Sharing.
    HttpServer::new(move || {
        let cors = Cors::default() // allowed_origin return access-control-allow-origin: * by default
            // .allowed_origin("http://localhost:3001/")
            // .allowed_origin("http://localhost:8000/")
            // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".localhost:3001"))
            // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".localhost:8000"))
            // .send_wildcard()
            .allow_any_origin() // FIXME: // remove after testing.
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                /* http::header::AUTHORIZATION, */ http::header::ACCEPT,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        // let k = RedisSession::new(redis.get_url(), redis_key.master())
        //     .cookie_name("test-session")
        //     .cookie_http_only(true)
        //     .cookie_max_age(Duration::MAX)
        //     // allow the cookie only from the current domain
        //     .cookie_same_site(cookie::SameSite::Lax);

        // let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
        // let message_store = CookieMessageStore::builder(secret_key.clone()).build();

        // Generate a random 32 byte key. Note that it is important to use a unique
        // private key for every project. Anyone with access to the key can generate
        // authentication cookies for any user!
        let hmac_secret_from_env_var = Secret::new("secret".to_string());
        // let redis_key = Key::from(hmac_secret_from_env_var.expose_secret().as_bytes());
        let redis_key = actix_web::cookie::Key::generate();
        // private_key.master()
        App::new()
        .wrap(cors)
        //  .wrap(TracingLogger::default())
        // cookie session middleware
        .wrap(Logger::default())
        .wrap(
            RedisSession::new(redis.get_url(), &[0; 32])
            // RedisSession::new(redis.get_url(), redis_key.master())
            .cookie_name("test-session")
            .cookie_http_only(true)
            // allow the cookie only from the current domain
            // .cookie_same_site(cookie::SameSite::Lax),
        )
            // Enable logger
            .app_data(web::Data::new(schema.clone()))
            .service(gql_playground)
            .service(index)
            .service(web::resource("/graphql/ws").to(index_ws))
        // .service(
        //     scope("/api").service(
        //         scope("/v1")
        //             .route("/signup", post().to(index)) // change index to signup
        //             .route("/login", post().to(index)) // change index to signin
        //             .route("/user-info", post().to(index)) // change index to user-info
        //             .route("/logout", post().to(index)), // change index to signgout
        //     ),
        // )
    })
    .bind(app_url)?
    .run()
    .await?;

    Ok(())
}
