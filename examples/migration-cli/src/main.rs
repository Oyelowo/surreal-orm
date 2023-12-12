use std::process::{Command, Stdio};

// use pretty_env_logger;
use surreal_models::migrations::Resources;
use surreal_orm::migrator::cli;
use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

async fn initialize_db() -> Surreal<Any> {
    let db = connect("http://localhost:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to signin");
    db.use_ns("test").use_db("test").await.unwrap();
    db
}

#[tokio::main]
async fn main() {
    // pretty_env_logger::init();
    let _db = initialize_db().await;
    // Directly run the cli
    // cli::migration_cli(Resources, Some(db)).await;
    cli::migration_cli(Resources).await;

    // Run the cli through cargo
    // _generate();
    // _run();
    // _rollback();
}

fn _generate() {
    // create
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("generate")
        .arg("--name")
        .arg("test migration")
        // .arg("--migrations-dir")
        // .arg("migrations")
        .arg("-r")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    // Wait for the command to finish
    let output = cmd.wait_with_output().expect("Failed to read stdout");

    // Validate output (replace this with your actual validation)
    assert!(output.status.success());
}

fn _run() {
    // RUN AGAINST DB
    let db_url = "http://localhost:8000";

    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("run")
        .arg("--migrations-dir")
        .arg("migrations")
        .arg("--db")
        .arg("test")
        .arg("--ns")
        .arg("test")
        .arg("--path")
        .arg(db_url)
        .arg("-r")
        .spawn()
        .expect("Failed to run command");

    let output = cmd.wait_with_output().expect("Failed to read stdout");
    // Validate output (replace this with your actual validation)
    assert!(output.status.success());
}

fn _rollback() {
    let db_url = "http://localhost:8000";
    // Rollback
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("rollback")
        .arg("--migrations-dir")
        .arg("migrations")
        .arg("--db")
        .arg("test")
        .arg("--ns")
        .arg("test")
        .arg("--path")
        .arg(db_url)
        .arg("-r")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    // Wait for the command to finish
    let output = cmd.wait_with_output().expect("Failed to read stdout");

    // Validate output (replace this with your actual validation)
    assert!(output.status.success());
}
