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
    let _db = initialize_db().await;
    // cli::migration_cli(Resources, Some(db)).await;
    cli::migration_cli(Resources, None).await;
}

#[cfg(test)]
mod test {
    use std::{
        io::Write,
        process::{Command, Stdio},
        time::Duration,
    };

    use surreal_models::migrations::Resources;
    use surreal_orm::{
        migrator::{cli::migration_cli, DbInfo, Informational, MigrationFileName},
        statements::info_for,
        Runnable,
    };
    use surrealdb::engine::any::connect;
    use tempfile::tempdir;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_generate_command_success() {
        let temp_test_migration_dir = "./migrations-tests";
        let test_migration_name = "test_migration";
        // Delete the test migration directory if it exists
        let _ = std::fs::remove_dir_all(temp_test_migration_dir);
        // read and assert the migration files
        // let migration_files =
        //     std::fs::read_dir(temp_test_migration_dir).expect_err("No such file or directory");
        // assert_eq!(migration_files.len(), 0);

        // create
        // Spawn the command
        let mut cmd = Command::new("cargo")
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

        // Write "yes" to stdin (change this to "no" if needed)
        // cmd.stdin.as_mut().unwrap().write_all(b"yes\n").unwrap();
        // cmd.stdin.as_mut().unwrap().write_all(b"1\n").unwrap();

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
            let file_name_parsed = MigrationFileName::try_from(file_name.to_string())
                .expect("Failed to parse file name");
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

        // let _ = std::fs::remove_dir_all(temp_test_migration_dir).expect("Failed to remove dir");

        // Run

        // let db = super::initialize_db().await;
        let dir = tempdir().expect("Failed to create temp directory");
        let db_path = dir.path().join("my_rocksdb_instance.db");
        let db_url = format!("file://{}", db_path.clone().display());
        // let db_url = format!("mem://");
        // let db_url = format!("http://localhost:8000");

        // let db = connect(&db_url).await.unwrap();
        // db.use_ns("test").use_db("test").await.unwrap();
        // migration_cli(Resources, Some(db.clone())).await;

        // let info = info_for()
        //     .database()
        //     .get_data::<DbInfo>(db.clone())
        //     .await
        //     .unwrap();
        //
        // let x = info.unwrap().tables().get_names();
        // assert_eq!(x, vec![] as Vec<String>, "valid tables");

        // sleep(Duration::from_secs(5)).await; // Wait for 5 seconds

        Command::new("ls").arg(db_path.clone()).spawn().unwrap();

        let mut cmd = Command::new("cargo")
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
            // .arg("--user")
            // .arg("root")
            // .arg("--pass")
            // .arg("root")
            .arg("-r")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to run command");

        // check db_path content

        // Wait for the command to finish
        let output = cmd.wait_with_output().expect("Failed to read stdout");

        // Validate output (replace this with your actual validation)
        assert!(output.status.success());

        Command::new("ls").arg(db_path.clone()).spawn().unwrap();
        // let db_path_content = std::fs::read_dir(&db_path).expect("Failed to read dir");
        // assert_eq!(db_path_content.count(), 7);

        // sleep(Duration::from_secs(5)).await; // Wait for 5 seconds
        // let info = info_for()
        //     .database()
        //     .get_data::<DbInfo>(db.clone())
        //     .await
        //     .unwrap();
        //
        // let x = info.unwrap().tables().get_names();
        // assert_eq!(x, vec!["xxx".to_string()], "valid tables");
        //
        // Rollback
        // let mut cmd = Command::new("cargo")
        //     .arg("run")
        //     .arg("--")
        //     .arg("rollback")
        //     .arg("--migrations-dir")
        //     .arg(temp_test_migration_dir)
        //     .arg("--db")
        //     .arg("test")
        //     .arg("--ns")
        //     .arg("test")
        //     .arg("-r")
        //     .stdin(Stdio::piped())
        //     .spawn()
        //     .expect("Failed to run command");
        //
        // // Wait for the command to finish
        // let output = cmd.wait_with_output().expect("Failed to read stdout");
        //
        // // Validate output (replace this with your actual validation)
        // assert!(output.status.success());
        //
        // // read and assert the migration files
        // let migration_files = std::fs::read_dir(temp_test_migration_dir)
        //     .expect("Failed to read dir")
        //     .collect::<Vec<_>>();
        // assert_eq!(migration_files.len(), 1);
    }
}
