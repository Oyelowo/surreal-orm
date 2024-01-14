/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use migrator_tests::{current_function, AssertionArg, TestConfig};
use surreal_orm::migrator::{FastForwardDelta, Mode};
use tempfile::tempdir;
use test_case::test_case;

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_can_prune_only_unapplied_migrations(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfig::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;
    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
    })
    .await;

    conf.run_prune().await;
    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 0,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: None,
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
    })
    .await;
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_can_prune_some_unapplied_and_some_applied_migrations(mode: Mode, reversible: bool) {
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

    conf.run_prune().await;
    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 32,
        expected_db_mig_meta_count: 32,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_32_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_32_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
    })
    .await;
}
