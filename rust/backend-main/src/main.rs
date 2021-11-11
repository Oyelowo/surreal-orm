#![warn(unused_imports)]
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use backend_main::{
    get_graphql_schema,
    GraphQLSchema,
    starwar::{model::StarWars},
};
use common::{self, alt_good_morning, good_morning, maths, sum};

async fn index(schema: web::Data<GraphQLSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = get_graphql_schema().data(StarWars::new()).finish();

    example_shared_libaray();
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

fn example_shared_libaray() {
    let sum = sum!(3, 3, 5, 6);
    print!("Sum of some: {:?}", sum);

    let sum2 = common::sum!(4);
    print!("Sum of some: {:?}", sum2);

    good_morning();
    alt_good_morning();

    let added = maths::add_one(3);
    println!("Hello, world!, {}", added);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        assert_eq!(super::maths::add_one(13), 14);
        assert_eq!(sum!(5, 5, 5), 15);
    }
}
