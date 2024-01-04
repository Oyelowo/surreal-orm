use migrator_tests::{assert_with_db_instance, AssertionArg, TestConfigNew};
use surreal_models::migrations::{Resources, ResourcesV2};
use surreal_orm::migrator::{FastForwardDelta, Init, MockPrompter, Mode, RenameOrDelete};
use tempfile::tempdir;
use test_case::test_case;

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_init_without_run(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;

    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("migration init".into())
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some("migration_init".into()),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::default()).await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: Some("migration_init".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_init".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_init_with_run(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;

    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("migration init".into())
            .run(true)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: Some("migration_init".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_init".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.run_up(&FastForwardDelta::default()).await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: Some("migration_init".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_init".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(
    expected = "Migrations already initialized. Run 'cargo run -- reset' to reset migration. \
                    You can also specify the '-r' or '--reversible' argument to set as reversible. \
                    Or delete the migrations directory and run 'cargo run -- init' again."
)]
async fn test_cannot_init_twice_consecutively_with_same_names(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;

    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("migration init".into())
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some("migration_init".into()),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("migration init".into())
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(
    expected = "Migrations already initialized. Run 'cargo run -- reset' to reset migration. \
                    You can also specify the '-r' or '--reversible' argument to set as reversible. \
                    Or delete the migrations directory and run 'cargo run -- init' again."
)]
async fn test_cannot_init_twice_consecutively_with_different_names(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;

    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("migration init".into())
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some("migration_init".into()),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("another name".into())
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;
}
