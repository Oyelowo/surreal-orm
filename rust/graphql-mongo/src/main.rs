use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use graphql_mongo::configs::{index, index_playground, Configs, GraphQlApp};
use log::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Configs { application, .. } = Configs::init();
    let app_url = &application.get_url();

    info!("Playground: {}", app_url);

    let schema = GraphQlApp::setup()
        .await
        .expect("Problem setting up graphql");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000/")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".localhost:8000"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .service(index)
            .service(index_playground)
    })
    .bind(app_url)?
    .run()
    .await?;

    Ok(())
}
