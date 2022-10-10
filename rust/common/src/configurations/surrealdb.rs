use super::utils::get_env_vars_by_prefix;
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use surrealdb::Datastore;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct SurrealdbConfigs {
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub host: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,

    #[serde(default = "default_require_ssl")]
    pub require_ssl: Option<bool>,
}

fn default_require_ssl() -> Option<bool> {
    Some(false)
}

impl Default for SurrealdbConfigs {
    fn default() -> Self {
        get_env_vars_by_prefix("SURREALDB_")
    }
}

impl SurrealdbConfigs {
    pub fn get_database(self) -> anyhow::Result<Datastore> {
        // let ds = Datastore::new("tikv://127.0.0.1:8000").await?;
        todo!()
    }
}
