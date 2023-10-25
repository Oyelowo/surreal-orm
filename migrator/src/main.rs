use std::fmt::Display;

use inquire::InquireError;
use m::{Planet, Student};
use migrator as m;
use surreal_orm::{
    statements::{begin_transaction, info_for},
    transaction, Buildable, Model, Node, Raw, Runnable, SurrealCrudNode, ToRaw,
};
use surrealdb::sql::{
    statements::{DefineStatement, DefineTokenStatement},
    Base, Statement,
};

#[tokio::main]
async fn main() {
    if let Err(e) = m::Database::generate_migrations(&"create_new_stuff".to_string(), true).await {
        println!("Error: {}", e);
    }
}
