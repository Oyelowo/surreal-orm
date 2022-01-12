#![warn(unused_imports)]
use actix_web::{guard, web, App, HttpServer};

mod configs;

use configs::{index, index_playground, Configs, GraphQlApp};



#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    mongo_main();
    let Configs { application, .. } = Configs::init();
    let domain = application.derive_domain();

    println!("Playground: {}", domain);

    let schema = GraphQlApp::setup().expect("Problem setting up graphql");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(domain)?
    .run()
    .await?;

    Ok(())
}

use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, FindOptions},
    Client,
};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};
// This trait is required to use `try_next()` on the cursor
use futures::stream::TryStreamExt;

#[derive(Debug, Serialize, Deserialize, TypedBuilder, Validate)]
struct Book {
    #[validate(length(min = 1), custom = "validate_unique_username")]
    #[serde(rename = "firstName")]
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

impl Book {
    const COLLECTION_NAME: &'static str = "book";
    async fn save(&self) -> anyhow::Result<&Self> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

        // Manually set an option.
        client_options.app_name = Some("My App".into());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
    let typed_collection = db.collection::<Book>("books");

        typed_collection.insert_one(self, None).await?;
        Ok(self)
    }
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}

async fn mongo_main() -> anyhow::Result<()> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

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

    let collection = db.collection::<Document>("books");
    let docs = vec![
        doc! {"title": "1984", "author": "George Orwell"},
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    collection.insert_many(docs, None).await?;

    let typed_collection = db.collection::<Book>("books");

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
    let filter = doc! { "author": "George Orwell" };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = typed_collection.find(filter, find_options).await?;

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await? {
        println!("title: {}", book.title);
    }
    Ok(())
}
