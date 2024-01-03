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
        expected_latest_migration_file_basename_normalized: "".into(),
        expected_latest_db_migration_meta_basename_normalized: "".into(),
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

    // Init
    conf.run_init_cmd(
        Init::builder()
            .name("migration_init".into())
            .reversible(reversible)
            .run(false)
            .build(),
        Resources,
        MockPrompter::default(),
    )
    .await;

    conf.run_up(&FastForwardDelta::builder().latest(true).build())
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
        expected_mig_files_count: get_mig_file_count(1),
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: "migration_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    // conf.run_up(&FastForwardDelta::default()).await;
    // assert_with_db_instance(AssertionArg {
    //     expected_down_mig_files_count: 0,
    //     expected_db_mig_meta_count: 0,
    //     expected_latest_migration_file_basename_normalized: "".into(),
    //     expected_latest_db_migration_meta_basename_normalized: "".into(),
    //     code_origin_line: std::line!(),
    //     config: conf.clone(),
    // })
    // .await;
}

// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn test_run_up_after_init_with_run(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(reversible)
//         .db_run(true)
//         .mode(mode)
//         .migration_basename("migration init".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//
//     let resources = Resources;
//     let mock_prompter = MockPrompter::builder()
//         .allow_empty_migrations_gen(true)
//         .rename_or_delete_single_field_change(RenameOrDelete::Rename)
//         .build();
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//
//     // Generating without init should not yield any migrations
//     // 1st run
//     conf.set_file_basename("migration init".to_string())
//         .init_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//     let cli_db = conf.db().clone();
//
//     // First time, should create migration files and db records
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 1,
//         // Init command runs newly pending generated migration against the current db
//         expected_db_mig_count: 1,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 1,
//         // Init command already ran newly pending generated migration against the current db
//         expected_db_mig_count: 1,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
// }
//
// async fn generate_test_migrations(
//     temp_test_migration_dir: PathBuf,
//     mode: Mode,
//     reversible: &bool,
// ) -> (TestConfig, PathBuf) {
//     let mock_prompter = MockPrompter::builder()
//         .allow_empty_migrations_gen(true)
//         .rename_or_delete_single_field_change(RenameOrDelete::Rename)
//         .build();
//     // let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(*reversible)
//         .db_run(false)
//         .mode(mode)
//         .migration_basename("".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//
//     // #### Init Phase ####
//     // Run 1 init
//     conf.set_file_basename("migration 1-init".to_string())
//         .init_cmd()
//         .await
//         .run_fn(Resources, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 2-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(Resources, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 3-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(Resources, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 4-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV2, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 5-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV3, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 6-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV4, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 7-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV5, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 8-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV6, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 9-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV7, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 10-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV8, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 11-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV9, mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 12-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(ResourcesV10, mock_prompter.clone())
//         .await;
//     (conf, temp_test_migration_dir.clone())
// }
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn t1(mode: Mode, ref reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, reversible).await;
//     let cli_db = conf.db().clone();
//     let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//
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
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn t2(mode: Mode, ref reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, reversible).await;
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
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(5).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 6,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_6_gen_after_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(0).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 6,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_6_gen_after_init".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(1).build();
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
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(5).build();
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
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(1000).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 12,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
//         // 1 is added to force a different snapshot name from
//         // the previous assertion
//         code_origin_line: std::line!() + 1,
//         config: conf.clone(),
//     })
//     .await;
// }
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn t3(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//     let cli_db = conf.db().clone();
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(1).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(59).build();
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
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn t4(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//     let cli_db = conf.db().clone();
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(12).build();
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
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn t5_zero_delta_moves_no_needle(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//     let cli_db = conf.db().clone();
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(0).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
// }
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn t5_disallow_negative_delta(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//     let cli_db = conf.db().clone();
//
//     let ref default_fwd_strategy = FastForwardDelta::builder().number(0).build();
//     conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
//     assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 12,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
//         expected_latest_db_migration_meta_basename_normalized: "".into(),
//         code_origin_line: std::line!(),
//         config: conf.clone(),
//     })
//     .await;
// }
//
// #[test_case(Mode::Strict, true; "Reversible Strict")]
// #[test_case(Mode::Lax, true; "Reversible Lax")]
// #[test_case(Mode::Strict, false; "Non-Reversible Strict")]
// #[test_case(Mode::Lax, false; "Non-Reversible Lax")]
// #[tokio::test]
// async fn test_apply_till_migration_filename_pointer(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let (mut conf, temp_test_migration_dir) =
//         generate_test_migrations(temp_test_migration_dir.clone(), mode, &reversible).await;
//     let cli_db = conf.db().clone();
//
//     let files = read_up_fwd_migrations_from_dir_sorted(&temp_test_migration_dir);
//
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
//     let file7 = files.get(6).unwrap().to_owned();
//     let ref default_fwd_strategy = FastForwardDelta::builder().till(file7).build();
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
// }
//
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
