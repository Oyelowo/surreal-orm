use actix_web::{guard, web, App, HttpServer};
mod configs;
use configs::{index, index_playground, Configs, GraphQlApp};
pub mod book;
pub mod user;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let Configs { application, .. } = Configs::init();
    // let url = application.get_url();

    let url = String::from(application);
    println!("Playground: {}", url);
    println!("Playground. into: {}", url);

    let schema = GraphQlApp::setup()
        .await
        .expect("Problem setting up graphql");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(url)?
    .run()
    .await?;

    Ok(())
}
