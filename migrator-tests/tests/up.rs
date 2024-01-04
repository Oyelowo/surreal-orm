use std::{
    fs::{self},
    path::PathBuf,
};

use chrono::Utc;
use migrator_tests::{assert_with_db_instance, AssertionArg, TestConfigNew};
use surreal_models::migrations::{
    Resources, ResourcesV10, ResourcesV2, ResourcesV3, ResourcesV4, ResourcesV5, ResourcesV6,
    ResourcesV7, ResourcesV8, ResourcesV9,
};
use surreal_orm::migrator::{
    config::{DatabaseConnection, UrlDb},
    Basename, FastForwardDelta, FileContent, Generate, Init, Migration, MigrationFilename,
    MigrationFlag, Migrator, MockPrompter, Mode, RenameOrDelete, SubCommand, Up,
};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tempfile::tempdir;
use test_case::test_case;
use typed_builder::TypedBuilder;

#[test_case(Mode::Strict, true; "Reversible Strict")]
#[test_case(Mode::Lax, true; "Reversible Lax")]
#[test_case(Mode::Strict, false; "Non-Reversible Strict")]
#[test_case(Mode::Lax, false; "Non-Reversible Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to detect migration type.")]
async fn test_one_way_cannot_run_up_without_init(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;
    let basename = "migration init";
    // let init_cmd = Init::builder()
    //     .name(basename.into())
    //     .reversible(reversible)
    //     .run(false)
    //     .build();
    // // let gen = SubCommand::Generate(Generate::builder().name(basename.into()).run(false).build());
    // conf.set_cmd(init_cmd)
    //     .run(Some(Resources), MockPrompter::default())
    //     .await;

    // conf.run_gen("migration gen 1".into(), Resources).await;
    //
    // conf.generate_12_test_migrations_reversible(reversible)
    //     .await;

    // 1st fwd
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
    let mut conf = TestConfigNew::new(mode, migration_dir).await;
    let get_mig_file_count = |num| {
        if reversible {
            num * 2
        } else {
            num
        }
    };

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
        expected_mig_files_count: get_mig_file_count(1),
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
        expected_mig_files_count: get_mig_file_count(1),
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
        expected_mig_files_count: get_mig_file_count(1),
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
async fn test_run_up_after_init_with_run(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;
    let get_mig_file_count = |num| {
        if reversible {
            num * 2
        } else {
            num
        }
    };

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
        expected_mig_files_count: get_mig_file_count(1),
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
        expected_mig_files_count: get_mig_file_count(1),
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
async fn test_run_up_default_which_is_latest(mode: Mode, reversible: bool) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;
    conf.run_up(&FastForwardDelta::default()).await;
    let get_mig_file_count = |num| {
        if reversible {
            num * 2
        } else {
            num
        }
    };

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        // expected_mig_files_count: get_mig_file_count(12),
        expected_mig_files_count: get_mig_file_count(12),
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
async fn test_run_up_with_explicit_number_delta_fwd_strategy(mode: Mode, reversible: bool) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;
    let get_mig_file_count = |num| {
        if reversible {
            num * 2
        } else {
            num
        }
    };

    conf.run_up(&FastForwardDelta::builder().number(1).build())
        .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: get_mig_file_count(12),
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
        expected_mig_files_count: get_mig_file_count(12),
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
        expected_mig_files_count: get_mig_file_count(12),
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
        expected_mig_files_count: get_mig_file_count(12),
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
        expected_mig_files_count: get_mig_file_count(12),
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
        expected_mig_files_count: get_mig_file_count(12),
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
async fn text_mixed_run_up_strategies_with_larger_runs(mode: Mode, reversible: bool) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_test_migrations_arbitrary(69, reversible.into())
        .await;
    let get_mig_file_count = |num| {
        if reversible {
            num * 2
        } else {
            num
        }
    };
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: get_mig_file_count(69),
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
        expected_mig_files_count: get_mig_file_count(69),
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
        expected_mig_files_count: get_mig_file_count(69),
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
async fn test_run_up_to_latest_with_number_delta_strategy(mode: Mode, reversible: bool) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_test_migrations_arbitrary(69, reversible.into())
        .await;
    let get_mig_file_count = |num| {
        if reversible {
            num * 2
        } else {
            num
        }
    };
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: get_mig_file_count(69),
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
        expected_mig_files_count: get_mig_file_count(69),
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
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_12_test_migrations_reversible(reversible.into())
        .await;
    let get_mig_file_count = |num| {
        if reversible {
            num * 2
        } else {
            num
        }
    };
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: get_mig_file_count(12),
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
        expected_mig_files_count: get_mig_file_count(12),
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: Some(
            "migration_12_gen_after_init".into(),
        ),
        expected_latest_db_migration_meta_basename_normalized: None,
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
async fn test_apply_till_migration_filename_pointer(mode: Mode, reversible: bool) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_12_test_migrations_reversible(reversible)
        .await;

    // First apply all generated migrations to the current db instance
    // conf.run_up(&FastForwardDelta::default()).await;
    let migration_position = |num| {
        if reversible {
            num * 2
        } else {
            num
        }
    };

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: migration_position(12),
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
            .till(conf.get_either_filename_type_at_position(migration_position(5)))
            .build(),
    )
    .await;

    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: migration_position(12),
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
                .till(conf.get_either_filename_type_at_position(migration_position(i)))
                .build(),
        )
        .await;

        assert_with_db_instance(AssertionArg {
            migration_type: reversible.into(),
            expected_mig_files_count: migration_position(12),
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
            .till(conf.get_either_filename_type_at_position(migration_position(12)))
            .build(),
    )
    .await;
    assert_with_db_instance(AssertionArg {
        migration_type: reversible.into(),
        expected_mig_files_count: migration_position(12),
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
        expected_mig_files_count: migration_position(12),
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

// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// #[should_panic(expected = "Failed to run migrations. Migration already run or not found")]
// async fn test_cannot_apply_already_applied(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//     let cli_db = conf.db().clone();
//
//     let files = read_up_fwd_migrations_from_dir_sorted(&temp_test_migration_dir);
//     let file12 = files.get(11).unwrap().to_owned();
//     let ref default_fwd_strategy = FastForwardDelta::builder().till(file12).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 12,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
// }
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// #[should_panic(expected = "Failed to run migrations. Migration already run or not found")]
// async fn test_cannot_apply_older(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//
//     let files = read_up_fwd_migrations_from_dir_sorted(&temp_test_migration_dir);
//     let file5 = files.get(4).unwrap().to_owned();
//     let ref default_fwd_strategy5 = FastForwardDelta::builder().till(file5).build();
//     conf.up_cmd(default_fwd_strategy5).await.run_up_fn().await;
//
//     let file12 = files.get(11).unwrap().to_owned();
//     let ref default_fwd_strategy12 = FastForwardDelta::builder().till(file12).build();
//     conf.up_cmd(default_fwd_strategy12).await.run_up_fn().await;
//
//     conf.up_cmd(default_fwd_strategy5).await.run_up_fn().await;
// }
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// #[should_panic(expected = "Failed to run migrations. Migration already run or not found")]
// async fn test_cannot_apply_nonexisting_migration(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//
//     let (mut conf, _) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//     let non_existing_filename = MigrationFilename::create_oneway(
//         Utc::now(),
//         &Basename::new("nonexesint migration hahahahahah"),
//     )
//     .unwrap();
//
//     let ref default_fwd_strategy5 = FastForwardDelta::builder()
//         .till(non_existing_filename)
//         .build();
//     conf.up_cmd(default_fwd_strategy5).await.run_up_fn().await;
// }
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn test_mixture_of_update_strategies(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//     let cli_db = conf.db().clone();
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(1).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 1,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_1_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
//
//     let files = read_up_fwd_migrations_from_dir_sorted(&temp_test_migration_dir);
//     let file5 = files.get(4).unwrap().to_owned();
//     let ref default_fwd_strategy = FastForwardDelta::builder().till(file5).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 5,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_5_gen_after_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(2).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 7,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_7_gen_after_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
//
//     let files = read_up_fwd_migrations_from_dir_sorted(&temp_test_migration_dir);
//     let file9 = files.get(8).unwrap().to_owned();
//     let ref default_fwd_strategy = FastForwardDelta::builder().till(file9).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 9,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_9_gen_after_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 12,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
// }
