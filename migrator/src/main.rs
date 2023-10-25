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
    m::Database::run_migrations(&"create_new_stuff".to_string(), false)
        .await
        .expect("Failed to run migrations");
}
