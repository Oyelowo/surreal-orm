use common::{get_test_function, local_function};

use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema, QueryRoot};
use async_graphql_actix_web::{Request, Response};
mod starwars;
use starwars::{models::*, resolvers::*};


async fn index(schema: web::Data<StarWarsSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}



async fn index2(
    // Schema now accessible here
    schema: web::Data<StarWarsSchema>,
    request: async_graphql_actix_web::Request,
) -> web::Json<Response> {
    web::Json(Response(schema.execute(request.into_inner()).await))
}




async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // common::get_test_function();
    // common::get_shared_function();
    // common::pub_struct!();
    // let kk = common::;
    let kk2 = local_function();

    println!("Hello, world!");
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    println!("Playground: http://localhost:8000");

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


