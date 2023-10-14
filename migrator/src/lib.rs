use async_trait::async_trait;
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use surreal_orm::{
    self,
    statements::{create, create_only, delete, info_for, select, select_value},
    *,
};
use surrealdb::{
    self,
    engine::local::{Db, Mem},
    Surreal,
};
use thiserror::Error;
// #[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "planet")]
// pub struct Planet {
//     pub id: SurrealSimpleId<Self>,
//     pub name: String,
//     #[surreal_orm(type_="int")]
//     pub population: u64,
//     pub created: DateTime<Utc>,
//     pub tags: Vec<u64>,
// }
//
//
// # Fields
// Planet::get_schema()
// vec![
//    DEFINE FIELD id TYPE id,  // permissions, assertions etc
//    DEFINE FIELD name TYPE string,
//    DEFINE FIELD population TYPE int,
//    DEFINE FIELD created TYPE datetime,
//    DEFINE FIELD tags TYPE array<int>,
// ]
//
//
// # Tables
// Planet::get_table_def()
//
// DEFINE TABLE planet; // permissions, assertions etc
//
//
// # Events
// DEFINE EVENT ....
//
// Indexes
// DEFINE INDEX ....
//

use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::Path,
};

// TODOs
// Extract the schema from the struct
// - get fields schema definitions
// - get table schema definition
// - get events schema definition
// - get indexes schema definition
//
//
//
//
//   # Create a migration directory with either both up/down migations or just up.
//   # Migration file format would be migrations/<timestamp>-__<direction>__<name>.sql
//   # Example: 2382320302303__up__create_planet.sql
//   # Example: 2382320302303__down__delete_planet.sql
//
// # Migration directory
// - get all fields defintions from migration directory
// - get all table defintions from migration directory
// - get all events defintions from migration directory
// - get all indexes defintions from migration directory
//
//
// # Create a migration table in the database with fields for the migration name, timestamp, direction, status, etc
// # Example: CREATE TABLE migration (id int, name string, timestamp int, direction string, status string)
//
// 1. # Get all migrations names from the database
// e.g: SELECT name FROM migrations
//
// 2. # Get all migrations names from the migration directory
//
// 3. # Compare the two lists and get the difference
//
// 4. # Run the migrations that are not in the database
//
// 5. # Update the migration table with the new migrations
//      - marking migration as registered
//
//
// # Support rolling back
//    - mark migration as unregistered i.e rolled back
//
//    CHANGE TYPES:
//    # Fields
//    - add field -> DEFINE FIELD age TYPE int;
//    - remove field -> Unset field or Set field as null to delete the field
//    - change field type (e.g int -> float) PREV -> DEFINE FIELD age TYPE int; NEW -> DEFINE FIELD age TYPE float;
//    - change field name (e.g age -> age2) PREV -> DEFINE FIELD age TYPE int; NEW -> DEFINE FIELD age2 TYPE int;
//       -- Main thing to do it to reliably detect a field name change and handle the data change
//       and migration. How do we reliably detect a name change?:
//
//      ##  STRATEGIES
//
//      ###  STRATEGY 1
//       Old version
//       struct Planet {
//            pub id: SurrealSimpleId<Self>,
//            pub name: String,
//       }
//
//       New version
//       Note: If using this strategy, we need to first confirm that the transformation/change had
//       not already previously been done. We can do this by checking the database schema for the
//       new field name. If it exists, then we can skip the migration.
//       struct Planet {
//            pub id: SurrealSimpleId<Self>,
//            #[surreal_orm(old_name="name")]
//            pub firstName: String,
//       }
//
//         DIFF  -> name -> firstName.
//         left[name] -> right[firstName]
//
//      ###  STRATEGY 2
//      - Prompt user when you detect a potential field name change especially, when there is an
//      addition and removal at the same time.
//
//       - - A bit mAore complex, we need to create a new field with the new name and copy the data from the old field to the new field
//       - - Then we can delete the old field
//
//
//
//    - add table
//    - remove table
//    - add event
//    - remove event
//    - add index
//    - remove index
//

#[derive(Error, Debug)]
enum MigrationError {
    #[error("Migration already exists")]
    MigrationAlreadyExists,
    #[error("Migration does not exist")]
    MigrationDoesNotExist,
    #[error("Migration not registered")]
    MigrationNotRegistered,
    #[error("Migration not unregistered")]
    MigrationNotUnregistered,
    #[error("Direction does not exist")]
    DirectionDoesNotExist,
    #[error("Migration name does not exist")]
    MigrationNameDoesNotExist,
    #[error("Invalid migration name")]
    InvalidMigrationName,
    #[error("Invalid timestamp")]
    InvalidTimestamp,

    #[error(transparent)]
    ProblemWithQuery(#[from] SurrealOrmError),

    #[error(transparent)]
    InvalidRegex(#[from] regex::Error),
}

pub type MigrationResult<T> = Result<T, MigrationError>;

enum MigrationType {
    Field,
    Table,
    Event,
    Index,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TableInfo {
    events: HashMap<String, String>,
    indexes: HashMap<String, String>,
    tables: HashMap<String, String>,
    fields: HashMap<String, String>,
}

impl TableInfo {
    pub fn get_fields(&self) -> Info {
        self.fields.clone()
    }

    pub fn get_fields_names(&self) -> Vec<String> {
        self.fields.keys().cloned().collect()
    }

    pub fn get_fields_definitions(&self) -> Vec<String> {
        self.fields.values().cloned().collect()
    }
}

// format: <timestamp>__<direction>__<name>.sql
#[derive(Debug, Clone)]
struct MigrationName(String);

impl<T: Into<String>> From<T> for MigrationName {
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl MigrationName {
    pub fn extract_at_index(
        &self,
        regex_term: &str,
        capture_index: usize,
    ) -> MigrationResult<&str> {
        let regex = Regex::new(regex_term).map_err(MigrationError::InvalidRegex)?;
        let captures = regex
            .captures(&self.0)
            .ok_or(MigrationError::InvalidMigrationName)?;
        let captured = captures
            .get(capture_index)
            .ok_or(MigrationError::InvalidMigrationName)?
            .as_str();
        Ok(captured)
    }

    pub fn timestamp_as_str(&self) -> MigrationResult<&str> {
        self.extract_at_index(r"(\d+)__(\w+).sql", 1)
    }

    pub fn timestamp(&self) -> MigrationResult<u64> {
        let timestamp = self.extract_at_index(r"(\d+)__(\w+).sql", 1)?;
        timestamp
            .parse::<u64>()
            .map_err(|_| MigrationError::InvalidTimestamp)
    }

    pub fn direction(&self) -> MigrationResult<Direction> {
        let direction = self.extract_at_index(r"(\d+)__(\w+).sql", 2)?;
        match direction {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(MigrationError::DirectionDoesNotExist),
        }
    }

    pub fn name_suffix(&self) -> MigrationResult<&str> {
        self.extract_at_index(r"(\d+)__(\w+).sql", 2)
    }
}

impl Display for MigrationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

pub struct Database {
    // connection details here
    db: Surreal<Db>,
}

// #[async_trait]
impl Database {
    pub async fn init() -> Self {
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        Self { db }
    }

    pub fn db(&self) -> Surreal<Db> {
        self.db.clone()
    }

    pub async fn get_db_info(&self) -> MigrationResult<DbInfo> {
        let info = info_for()
            .database()
            .get_data::<DbInfo>(self.db())
            .await?
            .unwrap();
        Ok(info)
    }

    pub async fn get_table_info(&self, table_name: String) -> MigrationResult<TableInfo> {
        let info = info_for()
            .table(table_name)
            .get_data::<TableInfo>(self.db())
            .await?
            .unwrap();
        Ok(info)
    }

    pub async fn get_current_schema(&self, table_name: &str) -> MigrationResult<TableInfo> {
        let info = info_for()
            .table(table_name.to_string())
            .get_data::<TableInfo>(self.db())
            .await?
            .unwrap();
        Ok(info)
    }

    pub async fn execute(&self, query: String) {
        println!("Executing query: {}", query);
        self.db().query(query).await.unwrap();
    }

    pub async fn get_applied_migrations_from_db(&self) -> MigrationResult<Vec<String>> {
        let migration::Schema { name, .. } = Migration::schema();
        let migration = Migration::table_name();

        // select [{ name: "Oyelowo" }]
        // select value [ "Oyelowo" ]
        // select_only. Just on object => { name: "Oyelowo" }
        let migration_names = select_value(name)
            .from(migration)
            .return_many::<String>(self.db())
            .await?;
        // vec![
        //     "20230912__up__add_name".into(),
        //     "20230912__down__remove_name".into(),
        // ]
        Ok(migration_names)
    }

    pub async fn mark_migration_as_applied(
        &self,
        migration_name: impl Into<MigrationName>,
    ) -> MigrationResult<MigrationMetadata> {
        let migration_name: MigrationName = migration_name.into();
        println!("Applying migration: {}", migration_name);

        let migration = MigrationMetadata {
            id: MigrationMetadata::create_id(migration_name.to_string()),
            name: migration_name.to_string(),
            timestamp: migration_name.timestamp().expect("Invalid timestamp"),
        }
        .create()
        .get_one(self.db())
        .await?;
        println!("Migration applied: {}", migration_name);

        Ok(migration)
    }

    pub async fn unmark_migration(
        &self,
        migration_name: impl Into<MigrationName>,
    ) -> MigrationResult<()> {
        let migration_name: MigrationName = migration_name.into();
        println!("Unmark migration: {}", migration_name);
        delete::<MigrationMetadata>(MigrationMetadata::create_id(migration_name.to_string()))
            .run(self.db());
        println!("Migration unmarked: {}", migration_name);
        Ok(())
    }
}
#[derive(Node, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "migration_metadata")]
pub struct MigrationMetadata {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub timestamp: u64,
    // pub timestamp: Datetime<Utc>,
    // status: String,
}

impl MigrationMetadata {}

// Warn when id field not included in a model

// Migratiions from migration directory
#[derive(Node, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "migration")]
pub struct Migration {
    pub id: SurrealId<Self, String>,
    pub name: String,
    timestamp: String,
    up: String,
    #[surreal_orm(type_ = "option<string>")]
    down: Option<String>,
    // status: String,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = match self {
            Self::Up => "up",
            Self::Down => "down",
        };
        write!(f, "{}", direction)
    }
}

impl Migration {
    pub fn get_all_from_migrations_dir() -> Vec<Migration> {
        // <number>__up__<name>.sql
        let regex_up = Regex::new(r"(\d+)__up__(\w+).sql").unwrap();
        let regex_down = Regex::new(r"(\d+)__down__(\w+).sql").unwrap();

        // read migrations directory
        let migrations = fs::read_dir("migrations/").unwrap();

        let mut migrations_meta = vec![];

        // get all migration names
        for migration in migrations {
            let migration = migration.unwrap();
            let path = migration.path();
            let parent_dir = path.parent().unwrap();
            let path = path.to_str().unwrap();
            let migration_name = path.split("/").last().unwrap();
            let migration_up_name = migration_name.to_string();

            if !regex_up.is_match(&migration_up_name.clone()) {
                continue;
            }

            let captures = regex_up.captures(&migration_up_name).unwrap();
            // let  "20230912__up__add_name.sql";
            let all = captures.get(0).unwrap().as_str();
            let timestamp = captures.get(1).unwrap().as_str();
            let name = captures.get(2).unwrap().as_str().to_string();
            let content_up = fs::read_to_string(path).unwrap();
            let migration_down_name = format!("{}__down__{}.sql", timestamp, name);
            let content_down = fs::read_to_string(parent_dir.join(migration_down_name));
            let content_down = match content_down {
                Ok(content_down) => Some(content_down),
                Err(_) => None,
            };

            let migration = Migration {
                id: Migration::create_id(migration_up_name.clone()),
                timestamp: timestamp.to_string(),
                name,
                up: content_up,
                down: content_down,
            };

            migrations_meta.push(migration);
        }
        migrations_meta
    }

    pub fn get_migrations_by_name(migration_name: impl Into<MigrationName>) -> Option<Migration> {
        //   # Migration file format would be migrations/<timestamp>-__<direction>__<name>.sql
        //   # Example: 2382320302303__up__create_planet.sql
        //   # Example: 2382320302303__down__delete_planet.sql
        let migration_name: MigrationName = migration_name.into();
        Self::get_all_from_migrations_dir()
            .into_iter()
            .find(|m| m.name == migration_name.to_string())
    }

    pub fn create_migration_file(
        query: String,
        direction: Direction,
        name: impl Into<String> + std::fmt::Display,
    ) {
        //   # Migration file format would be migrations/<timestamp>-__<direction>__<name>.sql
        //   # Example: 2382320302303__up__create_planet.sql
        //   # Example: 2382320302303__down__delete_planet.sql
        // let timestamp = Utc::now().timestamp();
        println!("Creating migration file: {}", name);
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let _ = fs::create_dir_all("migrations").expect("Problem creating migrations directory");

        let path = format!("migrations/{}__{}__{}.sql", timestamp, direction, name);
        let mut file = File::create(&path).unwrap();

        file.write_all(query.as_bytes()).unwrap();

        println!("Migration file created at: {}", path);
    }

    pub async fn run_migrations(db: &mut Database) -> MigrationResult<()> {
        let applied_migrations = db.get_applied_migrations_from_db();
        let all_migrations = Self::get_all_from_migrations_dir();

        let applied_migrations = applied_migrations.await?;
        for migration in all_migrations {
            if !applied_migrations.contains(&migration.name) {
                db.execute(migration.up);
                db.mark_migration_as_applied(migration.name);
            }
        }
        Ok(())
    }

    pub fn rollback_migration(db: &mut Database, migration_name: MigrationName) {
        let migration = Self::get_migrations_by_name(migration_name.clone());
        if let Some(migration) = migration {
            let down_migration = migration.down;
            if let Some(down_migration) = down_migration {
                // Raw::new(down_migration).run(db);
                db.execute(down_migration);
            } else {
                println!("No down migration found for migration: {}", migration_name);
            }
            db.unmark_migration(migration.name);
        } else {
            println!(
                "Cannot rollback migration: No migration found with name: {}",
                migration_name
            );
        }
    }
}

// INFO FOR DB
// [
//     {
//         "analyzers": {},
//         "functions": {},
//         "params": {},
//         "scopes": {},
//         "tables": {
//             "movie": "DEFINE TABLE movie SCHEMALESS",
//             "person": "DEFINE TABLE person SCHEMAFULL CHANGEFEED 1d"
//         },
//         "tokens": {},
//         "users": {}
//     }
// ]

type Info = HashMap<String, String>;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbInfo {
    analyzers: Info,
    functions: Info,
    params: Info,
    scopes: Info,
    tables: Info,
    tokens: Info,
    users: Info,
}

impl DbInfo {
    pub fn get_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "planet")]
pub struct Planet {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub population: u64,
    pub created: chrono::DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student")]
pub struct Student {
    pub id: SurrealSimpleId<Self>,
    pub school: String,
    pub age: u8,
    pub class: String,
}

// Migration files in migration directory
// Current schema in codebase
// Current schema in database
//

enum FieldChange {
    // Detection: When a new field exists in the codebase but not in the database
    Add { name: String, definition: String },
    // Detection: When a field exists in the database but not in the codebase
    Remove { name: String },

    // Detection: Strategies mentioned earlier.
    Rename { old_name: String, new_name: String },
}

pub fn extract_schema_from_models() -> TableInfo {
    let x = Planet::schema();
    TableInfo::default()
}

// // Get all up migrations from migration directory
//
// Run all up migrations as a transaction in an in-memory db
//
// Parse all the table names available in all the queries into a HashSet
// Get all the table names from the in-memory db
//
// Get all the tables and corresponding field definitiins from teh codebase e.g Field(for now)
// Do a left and right diff to know which tables/fields to remove or add, or change
