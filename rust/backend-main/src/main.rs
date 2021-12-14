#![warn(unused_imports)]
use actix_web::{guard, web, App, HttpServer};

mod configs;
use configs::{index, index_playground, GraphQlApp};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Playground: http://localhost:8000");

    let schema = GraphQlApp::setup().expect("Problem setting up graphql");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await 
}
