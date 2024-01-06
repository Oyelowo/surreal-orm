/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use migrator_tests::{current_function, AssertionArg, TestConfigNew};
use surreal_models::migrations::{Resources, ResourcesV3};
use surreal_orm::migrator::{FastForwardDelta, Generate, Init, MockPrompter, Mode, Reset, Up};
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
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    assert!(!migration_dir.exists());

    conf.assert_with_db_instance(AssertionArg {
        expected_mig_files_count: 0,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: None,
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
}

// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn test_can_reset_after_init_no_run(mode: Mode, reversible: bool) {
//     let migration_dir = tempdir().expect("Failed to create temp directory");
//     let migration_dir = &migration_dir.path().join("migrations-tests");
//     let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
//
//     conf.run_init_cmd(
//         Init::builder()
//             .reversible(reversible)
//             .name("migration init".into())
//             .run(false)
//             .build(),
//         Resources,
//         MockPrompter::default(),
//     )
//     .await;
//
//     conf.assert_with_db_instance(AssertionArg {
//         expected_mig_files_count: 1,
//         expected_db_mig_meta_count: 0,
//         expected_latest_migration_file_basename_normalized: Some("migration_init".into()),
//         expected_latest_db_migration_meta_basename_normalized: None,
//         code_origin_line: std::line!(),
//     })
//     .await;
//
//     conf.run_cmd(
//         Reset::builder()
//             .name("migration reset".into())
//             .reversible(reversible)
//             .run(false)
//             .build(),
//         ResourcesV3,
//         MockPrompter::default(),
//     )
//     .await;
//
//     conf.assert_with_db_instance(AssertionArg {
//         expected_mig_files_count: 2,
//         expected_db_mig_meta_count: 0,
//         expected_latest_migration_file_basename_normalized: Some("migration_reset".into()),
//         expected_latest_db_migration_meta_basename_normalized: None,
//         code_origin_line: std::line!(),
//     })
//     .await;
// }
