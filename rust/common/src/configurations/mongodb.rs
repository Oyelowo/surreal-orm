use super::utils::get_env_vars_by_prefix;
use anyhow::Context;
use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client, Database,
};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub struct MongodbConfigs {
    pub name: String,
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

impl MongodbConfigs {
    pub fn get() -> Self {
        get_env_vars_by_prefix("MONGODB_")
    }

    pub fn get_database(self) -> anyhow::Result<Database> {
        let credential = Credential::builder()
            .username(self.username)
            .password(self.password)
            .source(self.name.clone())
            .build();

        let hosts = vec![ServerAddress::Tcp {
            host: self.host,
            port: Some(self.port),
        }];

        let options = ClientOptions::builder()
            .app_name(Some("graphql-mongo".into()))
            .hosts(hosts)
            .credential(credential)
            .build();

        let db = Client::with_options(options)
            .context("Faulty db option")?
            .database(&self.name);
        Ok(db)
    }
}
