use std::process::{Command, Stdio};
use tempfile::tempdir;

#[tokio::test]
async fn test_generate_command_success() {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");

    let db_url = "memory";
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("init")
        .arg("--name")
        .arg("test migration 1")
        .arg("--dir")
        .arg(temp_test_migration_dir)
        .arg("--reversible")
        .arg("--run")
        .arg("--url")
        .arg(&db_url)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run command");
    let output = cmd.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("up")
        .arg("--dir")
        .arg(temp_test_migration_dir)
        .arg("--db")
        .arg("test")
        .arg("--ns")
        .arg("test")
        .arg("--user")
        .arg("root")
        .arg("--pass")
        .arg("root")
        .arg("--url")
        .arg(&db_url)
        .spawn()
        .expect("Failed to run command");

    let output = cmd.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    // Rollback
    // let cmd = Command::new("cargo")
    //     .arg("run")
    //     .arg("--")
    //     .arg("down")
    //     .arg("--dir")
    //     .arg(temp_test_migration_dir)
    //     .arg("--db")
    //     .arg("test")
    //     .arg("--ns")
    //     .arg("test")
    //     .arg("--url")
    //     .arg(&db_url)
    //     .stdin(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to run command");
}
