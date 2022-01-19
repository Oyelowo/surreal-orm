use actix_web::{guard, web, App, HttpServer};
use configs::{index, index_playground, Configs, GraphQlApp};
pub mod post;
pub mod configs;
pub mod user;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let Configs { application, .. } = Configs::init();
    let app_url: String = application.url.into();

    println!("Playground: {}", app_url);

    let schema = GraphQlApp::setup()
        .await
        .expect("Problem setting up graphql");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(app_url)?
    .run()
    .await?;

    Ok(())
}
