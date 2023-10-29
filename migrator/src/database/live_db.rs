use surreal_query_builder::{statements::*, *};
use surrealdb::{sql::Thing, Connection, Surreal};

use crate::{
    migration, FileManager, Migration, MigrationFileName, MigrationOneWay, MigrationResult,
    MigrationTwoWay,
};

struct LiveDb<C: Connection> {
    db: Surreal<C>,
    file_manager: FileManager,
}

impl From<MigrationTwoWay> for MigrationOneWay {
    fn from(m: MigrationTwoWay) -> Self {
        Self {
            id: m.id,
            name: m.name,
            timestamp: m.timestamp,
            content: m.up,
        }
    }
}

impl<C: Connection> LiveDb<C> {
    pub fn db(&self) -> Surreal<C> {
        self.db.clone()
    }

    pub async fn run_all_local_dir_up_migrations(&self) -> MigrationResult<()> {
        let all_migrations = self.file_manager.get_two_way_migrations()?;
        self.run_against_db(all_migrations).await?;

        Ok(())
    }
    pub async fn run_against_db(
        &self,
        all_migrations: Vec<impl Into<MigrationOneWay>>,
    ) -> MigrationResult<()> {
        let migration::Schema {
            name, timestamp, ..
        } = &Migration::schema();
        let migration_table = Migration::table_name();

        // Get the latest migration
        let latest_migration = select(All)
            .from(migration_table.clone())
            .order_by(timestamp.desc())
            .limit(1)
            .return_one::<Migration>(self.db())
            .await?;

        // Get migrations that are not yet applied
        let migrations_to_run = all_migrations
            .into_iter()
            .map(|m| {
                let m: MigrationOneWay = m.into();
                m
            })
            .filter(|m| {
                latest_migration.as_ref().map_or(true, |latest_migration| {
                    m.timestamp > latest_migration.timestamp
                })
            })
            .collect::<Vec<_>>();

        // Get queries to run
        let migration_queries = migrations_to_run
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<_>>()
            .join("\n");

        // Create queries to mark migrations as applied
        let mark_queries_registered_queries = migrations_to_run
            .iter()
            .map(|m| Migration::create_raw(m.id.clone(), m.name.clone(), m.timestamp).build())
            .collect::<Vec<_>>()
            .join("\n");

        println!("Running queries: {}", migration_queries);

        // Join migrations with mark queries
        let all = format!("{}\n{}", migration_queries, mark_queries_registered_queries);

        // Run them as a transaction against a local in-memory database
        if !all.trim().is_empty() {
            begin_transaction()
                .query(Raw::new(all))
                .commit_transaction()
                .run(self.db())
                .await?;
        }

        Ok(())
    }

    pub async fn get_applied_twoway_migrations(&self) -> MigrationResult<Vec<Migration>> {
        // let name = Migration::schema().name;
        let migration::Schema { name, .. } = Migration::schema();
        let migration = Migration::table_name();

        // select [{ name: "Oyelowo" }]
        // select value [ "Oyelowo" ]
        // select_only. Just on object => { name: "Oyelowo" }
        let migration_names = select_value(name)
            .from(migration)
            .return_many::<Migration>(self.db())
            .await?;
        Ok(migration_names)
    }

    pub async fn mark_migration_as_applied(
        &self,
        migration_name: impl Into<MigrationFileName>,
    ) -> MigrationResult<Migration> {
        let migration_name: MigrationFileName = migration_name.into();
        println!("Applying migration: {}", migration_name.clone());

        // let migration = Migration {
        //     id: Migration::create_id(migration_name.to_string()),
        //     name: migration_name.to_string(),
        //     timestamp: migration_name.timestamp(),
        // }
        // .create()
        // .get_one(self.db())
        let migration = Migration::create_raw(
            migration_name.clone(),
            migration_name.to_string(),
            migration_name.timestamp(),
        )
        .get_data::<Migration>(self.db())
        .await?;
        println!("Migration applied: {}", migration_name);

        Ok(migration.expect("Migration should be applied"))
    }

    pub async fn unmark_migration(&self, migration_name: MigrationFileName) -> MigrationResult<()> {
        println!("Unmark migration: {}", migration_name);
        self.db()
            .delete::<Option<Migration>>(Migration::create_id(migration_name.clone()))
            .await?;
        // delete::<Migration>(Migration::create_id(migration_name.to_string()))
        //     .run(self.db())
        //     .await?;
        println!("Migration unmarked: {}", migration_name);
        Ok(())
    }

    pub async fn rollback_migration(
        &self,
        migration_name: MigrationFileName,
        fm: FileManager,
    ) -> MigrationResult<()> {
        let migration = fm.get_two_way_migration_by_name(migration_name.clone())?;
        if let Some(migration) = migration {
            let down_migration = migration.down;
            if !down_migration.trim().is_empty() {
                // Raw::new(down_migration).run(db);
                self.db().query(down_migration).await?;
            } else {
                println!("No down migration found for migration: {}", migration_name);
            }
            self.unmark_migration(migration.name.try_into()?).await?;
        } else {
            println!(
                "Cannot rollback migration: No migration found with name: {}",
                migration_name
            );
        };
        Ok(())
    }
}

pub enum EmbeddedMigrations {
    OneWay(Vec<MigrationOneWay>),
    TwoWay(Vec<MigrationTwoWay>),
}
