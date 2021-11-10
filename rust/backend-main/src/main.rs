#![warn(unused_imports)]
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
// use async_graphql::;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_actix_web::{Request, Response};
use backend_main::starwar::StarWarsSchema;
use common::{self, alt_good_morning, good_morning, maths, sum};
use serde_json;


// use starwars_lib::{StarWars, StarWarsSchema};

// use starwars::{QueryRoot, StarWars, StarWarsSchema};

struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[derive(SimpleObject)]
pub struct Demo {
    pub id: usize,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn demo(&self, _ctx: &Context<'_>) -> Demo {
        Demo { id: 42 }
    }
}

//async fn index(schema: web::Data<StarWarsSchema>, req: Request) -> Response {
async fn index(schema: web::Data<StarWarsSchema>, req: Request) -> Response {
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
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        //.data(StarWars::new())
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
