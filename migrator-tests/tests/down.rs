/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use migrator_tests::{assert_with_db_instance, current_function, AssertionArg, TestConfigNew};
use surreal_orm::migrator::{
    FastForwardDelta, MigrationFilename, MigrationFlag, Mode, RollbackStrategyStruct,
};
use tempfile::tempdir;
use test_case::test_case;

#[test_case(Mode::Strict; "Reversible Strict")]
#[test_case(Mode::Lax; "Reversible Lax")]
#[tokio::test]
async fn test_rollback_previous(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_test_migrations().await;

    // let mut conf = TestConfigNew::new\(mode, &temp_test_migration_dir\).await;
    // let mut conf = TestConfigNew::new\(mode, &temp_test_migration_dir, current_function!\(\)).await;

    // First apply all generated migrations to the current db instance
    conf.run_up(&FastForwardDelta::default()).await;

    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    conf.run_down(
        &RollbackStrategyStruct::builder().previous(true).build(),
        false,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 11,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::default(), false)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 10,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_10_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(
        &RollbackStrategyStruct::builder().previous(true).build(),
        false,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    conf.run_up(default_fwd_strategy).await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    // Prune this time around
    conf.run_down(
        &RollbackStrategyStruct::builder().previous(true).build(),
        true,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 11,
        expected_db_mig_meta_count: 11,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::default(), false)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 11,
        expected_db_mig_meta_count: 10,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_10_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(
        &RollbackStrategyStruct::builder().previous(true).build(),
        false,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 11,
        expected_db_mig_meta_count: 9,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_9_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::default(), true)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 8,
        expected_db_mig_meta_count: 8,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_8_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_8_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    for i in 0..5 {
        conf.run_down(&RollbackStrategyStruct::default(), false)
            .await;
        assert_with_db_instance(AssertionArg {
            migration_type: MigrationFlag::TwoWay,
            expected_mig_files_count: 8,
            expected_db_mig_meta_count: 7 - i,
            expected_latest_migration_file_basename_normalized: Some(
                "migration_8_gen_after_init".into(),
            ),
            expected_latest_db_migration_meta_basename_normalized: Some(
                format!("migration_{}{}", 7 - i, "_gen_after_init".to_string()).into(),
            ),
            code_origin_line: std::line!(),
            config: conf.clone(),
        })
        .await;
    }

    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 8,
        expected_db_mig_meta_count: 3,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_8_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_3_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::default(), true)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 2,
        expected_db_mig_meta_count: 2,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_2_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_2_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::default(), true)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: Some("migration_1_init".into()),
        expected_latest_db_migration_meta_basename_normalized: Some("migration_1_init".into()),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::default(), false)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 1,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some("migration_1_init".into()),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::default(), true)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

#[test_case(Mode::Strict; "Reversible Strict")]
#[test_case(Mode::Lax; "Reversible Lax")]
#[tokio::test]
async fn test_rollback_number_delta(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_test_migrations().await;

    // First apply all generated migrations to the current db instance
    conf.run_up(&FastForwardDelta::default()).await;

    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    conf.run_down(&RollbackStrategyStruct::builder().number(1).build(), false)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 11,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::default(), false)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 10,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_10_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::builder().number(1).build(), false)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    conf.run_up(default_fwd_strategy).await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    conf.run_down(&RollbackStrategyStruct::builder().number(5).build(), false)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    conf.run_down(&RollbackStrategyStruct::builder().number(3).build(), true)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 4,
        expected_db_mig_meta_count: 4,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_4_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_4_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::builder().number(4).build(), false)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 4,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_4_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: None,
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_up(&FastForwardDelta::default()).await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 4,
        expected_db_mig_meta_count: 4,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_4_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_4_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackStrategyStruct::builder().number(400).build(), true)
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

#[test_case(Mode::Strict; "Reversible Strict")]
#[test_case(Mode::Lax; "Reversible Lax")]
#[tokio::test]
async fn test_rollback_till_pointer_mig_id(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_12_test_migrations_reversible(true).await;

    // First apply all generated migrations to the current db instance
    conf.run_up(&FastForwardDelta::default()).await;

    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    conf.run_down(
        &RollbackStrategyStruct::builder()
            .till(conf.get_down_filename_at_position(12))
            .build(),
        false,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 11,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(
        &RollbackStrategyStruct::builder()
            .till(conf.get_down_filename_at_position(11))
            .build(),
        false,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 10,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_10_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    for i in 10..0 {
        conf.run_down(
            &RollbackStrategyStruct::builder()
                .till(conf.get_down_filename_at_position(i))
                .build(),
            false,
        )
        .await;

        assert_with_db_instance(AssertionArg {
            migration_type: MigrationFlag::TwoWay,
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

    // Reset
    conf.run_up(&FastForwardDelta::default()).await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

    conf.run_down(
        &RollbackStrategyStruct::builder()
            .till(conf.get_down_filename_at_position(12))
            .build(),
        true,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 11,
        expected_db_mig_meta_count: 11,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_11_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(
        &RollbackStrategyStruct::builder()
            .till(conf.get_down_filename_at_position(4))
            .build(),
        true,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 3,
        expected_db_mig_meta_count: 3,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_3_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_3_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(
        &RollbackStrategyStruct::builder()
            .till(conf.get_down_filename_at_position(1))
            .build(),
        true,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
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

#[test_case(Mode::Strict; "Reversible Strict")]
#[test_case(Mode::Lax; "Reversible Lax")]
#[should_panic]
#[tokio::test]
async fn cannot_rollback_twice_to_same_cursor_cos_it_does_not_exist_the_second_time(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_test_migrations().await;

    // First apply all generated migrations to the current db instance
    conf.run_up(&FastForwardDelta::default()).await;

    let nonexisting_filename = MigrationFilename::try_from(
        "20231220050955_this_shit_dont_exist_hahahahahahah.up.surql".to_string(),
    )
    .expect("Failed to parse file name");
    conf.run_down(
        &RollbackStrategyStruct::builder()
            .till(nonexisting_filename)
            .build(),
        false,
    )
    .await;
    conf.assert_migration_queries_snapshot();
}

#[test_case(Mode::Strict; "Reversible Strict")]
#[test_case(Mode::Lax; "Reversible Lax")]
#[should_panic]
#[tokio::test]
async fn rollingback_to_nonexisting_filecursor_panics(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_test_migrations().await;

    // First apply all generated migrations to the current db instance
    conf.run_up(&FastForwardDelta::default()).await;

    // 12th exists the first time but not second
    conf.run_down(
        &RollbackStrategyStruct::builder()
            .till(conf.get_down_filename_at_position(12))
            .build(),
        false,
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: MigrationFlag::TwoWay,
        expected_mig_files_count: 12,
        expected_db_mig_meta_count: 10,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: Some(
            "migration_10_gen_after_init".into(),
        ),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    // 12th exists the first time but not second
    conf.run_down(
        &RollbackStrategyStruct::builder()
            .till(conf.get_down_filename_at_position(12))
            .build(),
        false,
    )
    .await;
    conf.assert_migration_queries_snapshot();
}
