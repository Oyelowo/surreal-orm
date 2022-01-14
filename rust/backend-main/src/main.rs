#![warn(unused_imports)]
use actix_web::{guard, web, App, HttpServer};

mod configs;

use anyhow::Context;
use configs::{index, index_playground, Configs, GraphQlApp};

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{ClientOptions, FindOptions, InsertOneOptions},
    results::InsertOneResult,
    Client, Database,
};

use serde::{de::value::Error, Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};
// This trait is required to use `try_next()` on the cursor
use futures::{stream::TryStreamExt, Future};
use async_trait::async_trait;
#[derive(Debug, Serialize, Deserialize, TypedBuilder, Validate)]
#[serde(rename_all = "camelCase")]
struct Book {
    // #[serde(with = "uuid_as_binary")]
    #[serde(rename = "_id")]
    #[builder(default)]
    pub id: ObjectId,

    #[validate(length(min = 1), custom = "validate_unique_username")]
    first_name: String,
    title: String,
    author: String,

    #[validate(email)]
    #[builder(default, setter(strip_option))]
    email: Option<String>,

    #[validate(range(min = 18, max = 50))]
    #[builder(default = 20)]
    age: u32,
}

// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum MongoCollectionError {
//     #[error("environment variable is not set")]
//     NotSet,

//     #[error("environment variable: `{0}` is invalid. Check that it is correctly spelled")]
//     Invalid(String),

//     #[error("unknown environment variable error. You are on your own. lol")]
//     Unknown,
// }
pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
trait OrmCollection {
    const COLLECTION_NAME: &'static str = "book";
    async fn save2(&self) -> anyhow::Result<&Self> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017/mydb").await?;

        // Manually set an option.
        client_options.app_name = Some("My App".into());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
        let typed_collection = db.collection::<Book>(Self::COLLECTION_NAME);

        let k = typed_collection.insert_one(self, None).await;
        Ok(self)
    }
    async fn save(
        &self,
        db: &Database,
        options: impl Into<Option<InsertOneOptions>>,
    ) -> InsertOneResult {
        let typed_collection = db.collection::<Book>(Self::COLLECTION_NAME);

        let k = typed_collection.insert_one(self, options).await.unwrap();
        k
    }
}

impl OrmCollection for Book {}

fn validate_unique_username(username: &str) -> std::result::Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // mongo_main().await.unwrap();
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017/mydb").await?;
    println!("kljhkl{:?}", client_options);
    // Manually set an option.
    client_options.app_name = Some("My App".into());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // Get a handle to a database.
    let db = client.database("mydb");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    let typed_collection = db.collection::<Book>("bookeeee");

    let books = vec![
        Book::builder()
            .title("Legend of Goro".into())
            .author("Oyelowo Oyedayo".into())
            .first_name("Oyelowo".into())
            .age(99)
            .build(),
        Book::builder()
            .title("Night of day".into())
            .author("Mari Koko".into())
            .first_name("Mari".into())
            .age(72)
            .build(),
    ];

    typed_collection.insert_many(books, None).await?;

    // Query the books in the collection with a filter and an option.
    // let filter = doc! { "author": "George Orwell" };
    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = typed_collection.find(None, None).await?;

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await? {
        println!("title: {:?}", book);
    }

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
    // // Parse a connection string into an options struct.
    // let mut client_options = ClientOptions::parse("mongodb://admin:password@localhost:27017/mydb").await?;
    // println!("kljhkl{:?}", client_options);
    // // Manually set an option.
    // client_options.app_name = Some("My App".into());

    // // Get a handle to the deployment.
    // let client = Client::with_options(client_options)?;

    // // List the names of the databases in that deployment.
    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    // // Get a handle to a database.
    // let db = client.database("mydb");

    // // List the names of the collections in that database.
    // for collection_name in db.list_collection_names(None).await? {
    //     println!("{}", collection_name);
    // }

    // let collection = db.collection::<Document>("writer");
    // let docs = vec![
    //     doc! {"title": "1984", "author": "George Orwell"},
    //     doc! { "title": "Animal Farm", "author": "George Orwell" },
    //     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    // ];

    // collection.insert_many(docs, None).await?;

    // let typed_collection = db.collection::<Book>("books");

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
    // let filter = doc! { "author": "George Orwell" };
    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    // let mut cursor = typed_collection.find(filter, find_options).await?;

    // // Iterate over the results of the cursor.
    // while let Some(book) = cursor.try_next().await? {
    //     println!("title: {}", book.title);
    // }
    Ok(())
}
