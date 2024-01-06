/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use migrator_tests::{assert_with_db_instance, current_function, AssertionArg, TestConfigNew};
use surreal_models::migrations::{
    invalid_cases, Animal, AnimalV2, Planet, PlanetV2, Resources, ResourcesV2,
};
use surreal_orm::{
    create_table_resources,
    migrator::{Generate, Init, MockPrompter, Mode, RenameOrDelete},
    DbResources, TableResources,
};
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
async fn test_successfully_handles_renaming(mode: Mode, reversible: bool) {
    let migration_dir = tempdir().expect("Failed to create temp directory");
    let migration_dir = &migration_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, migration_dir).await;
    #[derive(Debug, Clone)]
    pub struct ResourcesV1;
    impl DbResources for ResourcesV1 {
        create_table_resources!(Animal, Planet);
    }
    #[derive(Debug, Clone)]
    pub struct ResourcesV2;
    impl DbResources for ResourcesV2 {
        create_table_resources!(AnimalV2, PlanetV2);
    }

    conf.run_init_cmd(
        Init::builder()
            .reversible(reversible)
            .name("migration init".into())
            .run(false)
            .build(),
        ResourcesV1,
        MockPrompter::builder()
            .allow_empty_migrations_gen(true)
            .rename_or_delete_single_field_change(RenameOrDelete::Rename)
            .build(),
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
    let snapshot = conf.assert_migration_queries_snapshot(current_function!());

    let assert_forward_up_migrations_snaps_v1 = || {
        assert!(snapshot.contains("DELETE migration;"));
        assert!(snapshot.contains("DEFINE TABLE animal SCHEMAFULL PERMISSIONS NONE;"));
        assert!(snapshot
            .contains("DEFINE FIELD attributes ON animal TYPE array<string> PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD createdAt ON animal TYPE datetime PERMISSIONS FULL;")
        );
        assert!(
            snapshot.contains("DEFINE FIELD id ON animal TYPE record<animal> PERMISSIONS FULL;")
        );
        assert!(snapshot.contains("DEFINE FIELD species ON animal TYPE string PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD updatedAt ON animal TYPE datetime PERMISSIONS FULL;")
        );
        assert!(snapshot.contains("DEFINE FIELD velocity ON animal TYPE int PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE TABLE migration SCHEMAFULL PERMISSIONS NONE;"));
        assert!(snapshot
            .contains("DEFINE FIELD checksum_up ON migration TYPE string PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD name ON migration TYPE string PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD timestamp ON migration TYPE int PERMISSIONS FULL;"));

        assert!(snapshot.contains("DEFINE TABLE planet SCHEMAFULL PERMISSIONS NONE;"));
        assert!(
            snapshot.contains("DEFINE FIELD createdAt ON planet TYPE datetime PERMISSIONS FULL;")
        );

        assert!(snapshot.contains("DEFINE FIELD firstName ON planet TYPE string PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD id ON planet TYPE record<planet> PERMISSIONS FULL;")
        );
        assert!(
            snapshot.contains("DEFINE FIELD labels ON planet TYPE array<string> PERMISSIONS FULL;")
        );
        assert!(snapshot.contains("DEFINE FIELD population ON planet TYPE int PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD updatedAt ON planet TYPE datetime PERMISSIONS FULL;")
        );
    };

    if reversible {
        assert!(
            snapshot.contains("header: Basename - migration_init. Extension - down.surql"),
            "is reversible and has down"
        );
        assert!(
            !snapshot.contains("header: Basename - migration_init. Extension - surql",),
            "not one way"
        );

        assert!(snapshot.contains("REMOVE TABLE animal;"));
        assert!(snapshot.contains("REMOVE TABLE migration;"));
        assert!(snapshot.contains("REMOVE TABLE planet;"));

        assert!(snapshot.contains("header: Basename - migration_init. Extension - up.surql"));
        assert_forward_up_migrations_snaps_v1();
        assert!(snapshot
            .contains("DEFINE FIELD checksum_down ON migration TYPE string PERMISSIONS FULL;"));
    } else {
        assert!(
            snapshot.contains("header: Basename - migration_init. Extension - surql"),
            "should be one way"
        );
        assert!(
            !snapshot.contains("header: Basename - migration_init. Extension - down.surql"),
            "should not have down cos its one way"
        );
        assert!(
            !snapshot.contains("header: Basename - migration_init. Extension - up.surql"),
            "should not have up cos its one way"
        );
        assert!(!snapshot
            .contains("DEFINE FIELD checksum_down ON migration TYPE string PERMISSIONS FULL;"));
        assert_forward_up_migrations_snaps_v1();
    }

    conf.run_gen_cmd(
        Generate::builder()
            .name("migration gen 1".into())
            .run(false)
            .build(),
        ResourcesV2,
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

    // The implicit renaming strategy is set in mock prompter above
    let snapshot_v2_with_animal_explicit_planet_implicit_renaming =
        conf.assert_migration_queries_snapshot(current_function!());
    let assert_up_forward_v2_with_renaming = || {
        assert!(
            snapshot_v2_with_animal_explicit_planet_implicit_renaming.contains(
                "DEFINE FIELD characteristics ON animal TYPE array<string> PERMISSIONS FULL;\
        \nUPDATE animal SET characteristics = attributes;\
        \nREMOVE FIELD attributes ON TABLE animal;"
            ),
            "Successfully handles explicit renaming"
        );

        assert!(snapshot_v2_with_animal_explicit_planet_implicit_renaming
            .contains("REMOVE FIELD createdAt ON TABLE animal;"));
        assert!(snapshot_v2_with_animal_explicit_planet_implicit_renaming
            .contains("REMOVE FIELD updatedAt ON TABLE animal;"));

        assert!(
            snapshot_v2_with_animal_explicit_planet_implicit_renaming.contains(
                "DEFINE FIELD newName ON planet TYPE string PERMISSIONS FULL;\
        \nUPDATE planet SET newName = firstName;\
        \nREMOVE FIELD firstName ON TABLE planet;"
            ),
            "Successfully handles implicit renaming when single field changed"
        );
    };
    if reversible {
        assert!(snapshot_v2_with_animal_explicit_planet_implicit_renaming
            .contains("header: Basename - migration_gen_1. Extension - up.surql"));
        assert!(
            !snapshot_v2_with_animal_explicit_planet_implicit_renaming
                .contains("header: Basename - migration_gen_1. Extension - surql",),
            "not one way"
        );
        assert_forward_up_migrations_snaps_v1();
        assert_up_forward_v2_with_renaming();

        assert!(snapshot_v2_with_animal_explicit_planet_implicit_renaming
            .contains("header: Basename - migration_gen_1. Extension - down.surql"));

        assert!(
            snapshot_v2_with_animal_explicit_planet_implicit_renaming.contains(
                "DEFINE FIELD attributes ON animal TYPE array<string> PERMISSIONS FULL;\
        \nUPDATE animal SET attributes = characteristics;\
        \nREMOVE FIELD characteristics ON TABLE animal;"
            ),
            "Successfully handles explicit renaming reversal"
        );

        assert!(snapshot_v2_with_animal_explicit_planet_implicit_renaming
            .contains("DEFINE FIELD createdAt ON animal TYPE datetime PERMISSIONS FULL;"));

        assert!(snapshot_v2_with_animal_explicit_planet_implicit_renaming
            .contains("DEFINE FIELD updatedAt ON animal TYPE datetime PERMISSIONS FULL;"));

        assert!(
            snapshot_v2_with_animal_explicit_planet_implicit_renaming.contains(
                "DEFINE FIELD firstName ON planet TYPE string PERMISSIONS FULL;\
        \nUPDATE planet SET firstName = newName;\
        \nREMOVE FIELD newName ON TABLE planet;"
            ),
            "Successfully handles implicit renaming reversal"
        );
    } else {
        assert_forward_up_migrations_snaps_v1();
        assert_up_forward_v2_with_renaming();
        assert!(
            snapshot_v2_with_animal_explicit_planet_implicit_renaming
                .contains("header: Basename - migration_init. Extension - surql"),
            "should be one way"
        );
        assert!(
            !snapshot_v2_with_animal_explicit_planet_implicit_renaming
                .contains("header: Basename - migration_init. Extension - down.surql"),
            "should not have down cos its one way"
        );
        assert!(
            !snapshot_v2_with_animal_explicit_planet_implicit_renaming
                .contains("header: Basename - migration_init. Extension - up.surql"),
            "should not have up cos its one way"
        );
        assert_forward_up_migrations_snaps_v1();
    }

    assert!(migration_dir.exists());
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
    let snapshot = conf.assert_migration_queries_snapshot(current_function!());

    let assert_forward_up_migrations_snaps = || {
        assert!(snapshot.contains("DELETE migration;"));
        assert!(snapshot.contains("DEFINE TABLE animal SCHEMAFULL PERMISSIONS NONE;"));
        assert!(snapshot
            .contains("DEFINE FIELD attributes ON animal TYPE array<string> PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD createdAt ON animal TYPE datetime PERMISSIONS FULL;")
        );
        assert!(
            snapshot.contains("DEFINE FIELD id ON animal TYPE record<animal> PERMISSIONS FULL;")
        );
        assert!(snapshot.contains("DEFINE FIELD species ON animal TYPE string PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD updatedAt ON animal TYPE datetime PERMISSIONS FULL;")
        );
        assert!(snapshot.contains("DEFINE FIELD velocity ON animal TYPE int PERMISSIONS FULL;"));
        assert!(snapshot
            .contains("DEFINE INDEX species_speed_idx ON animal FIELDS species, velocity UNIQUE;"));
        assert!(snapshot.contains("DEFINE EVENT event1 ON animal WHEN (species = 'Homo Erectus') AND (velocity > 545) THEN (SELECT * FROM crop);"));
        assert!(snapshot.contains("DEFINE EVENT event2 ON animal WHEN (species = 'Homo Sapien') AND (velocity < 10) THEN (SELECT * FROM eats);"));

        assert!(snapshot.contains("DEFINE TABLE crop SCHEMAFULL PERMISSIONS NONE;"));
        assert!(snapshot.contains("DEFINE FIELD color ON crop TYPE string PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD id ON crop TYPE record<crop> PERMISSIONS FULL;"));

        assert!(snapshot.contains("DEFINE TABLE eats SCHEMAFULL PERMISSIONS NONE;"));
        assert!(snapshot.contains("DEFINE FIELD createdAt ON eats TYPE datetime PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD id ON eats TYPE record<eats> PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD in ON eats TYPE record<any> PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD out ON eats TYPE record<any> PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD place ON eats TYPE string PERMISSIONS FULL;"));

        assert!(snapshot.contains("DEFINE TABLE migration SCHEMAFULL PERMISSIONS NONE;"));
        assert!(snapshot
            .contains("DEFINE FIELD checksum_up ON migration TYPE string PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD name ON migration TYPE string PERMISSIONS FULL;"));
        assert!(snapshot.contains("DEFINE FIELD timestamp ON migration TYPE int PERMISSIONS FULL;"));

        assert!(snapshot.contains("DEFINE TABLE planet SCHEMAFULL PERMISSIONS NONE;"));
        assert!(
            snapshot.contains("DEFINE FIELD createdAt ON planet TYPE datetime PERMISSIONS FULL;")
        );

        assert!(snapshot.contains("DEFINE FIELD firstName ON planet TYPE string PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD id ON planet TYPE record<planet> PERMISSIONS FULL;")
        );
        assert!(
            snapshot.contains("DEFINE FIELD labels ON planet TYPE array<string> PERMISSIONS FULL;")
        );
        assert!(snapshot.contains("DEFINE FIELD population ON planet TYPE int PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD updatedAt ON planet TYPE datetime PERMISSIONS FULL;")
        );

        assert!(snapshot.contains("DEFINE TABLE student SCHEMAFULL PERMISSIONS NONE;"));
        assert!(snapshot.contains("DEFINE FIELD age ON student TYPE int PERMISSIONS FULL;"));
        assert!(
            snapshot.contains("DEFINE FIELD createdAt ON student TYPE datetime PERMISSIONS FULL;")
        );

        assert!(
            snapshot.contains("DEFINE FIELD id ON student TYPE record<student> PERMISSIONS FULL;")
        );
        assert!(
            snapshot.contains("DEFINE FIELD university ON student TYPE string PERMISSIONS FULL;")
        );
        assert!(
            snapshot.contains("DEFINE FIELD updatedAt ON student TYPE datetime PERMISSIONS FULL;")
        );
    };

    if reversible {
        assert!(
            snapshot.contains("header: Basename - migration_init. Extension - down.surql"),
            "is reversible and has down"
        );
        assert!(
            !snapshot.contains("header: Basename - migration_init. Extension - surql",),
            "not one way"
        );

        assert!(snapshot.contains("REMOVE TABLE animal;"));
        assert!(snapshot.contains("REMOVE TABLE crop;"));
        assert!(snapshot.contains("REMOVE TABLE eats;"));
        assert!(snapshot.contains("REMOVE TABLE migration;"));
        assert!(snapshot.contains("REMOVE TABLE planet;"));
        assert!(snapshot.contains("REMOVE TABLE student;"));

        assert!(snapshot.contains("header: Basename - migration_init. Extension - up.surql"));
        assert_forward_up_migrations_snaps();
        assert!(snapshot
            .contains("DEFINE FIELD checksum_down ON migration TYPE string PERMISSIONS FULL;"));
    } else {
        assert!(
            snapshot.contains("header: Basename - migration_init. Extension - surql"),
            "should be one way"
        );
        assert!(
            !snapshot.contains("header: Basename - migration_init. Extension - down.surql"),
            "should not have down cos its one way"
        );
        assert!(
            !snapshot.contains("header: Basename - migration_init. Extension - up.surql"),
            "should not have up cos its one way"
        );
        assert!(!snapshot
            .contains("DEFINE FIELD checksum_down ON migration TYPE string PERMISSIONS FULL;"));
        assert_forward_up_migrations_snaps();
    }

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
