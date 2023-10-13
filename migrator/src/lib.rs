use async_trait::async_trait;
use chrono::Utc;
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

struct Database {
    // connection details here
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SurrealSchema {
    events: HashMap<String, String>,
    indexes: HashMap<String, String>,
    tables: HashMap<String, String>,
    fields: HashMap<String, String>,
}

// #[async_trait]
impl Database {
    pub async fn get_current_schema(&self, table_name: &str) -> SurrealSchema {
        // db.query("INFO FOR TABLE planet")
        // SurrealSchema::default()
        let db = self.db().await;

        // db.query("INFO FOR TABLE type::table($table)")
        //     .bind("table", table_name)
        //     .run()
        //     .await
        //     .unwrap();

        info_for()
            .table(table_name.to_string())
            .get_data::<SurrealSchema>(db)
            .await
            .unwrap()
            .unwrap()
        // .run(db)
        // .await
        // .unwrap()
        // .take::<Option<SurrealSchema>>(0)
        // .expect("failed to take schema value")
        // .expect("schema value is empty")
    }

    pub async fn db(&self) -> Surreal<Db> {
        let db = Surreal::new::<Mem>(()).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        db
    }

    pub async fn execute(&self, query: String) {
        println!("Executing query: {}", query);
        self.db().await.query(query).await.unwrap();
    }

    pub async fn get_applied_migrations_from_db(&self) -> Vec<String> {
        // db.query("SELECT name FROM migrations")

        let migration::Schema { name, .. } = Migration::schema();
        let migration = Migration::table_name();

        // select [{ name: "Oyelowo" }]
        // select valeu [ "Oyelowo" ]
        // select_only. Just on object => { name: "Oyelowo" }
        let migration_names = select_value(name)
            .from(migration)
            .return_many::<String>(self.db().await)
            .await
            .unwrap();
        // vec![
        //     "20230912__up__add_name".into(),
        //     "20230912__down__remove_name".into(),
        // ]
        migration_names
    }

    pub async fn mark_migration_as_applied(&self, migration_name: &str) -> Migration {
        // db.query("CREATE migrations::name CONTENT {name: migration_name}")
        println!("Applying migration: {}", migration_name);
        // let query = format!(
        //     "CREATE migrations::name CONTENT {{name: {}}}",
        //     migration_name
        // );

        let migration = Migration {
            id: Migration::create_id(migration_name.into()),
            name: migration_name.into(),
            direction: Direction::Oneway { up: "".into() },
        };

        // self.execute(query)
        let migration = create()
            .content(migration)
            .get_one(self.db().await)
            .await
            .unwrap();
        migration
    }

    pub async fn unmark_migration(&self, migration_name: &str) {
        // Use either of the two
        // db.query("DELETE migrations::name")
        // db.query("DELETE migrations WHERE name = migration_name")
        println!("Unmark migration: {}", migration_name);
        // let query = format!("DELETE migrations::{}", migration_name);
        // self.execute(query)

        delete::<Migration>(Migration::create_id(migration_name)).run(self.db().await);
        println!("Migration unmarked: {}", migration_name);
    }
}

// enum Birectional {
//     Up(String),
//     Down(String),
// }
//
// enum Oneway {
//     Up(String),
// }
//
// enum Direction {
//     Birectional,
//     Oneway,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Direction {
    TwoWay { up: String, down: String },
    Oneway { up: String },
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Oneway { up } => write!(f, "{}", up),
            Self::TwoWay { up, down } => write!(f, "{}", up),
        }
    }
}

impl From<Direction> for String {
    fn from(direction: Direction) -> Self {
        direction.to_string()
    }
}

impl Direction {
    pub fn is_two_way(&self) -> bool {
        matches!(self, Direction::TwoWay { .. })
    }

    pub fn is_one_way(&self) -> bool {
        matches!(self, Direction::Oneway { .. })
    }

    pub fn get_up_migration(&self) -> String {
        match self {
            Direction::TwoWay { up, .. } => up.clone(),
            Direction::Oneway { up } => up.clone(),
        }
    }

    pub fn get_down_migration(&self) -> Option<String> {
        match self {
            Direction::TwoWay { down, .. } => Some(down.clone()),
            Direction::Oneway { up } => None,
        }
    }
}

// Warn when id field not included in a model

// Migratiions from migration directory
#[derive(Node, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "migration")]
pub struct Migration {
    id: SurrealId<Self, String>,
    // #[surreal_orm(type_ = "duration")]
    name: String,
    // timestamp: String,
    // up: String,
    // down: String,
    #[surreal_orm(type_ = "string")]
    direction: Direction,
    // status: String,
}

pub fn get_all_migrations_from_dir() -> Vec<Migration> {
    vec![
        Migration {
            id: Migration::create_id("20230912__up__add_name".into()),
            name: "20230912__up__add_name".into(),
            direction: Direction::Oneway {
                up: "CREATE TABLE planet;".into(),
            },
        },
        Migration {
            id: Migration::create_id("20230912__down__remove_name".into()),
            name: "20230912__down__remove_name".into(),
            direction: Direction::Oneway {
                up: "DROP TABLE planet;".into(),
            },
        },
    ]
}

pub fn get_migration_by_name_from_dir(migration_name: &str) -> Option<Migration> {
    //   # Migration file format would be migrations/<timestamp>-__<direction>__<name>.sql
    //   # Example: 2382320302303__up__create_planet.sql
    //   # Example: 2382320302303__down__delete_planet.sql

    // let migrations = get_all_migrations_from_dir();
    // migrations.into_iter().find(|m| m.name == migration_name)

    Some(Migration {
        id: Migration::create_id("20230912__up__add_name".into()),
        name: "20230912__up__add_name".into(),
        direction: Direction::Oneway {
            up: "CREATE TABLE planet;".into(),
        },
    })
}

pub async fn run_migrations(db: &mut Database) {
    let applied_migrations = db.get_applied_migrations_from_db();
    let all_migrations = get_all_migrations_from_dir();

    let applied_migrations = applied_migrations.await;
    for migration in all_migrations {
        if !applied_migrations.contains(&migration.name) {
            db.execute(migration.direction.get_up_migration());
            db.mark_migration_as_applied(&migration.name);
        }
    }
}

pub fn rollback_migration(db: &mut Database, migration_name: &str) {
    let migration = get_migration_by_name_from_dir(migration_name);
    if let Some(migration) = migration {
        let down_migration = migration.direction.get_down_migration();
        if let Some(down_migration) = down_migration {
            db.execute(down_migration);
        } else {
            println!("No down migration found for migration: {}", migration_name);
        }
        db.unmark_migration(&migration.name);
    } else {
        println!(
            "Cannot rollback migration: No migration found with name: {}",
            migration_name
        );
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
    pub tags: Vec<u64>,
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

pub fn create_migration_file(queries: Vec<String>, direction: Direction, name: &str) {
    //   # Migration file format would be migrations/<timestamp>-__<direction>__<name>.sql
    //   # Example: 2382320302303__up__create_planet.sql
    //   # Example: 2382320302303__down__delete_planet.sql
    // let timestamp = Utc::now().timestamp();
    println!("Creating migration file: {}", name);
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let _ = fs::create_dir_all("migrations").expect("Problem creating migrations directory");

    let path = format!("migrations/{}__{}__{}.sql", "timestamp", direction, name);
    let mut file = File::create(&path).unwrap();

    let queries = queries.join(";\n");
    file.write_all(queries.as_bytes()).unwrap();

    println!("Migration file created at: {}", path);
}

pub fn extract_schema_from_models() -> SurrealSchema {
    let x = Planet::schema();
    SurrealSchema::default()
}
