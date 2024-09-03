/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use migrator_tests::{current_function, AssertionArg, TestConfig};
use surreal_orm::migrator::{FastForwardDelta, Mode, Status};
use tempfile::tempdir;
use test_case::test_case;

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_migration_listing_based_on_status(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfig::new(mode, migration_dir, current_function!()).await;
    let db = conf.migrator.db().clone();
    let fm = conf.migrator.file_manager();

    conf.generate_test_migrations_arbitrary(89, reversible.into())
        .await;

    let migrations = |status| async move {
        async move {
            if reversible {
                fm.two_way().list_migrations(db, status, mode).await
            } else {
                fm.one_way().list_migrations(db, status, mode).await
            }
        }
        .await
        .unwrap()
    };

    conf.run_up(&FastForwardDelta::builder().number(32).build())
        .await;

    assert_eq!(migrations.clone()(Status::All).await.len(), 89);
    assert_eq!(migrations.clone()(Status::Pending).await.len(), 89 - 32);
    assert_eq!(migrations(Status::Applied).await.len(), 32);
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_migration_listing_is_idempotent_and_does_not_break_migration_state(
    mode: Mode,
    reversible: bool,
) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfig::new(mode, migration_dir, current_function!()).await;
    conf.generate_test_migrations_arbitrary(89, reversible.into())
        .await;
    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 89,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_89_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(32).build())
        .await;
    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 89,
        expected_db_mig_meta_count: 32,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_89_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_32_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
    })
    .await;

    conf.run_list(Status::All).await;
    conf.run_list(Status::Pending).await;
    conf.run_list(Status::Applied).await;

    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 89,
        expected_db_mig_meta_count: 32,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_89_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_32_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
    })
    .await;
}
