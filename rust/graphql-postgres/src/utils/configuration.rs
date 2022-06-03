use common::utils::get_config;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Local,
    Development,
    Staging,
    Production,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppUrl {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ApplicationConfigs {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub environment: Environment,
}

impl ApplicationConfigs {
    pub fn get_url(&self) -> String {
        let Self { host, port, .. } = self;
        // Url::parse(format!("http://{host}:{port}").as_ref()).expect("Problem parsing application uri")
        format!("{host}:{port}")
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct DatabaseConfigs {
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

impl DatabaseConfigs {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.name)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = match self.require_ssl {
            Some(true) => PgSslMode::Require,
            // Try an encrypted connection, fallback to unencrypted if it fails
            _ => PgSslMode::Prefer,
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .ssl_mode(ssl_mode)
        // .application_name("my-app");
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct RedisConfigs {
    // We have not created a stand-alone settings struct for Redis,
    // let's see if we need more than the uri first!
    // The URI is marked as secret because it may embed a password.
    pub redis_uri: String,
}

pub fn get_app_config() -> ApplicationConfigs {
    get_config("APP_")
}

pub fn get_postgres_config() -> DatabaseConfigs {
    get_config("POSTGRES_")
}

// pub fn get_redis_config() -> RedisConfigs {
//     get_config("REDIS_")
// }
