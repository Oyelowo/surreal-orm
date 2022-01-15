#![warn(unused_imports)]
use std::fmt::Display;

use actix_web::{guard, web, App, HttpServer};

mod configs;

use anyhow::Context;
use configs::{index, index_playground, Configs, GraphQlApp};

use futures::stream::StreamExt;
use mongodb::options::{FindOneOptions, FindOptions, ReadConcern};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Client,
    prelude::Model,
    Result,
};

#[derive(Debug, Serialize, Deserialize, TypedBuilder, Validate, Model)]
#[serde(rename_all = "camelCase")]
// #[model(index(keys=r#"doc!{"email": 1}"#, options=r#"doc!{"unique": true}"#))]
struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<ObjectId>,

    #[validate(length(min = 1), custom = "validate_unique_username")]
    first_name: String,
    title: String,
    author: String,

    // #[builder(default, setter(strip_option))]
    #[validate(email)]
    email: String,

    #[validate(range(min = 18, max = 50))]
    #[builder(default = 20)]
    age: u32,
}

// Define a model. Simple as deriving a few traits.
#[derive(Debug, Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"email": 1}"#, options = r#"doc!{"unique": true}"#))]
struct User {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The user's email address.
    pub email: String,
}

fn validate_unique_username(username: &str) -> std::result::Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // mongo_main().await.expect("mongo flop");
    let uri = "mongodb://localhost:27017/";
    let db = Client::with_uri_str(uri).await?.database("mydb");
    Book::sync(&db).await?;

    // Create a book.
    let mut book = Book::builder()
        .title("Steroid Legend of Goro".into())
        .author("Oyelowo Oyedayo".into())
        .first_name("Oyedayoo".into())
        .email("ye@gmail.com".into())
        .age(99)
        .build();

    println!("Booid before {:?}", book);
    book.save(&db, None).await?;
    println!("Booid after id {:?}", book.id);
    println!("Booid after id() {:?}", book.id());

    // // fetch all books
    let mut cursor = Book::find(&db, None, None).await?;

    while let Some(book) = cursor.next().await {
        println!("Book...{:?}", book);
    }

    let p = FindOneOptions::builder()
        .read_concern(ReadConcern::majority())
        .build();

    let k = Book::find_one(&db, doc!{"email": "ye2@gmail.com"}, p).await?;
    println!("fdgfg: {:?}", k);

    let Configs { application, .. } = Configs::init();
    let domain = application.derive_domain();

    println!("Playground: {}", domain);

    let schema = GraphQlApp::setup().expect("Problem setting up graphql");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(domain)?
    .run()
    .await?;

    Ok(())
}

async fn mongo_main() -> anyhow::Result<()> {
    let uri = "mongodb://localhost:27017/";
    let db = Client::with_uri_str(uri).await?.database("mydb");
    Book::sync(&db).await?;

    // Create a book.
    let mut book = Book::builder()
        .title("Steroid Legend of Goro".into())
        .author("Oyelowo Oyedayo".into())
        .first_name("Oyedayoo".into())
        .email("ye@gmail.com".into())
        .age(99)
        .build();

    println!("Booid before {:?}", book);
    book.save(&db, None).await?;
    println!("Booid after {:?}", book);

    // fetch all books
    let mut cursor = Book::find(&db, None, None).await?;

    while let Some(user) = cursor.next().await {
        println!("User...{:?}", user);
    }

    // let books = vec![
    //     Book::builder()
    //         .title("Legend of Goro".into())
    //         .author("Oyelowo Oyedayo".into())
    //         .first_name("Oyelowo".into())
    //         .age(99)
    //         .build(),
    //     Book::builder()
    //         .title("Night of day".into())
    //         .author("Mari Koko".into())
    //         .first_name("Mari".into())
    //         .age(72)
    //         .build(),
    // ];

    // typed_collection.insert_many(books, None).await?;

    // // Query the books in the collection with a filter and an option.
    // // let filter = doc! { "author": "George Orwell" };
    // // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    // let mut cursor = typed_collection.find(None, None).await?;

    // // Iterate over the results of the cursor.
    // while let Some(book) = cursor.try_next().await? {
    //     println!("title: {:?}", book);
    // }

    Ok(())
}
