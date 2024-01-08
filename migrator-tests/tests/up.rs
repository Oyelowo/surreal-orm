use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use surreal_models::migrations::{
    Resources, ResourcesV10, ResourcesV2, ResourcesV3, ResourcesV4, ResourcesV5, ResourcesV6,
    ResourcesV7, ResourcesV8, ResourcesV9,
};
use surreal_orm::{
    migrator::{
        config::{DatabaseConnection, UrlDb},
        Basename, FastForwardDelta, FileContent, Generate, Init, Migration, MigrationFilename,
        Migrator, MockPrompter, Mode, RenameOrDelete, SubCommand, Up,
    },
    DbResources,
};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tempfile::tempdir;
use test_case::test_case;
use typed_builder::TypedBuilder;

macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}

fn read_migs_from_dir(path: PathBuf) -> Vec<DirEntry> {
    std::fs::read_dir(path)
        .expect("Failed to read dir")
        .map(|p| p.expect("Failed to read dir2"))
        .collect::<Vec<_>>()
}
// #[should_panic]
// let result = std::panic::catch_unwind(|| {});
//    assert!(result.is_err());
//
//
//use std::panic;
//
// fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
//     let prev_hook = panic::take_hook();
//     panic::set_hook(Box::new(|_| {}));
//     let result = panic::catch_unwind(f);
//     panic::set_hook(prev_hook);
//     result
// }
//
//  #[should_panic(expected = "Divide result is zero")]

fn assert_migration_files_presence_and_format(
    migration_files: &Vec<DirEntry>,
    db_migrations: &Vec<Migration>,
    snapshot_disambiguator: String,
) {
    let mut migration_files = migration_files.iter().map(|f| f.path()).collect::<Vec<_>>();
    migration_files.sort_by(|a, b| {
        a.file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to get convert file name to string")
            .cmp(
                b.file_name()
                    .expect("Failed to get file name")
                    .to_str()
                    .expect("Failed to get convert file name to string"),
            )
    });

    for db_mig_record in db_migrations {
        let file_name = db_mig_record.clone().name;
        let file_name =
            MigrationFilename::try_from(file_name.to_string()).expect("Failed to parse file name");
        let timestamp = file_name.timestamp();
        let basename = file_name.basename();
        let extension = file_name.extension();

        let found_migration_file = |db_mig_name: MigrationFilename| {
            migration_files
                .iter()
                .map(|f| {
                    MigrationFilename::try_from(
                        f.file_name()
                            .expect("Failed to get file name")
                            .to_str()
                            .expect("Failed to get convert file name to string")
                            .to_string(),
                    )
                    .expect("Failed to parse file name")
                })
                .find(|filename| db_mig_name == filename.to_owned())
                .expect("Migration file not found in db")
        };

        // we want to test that the migration file metadata is stored in the db
        // e.g:  the name, timestamp and perhaps checksum?
        // ts_basename.up.surql
        // ts_basename.down.surql
        // ts_basename.sql
        match &file_name {
            MigrationFilename::Up(_) | MigrationFilename::Down(_) => {
                // select * from migration where name = up;
                // name, timestamp and checksum_up
                // We only store the up migration filename in the db
                // since we can always derive the down name from it.
                let found_mig_file = found_migration_file(file_name.clone());
                assert_eq!(file_name.extension().to_string(), "up.surql");
                assert_eq!(found_mig_file.to_string(), db_mig_record.name);
                assert_eq!(found_mig_file.timestamp(), db_mig_record.timestamp);

                // select * from migration where name = down.to_down();
                // name, timestamp and checksum_up
                let down_counterpart = found_migration_file(file_name.to_down());
                assert_eq!(down_counterpart.extension().to_string(), "down.surql");
                assert_eq!(down_counterpart.to_string(), db_mig_record.name);
                assert_eq!(down_counterpart.timestamp(), db_mig_record.timestamp);
            }
            MigrationFilename::Unidirectional(_uni) => {
                // select * from migration where name = down;
                // name, timestamp and checksum_up
                let found_mig_file = found_migration_file(file_name.clone());
                assert_eq!(file_name.extension().to_string(), "surql");
                assert_eq!(found_mig_file.to_string(), db_mig_record.name);
                assert_eq!(found_mig_file.timestamp(), db_mig_record.timestamp);
            }
        };

        assert_eq!(
            file_name.to_string(),
            format!("{timestamp}_{basename}.{extension}"),
            "File name should be in the format of {timestamp}_{basename}.{extension}"
        );
    }

    let mut migrations_contents = migration_files
        .iter()
        .enumerate()
        .map(|(i, filepath)| {
            let filename = MigrationFilename::try_from(
                filepath
                    .file_name()
                    .expect("Failed to get file name")
                    .to_str()
                    .expect("Failed to get convert file name to string")
                    .to_string(),
            )
            .expect("Failed to parse file name");

            let basename = filename.basename();
            let extension = filename.extension();
            // let timestamp = filename.timestamp();
            let file_content = fs::read_to_string(&filepath).expect("Failed to read file");
            format!("{basename}.{extension}\n{file_content}\n\n",)
        })
        .collect::<Vec<_>>();

    migrations_contents.sort();
    let migrations_contents: FileContent = migrations_contents.join("\n\n").into();
    insta::assert_display_snapshot!(
        format!("content: {}", snapshot_disambiguator.clone()),
        migrations_contents
    );
    insta::assert_display_snapshot!(
        format!("Checksum: {}", snapshot_disambiguator),
        migrations_contents.as_checksum().unwrap()
    );
}

async fn assert_with_db_instance(args: AssertionArg) {
    let AssertionArg {
        db,
        expected_mig_files_count: mig_files_count,
        expected_db_mig_count: db_mig_count,
        migration_files_dir,
        expected_latest_migration_file_basename_normalized,
        expected_latest_db_migration_meta_basename_normalized,
        code_origin_line,
        config,
    } = args;

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

    let migration_files = read_migs_from_dir(migration_files_dir.clone());
    let latest_file_name = migration_files
        .iter()
        .map(|f| {
            MigrationFilename::try_from(f.file_name().to_string_lossy().to_string())
                .expect("Failed to parse file name")
        })
        .max();

    if let Some(latest_file_name) = latest_file_name {
        assert_eq!(
            latest_file_name.basename(),
            expected_latest_migration_file_basename_normalized,
            "Base name in file does not match the base name in the db. Line: {code_origin_line}"
        );
    }

    assert_eq!(
        db_migrations.len() as u8,
        db_mig_count,
        "Line: {code_origin_line}",
    );
    assert_eq!(
        migration_files.len() as u8,
        mig_files_count,
        "Line:
            {code_origin_line}"
    );

    let snapshot_disambiguator = format!(
        "mig_files_count: {}, db_mig_count: {}, latest_migration_file_basename_normalized: {}, latest_db_migration_meta_basename_normalized: {}\n\
         reversible: {}, mode: {}. Line: {}",
        mig_files_count,
        db_mig_count,
        expected_latest_migration_file_basename_normalized,
        expected_latest_db_migration_meta_basename_normalized,
          config.reversible,
        config.mode,
        code_origin_line,
    );

    assert_migration_files_presence_and_format(
        &migration_files,
        &db_migrations,
        snapshot_disambiguator,
    );
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
    db: Surreal<Any>,
    expected_mig_files_count: u8,
    expected_db_mig_count: u8,
    migration_files_dir: PathBuf,
    expected_latest_migration_file_basename_normalized: Basename,
    expected_latest_db_migration_meta_basename_normalized: Basename,
    code_origin_line: u32,
    config: TestConfig,
}

#[derive(Clone, TypedBuilder)]
struct TestConfig {
    reversible: bool,
    db_run: bool,
    mode: Mode,
    migration_basename: Basename,
    migration_dir: PathBuf,
    #[builder(default)]
    db: Option<Surreal<Any>>,
}

impl TestConfig {
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

    pub(crate) async fn up_cmd(&mut self, fwd_delta: &FastForwardDelta) -> Migrator {
        let TestConfig {
            mode,
            migration_dir,
            ..
        } = self;
        let db_conn_config = get_db_connection_config();

        let up = Up::builder().fast_forward(fwd_delta.clone()).build();

        let mut migrator = Migrator::builder()
            .subcmd(SubCommand::Up(up))
            .verbose(3)
            .migrations_dir(migration_dir.clone())
            .db_connection(db_conn_config)
            .mode(*mode)
            .build();
        self.setup_db_if_none(&mut migrator).await;
        migrator
    }

    pub(crate) async fn generator_cmd(&mut self) -> Migrator {
        let TestConfig {
            db_run,
            mode,
            migration_basename,
            migration_dir,
            ..
        } = self;
        let db_conn_config = get_db_connection_config();

        let gen = Generate::builder()
            .name(migration_basename.clone())
            .run(*db_run)
            .build();

        let mut migrator = Migrator::builder()
            .subcmd(SubCommand::Generate(gen))
            .verbose(3)
            .migrations_dir(migration_dir.clone())
            .db_connection(db_conn_config)
            .mode(*mode)
            .build();
        self.setup_db_if_none(&mut migrator).await;
        migrator
    }

    pub(crate) async fn init_cmd(&mut self) -> Migrator {
        let TestConfig {
            reversible,
            db_run,
            mode,
            migration_basename,
            migration_dir,
            ..
        } = self;
        let init_conf = Init::builder()
            .basename(migration_basename.clone())
            .reversible(*reversible)
            .run(*db_run)
            .build();
        let db_conn_config = get_db_connection_config();

        let mut migrator = Migrator::builder()
            .subcmd(SubCommand::Init(init_conf))
            .verbose(3)
            .migrations_dir(migration_dir.clone())
            .db_connection(db_conn_config)
            .mode(*mode)
            .build();
        self.setup_db_if_none(&mut migrator).await;
        migrator
    }

    pub fn snapshot_name_str(self) -> String {
        let n = format!(
            "migration basename: {}, reversible: {}, db_run: {}, mode: {}",
            self.migration_basename, self.reversible, self.db_run, self.mode
        );
        n
    }
}

#[test_case(Mode::Strict; "Strict")]
#[test_case(Mode::Lax; "Lax")]
#[tokio::test]
#[should_panic(expected = "Failed to detect migration type.")]
async fn test_one_way_cannot_run_up_without_init(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfig::builder()
        .reversible(false)
        .db_run(false)
        .mode(mode)
        .migration_basename("migration init".into())
        .migration_dir(temp_test_migration_dir.clone())
        .build();

    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");

    // 1st fwd
    let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    // This should come after the first command initializes the db
    let cli_db = conf.db().clone();

    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 0,
        expected_db_mig_count: 0,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "".into(),
        expected_latest_db_migration_meta_basename_normalized: "".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
}

// Cannot generate without init first
#[test_case(Mode::Strict; "Strict")]
#[test_case(Mode::Lax; "Lax")]
#[tokio::test]
async fn test_run_up_after_init_with_no_run(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfig::builder()
        .reversible(false)
        .db_run(false)
        .mode(mode)
        .migration_basename("migration init".into())
        .migration_dir(temp_test_migration_dir.clone())
        .build();

    let resources = Resources;
    let mock_prompter = MockPrompter::builder()
        .allow_empty_migrations_gen(true)
        .rename_or_delete_single_field_change(RenameOrDelete::Rename)
        .build();
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");

    // Generating without init should not yield any migrations
    // 1st run
    conf.set_file_basename("migration init".to_string())
        .init_cmd()
        .await
        .run_fn(resources.clone(), mock_prompter.clone())
        .await;
    let cli_db = conf.db().clone();

    // First time, should create migration files and db records
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 1,
        expected_db_mig_count: 0,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 1,
        expected_db_mig_count: 1,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
}

#[test_case(Mode::Strict; "Strict")]
#[test_case(Mode::Lax; "Lax")]
#[tokio::test]
async fn test_run_up_after_init_with_run(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfig::builder()
        .reversible(false)
        .db_run(true)
        .mode(mode)
        .migration_basename("migration init".into())
        .migration_dir(temp_test_migration_dir.clone())
        .build();

    let resources = Resources;
    let mock_prompter = MockPrompter::builder()
        .allow_empty_migrations_gen(true)
        .rename_or_delete_single_field_change(RenameOrDelete::Rename)
        .build();
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");

    // Generating without init should not yield any migrations
    // 1st run
    conf.set_file_basename("migration init".to_string())
        .init_cmd()
        .await
        .run_fn(resources.clone(), mock_prompter.clone())
        .await;
    let cli_db = conf.db().clone();

    // First time, should create migration files and db records
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 1,
        // Init command runs newly pending generated migration against the current db
        expected_db_mig_count: 1,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 1,
        // Init command already ran newly pending generated migration against the current db
        expected_db_mig_count: 1,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;
}

async fn generate_test_migrations(
    temp_test_migration_dir: PathBuf,
    mode: Mode,
) -> (TestConfig, PathBuf) {
    let mock_prompter = MockPrompter::builder()
        .allow_empty_migrations_gen(true)
        .rename_or_delete_single_field_change(RenameOrDelete::Rename)
        .build();
    // let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let mut conf = TestConfig::builder()
        .reversible(false)
        .db_run(false)
        .mode(mode)
        .migration_basename("".into())
        .migration_dir(temp_test_migration_dir.clone())
        .build();

    // #### Init Phase ####
    // Run 1 init
    conf.set_file_basename("migration 1-init".to_string())
        .init_cmd()
        .await
        .run_fn(Resources, mock_prompter.clone())
        .await;
    let cli_db = conf.db().clone();

    conf.set_file_basename("migration 2-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(Resources, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 3-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(Resources, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 4-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV2, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 5-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV3, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 6-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV4, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 7-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV5, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 8-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV6, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 9-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV7, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 10-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV8, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 11-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV9, mock_prompter.clone())
        .await;

    conf.set_file_basename("migration 12-gen after init".to_string())
        .generator_cmd()
        .await
        .run_fn(ResourcesV10, mock_prompter.clone())
        .await;
    (conf, temp_test_migration_dir.clone())
}

#[test_case(Mode::Strict; "Strict")]
#[test_case(Mode::Lax; "Lax")]
#[tokio::test]
async fn t1(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let (mut conf, temp_test_migration_dir) =
        generate_test_migrations(temp_test_migration_dir.clone(), mode).await;
    let cli_db = conf.db().clone();
    let ref default_fwd_strategy = FastForwardDelta::builder().latest(true).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 12,
        expected_db_mig_count: 12,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    // init cmd should instantiate the database connection which is reused internally in test
    // config.
}

#[test_case(Mode::Strict; "Strict")]
#[test_case(Mode::Lax; "Lax")]
#[tokio::test]
async fn t2(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let (mut conf, temp_test_migration_dir) =
        generate_test_migrations(temp_test_migration_dir.clone(), mode).await;
    let cli_db = conf.db().clone();

    let ref default_fwd_strategy = FastForwardDelta::builder().number(1).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 12,
        expected_db_mig_count: 1,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_1_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    let ref default_fwd_strategy = FastForwardDelta::builder().number(5).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 12,
        expected_db_mig_count: 6,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_6_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    let ref default_fwd_strategy = FastForwardDelta::builder().number(1).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 12,
        expected_db_mig_count: 7,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_7_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    let ref default_fwd_strategy = FastForwardDelta::builder().number(5).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 12,
        expected_db_mig_count: 12,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        code_origin_line: std::line!(),
        config: conf.clone(),
    })
    .await;

    let ref default_fwd_strategy = FastForwardDelta::builder().number(1000).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 12,
        expected_db_mig_count: 12,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        // 1 is added to force a different snapshot name from
        // the previous assertion
        code_origin_line: std::line!() + 1,
        config: conf.clone(),
    })
    .await;
}

#[test_case(Mode::Strict; "Strict")]
#[test_case(Mode::Lax; "Lax")]
#[tokio::test]
async fn t3(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let (mut conf, temp_test_migration_dir) =
        generate_test_migrations(temp_test_migration_dir.clone(), mode).await;
    let cli_db = conf.db().clone();

    let ref default_fwd_strategy = FastForwardDelta::builder().number(1).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    let ref default_fwd_strategy = FastForwardDelta::builder().number(59).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 12,
        expected_db_mig_count: 12,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        // 1 is added to force a different snapshot name from
        // the previous assertion
        code_origin_line: std::line!() + 1,
        config: conf.clone(),
    })
    .await;
}

#[test_case(Mode::Strict; "Strict")]
#[test_case(Mode::Lax; "Lax")]
#[tokio::test]
async fn t4(mode: Mode) {
    let mig_dir = tempdir().expect("Failed to create temp directory");
    let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
    let (mut conf, temp_test_migration_dir) =
        generate_test_migrations(temp_test_migration_dir.clone(), mode).await;
    let cli_db = conf.db().clone();

    let ref default_fwd_strategy = FastForwardDelta::builder().number(1).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    let ref default_fwd_strategy = FastForwardDelta::builder().number(12).build();
    conf.up_cmd(default_fwd_strategy).await.run_up_fn().await;
    assert_with_db_instance(AssertionArg {
        db: cli_db.clone(),
        expected_mig_files_count: 12,
        expected_db_mig_count: 12,
        migration_files_dir: temp_test_migration_dir.clone(),
        expected_latest_migration_file_basename_normalized: "migration_12_gen_after_init".into(),
        expected_latest_db_migration_meta_basename_normalized: "migration_12_gen_after_init".into(),
        // 1 is added to force a different snapshot name from
        // the previous assertion
        code_origin_line: std::line!() + 1,
        config: conf.clone(),
    })
    .await;
}
// #[test_case(Mode::Strict; "Strict")]
// #[test_case(Mode::Lax; "Lax")]
// #[tokio::test]
// async fn test_one_way_can_generate_after_first_initializing_with_run(mode: Mode) {
//     let resources = Resources;
//     let resources_v2 = ResourcesV2;
//     let resources_v3 = ResourcesV3;
//     let mock_prompter = MockPrompter::builder()
//         .allow_empty_migrations_gen(true)
//         .rename_or_delete_single_field_change(RenameOrDelete::Rename)
//         .build();
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(false)
//         .db_run(true)
//         .mode(mode)
//         .migration_basename("migration init".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//
//     // #### Init Phase ####
//     // Run 1 init
//     conf.set_file_basename("migration init".to_string())
//         .init_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // init cmd should instantiate the database connection which is reused internally in test
//     // config.
//     let cli_db = conf.db().clone();
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 1,
//         expected_db_mig_count: 1,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         // name is normalized to snake case
//         expected_latest_migration_basename_normalized: "migration_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // #### Generate Phase ####
//     // Run 2: Generate  1st
//
//     // Set the db connection from the init migrator so that
//     // the generator can use the same db connection.
//     // generator.set_db_connection_from_migrator(&init);
//
//     if !joined_migration_files.is_empty() {
//         conf.set_file_basename("migration gen 1 after init".to_string())
//             .generator_cmd()
//             .await
//             .run_fn(resources_v2.clone(), mock_prompter.clone())
//             .await;
//
//         // First time, should create migration files and db records
//         let joined_migration_files = assert_with_db_instance(AssertionArg {
//             db: cli_db.clone(),
//             expected_mig_files_count: 2,
//             expected_db_mig_count: 2,
//             migration_files_dir: temp_test_migration_dir.clone(),
//             expected_latest_migration_basename_normalized: "migration_gen_1_after_init".into(),
//             code_origin_line: std::line!(),
//         })
//         .await;
//         insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//     }
//
//     // Run 3 generate
//     conf.set_file_basename("migration gen 2 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // Run 3 generate
//
//     // Second time, should not create migration files nor db records. i.e should be idempotent/
//     // Remain the same as the first time.
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 3,
//         expected_db_mig_count: 3,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration_gen_2_after_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
//     conf.set_file_basename("migration gen 3 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v3, mock_prompter)
//         .await;
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 4,
//         expected_db_mig_count: 4,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration_gen_3_after_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
// }
//
// // // Two way/Bidirectional(Up and down) migrations
// #[test_case(Mode::Strict; "Strict")]
// #[test_case(Mode::Lax; "Lax")]
// #[tokio::test]
// async fn test_two_way_cannot_generate_without_init_no_run(mode: Mode) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(true)
//         .db_run(false)
//         .mode(mode)
//         .migration_basename("migration init".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//
//     let resources = Resources;
//     let resources_v2 = ResourcesV2;
//     let mock_prompter = MockPrompter::builder()
//         .allow_empty_migrations_gen(true)
//         .rename_or_delete_single_field_change(RenameOrDelete::Rename)
//         .build();
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//
//     // 1st run
//     conf.set_file_basename("migration gen without init 1".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//     let cli_db = conf.db().clone();
//
//     // First time, should create migration files and db records
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 0,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration 1".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Initialize the 2nd time with same codebase resources. Should not allow creation the second time.
//     conf.set_file_basename("migration gen without init 2".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // Second time, should not create migration files nor db records. i.e should be idempotent/
//     // Remain the same as the first time.
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 0,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration 1".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
//     conf.set_file_basename("migration gen without init 2".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v2, mock_prompter)
//         .await;
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 0,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration 1".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
// }
//
// #[test_case(Mode::Strict; "Strict")]
// #[test_case(Mode::Lax; "Lax")]
// #[tokio::test]
// async fn test_two_way_cannot_generate_without_init_with_db_run(mode: Mode) {
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(true)
//         .db_run(true)
//         .mode(mode)
//         .migration_basename("migration init".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//
//     let resources = Resources;
//     let resources_v2 = ResourcesV2;
//     let mock_prompter = MockPrompter::builder()
//         .allow_empty_migrations_gen(true)
//         .rename_or_delete_single_field_change(RenameOrDelete::Rename)
//         .build();
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//
//     // Generating without init should not yield any migrations
//     // 1st run
//     conf.set_file_basename("migration gen without init 1".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//     let cli_db = conf.db().clone();
//
//     // First time, should create migration files and db records
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 0,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration 1".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Initialize the 2nd time with same codebase resources. Should not allow creation the second time.
//     conf.set_file_basename("migration gen without init 2".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // Second time, should not create migration files nor db records. i.e should be idempotent/
//     // Remain the same as the first time.
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 0,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration 1".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
//     conf.set_file_basename("migration gen without init 2".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v2, mock_prompter)
//         .await;
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 0,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration 1".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
// }
//
// #[test_case(Mode::Strict; "Strict")]
// #[test_case(Mode::Lax; "Lax")]
// #[tokio::test]
// async fn test_two_way_can_generate_after_first_initializing_no_run(mode: Mode) {
//     let resources = Resources;
//     let resources_v2 = ResourcesV2;
//     let resources_v3 = ResourcesV3;
//     let mock_prompter = MockPrompter::builder()
//         .allow_empty_migrations_gen(true)
//         .rename_or_delete_single_field_change(RenameOrDelete::Rename)
//         .build();
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(true)
//         .db_run(false)
//         .mode(mode)
//         .migration_basename("migration init".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//
//     // #### Init Phase ####
//     // Run 1 init
//     conf.set_file_basename("migration init".to_string())
//         .init_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // init cmd should instantiate the database connection which is reused internally in test
//     // config.
//     let cli_db = conf.db().clone();
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 2,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         // name is normalized to snake case
//         expected_latest_migration_basename_normalized: "migration_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // #### Generate Phase ####
//     // Run 2: Generate
//     conf.set_file_basename("migration gen 1 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v2.clone(), mock_prompter.clone())
//         .await;
//
//     // First time, should create migration files and db records
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 4,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration_gen_1_after_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Run 3 generate
//     conf.set_file_basename("migration gen 2 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // Run 3 generate
//
//     // Second time, should not create migration files nor db records. i.e should be idempotent/
//     // Remain the same as the first time.
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 6,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration_gen_2_after_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
//     conf.set_file_basename("migration gen 3 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v3, mock_prompter)
//         .await;
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 8,
//         expected_db_mig_count: 0,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration_gen_3_after_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
// }
//
// #[test_case(Mode::Strict; "Strict")]
// #[test_case(Mode::Lax; "Lax")]
// #[tokio::test]
// async fn test_two_way_can_generate_after_first_initializing_with_run(mode: Mode) {
//     let resources = Resources;
//     let resources_v2 = ResourcesV2;
//     let resources_v3 = ResourcesV3;
//     let mock_prompter = MockPrompter::builder()
//         .allow_empty_migrations_gen(true)
//         .rename_or_delete_single_field_change(RenameOrDelete::Rename)
//         .build();
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(true)
//         .db_run(true)
//         .mode(mode)
//         .migration_basename("migration init".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//
//     // #### Init Phase ####
//     // Run 1 init
//     conf.set_file_basename("migration init".to_string())
//         .init_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // init cmd should instantiate the database connection which is reused internally in test
//     // config.
//     let cli_db = conf.db().clone();
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 2,
//         expected_db_mig_count: 1,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         // name is normalized to snake case
//         expected_latest_migration_basename_normalized: "migration_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // #### Generate Phase ####
//     // Run 2: Generate  1st
//
//     // Set the db connection from the init migrator so that
//     // the generator can use the same db connection.
//     // generator.set_db_connection_from_migrator(&init);
//
//     if !joined_migration_files.is_empty() {
//         conf.set_file_basename("migration gen 1 after init".to_string())
//             .generator_cmd()
//             .await
//             .run_fn(resources_v2.clone(), mock_prompter.clone())
//             .await;
//
//         // First time, should create migration files and db records
//         let joined_migration_files = assert_with_db_instance(AssertionArg {
//             db: cli_db.clone(),
//             expected_mig_files_count: 4,
//             expected_db_mig_count: 2,
//             migration_files_dir: temp_test_migration_dir.clone(),
//             expected_latest_migration_basename_normalized: "migration_gen_1_after_init".into(),
//             code_origin_line: std::line!(),
//         })
//         .await;
//         insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//     }
//
//     // Run 3 generate
//     conf.set_file_basename("migration gen 2 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // Run 3 generate
//
//     // Second time, should not create migration files nor db records. i.e should be idempotent/
//     // Remain the same as the first time.
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 6,
//         expected_db_mig_count: 3,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration_gen_2_after_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
//     conf.set_file_basename("migration gen 3 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v3, mock_prompter)
//         .await;
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 8,
//         expected_db_mig_count: 4,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration_gen_3_after_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
// }
//
// #[test_case(Mode::Strict; "Strict")]
// #[test_case(Mode::Lax; "Lax")]
// #[tokio::test]
// async fn test_two_way_can_disallow_empty_migration_gen_on_no_diff(mode: Mode) {
//     let resources = Resources;
//     let resources_v2 = ResourcesV2;
//     let mock_prompter = MockPrompter::builder()
//         // disallow empty migration generation on no diffs
//         .allow_empty_migrations_gen(false)
//         .rename_or_delete_single_field_change(RenameOrDelete::Delete)
//         .build();
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(true)
//         .db_run(true)
//         .mode(mode)
//         .migration_basename("".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//     set_snapshot_suffix!("{}", conf.clone().snapshot_name_str());
//
//     // #### Init Phase ####
//     // Run 1 init
//     conf.set_file_basename("migration init no diff test".to_string())
//         .init_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // init cmd should instantiate the database connection which is reused internally in test
//     // config.
//     let cli_db = conf.db().clone();
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 2,
//         expected_db_mig_count: 1,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         // name is normalized to snake case
//         expected_latest_migration_basename_normalized: "migration_init_no_diff_test".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(joined_migration_files);
//
//     // #### Generate Phase ####
//     // Run 2: Generate  1st
//
//     // Set the db connection from the init migrator so that
//     // the generator can use the same db connection.
//     // generator.set_db_connection_from_migrator(&init);
//
//     if !joined_migration_files.is_empty() {
//         conf.set_file_basename("migration gen 1 after init no diff".to_string())
//             .generator_cmd()
//             .await
//             .run_fn(resources.clone(), mock_prompter.clone())
//             .await;
//
//         // First time, should create migration files and db records
//         let joined_migration_files = assert_with_db_instance(AssertionArg {
//             db: cli_db.clone(),
//             expected_mig_files_count: 2,
//             expected_db_mig_count: 1,
//             migration_files_dir: temp_test_migration_dir.clone(),
//             // The latest should stll remain the first generated at
//             // initialization becauase we set in the cli prompter
//             // to now allow empty migrations on no diffs.
//             // we are still using resources v1 here which is same
//             // as used at initialization.
//             expected_latest_migration_basename_normalized: "migration_init_no_diff_test".into(),
//             code_origin_line: std::line!(),
//         })
//         .await;
//         insta::assert_display_snapshot!(joined_migration_files);
//     }
//
//     // Run 3 generate
//     conf.set_file_basename("migration gen 2 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // Run 3 generate
//
//     // Second time, should not create migration files nor db records. i.e should be idempotent/
//     // Remain the same as the first time.
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 2,
//         expected_db_mig_count: 1,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         expected_latest_migration_basename_normalized: "migration_init_no_diff_test".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//
//     insta::assert_display_snapshot!(joined_migration_files);
//
//     // Initialize the 3rd time with different codebase resources. Should not allow creation the second time.
//     conf.set_file_basename("migration gen 3 after init new diff".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v2, mock_prompter)
//         .await;
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 4,
//         expected_db_mig_count: 2,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         // This should change unlike previous because we are using
//         // resources v2 which is different from the one used at
//         // initialization.
//         expected_latest_migration_basename_normalized: "migration_gen_3_after_init_new_diff".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(joined_migration_files);
// }
//
// #[test_case(Mode::Strict; "Strict")]
// #[test_case(Mode::Lax; "Lax")]
// #[tokio::test]
// #[should_panic]
// async fn should_panic_if_same_field_renaming_twice(mode: Mode) {
//     let resources = Resources;
//     let resources_v2 = ResourcesV2;
//     let mock_prompter = MockPrompter::builder()
//         // disallow empty migration generation on no diffs
//         .allow_empty_migrations_gen(false)
//         .rename_or_delete_single_field_change(RenameOrDelete::Rename)
//         .build();
//     let mig_dir = tempdir().expect("Failed to create temp directory");
//     let temp_test_migration_dir = &mig_dir.path().join("migrations-tests");
//     let mut conf = TestConfig::builder()
//         .reversible(true)
//         .db_run(true)
//         .mode(mode)
//         .migration_basename("migration init".into())
//         .migration_dir(temp_test_migration_dir.clone())
//         .build();
//
//     // #### Init Phase ####
//     // Run 1 init
//     conf.set_file_basename("migration init".to_string())
//         .init_cmd()
//         .await
//         .run_fn(resources.clone(), mock_prompter.clone())
//         .await;
//
//     // init cmd should instantiate the database connection which is reused internally in test
//     // config.
//     let cli_db = conf.db().clone();
//
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 2,
//         expected_db_mig_count: 1,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         // name is normalized to snake case
//         expected_latest_migration_basename_normalized: "migration_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // #### Generate Phase ####
//     // Run 2: Generate  1st
//
//     // Set the db connection from the init migrator so that
//     // the generator can use the same db connection.
//     // generator.set_db_connection_from_migrator(&init);
//
//     conf.set_file_basename("migration gen 1 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v2.clone(), mock_prompter.clone())
//         .await;
//
//     // First time, should create migration files and db records
//     let joined_migration_files = assert_with_db_instance(AssertionArg {
//         db: cli_db.clone(),
//         expected_mig_files_count: 4,
//         expected_db_mig_count: 2,
//         migration_files_dir: temp_test_migration_dir.clone(),
//         // The latest should stll remain the first generated at
//         // initialization becauase we set in the cli prompter
//         // to now allow empty migrations on no diffs.
//         // we are still using resources v1 here which is same
//         // as used at initialization.
//         expected_latest_migration_basename_normalized: "migration_gen_1_after_init".into(),
//         code_origin_line: std::line!(),
//     })
//     .await;
//     insta::assert_display_snapshot!(conf.clone().snapshot_name_str(), joined_migration_files);
//
//     // Run 3 generate
//     // Would panic because we are renaming the same field twice.
//     // within resources v2, animal struct table has the old_name
//     // attribute which must have already taken effect previously up here.
//     // We need to remove it before running another migration, otherwise,
//     // it will panic.
//     conf.set_file_basename("migration gen 2 after init".to_string())
//         .generator_cmd()
//         .await
//         .run_fn(resources_v2.clone(), mock_prompter.clone())
//         .await;
// }
