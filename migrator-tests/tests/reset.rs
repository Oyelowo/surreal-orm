/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use migrator_tests::{current_function, AssertionArg, TestConfig};
use surreal_models::migrations::{Resources, ResourcesV3};
use surreal_orm::migrator::{
    FastForwardDelta, Init, Migration, MigrationFilename, MockPrompter, Mode, Reset,
};
use tempfile::tempdir;
use test_case::test_case;

#[test_case(true,Mode::Strict,  true; "Reversible Strict Run")]
#[test_case(true,Mode::Lax,  false; "Reversible Lax No Run")]
#[test_case(false,Mode::Strict,  true; "Non-Reversible Strict Run")]
#[test_case(false, Mode::Lax,  false; "Non-Reversible Lax No Run")]
#[tokio::test]
async fn test_can_reset_before_init(reversible: bool, mode: Mode, run: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfig::new(mode, migration_dir, current_function!()).await;
    assert!(!migration_dir.exists());

    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 0,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: None,
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
    })
    .await;
    conf.assert_migration_queries_snapshot();

    conf.run_reset(
        Reset::builder()
            .name("migration reset".into())
            .reversible(reversible)
            .run(run)
            .build(),
        ResourcesV3,
        MockPrompter::default(),
    )
    .await;

    assert!(migration_dir.exists());
    if run {
        conf.assert_with_db_instance(AssertionArg {
            expected_mig_files_count: 1,
            expected_db_mig_meta_count: 1,
            expected_latest_migration_file_basename_normalized: Some("migration_reset".into()),
            expected_latest_db_migration_meta_basename_normalized: Some("migration_reset".into()),
            code_origin_line: std::line!(),
        })
        .await;
    } else {
        conf.assert_with_db_instance(AssertionArg {
            expected_mig_files_count: 1,
            expected_db_mig_meta_count: 0,
            expected_latest_migration_file_basename_normalized: Some("migration_reset".into()),
            expected_latest_db_migration_meta_basename_normalized: None,
            code_origin_line: std::line!(),
        })
        .await;
    }
    conf.assert_migration_queries_snapshot();
}

#[test_case(true,Mode::Strict,  true; "Reversible Strict No Run")]
#[test_case(true,Mode::Lax,  false; "Reversible Lax No Run")]
#[test_case(false,Mode::Strict,  true; "Non-Reversible Strict Run")]
#[test_case(false, Mode::Lax,  false; "Non-Reversible Lax No Run")]
#[tokio::test]
async fn test_can_reset_after_init_run(reversible: bool, mode: Mode, run: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfig::new(mode, migration_dir, current_function!()).await;
    assert!(!migration_dir.exists());
    conf.assert_migration_queries_snapshot();
    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 0,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: None,
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
    })
    .await;

    conf.run_init(
        Init::builder()
            .name("migration init".into())
            .reversible(reversible)
            .run(true)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;
    conf.assert_migration_queries_snapshot();
    assert!(migration_dir.exists());

    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: Some("migration_init".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_init".into()),
        code_origin_line: std::line!(),
    })
    .await;

    conf.run_reset(
        Reset::builder()
            .name("migration reset".into())
            .reversible(reversible)
            .run(run)
            .build(),
        ResourcesV3,
        MockPrompter::default(),
    )
    .await;
    conf.assert_migration_queries_snapshot();

    assert!(migration_dir.exists());
    if run {
        conf.assert_with_db_instance(AssertionArg {
            expected_mig_files_count: 1,
            expected_db_mig_meta_count: 1,
            expected_latest_migration_file_basename_normalized: Some("migration_reset".into()),
            expected_latest_db_migration_meta_basename_normalized: Some("migration_reset".into()),
            code_origin_line: std::line!(),
        })
        .await;
    } else {
        // When we reset, and not apply against the current database instance, the old
        // stale latest migration metadataa would still exist in the database.
        let latest_db_mig = Migration::get_latest(conf.migrator.db()).await.map(|m| {
            MigrationFilename::try_from(m.name)
                .expect("Failed to convert migration name to filename")
                .basename()
        });
        let mig_files = conf.read_migrations_from_dir_sorted_asc();
        let latest_file = mig_files.last().map(|f| f.basename());

        assert_eq!(latest_db_mig, Some("migration_init".into()));
        assert_eq!(latest_file, Some("migration_reset".into()));

        assert_eq!(mig_files.len(), if reversible { 2 } else { 1 });
        assert_eq!(Migration::get_all_desc(conf.migrator.db()).await.len(), 1);
    }
    conf.assert_migration_queries_snapshot();
}

#[test_case(true,Mode::Strict,  true; "Reversible Strict Run")]
#[test_case(true,Mode::Lax,  false; "Reversible Lax No Run")]
#[test_case(false,Mode::Strict,  true; "Non-Reversible Strict Run")]
#[test_case(false, Mode::Lax,  false; "Non-Reversible Lax No Run")]
#[tokio::test]
async fn test_reset_after_multiple_unapplied_migrations(reversible: bool, mode: Mode, run: bool) {
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

    conf.run_reset(
        Reset::builder()
            .name("migration reset".into())
            .reversible(reversible)
            .run(run)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;

    if run {
        conf.assert_with_db_instance(AssertionArg {
            expected_mig_files_count: 1,
            expected_db_mig_meta_count: 1,
            expected_latest_migration_file_basename_normalized: Some("migration_reset".into()),
            expected_latest_db_migration_meta_basename_normalized: Some("migration_reset".into()),
            code_origin_line: std::line!(),
        })
        .await;
    } else {
        // When we reset, and not apply against the current database instance, the old
        // stale latest migration metadataa would still exist in the database.
        let latest_db_mig = Migration::get_latest(conf.migrator.db()).await.map(|m| {
            MigrationFilename::try_from(m.name)
                .expect("Failed to convert migration name to filename")
                .basename()
        });
        let mig_files = conf.read_migrations_from_dir_sorted_asc();
        let latest_file = mig_files.last().map(|f| f.basename());

        assert_eq!(latest_db_mig, None);
        assert_eq!(Migration::get_all_desc(conf.migrator.db()).await.len(), 0);

        assert_eq!(latest_file, Some("migration_reset".into()));
        assert_eq!(mig_files.len(), if reversible { 2 } else { 1 });
    }
    conf.assert_migration_queries_snapshot();
}

#[test_case(true,Mode::Strict,  true; "Reversible Strict Run")]
#[test_case(true,Mode::Lax,  false; "Reversible Lax No Run")]
#[test_case(false,Mode::Strict,  true; "Non-Reversible Strict Run")]
#[test_case(false, Mode::Lax,  false; "Non-Reversible Lax No Run")]
#[tokio::test]
async fn test_reset_after_multiple_applied_migrations(reversible: bool, mode: Mode, run: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfig::new(mode, migration_dir, current_function!()).await;

    conf.generate_12_test_migrations_reversible(reversible)
        .await;
    conf.run_up(&FastForwardDelta::default()).await;

    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
    })
    .await;

    conf.run_reset(
        Reset::builder()
            .name("migration reset".into())
            .reversible(reversible)
            .run(run)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;

    if run {
        conf.assert_with_db_instance(AssertionArg {
            expected_mig_files_count: 1,
            expected_db_mig_meta_count: 1,
            expected_latest_migration_file_basename_normalized: Some("migration_reset".into()),
            expected_latest_db_migration_meta_basename_normalized: Some("migration_reset".into()),
            code_origin_line: std::line!(),
        })
        .await;
    } else {
        // When we reset, and not apply against the current database instance, the old
        // stale latest migration metadataa would still exist in the database.
        let latest_db_mig = Migration::get_latest(conf.migrator.db()).await.map(|m| {
            MigrationFilename::try_from(m.name)
                .expect("Failed to convert migration name to filename")
                .basename()
        });
        let mig_files = conf.read_migrations_from_dir_sorted_asc();
        let latest_file = mig_files.last().map(|f| f.basename());

        assert_eq!(Migration::get_all_desc(conf.migrator.db()).await.len(), 12);
        assert_eq!(latest_db_mig, Some("migration_12_gen_after_init".into()));

        assert_eq!(mig_files.len(), if reversible { 2 } else { 1 });
        assert_eq!(latest_file, Some("migration_reset".into()));
    }
    conf.assert_migration_queries_snapshot();
}

#[test_case(true,Mode::Strict,  true; "Reversible Strict Run")]
#[test_case(true,Mode::Lax,  false; "Reversible Lax No Run")]
#[test_case(false,Mode::Strict,  true; "Non-Reversible Strict Run")]
#[test_case(false, Mode::Lax,  false; "Non-Reversible Lax No Run")]
#[tokio::test]
async fn test_reset_after_multiple_pending_migrations(reversible: bool, mode: Mode, run: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfig::new(mode, migration_dir, current_function!()).await;

    conf.generate_12_test_migrations_reversible(reversible)
        .await;
    conf.run_up(&FastForwardDelta::builder().number(7).build())
        .await;

    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 7,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_7_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
    })
    .await;

    conf.run_reset(
        Reset::builder()
            .name("migration reset".into())
            .reversible(reversible)
            .run(run)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;

    if run {
        conf.assert_with_db_instance(AssertionArg {
            expected_mig_files_count: 1,
            expected_db_mig_meta_count: 1,
            expected_latest_migration_file_basename_normalized: Some("migration_reset".into()),
            expected_latest_db_migration_meta_basename_normalized: Some("migration_reset".into()),
            code_origin_line: std::line!(),
        })
        .await;
    } else {
        // When we reset, and not apply against the current database instance, the old
        // stale latest migration metadataa would still exist in the database.
        let latest_db_mig = Migration::get_latest(conf.migrator.db()).await.map(|m| {
            MigrationFilename::try_from(m.name)
                .expect("Failed to convert migration name to filename")
                .basename()
        });
        let mig_files = conf.read_migrations_from_dir_sorted_asc();
        let latest_file = mig_files.last().map(|f| f.basename());

        assert_eq!(Migration::get_all_desc(conf.migrator.db()).await.len(), 7);
        assert_eq!(latest_db_mig, Some("migration_7_gen_after_init".into()));

        assert_eq!(mig_files.len(), if reversible { 2 } else { 1 });
        assert_eq!(latest_file, Some("migration_reset".into()));
    }
    conf.assert_migration_queries_snapshot();
}
