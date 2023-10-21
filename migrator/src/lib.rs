// TODOs:
// Check within macro that:
// two fields do not have same old_name value
// old_name value is not same with any of the field names
// old name value is currently in migration directory/live db state, which means it has not yet
// been removed, therefore, still valid to be used as an annotation. The old_name attribute is
// meant to be used temporarily to help with migrations. Once the migration is done, the old_name
// attribute should be removed.
use async_trait::async_trait;
use chrono::Utc;
use inquire::InquireError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_until1, take_while1, take_while_m_n},
    character::{
        complete::{multispace0, multispace1},
        is_alphabetic,
    },
    combinator::opt,
    IResult,
};
use paste::paste;
use regex::Regex;
use serde::{Deserialize, Serialize};
use surreal_orm::{
    sql::{Query, Table},
    statements::{
        begin_transaction, create, create_only, define_event, define_index, delete, info_for,
        remove_analyzer, remove_database, remove_event, remove_field, remove_function,
        remove_index, remove_login, remove_namespace, remove_param, remove_scope, remove_table,
        remove_token, remove_user, select, select_value, update, NamespaceOrDatabase,
        UserPermissionScope,
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
// # Tables
//
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TableResources {
    events: Events,
    indexes: Indexes,
    tables: Tables,
    fields: Fields,
}

impl TableResources {
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
    pub fn find_field_with_oldname_attr(
        table_name: String,
        field_name: String,
    ) -> Option<FieldMetadata> {
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
                By::NewName(new_name) => {
                    f.name.to_string() == new_name.to_string() && f.old_name.is_some()
                }
                By::OldName(old_name) => {
                    f.old_name
                        .clone()
                        .filter(|n| n.to_string() == old_name.to_string())
                        .is_some()
                        && f.old_name.is_some()
                }
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

    pub fn get_codebase_schema_queries(db_resources: impl DbResources) -> String {
        let queries_joined = [
            db_resources.tokens(),
            db_resources.scopes(),
            db_resources.analyzers(),
            db_resources.params(),
            db_resources.functions(),
            db_resources.users(),
            db_resources.tables(),
        ]
        .iter()
        .flat_map(|res_raw| res_raw.iter().map(|r| r.to_raw().build()))
        .collect::<Vec<_>>()
        .join(";\n");
        // let queries_joined = format!("{};\n{}", tables, fields);

        queries_joined
    }
}

#[derive(Debug, Clone, Default)]
pub struct FullDbInfo {
    all_resources: DbInfo,
    table_resources: HashMap<Table, TableResources>,
}

impl FullDbInfo {
    pub fn analyzers(&self) -> Analyzers {
        self.all_resources.analyzers()
    }

    pub fn tables(&self) -> Tables {
        self.all_resources.tables()
    }

    pub fn params(&self) -> Params {
        self.all_resources.params()
    }

    pub fn scopes(&self) -> Scopes {
        self.all_resources.scopes()
    }

    pub fn functions(&self) -> Functions {
        self.all_resources.functions()
    }

    pub fn tokens(&self) -> Tokens {
        self.all_resources.tokens()
    }

    pub fn users(&self) -> Users {
        self.all_resources.users()
    }

    pub fn get_table_info(&self, table_name: Table) -> Option<&TableResources> {
        self.table_resources.get(&table_name)
    }

    pub fn get_table_names(&self) -> Vec<Table> {
        self.table_resources.keys().cloned().collect::<Vec<_>>()
    }

    pub fn get_field_def(
        &self,
        table_name: Table,
        field_name: Field,
    ) -> Option<DefineStatementRaw> {
        self.table_resources
            .get(&table_name)
            .map(|t| {
                let x = t.fields();
                x.get_definition(&field_name.to_string()).cloned()
            })
            .flatten()
    }

    pub fn get_table_indexes(&self, table_name: Table) -> Option<Indexes> {
        self.table_resources
            .get(&table_name)
            .map(|t| t.indexes().clone())
    }

    pub fn get_table_events(&self, table_name: Table) -> Option<Events> {
        self.table_resources
            .get(&table_name)
            .map(|t| t.events().clone())
    }

    pub fn get_table_fields(&self, table_name: Table) -> Option<Fields> {
        self.table_resources
            .get(&table_name)
            .map(|t| t.fields().clone())
    }

    pub fn get_table_field_names(&self, table_name: Table) -> Vec<String> {
        self.table_resources
            .get(&table_name)
            .map(|t| t.fields().clone())
            .unwrap_or_default()
            .get_names()
    }

    pub fn get_table_field_names_as_set(&self, table_name: Table) -> HashSet<String> {
        self.table_resources
            .get(&table_name)
            .map(|t| t.fields().clone())
            .unwrap_or_default()
            .get_names_as_set()
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

    pub async fn get_table_info(&self, table_name: String) -> MigrationResult<TableResources> {
        let info = info_for()
            .table(table_name)
            .get_data::<TableResources>(self.db())
            .await?
            .unwrap();
        Ok(info.into())
    }

    pub async fn get_all_resources(&self) -> MigrationResult<FullDbInfo> {
        let top_level_resources = self.get_db_info().await?;
        let mut fields_by_table = HashMap::new();
        for table_name in top_level_resources.tables().get_names() {
            let table_info = self.get_table_info(table_name.clone()).await?;
            fields_by_table.insert(table_name.into(), table_info);
        }
        let all_resources = FullDbInfo {
            all_resources: top_level_resources,
            table_resources: fields_by_table,
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
        let queries = CodeBaseMeta::get_codebase_schema_queries(Resources);
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
        let init = ComparisonsInit {
            left_resources: &left_db
                .get_all_resources()
                .await
                .expect("nothing for u on left"),
            right_resources: &right_db
                .get_all_resources()
                .await
                .expect("nothing for u on right"),
        };
        let tables = init.new_tables().queries();
        let analyzers = init.new_analyzers().queries();
        let params = init.new_params().queries();
        let functions = init.new_functions().queries();
        let scopes = init.new_scopes().queries();
        let tokens = init.new_tokens().queries();
        let users = init.new_users().queries();

        let resources = vec![tables, analyzers, params, functions, scopes, tokens, users];

        for resource in resources {
            up_queries.extend(resource.up);
            down_queries.extend(resource.down);
        }

        // TODO: Create a warning to prompt user if they truly want to create empty migrations
        let up_queries_str = if up_queries.is_empty() {
            "".to_string()
        } else {
            format!(
                "{};",
                up_queries
                    .iter()
                    .map(|s| s.trim_end_matches(";"))
                    .collect::<Vec<_>>()
                    .join(";\n")
                    .trim()
                    .trim_end_matches(";")
            )
        };
        let down_queries_str = if down_queries.is_empty() {
            "".to_string()
        } else {
            format!(
                "{};",
                down_queries
                    .iter()
                    .map(|s| s.trim_end_matches(";"))
                    .collect::<Vec<_>>()
                    .join(";\n")
                    .trim()
                    .trim_end_matches(";")
            )
        };
        if up_queries_str.is_empty() && down_queries_str.is_empty() {
            let confirmation = inquire::Confirm::new(
                "Are you sure you want to generate an empty migration? (y/n)",
            )
            .with_default(false)
            .with_help_message("This is good if you want to write out some queries manually")
            .prompt();

            match confirmation {
                Ok(true) => {
                    println!("UP MIGRATIOM: \n {}", up_queries_str);
                    println!("DOWN MIGRATIOM: \n {}", down_queries_str);
                    Migration::create_migration_file(
                        up_queries_str,
                        Some(down_queries_str),
                        "test_migration".to_string(),
                    );
                }
                Ok(false) => {
                    println!("No migration created");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            };
        } else {
            println!("HERE=====");
            println!("UP MIGRATIOM: \n {}", up_queries_str);
            println!("DOWN MIGRATIOM: \n {}", down_queries_str);
            // Migration::create_migration_file(
            //     up_queries_str,
            //     Some(down_queries_str),
            //     "test_migration".to_string(),
            // );
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
struct Info(HashMap<String, DefineStatementRaw>);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct DefineStatementRaw(String);

struct RemoveStatementRaw(String);
impl Display for RemoveStatementRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

struct FieldName(String);

struct DefineStmtName(String);

impl Display for DefineStmtName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T: Into<String>> From<T> for DefineStmtName {
    fn from(value: T) -> Self {
        let str: String = value.into();
        Self(str)
    }
}

struct RemoveStmtName(String);

impl From<String> for RemoveStatementRaw {
    fn from(value: String) -> Self {
        Self(value)
    }
}

struct TableRaw(String);

impl DefineStatementRaw {
    /// Table name is only required/necessary when generating table resources such as fields, indexes, events
    pub fn generate_remove_stmt(
        &self,
        define_statement_name: DefineStmtName,
        table: Option<Table>,
    ) -> RemoveStatementRaw {
        use surreal_orm::sql::{self, statements::DefineStatement, Base, Statement};
        let query = surreal_orm::sql::parse(&self.to_string()).expect("Invalid statment");
        let stmt = query[0].clone();
        let get_error = |resource_name: String| {
            if resource_name != define_statement_name.to_string() {
                panic!("Resource name - {} - in define statement does not match name - {} - in removal statement", resource_name, define_statement_name);
            }
        };
        let stmt = match stmt {
            Statement::Define(define_stmt) => {
                match define_stmt {
                    DefineStatement::Namespace(ns) => {
                        get_error(ns.name.to_raw());
                        remove_namespace(define_statement_name.to_string())
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Database(db) => {
                        get_error(db.name.to_raw());
                        remove_database(define_statement_name.to_string())
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Function(fn_) => {
                        get_error(fn_.name.to_raw());
                        remove_function(define_statement_name.to_string())
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Analyzer(analyzer) => {
                        get_error(analyzer.name.to_raw());
                        remove_analyzer(define_statement_name.to_string())
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Token(tk) => {
                        get_error(tk.name.to_raw());

                        let remove_init = remove_token(define_statement_name.to_string());
                        let remove_stmt = match tk.base {
                            Base::Ns => remove_init.on_namespace(),
                            Base::Db => remove_init.on_database(),
                            Base::Root => remove_init.on_database(),
                            Base::Sc(sc_name) => remove_init.on_scope(sc_name.to_raw()),
                        };
                        remove_stmt.to_raw().build()
                    }
                    DefineStatement::Scope(sc) => {
                        get_error(sc.name.to_raw());
                        remove_scope(define_statement_name.to_string())
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Param(_) => {
                        get_error(define_statement_name.to_string());
                        remove_param(define_statement_name.to_string())
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Table(table) => {
                        get_error(table.name.to_raw());
                        remove_table(define_statement_name.to_string())
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Event(ev) => {
                        get_error(ev.name.to_raw());
                        remove_event(define_statement_name.to_string())
                            .on_table(
                                table.expect("Invalid event. Event must be attached to a table."),
                            )
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Field(field) => {
                        get_error(field.name.to_string());
                        remove_field(define_statement_name.to_string())
                            .on_table(
                                table.expect("Invalid field. Field must be attached to a table."),
                            )
                            .to_raw()
                            .build()
                    }
                    DefineStatement::Index(index) => {
                        get_error(index.name.to_string());
                        remove_index(define_statement_name.to_string())
                            .on_table(
                                table.expect("Invalid index. Index must be attached to a table."),
                            )
                            .to_raw()
                            .build()
                    }
                    DefineStatement::User(user) => {
                        get_error(user.name.to_raw());
                        let remove_init = remove_user(define_statement_name.to_string());
                        let remove_stmt = match user.base {
                            Base::Ns => remove_init.on_namespace(),
                            Base::Db => remove_init.on_database(),
                            Base::Root => remove_init.on_database(),
                            Base::Sc(sc_name) => panic!("Users cannot be defined on scope"),
                        };
                        remove_stmt.to_raw().build()
                    }
                    DefineStatement::MlModel(ml) => {
                        // 	TODO: Implement define ml model statmement
                        // 	write!(f, "DEFINE MODEL ml::{}<{}>", self.name, self.version)?;
                        // 		write!(f, "PERMISSIONS {}", self.permissions)?;
                        // get_error(ml.name.to_raw());
                        // remove_ml_model(name.to_string()).to_raw().build()
                        todo!()
                    }
                }
            }
            _ => panic!("Not a define statement. Expected a define statement"),
        };

        stmt.into()
    }

    pub fn trim(&self) -> &str {
        self.0.trim()
    }
}

impl Display for DefineStatementRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.0)
    }
}

trait Informational {
    // skills[*] is a valid field name in this context
    fn get_names(&self) -> Vec<String>;

    fn get_names_as_set(&self) -> HashSet<String>;

    fn get_all_definitions(&self) -> Vec<DefineStatementRaw>;

    // Althought, I dont think u should do this, it is absolutely possible:
    // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
    // Above can be achieved just doing array<string> on the top level field - skills
    // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
    fn get_definition(&self, name: &String) -> Option<&DefineStatementRaw>;
}

impl Informational for Info {
    // skills[*] is a valid field name in this context
    fn get_names(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    fn get_names_as_set(&self) -> HashSet<String> {
        HashSet::from_iter(self.get_names())
    }

    fn get_all_definitions(&self) -> Vec<DefineStatementRaw> {
        self.0.values().cloned().collect()
    }

    // Althought, I dont think u should do this, it is absolutely possible:
    // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
    // Above can be achieved just doing array<string> on the top level field - skills
    // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
    fn get_definition(&self, name: &String) -> Option<&DefineStatementRaw> {
        self.0.get(name)
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

                fn get_all_definitions(&self) -> Vec<DefineStatementRaw> {
                    self.0.0.values().cloned().collect()
                }

                // Althought, I dont think u should do this, it is absolutely possible:
                // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
                // Above can be achieved just doing array<string> on the top level field - skills
                // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
                fn get_definition(&self, name: &String) -> Option<&DefineStatementRaw> {
                    self.0.0.get(name)
                }
            }

        )*
    };
}

define_object_info!(
    Analyzers, Functions, Params, Scopes, Tables, Tokens, Users, Fields, Events, Indexes
);

struct ComparisonParam<'a> {
    resources: &'a ComparisonsInit<'a>,
}

impl<'a> DbResourcesMeta<Params> for ComparisonParam<'a> {
    fn get_left(&self) -> Params {
        self.resources.left_resources.params()
    }

    fn get_right(&self) -> Params {
        self.resources.right_resources.params()
    }
}

macro_rules! define_top_level_resource {
    ($($resource_name:ident, $resource_type:ident);*) => {
        paste::paste! {
            $(
                struct [<Comparison $resource_type>]<'a> {
                    resources: ComparisonsInit<'a>
                }

                impl<'a> DbResourcesMeta<[<$resource_type>]> for [<Comparison $resource_type>] <'a>{
                    fn get_left(&self) -> [<$resource_type>] {
                        self.resources.left_resources.[<$resource_name>]()
                    }

                    fn get_right(&self) -> [<$resource_type>] {
                        self.resources.right_resources.[<$resource_name>]()
                    }
                }
            )*
        }
    };
}

define_top_level_resource!(
    tables, Tables;
    analyzers, Analyzers;
    functions, Functions;
    params, Params;
    scopes, Scopes;
    tokens, Tokens;
    users, Users
);

trait DbResourcesMeta<T>
where
    T: Informational,
{
    // Left is from migration dir
    fn get_left(&self) -> T;
    // Right is from codebase
    fn get_right(&self) -> T;

    fn queries(&self) -> Queries {
        // Update of resource in codebase i.e Present in both migration directory and codebase but
        // changed
        // [a0, b0, c]  = [a0, b1, d] => [a, b, c, d] =>
        //Table -> [(update, removal), }(update, removal), (removal, update), (creation/definition, removal)]
        // SubResource e.g events, indexes and fields
        // -> [(no_change, no_change), }(change, change), (removal, prev_left_def), (creation/definition, removal)]
        // -> [(Empty, Empty), }(right_def, left_def), (removal, prev_left_def), (right_def, removal)]
        let queries = self
            .get_right()
            .get_names_as_set()
            .union(&self.get_left().get_names_as_set())
            .into_iter()
            .fold(Queries::default(), |mut acc, name| {
                let def_right = self.get_right().get_definition(name).cloned();
                let def_left = self.get_left().get_definition(name).cloned();

                match (def_left, def_right) {
                    (None, Some(r)) => {
                        acc.add_up(QueryType::Define(r));
                        acc.add_down(QueryType::Remove(r.generate_remove_stmt(name.into(), None)));
                    }
                    (Some(l), None) => {
                        acc.add_up(QueryType::Remove(l.generate_remove_stmt(name.into(), None)));
                        acc.add_down(QueryType::Define(l));
                    }
                    (Some(l), Some(r)) => {
                        if l.trim() != r.trim() {
                            acc.add_up(QueryType::Define(r));
                            acc.add_down(QueryType::Define(l));
                        }
                    }
                    // This should never happen cos we're getting a union of left and right
                    (None, None) => unreachable!(),
                };

                acc
            });
        queries
    }
}

struct ComparisonEvents<'a> {
    table: String,
    resources: &'a ComparisonsInit<'a>,
}

impl<'a> TableResourcesMeta<Events> for ComparisonEvents<'a> {
    fn get_left(&self) -> Events {
        self.resources
            .left_resources
            .get_table_events(self.get_table())
            .unwrap_or_default()
    }

    fn get_right(&self) -> Events {
        self.resources
            .right_resources
            .get_table_events(self.get_table())
            .unwrap_or_default()
    }

    fn get_table(&self) -> String {
        self.table.clone()
    }
}

trait TableResourcesMeta<T>
where
    T: Informational,
{
    // Left is from migration dir
    fn get_left(&self) -> T;
    // Right is from codebase
    fn get_right(&self) -> T;

    fn get_table(&self) -> String;

    fn queries(&self) -> Queries {
        // Update of resource in codebase i.e Present in both migration directory and codebase but
        // changed
        // [a0, b0, c]  = [a0, b1, d] => [a, b, c, d] =>
        //Table -> [(update, removal), }(update, removal), (removal, update), (creation/definition, removal)]
        // SubResource e.g events, indexes and fields
        // -> [(no_change, no_change), }(change, change), (removal, prev_left_def), (creation/definition, removal)]
        // -> [(Empty, Empty), }(right_def, left_def), (removal, prev_left_def), (right_def, removal)]
        // let mut queries = Queries::default();
        let queries = self
            .get_right()
            .get_names_as_set()
            .union(&self.get_left().get_names_as_set())
            .into_iter()
            .fold(Queries::default(), |mut acc, name| {
                let def_right = self.get_right().get_definition(name).cloned();
                let def_left = self.get_left().get_definition(name).cloned();

                match (def_left, def_right) {
                    (None, Some(r)) => {
                        let down = generate_removal_statement(
                            &r,
                            name.to_string(),
                            Some(self.get_table()),
                        );
                        acc.add_up(r);
                        acc.add_down(down);
                    }
                    (Some(l), None) => {
                        let up = generate_removal_statement(
                            &l,
                            name.to_string(),
                            Some(self.get_table()),
                        );
                        acc.add_up(up);
                        acc.add_down(l);
                    }
                    (Some(l), Some(r)) => {
                        if l.trim() != r.trim() {
                            acc.add_up(r);
                            acc.add_down(l);
                        }
                    }
                    (None, None) => unreachable!(),
                };

                acc
            });
        queries
    }
}

// Update of resource in codebase i.e Present in both migration directory and codebase but
// changed
// [a0, b0, c]  = [a0, b1, d] => [a, b, c, d] =>
//Table -> [(update, removal), }(update, removal), (removal, update), (creation/definition, removal)]
// SubResource e.g events, indexes and fields
// -> [(no_change, no_change), }(change, change), (removal, prev_left_def), (creation/definition, removal)]
// -> [(Empty, Empty), }(right_def, left_def), (removal, prev_left_def), (right_def, removal)]

// changes
// Direction or in terms of change type

enum ChangeType {}

pub struct RightDef(String);
pub struct RightNone;
pub struct LeftDef(String);
pub struct RemoveRightDef;
pub struct RemoveLeftDef;

pub enum DiffChange {
    NoChange,
    Update {
        // up: RightDef,
        // down: LeftDef
        up: String,
        down: String,
    },
    Removal {
        // up: RemoveLeftDef, // gen_removal_statement(LeftDef)
        // down: LeftDef
        up: String,
        down: String,
    },
    Creation {
        // up: RightDef,
        // down: RemoveRightDef
        up: String,
        down: String,
    },
}

pub struct Nothing;
pub struct Something(String);

pub enum Changes2 {
    NoChange,
    Update { left: Something, right: Something },
    Removal { left: Something, right: Nothing },
    Creation { left: Nothing, right: Something },
}

#[derive(Debug, Default)]
struct BiQueries {
    definitions: Vec<String>,
    removals: Vec<String>,
}

impl BiQueries {
    fn new() -> Self {
        Self {
            definitions: vec![],
            removals: vec![],
        }
    }

    fn add_def(&mut self, query: String) {
        self.definitions.push(query);
    }

    fn add_removal(&mut self, query: String) {
        self.removals.push(query);
    }
}

#[derive(Debug, Default)]
struct Queries {
    up: Vec<String>,
    down: Vec<String>,
}

enum QueryType {
    Define(DefineStatementRaw),
    Remove(RemoveStatementRaw),
}

impl Queries {
    fn new() -> Self {
        Self {
            up: vec![],
            down: vec![],
        }
    }

    fn add_up(&mut self, query: QueryType) {
        self.up.push(match query {
            QueryType::Define(def) => def.to_string(),
            QueryType::Remove(rem) => rem.to_string(),
        });
    }

    fn add_down(&mut self, query: QueryType) {
        self.down.push(match query {
            QueryType::Define(def) => def.to_string(),
            QueryType::Remove(rem) => rem.to_string(),
        });
    }
}

#[derive(Debug, Clone)]
struct ComparisonsInit<'a> {
    // Migrations latest state tables
    left_resources: &'a FullDbInfo,
    // Codebase latest state tables
    right_resources: &'a FullDbInfo,
}

impl<'a> ComparisonsInit<'a> {
    pub fn new_fields(&self, table: String) -> ComparisonFields {
        ComparisonFields {
            table: table.to_string(),
            resources: self,
        }
    }
    pub fn new_indexes(&self, table: String) -> ComparisonIndexes {
        ComparisonIndexes {
            table: table.to_string(),
            resources: self,
        }
    }

    pub fn new_events(&self, table: String) -> ComparisonEvents {
        ComparisonEvents {
            table: table.to_string(),
            resources: self,
        }
    }

    pub fn new_tables(&self) -> ComparisonTables {
        ComparisonTables { resources: self }
    }

    pub fn new_analyzers(&self) -> ComparisonAnalyzers {
        ComparisonAnalyzers { resources: self }
    }

    pub fn new_params(&self) -> ComparisonAnalyzers {
        ComparisonAnalyzers { resources: self }
    }

    pub fn new_scopes(&self) -> ComparisonScopes {
        ComparisonScopes { resources: self }
    }

    pub fn new_tokens(&self) -> ComparisonTokens {
        ComparisonTokens { resources: self }
    }

    pub fn new_users(&self) -> ComparisonUsers {
        ComparisonUsers { resources: self }
    }

    pub fn new_functions(&self) -> ComparisonFunctions {
        ComparisonFunctions { resources: self }
    }
}

struct ComparisonFields<'a> {
    table: String,
    resources: &'a ComparisonsInit<'a>,
}

impl<'a> TableResourcesMeta<Fields> for ComparisonFields<'a> {
    fn get_left(&self) -> Fields {
        self.resources
            .left_resources
            .get_table_fields(self.get_table())
            .unwrap_or_default()
    }

    fn get_right(&self) -> Fields {
        self.resources
            .right_resources
            .get_table_fields(self.get_table())
            .unwrap_or_default()
    }

    fn get_table(&self) -> String {
        self.table.clone()
    }

    fn queries(&self) -> Queries {
        // Update of resource in codebase i.e Present in both migration directory and codebase but
        // changed
        // [a0, b0, c]  = [a0, b1, d] => [a, b, c, d] =>
        //Table -> [(update, removal), }(update, removal), (removal, update), (creation/definition, removal)]
        // SubResource e.g events, indexes and fields
        // -> [(no_change, no_change), }(change, change), (removal, prev_left_def), (creation/definition, removal)]
        // -> [(Empty, Empty), }(right_def, left_def), (removal, prev_left_def), (right_def, removal)]
        // let mut queries = Queries::default();
        let queries = self
            .get_right()
            .get_names_as_set()
            .union(&self.get_left().get_names_as_set())
            .into_iter()
            .fold(Queries::default(), |mut acc, name| {
                let def_right = self.get_right().get_definition(name).cloned();
                let def_left = self.get_left().get_definition(name).cloned();

                let has_old_name = CodeBaseMeta::find_field_has_old_name(
                    self.get_table(),
                    By::NewName(name.to_string()),
                );

                match (def_left, def_right) {
                    (None, Some(r)) => {
                        let down = generate_removal_statement(
                            &r,
                            name.to_string(),
                            Some(self.get_table()),
                        );

                        acc.add_up(r);
                        if let Some(has_old_name) = has_old_name {
                            let old_name = has_old_name.old_name.clone().unwrap();
                            let copy_old_to_new = Raw::new(format!(
                                "UPDATE {table} SET {new_name} = {old_name}",
                                table = self.get_table(),
                                old_name = old_name,
                                new_name = name
                            ));

                            let up = generate_removal_statement(
                                &r,
                                old_name.to_string(),
                                Some(self.get_table()),
                            );
                            acc.add_up(copy_old_to_new.build());
                            acc.add_up(up);
                        }

                        acc.add_down(down);
                    }
                    (Some(l), None) => {
                        let up = generate_removal_statement(
                            &l,
                            name.to_string(),
                            Some(self.get_table()),
                        );
                        acc.add_up(up);
                        acc.add_down(l);
                    }
                    (Some(l), Some(r)) => {
                        if l.trim() != r.trim() {
                            acc.add_up(r);
                            acc.add_down(l);
                        }
                    }
                    (None, None) => unreachable!(),
                };

                acc
            });
        queries
    }
}

impl ComparisonFields {
    fn rename_field(rename_opts: FieldRenameOptions) -> Queries {
        let FieldRenameOptions {
            table,
            old_name,
            new_name,
            left_definition,
        } = rename_opts;
        let mut up_queries = vec![];
        let mut down_queries = vec![];
        up_queries.push(
            Raw::new(format!("UPDATE {table} SET {new_name} = {old_name}"))
                .to_raw()
                .build(),
        );
        up_queries.push(
            remove_field(old_name.to_string())
                .on_table(table.to_string())
                .to_raw()
                .build(),
        );

        down_queries.push(left_definition.to_string());
        down_queries.push(
            Raw::new(format!("UPDATE {table} SET {old_name} = {new_name}"))
                .to_raw()
                .build(),
        );
        down_queries.push(
            remove_field(new_name.to_string())
                .on_table(table.to_string())
                .to_raw()
                .build(),
        );

        Queries {
            up: up_queries,
            down: down_queries,
        }
    }

    fn validate_field_rename(&self) {
        let table = self.table.clone();
        let left_table_fields = self
            .left_resources()
            .get_table_field_names(table.to_string());
        let right_table_fields = self
            .right_resources()
            .get_table_field_names(table.to_string());
        for field in &right_table_fields {
            let field_with_old_name = CodeBaseMeta::find_field_has_old_name(
                table.to_string(),
                By::NewName(field.to_string()),
            );

            if let Some(field_with_old_name) = field_with_old_name {
                let old_name = field_with_old_name.old_name.clone().unwrap();
                if self
                    .left_resources()
                    .get_field_def(table.to_string(), field.to_string())
                    .is_some()
                {
                    panic!("Cannot rename '{old_name}' to '{field}' on table '{table}'. '{field}' field on '{table}' table is already in use in migration/live db. \
                        Use a different name");
                }
                if right_table_fields.contains(&old_name.to_string()) {
                    panic!(
                        "Invalid value '{old_name}' on struct' {table}'. '{old_name}' \
                            is currently used as a field on the struct. Please, use a different \
                            value for the old_name on field '{field}'"
                    );
                }

                if self
                    .left_resources()
                    .get_field_def(table.to_string(), old_name.to_string())
                    .is_none()
                {
                    panic!(
                        "'{old_name}' as old_name value on the '{table}' model struct/table \
                        is invalid. You are attempting to rename \
                        from {old_name} to {field} but {old_name} is not \
                            currently in use in migration/live database. Please, \
                            check that the field is not mispelled and is in the \
                            right case or use one of the currently available \
                            fields {}",
                        left_table_fields.join(", ")
                    );
                }
            }
        }
    }
}

struct FieldRenameOptions<'a> {
    table: &'a String,
    old_name: String,
    new_name: String,
    left_definition: &'a String,
}

struct Change {
    table: String,
    old_name: String,
    new_name: String,
}

enum SingleFieldChangeType<'a> {
    // Delete { old_name: String, new_name: String },
    // Rename { old_name: String, new_name: String },
    Delete(&'a Change),
    Rename(&'a Change),
}

impl<'a> Display for SingleFieldChangeType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleFieldChangeType::Delete(change) => write!(
                f,
                "Delete old field '{}' and create a new one '{}' on table '{}'",
                change.old_name, change.new_name, change.table
            ),
            SingleFieldChangeType::Rename(change) => write!(
                f,
                "Rename old field '{}' to new field '{}' on table '{}'",
                change.old_name, change.new_name, change.table
            ),
        }
    }
}

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
#[surreal_orm(table_name = "planet", schemafull)]
pub struct Planet {
    // Test renaming tomorrow
    pub id: SurrealSimpleId<Self>,
    #[surreal_orm(old_name = "name")]
    pub first_name: String,
    pub population: u64,
    pub created: chrono::DateTime<Utc>,
    pub tags: Vec<String>,
}

impl TableEvents for Planet {}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "student", schemafull)]
pub struct Student {
    pub id: SurrealSimpleId<Self>,
    pub school: String,
    pub age: u8,
    pub class: String,
}

impl TableEvents for Student {}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "animal", schemafull)]
pub struct Animal {
    pub id: SurrealSimpleId<Self>,
    pub species: String,
    // Improve error essage for old_nmae using word similarity algo
    #[surreal_orm(old_name = "attributes")]
    pub characteristics: Vec<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub err: String,
    pub speed: u64,
}

impl TableEvents for Animal {
    fn events() -> Vec<Raw> {
        let animal::Schema { species, speed, .. } = Self::schema();

        let event1 = define_event("event1".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Erectus").and(speed.gt(545))))
            .then(select(All).from(Crop::table_name()))
            .to_raw();

        let event2 = define_event("event2".to_string())
            .on_table("animal".to_string())
            .when(cond(species.eq("Homo Sapien").and(speed.lt(10))))
            .then(select(All).from(AnimalEatsCrop::table_name()))
            .to_raw();
        vec![event1, event2]
    }

    fn indexes() -> Vec<Raw> {
        let animal::Schema { species, speed, .. } = Self::schema();

        let idx1 = define_index("species_speed_idx".to_string())
            .on_table(Self::table_name())
            .fields(arr![species, speed])
            .unique()
            .to_raw();

        vec![idx1]
    }
}

trait TableEvents
where
    Self: Model,
{
    fn events() -> Vec<Raw> {
        vec![]
    }

    fn indexes() -> Vec<Raw> {
        vec![]
    }

    fn fields() -> Vec<Raw> {
        Self::define_fields()
    }

    fn table() -> Raw {
        Self::define_table()
    }
}

macro_rules! create_table_defs {
    ($($struct_table: ident),*) => {
        ::std::vec![
            $(
                ::std::vec![<$struct_table as TableEvents>::table()],
                <$struct_table as TableEvents>::fields(),
                <$struct_table as TableEvents>::indexes(),
                <$struct_table as TableEvents>::events(),
            )*
        ].into_iter().flatten().collect::<::std::vec::Vec<::surreal_orm::Raw>>()
    };
}

trait DbResources {
    fn tables(&self) -> Vec<Raw> {
        vec![]
    }

    fn analyzers(&self) -> Vec<Raw> {
        vec![]
    }

    fn functions(&self) -> Vec<Raw> {
        vec![]
    }

    fn params(&self) -> Vec<Raw> {
        vec![]
    }

    fn scopes(&self) -> Vec<Raw> {
        vec![]
    }

    fn tokens(&self) -> Vec<Raw> {
        vec![]
    }

    fn users(&self) -> Vec<Raw> {
        vec![]
    }
}

struct Resources;

impl DbResources for Resources {
    fn tables(&self) -> Vec<Raw> {
        create_table_defs!(Animal, Crop, AnimalEatsCrop, Student, Planet)
    }
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
impl TableEvents for AnimalEatsCrop {}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "crop", schemafull)]
pub struct Crop {
    pub id: SurrealSimpleId<Self>,
    pub color: String,
}

impl TableEvents for Crop {}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_remove_statement_generation_for_define_user_on_namespace() {
        let stmt = generate_removal_statement(
            &"DEFINE USER Oyelowo ON NAMESPACE PASSWORD 'mapleleaf' ROLES OWNER".to_string(),
            "Oyelowo".into(),
            None,
        );
        assert_eq!(stmt, "REMOVE USER Oyelowo ON NAMESPACE".to_string());
    }

    fn test_remove_statement_generation_for_define_user_on_database() {
        let stmt = generate_removal_statement(
            &"DEFINE USER Oyelowo ON DATABASE PASSWORD 'mapleleaf' ROLES OWNER".to_string(),
            "Oyelowo".into(),
            None,
        );
        assert_eq!(stmt, "REMOVE USER Oyelowo ON DATABASE".to_string());
    }

    //         let xx = "
    // -- Set the name of the token
    // DEFINE TOKEN token_name
    //   -- Use this OAuth provider for scope authorization
    //   ON NAMESPACE
    //   -- Specify the cryptographic signature algorithm used to sign the token
    //   TYPE HS512
    //   -- Specify the public key so we can verify the authenticity of the token
    //   VALUE 'sNSYneezcr8kqphfOC6NwwraUHJCVAt0XjsRSNmssBaBRh3WyMa9TRfq8ST7fsU2H2kGiOpU4GbAF1bCiXmM1b3JGgleBzz7rsrz6VvYEM4q3CLkcO8CMBIlhwhzWmy8'
    // ;
    // ".into();
    //     let stm = generate_removal_statement2(xx, "token_name".into(), None);
}
