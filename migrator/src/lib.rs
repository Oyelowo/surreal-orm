use async_trait::async_trait;
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use surreal_orm::{
    statements::{
        begin_transaction, create, create_only, delete, info_for, remove_field, remove_table,
        select, select_value, update,
    },
    Edge, Node, *,
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
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::{self, File},
    io::{stdin, Write},
    ops::Deref,
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
//            pub name: String,
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
//
//
// DB info
// [
//     {
//         "analyzers": {},
//         "functions": {},
//         "params": {},
//         "scopes": {},
//         "tables": {
//             "movie": "DEFINE TABLE movie SCHEMALESS",
//             "person": "DEFINE TABLE person SCHEMAFULL CHANGEFEED 1d",
//             "user": "DEFINE TABLE user SCHEMAFULL",
//             "userr": "DEFINE TABLE userr SCHEMAFULL"
//         },
//         "tokens": {},
//         "users": {}
//     }
// ]
//
//
// Table Info
// [
//     {
//         "events": {},
//         "fields": {
//             "firstName": "DEFINE FIELD firstName ON person TYPE option<string>",
//             "skills": "DEFINE FIELD skills ON person TYPE option<array>",
//             "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
//         },
//         "indexes": {},
//         "tables": {}
//     }
// ]

#[derive(Error, Debug)]
pub enum MigrationError {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableInfo {
    events: Events,
    indexes: Indexes,
    tables: Tables,
    fields: Fields,
}

impl TableInfo {
    pub fn events(&self) -> Events {
        self.events.clone()
    }

    pub fn indexes(&self) -> Indexes {
        self.indexes.clone()
    }

    // This is usually empty in when getting the info from a table.
    // Only used when getting the info from a database.
    // So, turning it off for now
    // pub fn tables(&self) -> Tables {
    //     self.tables
    // }

    pub fn fields(&self) -> Fields {
        self.fields.clone()
    }
}

// format: <timestamp>__<direction>__<name>.sql
#[derive(Debug, Clone)]
pub struct MigrationName(String);

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

struct CodeBaseMeta;
type TableName = String;


pub enum By {
   NewName(String),
   OldName(String),
}

impl CodeBaseMeta {
    pub fn find_field_with_oldname_attr(table_name: String, field_name: String) -> Option<FieldMetadata> {
        Self::get_codebase_renamed_fields_meta()
            .get(&table_name)
            .unwrap_or(&vec![])
            .clone()
            .into_iter()
            .find(|f| f.name.to_string() == field_name && f.old_name.is_some())
    }
    
    pub fn find_field_has_old_name(table_name: String, by: By) -> Option<FieldMetadata> {
        Self::get_codebase_renamed_fields_meta()
            .get(&table_name)
            .unwrap_or(&vec![])
            .clone()
            .into_iter()
            .find(|f| match &by {
                By::NewName(new_name) => f.name.to_string() == new_name.to_string() && f.old_name.is_some(),
                By::OldName(old_name) =>  f.old_name.clone().filter(|n| n.to_string() == old_name.to_string()).is_some() && f.old_name.is_some(),
            })
    }

    pub fn get_codebase_renamed_fields_meta() -> HashMap<TableName, Vec<FieldMetadata>> {
        let mut code_renamed_fields = HashMap::new();
        let animal_fields_renamed = Animal::get_field_meta()
            .into_iter()
            .filter(|f| {
                let x = f
                    .old_name
                    .clone()
                    .is_some_and(|o| !o.to_string().is_empty());
                x
            })
            .collect::<Vec<_>>();
        code_renamed_fields.insert(Animal::table_name().to_string(), animal_fields_renamed);

        let animal_eats_crop_fields_renamed = AnimalEatsCrop::get_field_meta()
            .into_iter()
            .filter(|f| {
                let x = f
                    .old_name
                    .clone()
                    .is_some_and(|o| !o.to_string().is_empty());
                x
            })
            .collect::<Vec<_>>();
        code_renamed_fields.insert(
            AnimalEatsCrop::table_name().to_string(),
            animal_eats_crop_fields_renamed,
        );

        let crop_fields_renamed = Crop::get_field_meta()
            .into_iter()
            .filter(|f| {
                let x = f
                    .old_name
                    .clone()
                    .is_some_and(|o| !o.to_string().is_empty());
                x
            })
            .collect::<Vec<_>>();
        code_renamed_fields.insert(Crop::table_name().to_string(), crop_fields_renamed);

        code_renamed_fields
    }

    pub fn get_codebase_schema_queries() -> String {
        // Test data
        let animal_tables = Animal::define_table().to_raw().build();
        let animal_fields = Animal::define_fields()
            .iter()
            .map(|f| f.to_raw().build())
            .collect::<Vec<_>>()
            .join(";\n");
        let animal_eats_crop_tables = AnimalEatsCrop::define_table().to_raw().build();
        let animal_eats_crop_fields = AnimalEatsCrop::define_fields()
            .iter()
            .map(|f| f.to_raw().build())
            .collect::<Vec<_>>()
            .join(";\n");
        let crop_tables = Crop::define_table().to_raw().build();
        let crop_fields = Crop::define_fields()
            .iter()
            .map(|f| f.to_raw().build())
            .collect::<Vec<_>>()
            .join(";\n");

        let queries_joined = [
            animal_tables,
            animal_fields,
            animal_eats_crop_tables,
            animal_eats_crop_fields,
            // crop_tables,
            // crop_fields,
        ]
        .join(";\n");
        // let queries_joined = format!("{};\n{}", tables, fields);

        queries_joined
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FullDbInfo {
    all_resources: DbInfo,
    fields_by_table: HashMap<TableName, TableInfo>,
}

impl FullDbInfo {
    pub fn tables(&self) -> Tables {
        self.all_resources.tables()
    }

    pub fn get_table_info(&self, table_name: String) -> Option<&TableInfo> {
        self.fields_by_table.get(&table_name)
    }

    pub fn get_table_names(&self) -> Vec<String> {
        self.fields_by_table.keys().cloned().collect::<Vec<_>>()
    }

    pub fn get_field_def(&self, table_name: String, field_name: String) -> Option<String> {
        self.fields_by_table
            .get(&table_name)
            .map(|t|{ 
                let x = t.fields();
                x.get_definition(field_name).cloned()
            }).flatten()
    }
    
    pub fn get_table_fields_data(&self, table_name: String) -> Option<Fields> {
        self.fields_by_table
            .get(&table_name)
            .map(|t| t.fields().clone())
    }
    
    pub fn get_table_field_names(&self, table_name: String) -> Vec<String> {
        self.fields_by_table
            .get(&table_name)
            .map(|t| t.fields().clone()).unwrap_or_default().get_names()
    }
    
    pub fn get_table_field_names_as_set(&self, table_name: String) -> HashSet<String> {
        self.fields_by_table
            .get(&table_name)
            .map(|t| t.fields().clone()).unwrap_or_default().get_names_as_set()
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
        Ok(info.into())
    }

    pub async fn get_table_info(&self, table_name: String) -> MigrationResult<TableInfo> {
        let info = info_for()
            .table(table_name)
            .get_data::<TableInfo>(self.db())
            .await?
            .unwrap();
        Ok(info.into())
    }

    pub async fn get_all_resources(&self) -> MigrationResult<FullDbInfo> {
        let top_level_resources = self.get_db_info().await?;
        let mut fields_by_table = HashMap::new();
        for table_name in top_level_resources.tables().get_names() {
            let table_info = self.get_table_info(table_name.clone()).await?;
            fields_by_table.insert(table_name, table_info);
        }
        let all_resources = FullDbInfo {
            all_resources: top_level_resources,
            fields_by_table,
        };
        Ok(all_resources)
        
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

    pub async fn run_codebase_schema_queries(&self) -> MigrationResult<()> {
        let queries = CodeBaseMeta::get_codebase_schema_queries();
        begin_transaction()
            .query(Raw::new(queries))
            .commit_transaction()
            .run(self.db())
            .await?;

        Ok(())
    }

    pub async fn run_local_dir_migrations(&self) -> MigrationResult<()> {
        let mut all_migrations = Migration::get_all_from_migrations_dir();
        let queries = all_migrations
            .into_iter()
            .map(|m| m.up)
            .collect::<Vec<_>>()
            .join("\n");

        // Run them as a transaction against a local in-memory database
        if !queries.trim().is_empty() {
            begin_transaction()
                .query(Raw::new(queries))
                .commit_transaction()
                .run(self.db())
                .await?;
        }
        Ok(())
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

    pub async fn run_migrations() -> MigrationResult<()> {
        println!("Running migrations");
        let mut up_queries = vec![];
        let mut down_queries: Vec<String> = vec![];
        // let mut diff_queries_to_add = vec![];
        //  DIFFING
        //  LEFT
        //
        // Left = migration directory
        // Right = codebase
        // ### TABLES
        // 1. Get all migrations from migration directory synced with db - Left
        let left_db = Self::init().await;
        let left = left_db.run_local_dir_migrations().await.expect("flops");
        let left_db_info = left_db.get_db_info().await.unwrap();
        let left_tables = left_db_info.tables();
        println!("left db info: {:#?}", left_db_info.tables());
        // let left_table_info = db.get_table_info("planet".into()).await.unwrap();
        //
        // 2. Get all migrations from codebase synced with db - Right
        let right_db = Self::init().await;
        let right = right_db.run_codebase_schema_queries().await?;
        let right_db_info = right_db.get_db_info().await.unwrap();
        let right_tables = right_db_info.tables();
        println!("right db info: {:#?}", right_db_info.tables());
        // let rightt_table_info = db.get_table_info("planet".into()).await.unwrap();
        let tables = ComparisonTables {
            left: left_tables,
            right: right_tables.clone(),
            left_resources: left_db.get_all_resources().await.expect("nothing for u on left"),
            right_resources: right_db.get_all_resources().await.expect("nothing for u on right"),
        }.get_queries();
        
        up_queries.extend(tables.up);
        down_queries.extend(tables.down);

        
        // for table_name in right_tables.get_names() {
        //     let left_table_info = left_db.get_table_info(table_name.clone()).await.expect("could not get left db table info");
        //     let right_table_info = right_db.get_table_info(table_name.clone()).await.expect("could not get right  db table info");
        //
        //     let queries = TableComparisonFields {
        //         left: left_table_info.fields(),
        //         right: right_table_info.fields(),
        //         right_table: table_name.clone(),
        //     }.get_queries();
        //
        //     up_queries.extend(queries.up);
        //     down_queries.extend(queries.down);
        // }



        // TODO: Create a warning to prompt user if they truly want to create empty migrations
        let up_queries_str = if up_queries.is_empty() {
            "".to_string()
        } else {
            format!("{};", up_queries.iter().map(|s|s.trim_end_matches(";")).collect::<Vec<_>>().join(";\n").trim().trim_end_matches(";"))
        };
        let down_queries_str = if down_queries.is_empty() {
            "".to_string()
        } else {
            format!("{};", down_queries.iter().map(|s|s.trim_end_matches(";")).collect::<Vec<_>>().join(";\n").trim().trim_end_matches(";"))
        };
        if up_queries_str.is_empty() && down_queries_str.is_empty() {
            println!("Are you sure you want to generate an empty migration? (y/n)");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "y" {
                Migration::create_migration_file(
                    up_queries_str,
                    Some(down_queries_str),
                    "test_migration".to_string(),
                );
            }
        } else {
            Migration::create_migration_file(
                up_queries_str,
                Some(down_queries_str),
                "test_migration".to_string(),
            );
        }
        //
        // For Fields
        //  a. If there is a Field in left that is not in right,
        //          (i) up => REMOVE FIELD
        //          (ii) down => ADD FIELD
        //  b. If there a Field in right that is not in left,
        //        (i) up => ADD FIELD
        //        (ii) down => REMOVE FIELD
        //  c. If there a Field in left and in right,
        //      (i) up => Use Right field definitions
        //      (ii) down => Use Left field definitions
        //  d. If there is a field name change,
        //    Get old and new names. Surrealdb does not support Alter statement
        //    (i) up =>
        //              DEFINE FIELD new_name on TABLE table_name;
        //              UPDATE table_name SET new_name = old_name;
        //              REMOVE old_name on TABLE table_name; or UPDATE table_name SET old_name = none;
        //    (ii) down =>
        //          DEFINE FIELD old_name on TABLE table_name;
        //          UPDATE table_name SET old_name = new_name;
        //          REMOVE new_name on TABLE table_name; or UPDATE table_name SET new_name = none;
        //
        //o
        //
        // 4. Aggregate all the new up and down queries
        // 5. Run the queries as a transaction
        // 6. Update the migration directory with the new migrations queries i.e m::create_migration_file(up, down, name);
        // 7. Mark the queries as registered i.e mark_migration_as_applied

        // Run the diff
        // 5. Update the migration directory
        //

        // Old rough implementation
        // let applied_migrations = db.get_applied_migrations_from_db();
        // let all_migrations = Self::get_all_from_migrations_dir();
        //
        // let applied_migrations = applied_migrations.await?;
        // for migration in all_migrations {
        //     if !applied_migrations.contains(&migration.name) {
        //         db.execute(migration.up);
        //         db.mark_migration_as_applied(migration.name);
        //     }
        // }
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
    pub timestamp: String,
    pub up: String,
    #[surreal_orm(type_ = "option<string>")]
    pub down: Option<String>,
    // status: String,
}

#[derive(Debug)]
pub enum Direction {
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
        // check if dir exists
        let migrations = fs::read_dir("migrations/");
        if migrations.is_err() {
            return vec![];
        }
            

        let mut migrations_meta = vec![];

        // get all migration names
        for migration in migrations.expect("Problem reading migrations directory") {
            let migration = migration.expect("Problem reading migration");
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
        migrations_meta.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
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
        up_query: String,
        down_query: Option<String>,
        name: impl Into<String> + std::fmt::Display,
    ) {
        //   # Migration file format would be migrations/<timestamp>-__<direction>__<name>.sql
        //   # Example: 2382320302303__up__create_planet.sql
        //   # Example: 2382320302303__down__delete_planet.sql
        // let timestamp = Utc::now().timestamp();
        println!("Creating migration file: {}", name);
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let _ = fs::create_dir_all("migrations").expect("Problem creating migrations directory");

        let create_path =
            |direction: Direction| format!("migrations/{}__{}__{}.sql", timestamp, direction, name);

        let up_file_path = create_path(Direction::Up);
        let mut up_file = File::create(&up_file_path).unwrap();
        up_file.write_all(up_query.as_bytes()).unwrap();

        println!("Up Migration file created at: {}", up_file_path);

        if let Some(down_query) = down_query {
            let down_file_path = create_path(Direction::Down);
            let mut down_file = File::create(&down_file_path).unwrap();
            down_file.write_all(down_query.as_bytes()).unwrap();
            println!("Down Migration file created at: {}", down_file_path);
        }

        println!("Migration file created: {}", name);
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Info(HashMap<String, String>);

trait Informational {
    // skills[*] is a valid field name in this context
    fn get_names(&self) -> Vec<String>;

    fn get_names_as_set(&self) -> HashSet<String>;

    fn get_all_definitions(&self) -> Vec<String>;

    // Althought, I dont think u should do this, it is absolutely possible:
    // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
    // Above can be achieved just doing array<string> on the top level field - skills
    // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
    fn get_definition(&self, name: String) -> Option<&String>;
}

impl Informational for Info {
    // skills[*] is a valid field name in this context
    fn get_names(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    fn get_names_as_set(&self) -> HashSet<String> {
        HashSet::from_iter(self.get_names())
    }

    fn get_all_definitions(&self) -> Vec<String> {
        self.0.values().cloned().collect()
    }

    // Althought, I dont think u should do this, it is absolutely possible:
    // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
    // Above can be achieved just doing array<string> on the top level field - skills
    // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
    fn get_definition(&self, name: String) -> Option<&String> {
        self.0.get(&name)
    }
}

macro_rules! define_object_info {
    ($($ident: ident),*) => {
        $(
            #[derive(Serialize, Deserialize, Clone, Debug, Default)]
            pub struct $ident(Info);

            // impl Deref for $ident {
            //     type Target = Info;
            //
            //     fn deref(&self) -> &Self::Target {
            //         &self.0
            //     }
            // }


            impl Informational for $ident {
                // skills[*] is a valid field name in this context
                fn get_names(&self) -> Vec<String> {
                    self.0.0.keys().cloned().collect()
                }

                fn get_names_as_set(&self) -> HashSet<String> {
                    HashSet::from_iter(self.get_names())
                }

                fn get_all_definitions(&self) -> Vec<String> {
                    self.0.0.values().cloned().collect()
                }

                // Althought, I dont think u should do this, it is absolutely possible:
                // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
                // Above can be achieved just doing array<string> on the top level field - skills
                // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
                fn get_definition(&self, name: String) -> Option<&String> {
                    self.0.0.get(&name)
                }
            }

        )*
    };
}

define_object_info!(
    Analyzers, Functions, Params, Scopes, Tables, Tokens, Users, Fields, Events, Indexes
);

#[async_trait::async_trait]
trait DbObject<T>
where
    T: Informational,
{
    // Left is from migration dir
    // Right is from codebase
    fn get_left(&self) -> T;
    fn get_right(&self) -> T;

    fn diff_left(&self) -> Vec<String> {
        self.get_left()
            .get_names_as_set()
            .difference(&self.get_right().get_names_as_set())
            .collect::<Vec<_>>().into_iter().map(ToOwned::to_owned).collect::<Vec<_>>()
    }
    fn diff_right(&self) -> Vec<String> {
        self.get_right()
            .get_names_as_set()
            .difference(&self.get_left().get_names_as_set())
            .collect::<Vec<_>>() .into_iter().map(ToOwned::to_owned).collect::<Vec<_>>()
    }

    fn diff_intersect(&self) -> Vec<String> {
        self.get_left()
            .get_names_as_set()
            .intersection(&self.get_right().get_names_as_set())
            .cloned()
            .collect::<Vec<_>>()
    }

    fn get_removal_query(&self, name: String) -> String;

    fn get_queries(&self) -> Queries {
        let mut up_queries = vec![];
        let mut down_queries = vec![];
        // 3. Diff them
        //
        // // For Tables (Can probably use same heuristics for events, indexes, analyzers, functions,
        // params, etc)
        // a. If there a Table in left that is not in right, left - right =
        // left.difference(right)
        // let left_diff = left_tables.clone();

        // DEAL WITH LEFT SIDE. i.e MIGRATION DIR
        //     (i) up => REMOVE TABLE table_name;
        up_queries.extend(
            self.diff_left()
                .iter()
                .map(|n| self.get_removal_query(n.to_string()))
                .collect::<Vec<_>>(),
        );

        //     (ii) down => DEFINE TABLE table_name; (Use migration directory definition)
        down_queries.extend(
            self.diff_left()
                .iter()
                .map(|t| self.get_left()
                    .get_definition(t.to_string())
                    .expect("Object must be present. This is a bug. Please, report it to surrealorm repository.").to_string())
                .collect::<Vec<_>>(),
        );

        // DEAL WITH RIGHT SIDE. i.e CODEBASE
        //    (i) up => DEFINE <OBJECT> table_name; (Use codebase definition)
        up_queries.extend(
            self.diff_right()
                .iter()
                .map(|t_name| self.get_right()
                    .get_definition(t_name.to_string())
                    .expect("Object must be present. This is a bug. Please, report it to surrealorm repository.").to_string())
                .collect::<Vec<_>>(),
        );

        //    (ii) down => REMOVE <OBJECT> table_name;
        down_queries.extend(
            self.diff_right()
                .iter()
                .map(|t_name| self.get_removal_query(t_name.to_string()))
                .collect::<Vec<_>>(),
        );

        // HANDLE INTERSECTION
        for object in self.diff_intersect() {
            let right_objects =self.get_right();
            let right_object_def = right_objects.get_definition(object.to_string());
            let left_objects = self.get_left();
            let left_object_def = left_objects.get_definition(object.to_string());
            // compare the two object definitions
            match (left_object_def, right_object_def) {
                //    First check if left def is same as right def
                (Some(l), Some(r)) if l == r => {
                    //          if same:
                    //                  do nothing
                    //          else:
                    // do nothing
                    println!("Object {} is the same in both left and right", object);
                }
                (Some(l), Some(r)) => {
                    println!("Object {} is different in both left and right. Use codebase as master/super", object);
                    println!("Left: {}", l);
                    println!("Right: {}", r);
                    
                    // (i) up => Use Right object definitions(codebase definition)
                    up_queries.push(r.to_string());
                    // (ii) down => Use Left object definitions(migration directory definition)
                    down_queries.push(l.to_string());
                }
                _ => {
                    panic!("This should never happen since it's an intersection and all table keys should have corresponding value definitions")
                }
            }
        }
        Queries {
            up: up_queries,
            down: down_queries,
        }

    }
}

struct Queries {
    up: Vec<String>,
    down: Vec<String>,
}

struct ComparisonTables {
    // Migrations latest state tables
    left: Tables,
    left_resources: FullDbInfo,
    // Codebase latest state tables
    right: Tables,
    right_resources: FullDbInfo,
}

impl DbObject<Tables> for ComparisonTables {
    fn get_left(&self) -> Tables {
        self.left.clone()
    }

    fn get_right(&self) -> Tables {
        self.right.clone()
    }

    fn get_removal_query(&self, name: String) -> String {
         remove_table(name.to_string()).to_raw().build()
    }


    fn get_queries(&self) -> Queries {
        let mut up_queries = vec![];
        let mut down_queries = vec![];

        // validate old_name in codebase. If it exists on any field but not on any field in
        // codebase
        // in migration directory, throw an error because it means, it must have already been
        // renamed or removed or first time migration is being created.
        for table in self.get_right().get_names() {
            let left_table_fields = self.left_resources.get_table_field_names(table.to_string());
            let right_table_fields = self.right_resources.get_table_field_names(table.to_string());
            for field in &right_table_fields {
                let field_with_old_name = CodeBaseMeta
                    ::find_field_has_old_name(table.to_string(), By::NewName(field.to_string()));    
                println!("Table: {} Field: {} field_with_old_name: {:#?}", table, field, field_with_old_name.clone());
                if let Some(field_with_old_name) = field_with_old_name {
                    println!("Inner = Table: {} Field: {} field_with_old_name: {:#?}", table.clone(), field.clone(), field_with_old_name.clone());
                    let old_name = field_with_old_name.old_name.unwrap();
                    if self.left_resources.get_field_def(table.to_string(), old_name.to_string()).is_none() {
                        panic!("'{old_name}' as old_name value on the '{table}' model struct/table \
                        is invalid. You are attempting to rename \
                        from {old_name} to {field} but {old_name} is not \
                            currently in use in migration/live database. Please, \
                            check that the field is not mispelled and is in the \
                            right case or use one of the currently available \
                            fields {}", left_table_fields.join(", "));
                    }
                }
            }
            println!("Table name: {}", table);
            let l = self.left_resources.get_field_def(table.to_string(), "name".to_string()); 
            let r = self.right_resources.get_field_def(table.to_string(), "name".to_string()); 
            println!("Left Field name: {l:?}");
            println!("Right Field name: {r:?}");
        }

        // 3. Diff them
        //
        // // For Tables (Can probably use same heuristics for events, indexes, analyzers, functions,
        // params, etc)
        // a. If there a Table in left that is not in right, left - right =
        // left.difference(right)
        // let left_diff = left_tables.clone();

        // DEAL WITH LEFT SIDE. i.e MIGRATION DIR
        //     (i) up => REMOVE TABLE table_name;
        //     We dont need to remove fields since removing a table removes all fields
        up_queries.extend(
            self.diff_left()
                .iter()
                .map(|t| self.get_removal_query(t.to_string()))
                .collect::<Vec<_>>(),
        );

        //     (ii) down => DEFINE TABLE table_name; (Use migration directory definition)
        //            We also use migration directory definition for fields for down reverse migration
        down_queries.extend(
            self.diff_left()
                .iter()
                .flat_map(|t| {
                   let left_table_def = self.get_left()
                    .get_definition(t.to_string())
                    .expect("Object must be present. This is a bug. Please, report it to surrealorm repository.").to_string();
                    
                    let table_info = self.left_resources.get_table_info(t.to_string());
                    let mut fields_defs = if let Some(table_info) = table_info {
                        let fields = table_info.fields();
                        fields.get_all_definitions()
                    } else {
                        println!("Table fields definitions {} not found in migrations state", t);
                        vec![]
                    };
                    fields_defs.insert(0, left_table_def);
                    fields_defs
                })
                .collect::<Vec<_>>()
        );

        // DEAL WITH RIGHT SIDE. i.e CODEBASE
        //    (i) up => DEFINE <OBJECT> table_name; (Use codebase definition)
        //        Since this tables would only exist in the codebase now and not in migration
        //        directory, we are using codebase field definitions for up migration
        up_queries.extend(
            self.diff_right()
                .iter()
                .flat_map(|t_name| {
                    let right_table_def = self.get_right()
                    .get_definition(t_name.to_string())
                    .expect("Object must be present. This is a bug. Please, report it to surrealorm repository.").to_string();

                    let table_info = self.right_resources.get_table_info(t_name.to_string());
                    let mut right_fields_defs = if let Some(table_info) = table_info {
                        let fields = table_info.fields();
                        fields.get_all_definitions()
                    } else {
                        println!("Table fields definitions {} not found in codebase state", t_name);
                        vec![]
                    };
                    right_fields_defs.insert(0, right_table_def);
                    right_fields_defs
                })
                .collect::<Vec<_>>()
        );

        //    (ii) down => REMOVE <OBJECT> table_name;
        //         We dont need to remove fields since removing a table removes all fields
        down_queries.extend(
            self.diff_right()
                .iter()
                .map(|t_name| self.get_removal_query(t_name.to_string()))
                .collect::<Vec<_>>(),
        );

        // HANDLE INTERSECTION
        for table in self.diff_intersect() {
            let right_objects =self.get_right();
            let right_object_def = right_objects.get_definition(table.to_string());
            let left_objects = self.get_left();
            let left_object_def = left_objects.get_definition(table.to_string());
            // compare the two object definitions
            match (left_object_def, right_object_def) {
                (Some(l), Some(r)) if l == r => {
                    // do nothing
                    println!("Object {} is the same in both left and right", table);
                }
                (Some(l), Some(r)) => {
                    println!("Object {} is different in both left and right. Use codebase as master/super", table);
                    println!("Left: {}", l);
                    println!("Right: {}", r);
                    
                    // (i) up => Use Right object definitions(codebase definition)
                    up_queries.push(r.to_string());
                    // (ii) down => Use Left object definitions(migration directory definition)
                    down_queries.push(l.to_string());
                }
                _ => {
                    panic!("This should never happen since it's an intersection and all table keys should have corresponding value definitions")
                }
            }

            // we have to diff left and right fields and prefer right if they are not same
            let left_table_info = self.left_resources.get_table_fields_data(table.to_string()).expect("Table must be present. This is a bug. Please, report it to surrealorm repository.");
            let right_table_info = self.right_resources.get_table_fields_data(table.to_string()).expect("Table must be present. This is a bug. Please, report it to surrealorm repository.");
            
            let left_fields = self.left_resources.get_table_field_names_as_set(table.to_string());
            let right_fields = self.right_resources.get_table_field_names_as_set(table.to_string());
            // add right field definition if left and right are different or left does not yet have
            // the field
            for fname in right_fields.union(&left_fields).collect::<Vec<_>>() {
                let left_field_def = left_table_info.get_definition(fname.to_string());
                let right_field_def = right_table_info.get_definition(fname.to_string());
                match (left_field_def.cloned(), right_field_def.cloned()) {
                    //    First check if left def is same as right def
                    (Some(ldef), Some(rdef)) if ldef.trim() == rdef.trim() => {
                        // do nothing
                        println!("Field {} is the same in both left and right", fname);
                    }
                    (ldef, rdef) => {
                        println!("Field {} is different in both left and right. Use codebase as master/super", fname);
                        println!("Right: {:?}", rdef);

                        let renamed_field_meta = CodeBaseMeta::find_field_has_old_name(table.to_string(), By::NewName(fname.to_string()));    
                        if let Some(rfm) = renamed_field_meta  {
                            let old_name = rfm.old_name.expect("Old name should be present here. If not, this is a bug and should be reported");
                            let new_name = rfm.name;

                            if let Some(rd) = rdef.clone() {
                                up_queries.push(rd.to_string());
                            }
                            if let Some(ld) = left_table_info.get_definition(old_name.to_string()) {
                                    // Set old name to new name
                                    up_queries.push(Raw::new(format!("UPDATE {table} SET {new_name} = {old_name}"))
                                        .to_raw()
                                        .build());
                                    up_queries.push(remove_field(old_name.to_string()).on_table(table.to_string()).to_raw().build());
                            
                                    down_queries.push(ld.to_string());
                                    down_queries.push(Raw::new(format!("UPDATE {table} SET {old_name} = {new_name}"))
                                        .to_raw()
                                        .build());
                                    down_queries.push(remove_field(new_name.to_string()).on_table(table.to_string()).to_raw().build());
                            }
                        } else {
                            // (ii) down => Use Left object definitions(migration directory definition)
                            if let Some(rd) = rdef.clone() {
                                up_queries.push(rd.to_string());
                                down_queries.push(remove_field(fname.to_string()).on_table(table.to_string()).to_raw().build());
                            }
                            
                            if let (Some(l), None) = (ldef, rdef) {
                                // This is an old name in the migration file not in the code
                                // base, but we want to be sure we've not already handled it
                                // earlier above if any field has it as an old name
                                let field_with_old_name = CodeBaseMeta::find_field_has_old_name(table.to_string(), By::OldName(fname.to_string()));    
                                if field_with_old_name.is_none(){
                                    up_queries.push(remove_field(fname.to_string()).on_table(table.to_string()).to_raw().build());
                                    down_queries.push(l.to_string());
                                }
                            };

                        }
                    }
                }
            }
        }
        Queries {
            up: up_queries,
            down: down_queries,
        }

    }
}

struct ComparisonEvents {
    // Migrations latest state events
    left: Events,
    // Codebase latest state events
    right: Events,
}

impl DbObject<Events> for ComparisonEvents {
    fn get_left(&self) -> Events {
        self.left.clone()
    }

    fn get_right(&self) -> Events {
        self.right.clone()
    }

    fn get_removal_query(&self, name: String) -> String {
         remove_table(name.to_string()).to_raw().build()
    }
}

trait Tabular {
    fn tabe_name(&self) -> String;
}

struct TableComparisonFields {
    // Migrations latest state tables
    left: Fields,
    // Codebase latest state tables
    right: Fields,
    right_table: String,
}

// impl Tabular for ComparisonFields {
//     fn table_name(&self) -> String {
//         "tables".to_string()
//     }
// }

impl DbObject<Fields> for TableComparisonFields {
    fn get_left(&self) -> Fields {
        self.left.clone()
    }

    fn get_right(&self) -> Fields {
        self.right.clone()
    }

    fn get_removal_query(&self, name: String) -> String {
                    remove_field(name.to_string())
                        .on_table(self.right_table.clone())
                        .to_raw()
                        .build()
    }


    fn get_queries(&self) -> Queries {
        let mut up_queries = vec![];
        let mut down_queries = vec![];
        // DEAL WITH RIGHT SIDE. i.e CODEBASE
        // b. If there a Table in right that is not in left, right - left =
        // right.difference(left)
        //    (i) up => ADD FIELD field_name on TABLE table_name;
        up_queries.extend(
            self.diff_right()
                .iter()
                .map(|f_name| {
                    self.get_right()
                        .get_definition(f_name.to_string())
                        .expect("Field must be present. This is a bug. Please, report it to surrealorm repository.")
                        .to_string()
                })
                .collect::<Vec<_>>(),
        );

        let diff_right = self.diff_right();
        let create = |direction: Direction| {
            diff_right
                .iter()
                .filter_map(|r_field| {
                    CodeBaseMeta::find_field_with_oldname_attr(
                        self.right_table.to_string(),
                        r_field.to_string(),
                    )
                })
                .map(|field_meta| {
                    let table = self.right_table.to_string();
                    let new_name = field_meta.name;
                    let old_field_name =  field_meta.old_name.expect("Old name should be present here. If not, this is a bug and should be reported");

                    let (left, right) = match direction {
                        Direction::Up => (new_name, old_field_name),
                        Direction::Down => (old_field_name, new_name),
                    };
                    
                    Raw::new(format!("UPDATE {table} SET {left} = {right}"))
                        .to_raw()
                        .build()
                })
                .collect::<Vec<_>>()
        };
        up_queries.extend(create(Direction::Up) );
        
        //    (ii) down => REMOVE FIELD field_name on TABLE table_name;
        down_queries.extend(
            diff_right
                .iter()
                .map(|f_name| {
                // TODO: Removal should be done only if table exists in migration dir
                    remove_field(f_name.to_string())
                        .on_table(self.right_table.clone())
                        .to_raw()
                        .build()
                })
                .collect::<Vec<_>>(),
        );

        // DEAL WITH LEFT SIDE. i.e MIGRATION DIR
        //     (i) up => REMOVE FIELD field_name on TABLE table_name;
        up_queries.extend(
            self.diff_left()
                .iter()
                .map(|f_name| {
                // TODO: Removal should be done only if table exists in codebase
                    remove_field(f_name.to_string())
                        .on_table(self.right_table.clone())
                        .to_raw()
                        .build()
                })
                .collect::<Vec<_>>(),
        );

        //     (ii) down => ADD FIELD field_name on TABLE table_name;
        down_queries.extend(
            self.diff_left()
                .iter()
                .map(|f_name| {
                    self.get_left()
                        .get_definition(f_name.to_string())
                        .expect("Field must be present. This is a bug. Please, report it to surrealorm repository.")
                        .to_string()
                })
                .collect::<Vec<_>>(),
        );
        down_queries.extend(create(Direction::Down) );

        // if field has attribute - old_name
        // old_name => left = true, right = false
        // new_name => left = false, right = true
        //
        // left  -> migration dir
        // up =>
        //      1. define_field_left(new_name) // use def from right i.e codebase
        //      2. UPDATE table_name SET new_name = old_name
        //      3. remove(old_name)
        //
        // down => define_field_left(old_name) i.e use old def from left i.e migration dir
        //      1. define_field_left(old_name) . Use old def from left i.e migration dir
        //      2. UPDATE table_name SET old_name = new_name
        //      3. remove(new_name)
        //
        // right -> codebase
        // up => define_field_right(new_name) i.e use new def from right i.e codebase
        // down => remove(new_name)

        for field in self.diff_intersect() {
            let right_tables = self.get_right();
            let left_tables = self.get_left();
            let right_field_def = right_tables.get_definition(field.to_string());
            let left_field_def = left_tables.get_definition(field.to_string());

            // compare the two field definitions
            match (left_field_def, right_field_def) {
                //    First check if left def is same as right def
                (Some(l), Some(r)) if l == r => {
                    //          if same:
                    //                  do nothing
                    //          else:
                    // do nothing
                    println!("Field {} is the same in both left and right", field);
                }
                (Some(l), Some(r)) => {
                    println!("Field {} is different in both left and right. Use codebase as master/super", field);
                    // (i) up => Use Right table definitions(codebase definition)
                    up_queries.push(r.to_string());
                    // (ii) down => Use Left table definitions(migration directory definition)
                    down_queries.push(l.to_string());
                }
                _ => {
                    panic!("This should never happen since it's an intersection and all table keys should have corresponding value definitions")
                }
            }
        }

        Queries {
            up: up_queries,
            down: down_queries,
        }
    }
}

// struct AllTableComparisonFields {
//     // Migrations latest state tables
//     left: Fields,
//     // Codebase latest state tables
//     right: Fields,
//     right_table: String,
// }



#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DbInfo {
    pub analyzers: Analyzers,
    pub functions: Functions,
    pub params: Params,
    pub scopes: Scopes,
    pub tables: Tables,
    pub tokens: Tokens,
    pub users: Users,
}

impl DbInfo {
    pub fn analyzers(&self) -> Analyzers {
        self.analyzers.clone()
    }

    pub fn functions(&self) -> Functions {
        self.functions.clone()
    }

    pub fn params(&self) -> Params {
        self.params.clone()
    }

    pub fn scopes(&self) -> Scopes {
        self.scopes.clone()
    }

    pub fn tables(&self) -> Tables {
        self.tables.clone()
    }

    pub fn tokens(&self) -> Tokens {
        self.tokens.clone()
    }

    pub fn users(&self) -> Users {
        self.users.clone()
    }
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "planet")]
pub struct Planet {
    // Test renaming tomorrow
    pub id: SurrealSimpleId<Self>,
    #[surreal_orm(old_name = "name")]
    pub first_name: String,
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

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "animal", schemafull)]
pub struct Animal {
    pub id: SurrealSimpleId<Self>,
    #[surreal_orm(old_name = "species")]
    pub name: String,
    pub attributes: Vec<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub terr: String,
}

#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "eats", schemafull)]
pub struct Eats<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: In,
    pub out: Out,
    pub place: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub type AnimalEatsCrop = Eats<Animal, Crop>;

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "crop", schemafull)]
pub struct Crop {
    pub id: SurrealSimpleId<Self>,
    pub color: String,
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

// pub fn extract_schema_from_models() -> TableInfo {
//     let x = Planet::schema();
//     TableInfo::default()
// }

// // Get all up migrations from migration directory
//
// Run all up migrations as a transaction in an in-memory db
//
// Parse all the table names available in all the queries into a HashSet
// Get all the table names from the in-memory db
//
// Get all the tables and corresponding field definitiins from teh codebase e.g Field(for now)
// Do a left and right diff to know which tables/fields to remove or add, or change
