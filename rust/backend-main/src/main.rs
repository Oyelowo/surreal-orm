#![warn(unused_imports)]
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};
use async_graphql_actix_web::{Request, Response};
use std::{env};
use common::{self, alt_good_morning, good_morning, maths, sum};

pub mod starwar;
pub mod user;

use starwar::{StarWarQueryRoot, StarWars};
use user::{UserData, UserMutationRoot, UserQueryRoot};

#[derive(MergedObject, Default)]
pub struct Query(StarWarQueryRoot, UserQueryRoot);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutationRoot);

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;
// pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn get_graphql_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    return Schema::build(Query::default(), Mutation::default(), EmptySubscription);
}


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


// #[derive(Serialize)]
// enum ENV {
//     DEVEVELOPMENT,
//     PRODUCTION,
//     STAGING
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = env::var("RUST_ENV").unwrap_or("development".to_string());
    let limit = if env == "development" {
        usize::max_value()
    } else {
        5
    };
    let schema = get_graphql_schema()
        .data(StarWars::new())
        .data(UserData::new())
        .limit_depth(limit)
        // .limit_depth(5) // This and also limi_complexity will prevent the graphql playground document from showing because it's unable to do the complete tree parsing. TODO: Add it conditionally. i.e if not in development or test environemnt.
        .finish();

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
