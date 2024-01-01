use std::{fs, path::PathBuf};

use chrono::Utc;
use clap::Subcommand;
use surreal_models::migrations::{
    Resources, ResourcesV10, ResourcesV2, ResourcesV3, ResourcesV4, ResourcesV5, ResourcesV6,
    ResourcesV7, ResourcesV8, ResourcesV9,
};
use surreal_orm::{
    migrator::{
        config::{self, DatabaseConnection, UrlDb},
        Basename, Down, FastForwardDelta, Generate, Init, Migration, MigrationFilename, Migrator,
        MockPrompter, Mode, Prompter, RenameOrDelete, RollbackDelta, RollbackStrategy, SubCommand,
        Up,
    },
    DbResources,
};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tempfile::tempdir;
use test_case::test_case;
use typed_builder::TypedBuilder;

async fn assert_with_db_instance<'a>(args: AssertionArg<'a>) {
    let AssertionArg {
        expected_down_mig_files_count,
        expected_db_mig_meta_count: expected_db_mig_count,
        expected_latest_migration_file_basename_normalized,
        expected_latest_db_migration_meta_basename_normalized,
        code_origin_line,
        config,
    } = args;

    let db = config.db().clone();
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

    let expected_down_mig_files_count = if config.reversible {
        expected_down_mig_files_count
    } else {
        // Non reversible migrations should not have down migrationser
        0
    };
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

        dbg!(&config.migration_dir);
        // dbg!(&down_migration_files);
        let found_migration_file = |db_mig_name: MigrationFilename| {
            down_migration_files
                .iter()
                .find(|filename| {
                    // dbg!(&filename);
                    &&db_mig_name.to_down() == filename
                })
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
struct AssertionArg<'a> {
    expected_down_mig_files_count: u8,
    expected_db_mig_meta_count: u8,
    expected_latest_migration_file_basename_normalized: Basename,
    expected_latest_db_migration_meta_basename_normalized: Basename,
    code_origin_line: u32,
    config: &'a TestConfig<'a>,
}

#[derive(Clone, TypedBuilder)]
struct TestConfigNew {
    migrator: Migrator,
}

impl TestConfigNew {
    pub(crate) async fn new(migration_dir: &PathBuf) -> Self {
        let db_conn_config = get_db_connection_config();

        let mut migrator = Migrator::builder()
            .verbose(3)
            .migrations_dir(migration_dir.clone())
            .db_connection(db_conn_config)
            .mode(Mode::Strict)
            .build();

        migrator.setup_db().await;

        Self::builder().migrator(migrator).build()
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

    pub(crate) async fn run_up(&mut self, fwd_delta: &FastForwardDelta) -> &mut Self {
        Up::builder()
            .fast_forward(fwd_delta.clone())
            .build()
            .run(&mut self.migrator)
            .await;
        // self.migrator.run_up_fn().await;
        self
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
                    .basename("migration 1-init".into())
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

#[derive(Clone, TypedBuilder)]
struct TestConfig<'a> {
    reversible: bool,
    db_run: bool,
    mode: Mode,
    migration_basename: Basename,
    migration_dir: &'a PathBuf,
    #[builder(default)]
    db: Option<Surreal<Any>>,
}

impl<'a> TestConfig<'a> {
    pub(crate) async fn setup_db(&mut self) -> &mut Self {
        if self.db.is_none() {
            self.db = get_db_connection_config().setup().await.db();
        }
        self
    }

    pub(crate) fn db(&self) -> Surreal<Any> {
        self.db.clone().expect("Failed to get db")
    }

    pub(crate) fn set_file_basename(&mut self, basename: impl Into<Basename>) -> &mut Self {
        self.migration_basename = basename.into();
        self
    }

    async fn setup_db_if_none(&mut self, migrator: &mut Migrator) {
        match &self.db {
            Some(db) => migrator.set_db(db.clone()),
            None => {
                migrator.setup_db().await;
                self.db = Some(migrator.db().clone());
            }
        }
    }

    // pub(crate) async fn run_down(&mut self, rollback_strategy: &RollbackDelta, prune: bool) {
    //     let TestConfig {
    //         mode,
    //         migration_dir,
    //         ..
    //     } = self;
    //     let db_conn_config = get_db_connection_config();
    //
    //     let down = Down::builder()
    //         .strategy(rollback_strategy.clone())
    //         .prune(prune)
    //         .build();
    //
    //     let mut migrator = Migrator::builder()
    //         .subcmd(SubCommand::Down(down))
    //         .verbose(3)
    //         .migrations_dir(migration_dir.clone())
    //         .db_connection(db_conn_config)
    //         .mode(*mode)
    //         .build();
    //     self.setup_db_if_none(&mut migrator).await;
    //     migrator.run_down_fn().await;
    // }
    // pub(crate) async fn run_up(&mut self, fwd_delta: &FastForwardDelta) {
    //     let TestConfig {
    //         mode,
    //         migration_dir,
    //         ..
    //     } = self;
    //     let db_conn_config = get_db_connection_config();
    //
    //     let up = Up::builder().fast_forward(fwd_delta.clone()).build();
    //
    //     let mut migrator = Migrator::builder()
    //         .subcmd(SubCommand::Up(up))
    //         .verbose(3)
    //         .migrations_dir(migration_dir.clone())
    //         .db_connection(db_conn_config)
    //         .mode(*mode)
    //         .build();
    //     self.setup_db_if_none(&mut migrator).await;
    //
    //     migrator.run_up_fn().await;
    // }
    //
    // pub(crate) async fn generator_cmd(
    //     &mut self,
    //     codebase_resources: impl DbResources,
    //     prompter: impl Prompter,
    // ) {
    //     let TestConfig {
    //         db_run,
    //         mode,
    //         migration_basename,
    //         migration_dir,
    //         ..
    //     } = self;
    //     let db_conn_config = get_db_connection_config();
    //
    //     let gen = Generate::builder()
    //         .name(migration_basename.clone())
    //         .run(*db_run)
    //         .build();
    //
    //     let mut migrator = Migrator::builder()
    //         .subcmd(SubCommand::Generate(gen))
    //         .verbose(3)
    //         .migrations_dir(migration_dir.clone())
    //         .db_connection(db_conn_config)
    //         .mode(*mode)
    //         .build();
    //     self.setup_db_if_none(&mut migrator).await;
    //     migrator.run_fn(codebase_resources, prompter).await;
    // }
    //
    // pub(crate) async fn run_init_cmd(
    //     &mut self,
    //     codebase_resources: impl DbResources,
    //     prompter: impl Prompter,
    // ) {
    //     let TestConfig {
    //         reversible,
    //         db_run,
    //         mode,
    //         migration_basename,
    //         migration_dir,
    //         ..
    //     } = self;
    //     let init_conf = Init::builder()
    //         .basename(migration_basename.clone())
    //         .reversible(*reversible)
    //         .run(*db_run)
    //         .build();
    //     let db_conn_config = get_db_connection_config();
    //
    //     let mut migrator = Migrator::builder()
    //         .subcmd(SubCommand::Init(init_conf))
    //         .verbose(3)
    //         .migrations_dir(migration_dir.clone())
    //         .db_connection(db_conn_config)
    //         .mode(*mode)
    //         .build();
    //     self.setup_db_if_none(&mut migrator).await;
    //     migrator.run_fn(codebase_resources, prompter).await;
    // }

    fn read_down_migrations_from_dir_sorted(&self) -> Vec<MigrationFilename> {
        let mut files = std::fs::read_dir(&self.migration_dir)
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
}

#[test_case(Mode::Strict; "Reversible Strict")]
#[test_case(Mode::Lax; "Reversible Lax")]
#[tokio::test]
async fn test_rollback(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    // dbg!("namamama", &temp_test_migration_dir);
    //
    let mut conf = TestConfigNew::new(&temp_test_migration_dir).await;
    conf.generate_test_migrations();

    conf.run_up(&FastForwardDelta::default()).await;
    // let cli_db = conf.db().clone();

    // First time, should create migration files and db records
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: &conf,
    })
    .await;

    conf.run_down(&RollbackDelta::default(), false).await;
    // conf.run_down(&RollbackDelta::builder().number(3).build(), false)
    //     .await;
    //
    // // First time, should create migration files and db records
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: &conf,
    })
    .await;
    // let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    // conf.run_up(default_fwd_strategy).await;
    // assert_with_db_instance(AssertionArg {
    //     db: cli_db.clone(),
    //     expected_down_mig_files_count: 1,
    //     expected_db_mig_meta_count: 1,
    //     // migration_files_dir: temp_test_migration_dir.clone(),
    //     expected_latest_migration_file_basename_normalized: "migration_init".into(),
    //     expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
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
// #[should_panic(expected = "Failed to detect migration type.")]
// #[ignore]
// async fn test_one_way_cannot_run_up_without_init(mode: Mode, reversible: bool) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(reversible)
//         .db_run(false)
//         .mode(mode)
//         .migration_basename("migration init".into())
//         .migration_dir(temp_test_migration_dir)
//         .build();
//
//     // let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//
//     // 1st fwd
//     let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
//     conf.run_up(default_fwd_strategy).await;
//     conf.run_up(default_fwd_strategy).await;
//     conf.run_up(default_fwd_strategy).await;
//     conf.run_up(default_fwd_strategy).await;
//     // This should come after the first command initializes the db
//
//     assert_with_db_instance(AssertionArg {
//         expected_down_mig_files_count: 0,
//         expected_db_mig_meta_count: 0,
//         // migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_file_basename_normalized: "".into(),
//         expected_latest_db_migration_meta_basename_normalized: "".into(),
//         code_origin_line: std::line!(),
//         config: &conf,
//     })
//     .await;
// }

// Cannot generate without init first
#[test_case(Mode::Strict; "Reversible Strict")]
#[test_case(Mode::Lax; "Reversible Lax")]
#[tokio::test]
#[ignore]
async fn test_run_up_after_init_with_no_run(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    // dbg!("namamama", &temp_test_migration_dir);
    //
    let mut conf = TestConfig::generate_test_migrations(temp_test_migration_dir, mode, &true).await;
    conf.run_up(&FastForwardDelta::default()).await;
    // let cli_db = conf.db().clone();

    // First time, should create migration files and db records
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: &conf,
    })
    .await;

    conf.run_down(&RollbackDelta::default(), false).await;
    // conf.run_down(&RollbackDelta::builder().number(3).build(), false)
    //     .await;
    //
    // // First time, should create migration files and db records
    assert_with_db_instance(AssertionArg {
        expected_down_mig_files_count: 12,
        expected_db_mig_meta_count: 12,
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: &conf,
    })
    .await;
    // let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    // conf.run_up(default_fwd_strategy).await;
    // assert_with_db_instance(AssertionArg {
    //     db: cli_db.clone(),
    //     expected_down_mig_files_count: 1,
    //     expected_db_mig_meta_count: 1,
    //     // migration_files_dir: temp_test_migration_dir.clone(),
    //     expected_latest_migration_file_basename_normalized: "migration_init".into(),
    //     expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
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
//         .run(Some(ResourcesV2), mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 5-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run(Some(ResourcesV3), mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 6-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run(Some(ResourcesV4), mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 7-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run(Some(ResourcesV5), mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 8-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run(Some(ResourcesV6), mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 9-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run(Some(ResourcesV7), mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 10-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run(Some(ResourcesV8), mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 11-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run(Some(ResourcesV9), mock_prompter.clone())
//         .await;
//
//     conf.set_file_basename("migration 12-gen after init".to_string())
//         .generator_cmd()
//         .await
//         .run(Some(ResourcesV0), mock_prompter.clone())
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
//     let files = read_down_migrations_from_dir_sorted(&temp_test_migration_dir);
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
//     let files = read_down_migrations_from_dir_sorted(&temp_test_migration_dir);
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
//     let files = read_down_migrations_from_dir_sorted(&temp_test_migration_dir);
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
//     let files = read_down_migrations_from_dir_sorted(&temp_test_migration_dir);
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
//     let files = read_down_migrations_from_dir_sorted(&temp_test_migration_dir);
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
