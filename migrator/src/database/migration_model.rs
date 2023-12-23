use std::fmt::Display;

use serde::{Deserialize, Serialize};
use surreal_query_builder::{
    statements::{
        begin_transaction, define_field, define_table, remove_table, select, DefineTableStatement,
        RemoveTableStatement,
    },
    All, Field, FieldType, Raw, ReturnableSelect, SurrealOrmResult, Table, ToRaw,
};
use surrealdb::{engine::any::Any, sql::Thing, Surreal};

use crate::*;

// #[derive(Node, Serialize, Deserialize, Clone, Debug)]
#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "migration", schemafull)]
pub struct Migration {
    // pub id: SurrealId<Self, String>,
    pub id: Thing,
    pub name: String,
    pub timestamp: Timestamp,
    pub checksum_up: Checksum,
    pub checksum_down: Option<Checksum>,
    // pub timestamp: DateTime<Utc>,
    // status: String,
}

impl Ord for Migration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Migration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}

impl PartialEq for Migration {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for Migration {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Timestamp(timestamp) = self;
        write!(f, "{}", timestamp)
    }
}

impl From<Timestamp> for u64 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

impl Timestamp {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl From<u64> for Timestamp {
    fn from(timestamp: u64) -> Self {
        Self(timestamp)
    }
}

pub struct MigrationSchema {
    pub id: Field,
    pub name: Field,
    pub timestamp: Field,
    pub checksum_up: Field,
    pub checksum_down: Field,
}

impl Migration {
    pub fn create_id(filename: &MigrationFilename) -> Thing {
        Thing {
            tb: Migration::table_name().to_string(),
            id: filename.to_string().into(),
        }
    }

    pub fn create_reinitialize_table_raw_tx(
        filename: &MigrationFilename,
        checksum_up: &Checksum,
        checksum_down: Option<&Checksum>,
        migration_type: MigrationFlag,
    ) -> Raw {
        let mut tx = begin_transaction()
            .query(Self::remove_table())
            .query(Self::define_table());

        for def in Self::define_fields(migration_type) {
            tx = tx.query(def);
        }

        tx.query(Self::create_raw(&filename, checksum_up, checksum_down))
            .commit_transaction()
            .to_raw()
    }

    pub fn create_raw(
        filename: &MigrationFilename,
        checksum_up: &Checksum,
        checksum_down: Option<&Checksum>,
    ) -> Raw {
        let migration::Schema {
            id: _id_field,
            name: name_field,
            timestamp: timestamp_field,
            checksum_up: checksum_up_field,
            checksum_down: checksum_down_field,
        } = Self::schema();

        let record_id = Self::create_id(&filename);
        let name = filename.to_string();
        let timestamp = filename.timestamp().into_inner();
        let checksum_up = checksum_up.to_string();
        let checksum_down = checksum_down
            .map(|c| c.to_string())
            .unwrap_or("null".into());

        Raw::new(format!(
            "CREATE {record_id} SET {name_field}='{name}', {timestamp_field}={timestamp}, \
        {checksum_up_field}='{checksum_up}', {checksum_down_field}='{checksum_down}';"
        ))
    }

    pub fn delete_raw(migration_id: &Thing) -> Raw {
        // let record_id = Self::create_id(filename);
        Raw::new(format!("DELETE {migration_id};"))
    }

    pub fn schema() -> MigrationSchema {
        MigrationSchema {
            id: "id".into(),
            name: Field::new("name"),
            timestamp: Field::new("timestamp"),
            checksum_up: Field::new("checksum_up"),
            checksum_down: Field::new("checksum_down"),
        }
    }

    pub fn table_name() -> Table {
        Table::new("migration")
    }

    pub fn define_table() -> DefineTableStatement {
        define_table(Migration::table_name()).schemafull()
    }

    pub fn remove_table() -> RemoveTableStatement {
        remove_table(Self::table_name())
    }

    pub fn delete_all() -> Raw {
        Raw::new(format!("DELETE {};", Self::table_name()))
    }

    pub fn define_fields(migration_type: MigrationFlag) -> Vec<Raw> {
        let migration::Schema {
            id,
            name,
            timestamp,
            checksum_up,
            checksum_down,
        } = Migration::schema();
        let id = define_field(id)
            .type_(FieldType::Record(vec![Self::table_name()]))
            .on_table(Self::table_name())
            .to_raw();

        let name = define_field(name)
            .type_(FieldType::String)
            .on_table(Self::table_name())
            .to_raw();

        let timestamp = define_field(timestamp)
            .type_(FieldType::Int)
            .on_table(Migration::table_name())
            .to_raw();

        let checksum_up = define_field(checksum_up)
            .type_(FieldType::String)
            .on_table(Migration::table_name())
            .to_raw();

        let mut fields = vec![id, name, timestamp, checksum_up];

        let checksum_down = define_field(checksum_down)
            .type_(FieldType::String)
            .on_table(Migration::table_name())
            .to_raw();

        if migration_type.is_twoway() {
            fields.push(checksum_down);
        }

        fields
    }

    // pub async fn get_all(db: Surreal<Any>) -> SurrealOrmResult<Vec<Self>> {
    pub async fn get_all(db: Surreal<Any>) -> Vec<Self> {
        select(All)
            .from(Self::table_name())
            .return_many::<Self>(db.clone())
            .await
            .expect("Failed to get migrations")
    }
}

pub mod migration {
    pub type Schema = super::MigrationSchema;
}

impl Migration {}
