/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use migrator_tests::{assert_with_db_instance, current_function, AssertionArg, TestConfigNew};
use surreal_models::migrations::{invalid_cases, Resources, ResourcesV2};
use surreal_orm::migrator::{Generate, Init, MockPrompter, Mode, RenameOrDelete};
use tempfile::tempdir;
use test_case::test_case;

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to detect migration type")]
async fn test_cannot_generate_without_db_run_without_init(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen 1".into())
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 0,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: None,
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot(current_function!());

    assert!(!migration_dir.exists(), "Migration directory cannot be created with generate if not migration not already initialized");
    assert!(
        false,
        "Should panic because we havent yet initialized migration. So, we should't get here."
    );
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to detect migration type")]
async fn test_cannot_generate_with_db_run_without_init(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen 1".into())
            .run(true)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 0,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: None,
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot(current_function!());
    assert!(!migration_dir.exists(), "Migration directory cannot be created with generate if not migration not already initialized");
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_can_generate_after_first_initializing_no_db_run(mode: Mode, reversible: bool) {
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
    assert!(migration_dir.exists());
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
    conf.assert_migration_queries_snapshot(current_function!());

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen 1".into())
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 2,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some("migration_gen_1".into()),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot(current_function!());
    assert!(migration_dir.exists());
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_can_generate_after_first_initializing_with_run(mode: Mode, reversible: bool) {
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
    assert!(migration_dir.exists());
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
    conf.assert_migration_queries_snapshot(current_function!());

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen 1".into())
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 2,
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: Some("migration_gen_1".into()),
        // we didnt run after genreate, so the latest db migration meta should remain the same
        // as the one created at initialization.
        expected_latest_db_migration_meta_basename_normalized: Some("migration_init".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot(current_function!());
    assert!(migration_dir.exists());
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_can_generate_with_run_after_first_initializing_with_run(
    mode: Mode,
    reversible: bool,
) {
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
    assert!(migration_dir.exists());
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
    conf.assert_migration_queries_snapshot(current_function!());

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen 1".into())
            .run(true)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 2,
        expected_db_mig_meta_count: 2,
        expected_latest_migration_file_basename_normalized: Some("migration_gen_1".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_gen_1".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot(current_function!());
    assert!(migration_dir.exists());
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_multiple_generation(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;
    assert!(migration_dir.exists());
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.assert_migration_queries_snapshot(current_function!());
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_two_way_can_disallow_empty_migration_gen_on_no_diff(mode: Mode, reversible: bool) {
    let resourve_v1 = || Resources;
    let mock_prompter_disallow_gen_on_empty_diff = || {
        MockPrompter::builder()
            // disallow empty migration generation on no diffs
            .allow_empty_migrations_gen(false)
            .rename_or_delete_single_field_change(RenameOrDelete::Delete)
            .build()
    };
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;
    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("migration init".into())
            .run(true)
            .build(),
        resourve_v1(),
        mock_prompter_disallow_gen_on_empty_diff(),
    )
    .await;
    assert!(migration_dir.exists());
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
    conf.assert_migration_queries_snapshot(current_function!());

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen 1 but we are not accept empty generation on no diff in mock prompter".into())
            .run(true)
            .build(),
        resourve_v1(),
        mock_prompter_disallow_gen_on_empty_diff(),
    )
    .await;
    assert!(migration_dir.exists());
    // New files wont be generated because there is no diff (Resources -> Resources), and we disallowed empty migrations
    // in mock prompter above
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
    conf.assert_migration_queries_snapshot(current_function!());

    // Redo and make sure
    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen again but we are not accept empty generation on no diff in mock prompter".into())
            .run(true)
            .build(),
        resourve_v1(),
        mock_prompter_disallow_gen_on_empty_diff(),
    )
    .await;
    assert!(migration_dir.exists());
    // New files wont be generated because there is no diff (Resources -> Resources), and we disallowed empty migrations
    // in mock prompter above
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
    conf.assert_migration_queries_snapshot(current_function!());

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen 1 this time we allow mock prompter to generate empty migration on no diff".into())
            .run(true)
            .build(),
        resourve_v1(),
        MockPrompter::builder()
        .allow_empty_migrations_gen(true)
        .rename_or_delete_single_field_change(RenameOrDelete::Delete)
        .build()
    )
    .await;
    assert!(migration_dir.exists());
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 2,
        expected_db_mig_meta_count: 2,
        expected_latest_migration_file_basename_normalized: Some("migration_gen_1_this_time_we_allow_mock_prompter_to_generate_empty_migration_on_no_diff".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_gen_1_this_time_we_allow_mock_prompter_to_generate_empty_migration_on_no_diff".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot(current_function!());
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to generate migrations")]
async fn should_panic_if_same_field_renaming_twice(mode: Mode, reversible: bool) {
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
    assert!(migration_dir.exists());
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
    conf.assert_migration_queries_snapshot(current_function!());

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration 2 gen".into())
            .run(true)
            .build(),
        ResourcesV2,
        MockPrompter::default(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 2,
        expected_db_mig_meta_count: 2,
        expected_latest_migration_file_basename_normalized: Some("migration_2_gen".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_2_gen".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot(current_function!());
    assert!(migration_dir.exists());

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration 3 gen".into())
            .run(true)
            .build(),
        ResourcesV2,
        MockPrompter::default(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 3,
        expected_db_mig_meta_count: 3,
        expected_latest_migration_file_basename_normalized: Some("migration_3_gen".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_3_gen".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    assert!(
        false,
        "Should panic because we are renaming the same field twice. So, we should't get here."
    );
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to generate migrations")]
async fn test_should_panic_if_same_field_renaming_using_same_old_field_cos_its_not_allowed(
    mode: Mode,
    reversible: bool,
) {
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
    assert!(migration_dir.exists());
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
    conf.assert_migration_queries_snapshot(current_function!());

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration 2 gen".into())
            .run(true)
            .build(),
        invalid_cases::ResourcesVRenamingWithSameOldFieldNameDisallowed,
        MockPrompter::default(),
    )
    .await;
    assert!(
        false,
        "Should panic because we are renaming using same old field name. So, we should't get here."
    );
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to generate migrations")]
async fn test_should_panic_if_renaming_from_currently_used_field(mode: Mode, reversible: bool) {
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
    assert!(migration_dir.exists());
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

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration 2 gen".into())
            .run(true)
            .build(),
        invalid_cases::ResourcesVRenamingFromCurrentlyUsedFieldNameDisallowed,
        MockPrompter::default(),
    )
    .await;
    assert!(
        false,
        "Should panic because we are renaming using same old field name. So, we should't get here."
    );
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to generate migrations")]
async fn test_should_panic_if_renaming_from_non_existing_field_in_migration_directory_state(
    mode: Mode,
    reversible: bool,
) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;
    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("migration init".into())
            .run(true)
            .build(),
        // Oyelowo January 5, 2023: we are using V2 here because AnimalV2 tries to rename but since we are not
        // initing from V1, we dont have the field to rename from, so this should panic
        ResourcesV2,
        MockPrompter::default(),
    )
    .await;
    assert!(migration_dir.exists());
    assert!(
        false,
        "Should panic because we are renaming using same old field name. So, we should't get here."
    );
}
