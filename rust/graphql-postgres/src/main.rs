use actix_web::{guard, web, App, HttpServer};
use configs::{index, index_playground, Configs, GraphQlApp};
pub mod app;
pub mod configs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_url = &Configs::init().application_settings.get_url();

    println!("Playground: {}", app_url);

    let schema = GraphQlApp::setup()
        .await
        .expect("Problem setting up graphql");

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
