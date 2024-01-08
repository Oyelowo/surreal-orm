use std::process::{Command, Stdio};
use surreal_models::migrations::Resources;
use surreal_orm::migrator::Migrator;

#[tokio::main]
async fn main() {
    Migrator::run(Resources).await;
}

fn _init() {
    // create
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("init")
        .arg("--name")
        .arg("-r")
        .arg("--dir")
        .arg("test migration")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    // Wait for the command to finish
    let output = cmd.wait_with_output().expect("Failed to read stdout");

    // Validate output (replace this with your actual validation)
    assert!(output.status.success());
}
fn _generate() {
    // create
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("gen")
        .arg("--name")
        .arg("test migration")
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
        .arg("up")
        .arg("--dir")
        .arg("migrations")
        .arg("--db")
        .arg("test")
        .arg("--ns")
        .arg("test")
        .arg("--url")
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
        .arg("down")
        .arg("--dir")
        .arg("migrations")
        .arg("--db")
        .arg("test")
        .arg("--ns")
        .arg("test")
        .arg("--url")
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
