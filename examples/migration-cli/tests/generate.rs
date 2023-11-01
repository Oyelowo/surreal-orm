use std::{io::Stdout, process::Command};

#[test]
fn test_migration_cli() {
    // cargo run -- generate --name create_user --reversible --migrations-dir ./migrations
    // cargo run -- run --reversible --migrations-dir ./migrations
    // cargo run -- rollback --reversible --migrations-dir ./migrations
    // cargo run -- reset --reversible --migrations-dir ./migrations
    // cargo run -- redo --reversible --migrations-dir ./migrations
    // cargo run -- status --reversible --migrations-dir ./migrations

    // Write tests for the above commands
}

#[test]
fn test_generate_command_success() {
    // let mut cmd = Command::cargo_bin("migration-cli").unwrap();
    // let mut cmd = Command::new("cargo");
    // let cmd = cmd
    //     .arg("run")
    //     .arg("--")
    //     .arg("generate")
    //     .arg("--name")
    //     .arg("test migration")
    //     .arg("-r")
    //     .spawn()
    //     .expect("Failed to run command");

    // cmd.stdout(Stdout);

    // .assert()
    // .success()
    // .stdout(predicate::str::contains(
    //     "Migration test_migration generated successfully",
    // ));
}
