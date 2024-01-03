use std::{fs, path::PathBuf};

use surreal_models::migrations::{
    Resources, ResourcesV10, ResourcesV2, ResourcesV3, ResourcesV4, ResourcesV5, ResourcesV6,
    ResourcesV7, ResourcesV8, ResourcesV9,
};
use surreal_orm::{
    migrator::{
        config::{DatabaseConnection, UrlDb},
        Basename, Down, FastForwardDelta, Generate, Init, Migration, MigrationFilename, Migrator,
        MockPrompter, Mode, RenameOrDelete, RollbackDelta, SubCommand, Up,
    },
    DbResources,
};
use tempfile::tempdir;
use test_case::test_case;
use typed_builder::TypedBuilder;

async fn assert_with_db_instance(args: AssertionArg) {
    let AssertionArg {
        expected_down_mig_files_count,
        expected_db_mig_meta_count: expected_db_mig_count,
        expected_latest_migration_file_basename_normalized,
        expected_latest_db_migration_meta_basename_normalized,
        code_origin_line,
        config,
    } = args;

    let db = config.migrator.db().clone();
    let db_migrations = Migration::get_all_desc(db.clone()).await;
    let latest_migration = Migration::get_latest(db.clone()).await;

    if let Some(latest_migration_name) = latest_migration {
        let name = MigrationFilename::try_from(latest_migration_name.name.clone())
            .expect("Failed to parse file name");
        assert_eq!(
            name.basename(),
            expected_latest_db_migration_meta_basename_normalized,
            "Base name in file does not match the base name in the db. Line: {code_origin_line}",
        );
    }

    let down_migration_files = config.read_down_migrations_from_dir_sorted();
    let down_migration_files_content = config.read_down_migrations_content_from_dir_sorted();

    let latest_file_name = down_migration_files.iter().max();

    if let Some(latest_file_name) = latest_file_name {
        assert_eq!(
            latest_file_name.basename(),
            expected_latest_migration_file_basename_normalized,
            "Base name in file does not match the base name in the db. Line: {code_origin_line}"
        );
    }

    assert_eq!(
        db_migrations.len() as u8,
        expected_db_mig_count,
        "migration Counts do not match with what is in the db. Line: {code_origin_line}",
    );

    assert_eq!(
        down_migration_files.len() as u8,
        expected_down_mig_files_count,
        "File counts do not match. Line: {code_origin_line}"
    );

    for db_mig_record in db_migrations {
        let file_name = db_mig_record.clone().name;
        let mig_name_from_db =
            MigrationFilename::try_from(file_name.to_string()).expect("Failed to parse file name");
        let timestamp = mig_name_from_db.timestamp();
        let basename = mig_name_from_db.basename();
        let extension = mig_name_from_db.extension();

        let found_migration_file = |db_mig_name: MigrationFilename| {
            down_migration_files
                .iter()
                .find(|filename| &&db_mig_name.to_down() == filename)
                .expect("Db Migration not found amongst the files in migration directory.")
        };

        let found_mig_file = found_migration_file(mig_name_from_db.clone());
        assert_eq!(found_mig_file.to_up(), mig_name_from_db);
        assert_eq!(mig_name_from_db.extension().to_string(), "up.surql");
        assert_eq!(found_mig_file.extension().to_string(), "down.surql");
        assert_eq!(found_mig_file.to_up().to_string(), db_mig_record.name);
        assert_eq!(found_mig_file.timestamp(), db_mig_record.timestamp);

        assert_eq!(
            mig_name_from_db.to_string(),
            format!("{timestamp}_{basename}.{extension}"),
            "File name should be in the format of {timestamp}_{basename}.{extension}"
        );
    }
}

fn get_db_connection_config() -> DatabaseConnection {
    DatabaseConnection::builder()
        .db("test".into())
        .ns("test".into())
        .user("root".into())
        .pass("root".into())
        .url(UrlDb::Memory)
        .build()
}

#[derive(Clone, TypedBuilder)]
struct AssertionArg {
    expected_down_mig_files_count: u8,
    expected_db_mig_meta_count: u8,
    expected_latest_migration_file_basename_normalized: Basename,
    expected_latest_db_migration_meta_basename_normalized: Basename,
    code_origin_line: u32,
    config: TestConfigNew,
}

#[derive(Clone, TypedBuilder)]
struct TestConfigNew {
    migrator: Migrator,
}

impl TestConfigNew {
    pub(crate) async fn new(mode: Mode, migration_dir: &PathBuf) -> Self {
        let db_conn_config = get_db_connection_config();

        let mut migrator = Migrator::builder()
            .verbose(3)
            .migrations_dir(migration_dir.clone())
            .db_connection(db_conn_config)
            .mode(mode)
            .build();

        migrator.setup_db().await;

        Self::builder().migrator(migrator).build()
    }

    fn read_down_migrations_content_from_dir_sorted(&self) -> Vec<String> {
        self.read_down_migrations_from_dir_sorted()
            .iter()
            .map(|f| {
                fs::read_to_string(
                    &self
                        .migrator
                        .file_manager()
                        .get_migration_dir()
                        .unwrap()
                        .join(f.to_string()),
                )
                .expect("Failed to read file")
            })
            .collect::<Vec<String>>()
    }

    fn read_down_migrations_from_dir_sorted(&self) -> Vec<MigrationFilename> {
        let mut files =
            std::fs::read_dir(&self.migrator.file_manager().get_migration_dir().unwrap())
                .expect("Failed to read dir")
                .filter_map(|p| {
                    p.expect("Failed to read dir")
                        .file_name()
                        .to_string_lossy()
                        .to_string()
                        .try_into()
                        .ok()
                })
                .filter(|f: &MigrationFilename| f.is_down())
                .collect::<Vec<MigrationFilename>>();

        files.sort_by_key(|a| a.timestamp());
        files
    }

    pub(crate) async fn run(
        &mut self,
        codebase_resources: Option<impl DbResources>,
        prompter: MockPrompter,
    ) -> &mut Self {
        self.migrator.run_test(codebase_resources, prompter).await;
        self
    }

    pub(crate) fn set_cmd(&mut self, cmd: SubCommand) -> &mut Self {
        self.migrator.set_cmd(cmd);
        self
    }

    pub(crate) async fn run_down(
        &mut self,
        rollback_strategy: &RollbackDelta,
        prune: bool,
    ) -> &mut Self {
        self.set_cmd(SubCommand::Down(
            Down::builder()
                .strategy(rollback_strategy.clone())
                .prune(prune)
                .build(),
        ))
        .run(
            Some(ResourcesV10),
            MockPrompter::builder()
                .allow_empty_migrations_gen(true)
                .rename_or_delete_single_field_change(RenameOrDelete::Rename)
                .build(),
        )
        .await
    }
    pub(crate) async fn run_up(&mut self, fwd_delta: &FastForwardDelta) -> &mut Self {
        // Up::builder()
        //     .fast_forward(fwd_delta.clone())
        //     .build()
        //     .run(&mut self.migrator)
        //     .await;
        // self.migrator.run_up_fn().await;
        self.set_cmd(SubCommand::Up(
            Up::builder().fast_forward(fwd_delta.clone()).build(),
        ))
        .run(
            Some(ResourcesV10),
            MockPrompter::builder()
                .allow_empty_migrations_gen(true)
                .rename_or_delete_single_field_change(RenameOrDelete::Rename)
                .build(),
        )
        .await
    }

    // async fn run_init_cmd(
    //     &mut self,
    //     codebase_resources: impl DbResources,
    //     prompter: impl Prompter,
    // ) {
    //     let TestConfigNew { migrator } = self;
    //     let init_conf = Init::builder()
    //         .basename("migration init".into())
    //         .reversible(true)
    //         .run(false)
    //         .build()
    //         .run(&mut self.migrator, codebase_resources, prompter);
    // }

    async fn generate_test_migrations(&mut self) -> &mut Self {
        let mock_prompter = MockPrompter::builder()
            .allow_empty_migrations_gen(true)
            .rename_or_delete_single_field_change(RenameOrDelete::Rename)
            .build();

        self.migrator
            .set_cmd(SubCommand::Init(
                Init::builder()
                    .reversible(true)
                    .name("migration 1-init".into())
                    .run(false)
                    .build(),
            ))
            .run_fn(Resources, mock_prompter.clone())
            .await;

        let gen = |basename: &'static str| {
            SubCommand::Generate(Generate::builder().name(basename.into()).run(false).build())
        };

        // Generate::builder()
        //     .name("er".into())
        //     .run(false)
        //     .build()
        //     .run(&mut self.migrator, Resources, mock_prompter.clone());
        self.set_cmd(gen("migration 2-gen after init"))
            .run(Some(Resources), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 3-gen after init"))
            .run(Some(Resources), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 4-gen after init"))
            .run(Some(ResourcesV2), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 5-gen after init"))
            .run(Some(ResourcesV3), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 6-gen after init"))
            .run(Some(ResourcesV4), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 7-gen after init"))
            .run(Some(ResourcesV5), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 8-gen after init"))
            .run(Some(ResourcesV6), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 9-gen after init"))
            .run(Some(ResourcesV7), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 10-gen after init"))
            .run(Some(ResourcesV8), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 11-gen after init"))
            .run(Some(ResourcesV9), mock_prompter.clone())
            .await;

        self.set_cmd(gen("migration 12-gen after init"))
            .run(Some(ResourcesV10), mock_prompter.clone())
            .await;

        self
    }
}

#[test_case(Mode::Strict; "Reversible Strict")]
#[test_case(Mode::Lax; "Reversible Lax")]
#[tokio::test]
async fn test_rollback(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfigNew::new(mode, &temp_test_migration_dir).await;
    conf.generate_test_migrations().await;

    // First apply all generated migrations to the current db instance
    conf.run_up(&FastForwardDelta::default()).await;

    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    // Down::builder()
    //     .strategy(RollbackDelta::default())
    //     .prune(false)
    //     .build()
    //     .run(&mut conf.migrator)
    //     .await;
    conf.run_down(&RollbackDelta::default(), false).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 11,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_11_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), false).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 10,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_10_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), false).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 9,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_9_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    conf.run_up(default_fwd_strategy).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    // Prune this time around
    conf.run_down(&RollbackDelta::default(), true).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 11,
        expected_db_mig_meta_count: 11,
        expected_latest_migration_file_basename_normalized: "migration_11_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_11_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), false).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 11,
        expected_db_mig_meta_count: 10,
        expected_latest_migration_file_basename_normalized: "migration_11_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_10_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), false).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 11,
        expected_db_mig_meta_count: 9,
        expected_latest_migration_file_basename_normalized: "migration_11_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_9_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), true).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 8,
        expected_db_mig_meta_count: 8,
        expected_latest_migration_file_basename_normalized: "migration_8_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_8_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    for i in 0..5 {
        conf.run_down(&RollbackDelta::default(), false).await;
        assert_with_db_instance(AssertionArg {
            expected_down_mig_files_count: 8,
            expected_db_mig_meta_count: 7 - i,
            expected_latest_migration_file_basename_normalized: "migration_8_gen_after_init".into(),
            expected_latest_db_migration_meta_basename_normalized: format!(
                "migration_{}{}",
                7 - i,
                "_gen_after_init".to_string()
            )
            .into(),
            code_origin_line: std::line!(),
            config: conf.clone(),
        })
        .await;
    }

    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 8,
        expected_db_mig_meta_count: 3,
        expected_latest_migration_file_basename_normalized: "migration_8_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_3_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), true).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 2,
        expected_db_mig_meta_count: 2,
        expected_latest_migration_file_basename_normalized: "migration_2_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_2_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), true).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 1,
        expected_db_mig_meta_count: 1,
        expected_latest_migration_file_basename_normalized: "migration_1_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_1_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), false).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 1,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: "migration_1_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_1_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    conf.run_down(&RollbackDelta::default(), true).await;
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 0,
        expected_db_mig_meta_count: 0,
        expected_latest_migration_file_basename_normalized: "".into(),
        expected_latest_db_migration_meta_basename_normalized: "".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
}
