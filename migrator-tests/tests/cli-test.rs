use std::process::{Command, Stdio};
use tempfile::tempdir;

use surreal_orm::migrator::MigrationFileName;

#[tokio::test]
async fn test_generate_command_success() {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let test_migration_name = "test_migration";
    let _ = std::fs::read_dir(temp_test_migration_dir).expect_err("No such file or directory");

    // create
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("generate")
        .arg("--name")
        .arg("test migration")
        .arg("--migrations-dir")
        .arg(temp_test_migration_dir)
        .arg("-r")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    // Wait for the command to finish
    let output = cmd.wait_with_output().expect("Failed to read stdout");

    // Validate output (replace this with your actual validation)
    assert!(output.status.success());

    // read and assert the migration files
    let migration_files = std::fs::read_dir(temp_test_migration_dir)
        .expect("Failed to read dir")
        .collect::<Vec<_>>();
    assert_eq!(migration_files.len(), 2);

    for f in migration_files.iter() {
        let binding = f.as_ref().expect("Failed to read dir").path();
        let file_name = binding
            .file_name() //.expect("Failed to get file namezz
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to get file name");
        let file_name_parsed =
            MigrationFileName::try_from(file_name.to_string()).expect("Failed to parse file name");
        assert_eq!(
            file_name_parsed.timestamp(),
            file_name
                .split('_')
                .next()
                .expect("Failed to get timestamp")
                .parse::<u64>()
                .expect("Failed to parse timestamp")
        );

        if file_name.ends_with("_up.surql") {
            assert!(file_name.ends_with(&format!("{}_up.surql", test_migration_name)));
        } else if file_name.ends_with("_down.surql") {
            assert!(file_name.ends_with(&format!("{}_down.surql", test_migration_name)));
        }
    }

    // RUN AGAINST DB
    let dir = tempdir().expect("Failed to create temp directory");
    let db_path = &dir.path().join("my_rocksdb_instance.db");
    let db_url = format!("file://{}", &db_path.clone().display());

    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("run")
        .arg("--migrations-dir")
        .arg(temp_test_migration_dir)
        .arg("--db")
        .arg("test")
        .arg("--ns")
        .arg("test")
        .arg("--path")
        .arg(&db_url)
        .arg("-r")
        .spawn()
        .expect("Failed to run command");

    let output = cmd.wait_with_output().expect("Failed to read stdout");
    // Validate output (replace this with your actual validation)
    assert!(output.status.success());

    Command::new("ls")
        .arg(db_path)
        .status()
        .expect("Failed to ls");

    Command::new("rm")
        .arg("--rf")
        .arg(format!("{}/LOCK", &db_path.display()))
        .status()
        .expect("Failed to ls");

    let migration_files = std::fs::read_dir(temp_test_migration_dir)
        .expect("Failed to read dir")
        .collect::<Vec<_>>();

    for f in migration_files.iter() {
        let binding = f.as_ref().expect("Failed to read dir").path();
        let file_name = binding
            .file_name() //.expect("Failed to get file namezz
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to get file name");
        let file_name_parsed =
            MigrationFileName::try_from(file_name.to_string()).expect("Failed to parse file name");
        assert_eq!(
            file_name_parsed.timestamp(),
            file_name
                .split('_')
                .next()
                .expect("Failed to get timestamp")
                .parse::<u64>()
                .expect("Failed to parse timestamp")
        );

        if file_name.ends_with("_up.surql") {
            assert!(file_name.ends_with(&format!("{}_up.surql", test_migration_name)));
        } else if file_name.ends_with("_down.surql") {
            assert!(file_name.ends_with(&format!("{}_down.surql", test_migration_name)));
        }
    }
    assert_eq!(migration_files.len(), 2);

    // Rollback
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("rollback")
        .arg("--migrations-dir")
        .arg(temp_test_migration_dir)
        .arg("--db")
        .arg("test")
        .arg("--ns")
        .arg("test")
        .arg("--path")
        .arg(&db_url)
        .arg("-r")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    // Wait for the command to finish
    let output = cmd.wait_with_output().expect("Failed to read stdout");

    // Validate output (replace this with your actual validation)
    assert!(output.status.success());

    // read and assert the migration files
    let migration_files = std::fs::read_dir(temp_test_migration_dir)
        .expect("Failed to read dir")
        .collect::<Vec<_>>();
    assert_eq!(migration_files.len(), 0);
}
