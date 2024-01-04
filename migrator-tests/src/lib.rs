use std::{fs, path::PathBuf};

use surreal_models::migrations::{
    Resources, ResourcesV10, ResourcesV2, ResourcesV3, ResourcesV4, ResourcesV5, ResourcesV6,
    ResourcesV7, ResourcesV8, ResourcesV9,
};
use surreal_orm::{
    migrator::{
        config::{DatabaseConnection, UrlDb},
        Basename, Down, FastForwardDelta, Generate, Init, Migration, MigrationFilename,
        MigrationFlag, Migrator, MockPrompter, Mode, RenameOrDelete, RollbackStrategyStruct,
        SubCommand, Up,
    },
    DbResources,
};

use typed_builder::TypedBuilder;
pub async fn assert_with_db_instance(args: AssertionArg) {
    let AssertionArg {
        expected_mig_files_count,
        expected_db_mig_meta_count: expected_db_mig_count,
        expected_latest_migration_file_basename_normalized,
        expected_latest_db_migration_meta_basename_normalized,
        code_origin_line,
        config,
        migration_type,
    } = args;

    let db = config.migrator.db().clone();
    let db_migrations = Migration::get_all_desc(db.clone()).await;
    let latest_migration_basename = Migration::get_latest(db.clone()).await.map(|m| {
        MigrationFilename::try_from(m.name.clone())
            .expect("Failed to parse file name")
            .basename()
    });
    dbg!(&latest_migration_basename);
    assert_eq!(
        latest_migration_basename, expected_latest_db_migration_meta_basename_normalized,
        "Base name in file does not match the base name in the db. Line: {code_origin_line}",
    );

    let migration_files = config.read_migrations_from_dir_sorted_asc();

    let latest_file_name = migration_files.iter().max();

    assert_eq!(
        latest_file_name.map(|lfn| lfn.basename()),
        expected_latest_migration_file_basename_normalized,
        "Base name in file does not match the base name in the db. Line: {code_origin_line}"
    );

    assert_eq!(
        db_migrations.len() as u8,
        expected_db_mig_count,
        "migration Counts do not match with what is in the db. Line: {code_origin_line}",
    );

    assert_eq!(
        migration_files.len() as u8,
        match migration_type {
            MigrationFlag::TwoWay => expected_mig_files_count * 2,
            MigrationFlag::OneWay => expected_mig_files_count,
        },
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
            migration_files
                .iter()
                .filter(|filename| match migration_type {
                    MigrationFlag::TwoWay => {
                        //
                        db_mig_name.to_up() == filename.to_up()
                    }
                    MigrationFlag::OneWay => &db_mig_name == *filename,
                })
                .collect::<Vec<_>>()
        };

        match migration_type {
            MigrationFlag::TwoWay => {
                let found_mig_file = found_migration_file(mig_name_from_db.clone());
                assert_eq!(found_mig_file.len(), 2);

                let ups = found_mig_file
                    .iter()
                    .filter(|m| m.is_up())
                    .collect::<Vec<_>>();
                assert_eq!(ups.len(), 1);
                assert_eq!(ups.first().cloned(), Some(&&mig_name_from_db));
                assert_eq!(mig_name_from_db.extension().to_string(), "up.surql");
                assert_eq!(
                    ups.first().map(|u| u.extension().to_string()),
                    Some("up.surql".into())
                );
                assert_eq!(
                    ups.first().map(|u| u.to_string()),
                    Some(db_mig_record.clone().name)
                );
                assert_eq!(
                    ups.first().map(|u| u.timestamp()),
                    Some(db_mig_record.clone().timestamp)
                );

                let downs = found_mig_file
                    .iter()
                    .filter(|m| m.is_down())
                    .collect::<Vec<_>>();
                assert_eq!(downs.len(), 1);
                assert_eq!(downs.first().cloned(), Some(&&mig_name_from_db.to_down()));
                assert_eq!(
                    downs.first().map(|u| u.extension().to_string()),
                    Some("down.surql".into())
                );
                assert_eq!(
                    // we store reversible migration meta with up extension
                    downs.first().map(|u| u.to_up().to_string()),
                    Some(db_mig_record.clone().name)
                );
                assert_eq!(
                    downs.first().map(|u| u.timestamp()),
                    Some(db_mig_record.clone().timestamp)
                );
            }
            MigrationFlag::OneWay => {
                let found_mig_file = found_migration_file(mig_name_from_db.clone());
                assert_eq!(found_mig_file.len(), 1);
                let found_mig_file = found_mig_file.first().expect("File must exist");
                assert_eq!(found_mig_file.basename(), basename);
                assert_eq!(found_mig_file.extension(), extension);
                assert_eq!(found_mig_file.timestamp(), timestamp);
                assert_eq!(found_mig_file.to_string(), db_mig_record.name);
            }
        }

        assert_eq!(
            mig_name_from_db.to_string(),
            format!("{timestamp}_{basename}.{extension}"),
            "File name should be in the format of {timestamp}_{basename}.{extension}"
        );
    }
}

pub fn get_db_connection_config() -> DatabaseConnection {
    DatabaseConnection::builder()
        .db("test".into())
        .ns("test".into())
        .user("root".into())
        .pass("root".into())
        .url(UrlDb::Memory)
        .build()
}

#[derive(Clone, TypedBuilder)]
pub struct AssertionArg {
    pub migration_type: MigrationFlag,
    pub expected_mig_files_count: u8,
    pub expected_db_mig_meta_count: u8,
    pub expected_latest_migration_file_basename_normalized: Option<Basename>,
    pub expected_latest_db_migration_meta_basename_normalized: Option<Basename>,
    pub code_origin_line: u32,
    pub config: TestConfigNew,
}

#[derive(Clone, TypedBuilder)]
pub struct TestConfigNew {
    migrator: Migrator,
}

impl TestConfigNew {
    pub async fn new(mode: Mode, migration_dir: &PathBuf) -> Self {
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

    pub fn assert_migration_queries_snapshot(
        &self,
        migration_type: MigrationFlag,
        mode: Mode,
        file: &str,
        line: u32,
    ) {
        let migration_dir = self.migrator.migrations_dir.as_ref().unwrap().clone();
        let migration_dir_str = migration_dir.join("*.surql").to_string_lossy().to_string();
        let migration_queries_snaps = glob::glob(&migration_dir_str)
            .unwrap()
            .map(|x| {
                let path = x.unwrap();
                let input = fs::read_to_string(&path).unwrap();
                let header = MigrationFilename::try_from(
                    path.file_name().unwrap().to_string_lossy().to_string(),
                )
                .unwrap();
                let basename = header.basename();
                let extension = header.extension();
                let snaps = format!(
                "header: Basename - {basename}. Extension - {extension}\n Migration Query: {input}"
            );
                snaps
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let name_differentiator =
            format!("source_{file}___line_{line}___migration_type_{migration_type}___mode_{mode}");
        insta::assert_snapshot!(name_differentiator, migration_queries_snaps);
    }

    pub fn assert_migration_queries_snapshot_experiment(
        &self,
        migration_type: MigrationFlag,
        mode: Mode,
        file: &str,
        line: u32,
    ) -> (String, String) {
        let migration_dir = self.migrator.migrations_dir.as_ref().unwrap().clone();
        let migration_dir_str = migration_dir.join("*.surql").to_string_lossy().to_string();
        let migration_queries_snaps = glob::glob(&migration_dir_str)
            .unwrap()
            .map(|x| {
                let path = x.unwrap();
                let input = fs::read_to_string(&path).unwrap();
                let header = MigrationFilename::try_from(
                    path.file_name().unwrap().to_string_lossy().to_string(),
                )
                .unwrap();
                let basename = header.basename();
                let extension = header.extension();
                let snaps = format!(
                "header: Basename - {basename}. Extension - {extension}\n Migration Query: {input}"
            );
                snaps
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let name_differentiator =
            format!("source_{file}___line_{line}___migration_type_{migration_type}___mode_{mode}");
        (name_differentiator, migration_queries_snaps)
    }

    pub fn read_down_migrations_content_from_dir_sorted(&self) -> Vec<String> {
        self.read_down_migrations_from_dir_sorted_asc()
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

    pub fn read_migrations_from_dir_sorted_asc(&self) -> Vec<MigrationFilename> {
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
                .filter(|f: &MigrationFilename| f.is_down() || f.is_up() || f.is_unidirectional())
                .collect::<Vec<MigrationFilename>>();

        files.sort_by_key(|a| a.timestamp());
        files
    }

    pub fn read_down_migrations_from_dir_sorted_asc(&self) -> Vec<MigrationFilename> {
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

    pub async fn run(
        &mut self,
        codebase_resources: Option<impl DbResources>,
        prompter: MockPrompter,
    ) -> &mut Self {
        self.migrator.run_test(codebase_resources, prompter).await;
        self
    }

    pub fn set_cmd(&mut self, cmd: impl Into<SubCommand>) -> &mut Self {
        self.migrator.set_cmd(cmd.into());
        self
    }

    pub async fn run_init_cmd(
        &mut self,
        init_cmd: Init,
        codebase_resources: impl DbResources,
        prompter: MockPrompter,
    ) -> &mut Self {
        self.set_cmd(init_cmd)
            .run(Some(codebase_resources), prompter)
            .await;
        self
    }

    pub async fn run_gen(&mut self, basename: Basename, resources: impl DbResources) -> &mut Self {
        self.set_cmd(Generate::builder().name(basename.into()).run(false).build())
            .run(
                Some(resources),
                MockPrompter::builder()
                    .allow_empty_migrations_gen(true)
                    .rename_or_delete_single_field_change(RenameOrDelete::Rename)
                    .build(),
            )
            .await
    }

    pub async fn run_down(
        &mut self,
        rollback_strategy: &RollbackStrategyStruct,
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

    pub async fn run_up(&mut self, fwd_delta: &FastForwardDelta) -> &mut Self {
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

    pub async fn generate_test_migrations_arbitrary(
        &mut self,
        number_of_migs_to_gen: usize,
        migration_type: MigrationFlag,
    ) -> &mut Self {
        if number_of_migs_to_gen == 0 {
            panic!("Generating 0 migrations not allowed.");
        }

        let mock_prompter = MockPrompter::builder()
            .allow_empty_migrations_gen(true)
            .rename_or_delete_single_field_change(RenameOrDelete::Rename)
            .build();

        self.set_cmd(SubCommand::Init(
            Init::builder()
                .reversible(match migration_type {
                    MigrationFlag::TwoWay => true,
                    MigrationFlag::OneWay => false,
                })
                .name("migration 1-init".into())
                .run(false)
                .build(),
        ))
        .run(Some(Resources), mock_prompter.clone())
        .await;

        let gen = |basename: String| {
            SubCommand::Generate(Generate::builder().name(basename.into()).run(false).build())
        };

        for i in 2..=number_of_migs_to_gen {
            self.set_cmd(gen(format!("migration {}-gen after init", i)))
                .run(Some(Resources), mock_prompter.clone())
                .await;
        }

        self
    }
    pub async fn generate_12_test_migrations_reversible(&mut self, reversible: bool) -> &mut Self {
        let mock_prompter = MockPrompter::builder()
            .allow_empty_migrations_gen(true)
            .rename_or_delete_single_field_change(RenameOrDelete::Rename)
            .build();

        self.migrator
            .set_cmd(SubCommand::Init(
                Init::builder()
                    .reversible(reversible)
                    .name("migration 1-init".into())
                    .run(false)
                    .build(),
            ))
            .run_fn(Resources, mock_prompter.clone())
            .await;

        let gen =
            |basename: &'static str| Generate::builder().name(basename.into()).run(false).build();

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

    pub async fn generate_test_migrations(&mut self) -> &mut Self {
        self.generate_12_test_migrations_reversible(true).await
    }

    pub fn get_either_filename_type_at_position(
        &self,
        position: u8,
        migration_type: impl Into<MigrationFlag>,
    ) -> MigrationFilename {
        if position == 0 {
            panic!(
                "Position cannot be 0. Must start from 1. This uses position rather than index."
            );
        }
        let position = match migration_type.into() {
            MigrationFlag::TwoWay => position * 2,
            MigrationFlag::OneWay => position,
        };
        self.read_migrations_from_dir_sorted_asc()[position as usize - 1].clone()
    }
    // from 1st upwards. Starts from 1
    pub fn get_down_filename_at_position(&self, position: usize) -> MigrationFilename {
        if position == 0 {
            panic!(
                "Position cannot be 0. Must start from 1. This uses position rather than index."
            );
        }
        let xx = self.read_down_migrations_from_dir_sorted_asc()[position - 1].clone();
        dbg!(&xx);
        xx
    }
}
