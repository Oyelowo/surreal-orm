/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use migrator_tests::{assert_with_db_instance, current_function, AssertionArg, TestConfigNew};
use surreal_models::migrations::Resources;
use surreal_orm::migrator::{FastForwardDelta, Init, MigrationFilename, MockPrompter, Mode};
use tempfile::tempdir;
use test_case::test_case;

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to detect migration type.")]
async fn test_one_way_cannot_run_up_without_init(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;

    conf.run_up(&FastForwardDelta::default()).await;

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
    conf.assert_migration_queries_snapshot();
}

// Cannot generate without init first
#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_run_up_after_init_with_no_run(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;

    // Init
    conf.run_init_cmd(
        Init::builder()
            .name("migration_init".into())
            .reversible(reversible)
            //  only initialize, do not run against db
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

    conf.run_up(&FastForwardDelta::builder().latest(true).build())
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
    conf.assert_migration_queries_snapshot();
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_run_up_after_init_with_run(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;

    // Init
    conf.run_init_cmd(
        Init::builder()
            .name("migration_init".into())
            .reversible(reversible)
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
    conf.assert_migration_queries_snapshot();
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_run_up_default_which_is_latest(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;
    conf.run_up(&FastForwardDelta::default()).await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot();
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_run_up_with_explicit_number_delta_fwd_strategy(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;

    conf.run_up(&FastForwardDelta::builder().number(1).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_1_init".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(5).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 6,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_6_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(0).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 6,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_6_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(1).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 7,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_7_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(5).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(1000).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot();
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn text_mixed_run_up_strategies_with_larger_runs(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_test_migrations_arbitrary(69, reversible.into())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 69,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_69_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(26).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 69,
        expected_db_mig_meta_count: 26,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_69_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_26_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().latest(true).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 69,
        expected_db_mig_meta_count: 69,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_69_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_69_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
    conf.assert_migration_queries_snapshot();
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_run_up_to_latest_with_number_delta_strategy(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_test_migrations_arbitrary(69, reversible.into())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 69,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_69_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(69).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 69,
        expected_db_mig_meta_count: 69,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_69_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_69_gen_after_init".into(),
        ),
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
async fn test_zero_delta_moves_no_needle(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible.into())
        .await;
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

    conf.run_up(&FastForwardDelta::builder().number(0).build())
        .await;
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
    conf.assert_migration_queries_snapshot();
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_apply_till_migration_filename_pointer(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;

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

    conf.run_up(
        &FastForwardDelta::builder()
            .till(conf.get_either_filename_type_at_position(5, reversible))
            .build(),
    )
    .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 5,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_5_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    for i in 6..=11 {
        conf.run_up(
            &FastForwardDelta::builder()
                .till(conf.get_either_filename_type_at_position(i, reversible))
                .build(),
        )
        .await;

        assert_with_db_instance(AssertionArg {
            migration_type: reversible.into(),
            expected_mig_files_count: 12,
            expected_db_mig_meta_count: i as u8,
            expected_latest_migration_file_basename_normalized: Some(
                "migration_12_gen_after_init".into(),
            ),
            expected_latest_db_migration_meta_basename_normalized: Some(
                format!("migration_{}{}", i, "_gen_after_init".to_string()).into(),
            ),
            code_origin_line: std::line!(),
            config: conf.clone(),
        })
        .await;
    }

    conf.run_up(
        &FastForwardDelta::builder()
            .till(conf.get_either_filename_type_at_position(12, reversible))
            .build(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::default()).await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
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
#[should_panic(expected = "Failed to run migrations. Migration already run or not found")]
async fn test_cannot_apply_already_applied(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible.into())
        .await;

    conf.run_up(
        &FastForwardDelta::builder()
            .till(conf.get_either_filename_type_at_position(12, reversible))
            .build(),
    )
    .await;
    conf.run_up(
        &FastForwardDelta::builder()
            .till(conf.get_either_filename_type_at_position(12, reversible))
            .build(),
    )
    .await;
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to run migrations. Migration already run or not found")]
async fn test_cannot_apply_older(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible.into())
        .await;

    conf.run_up(
        &FastForwardDelta::builder()
            .till(conf.get_either_filename_type_at_position(12, reversible))
            .build(),
    )
    .await;
    conf.run_up(
        &FastForwardDelta::builder()
            .till(conf.get_either_filename_type_at_position(5, reversible))
            .build(),
    )
    .await;
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to run migrations. Migration already run or not found")]
async fn test_cannot_apply_nonexisting_migration(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible.into())
        .await;
    let nonexisting_filename = MigrationFilename::try_from(
        "20231220050955_this_shit_dont_exist_hahahahahahah.up.surql".to_string(),
    )
    .expect("Failed to parse file name");
    let non_existing_name = if reversible {
        nonexisting_filename.to_up()
    } else {
        nonexisting_filename.to_unidirectional()
    };

    conf.run_up(&FastForwardDelta::builder().till(non_existing_name).build())
        .await;
}

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
async fn test_mixture_of_update_strategies(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir, current_function!()).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;

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

    conf.run_up(
        &FastForwardDelta::builder()
            .till(conf.get_either_filename_type_at_position(3, reversible))
            .build(),
    )
    .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 3,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_3_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().number(4).build())
        .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 7,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_7_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(
        &FastForwardDelta::builder()
            .till(conf.get_either_filename_type_at_position(9, reversible))
            .build(),
    )
    .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 9,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_9_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::builder().latest(true).build())
        .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
}
