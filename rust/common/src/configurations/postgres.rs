use super::utils::{get_env_vars_by_prefix, Configurable};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct PosgresConfigs {
    pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,

    #[serde(default = "default_require_ssl")]
    pub require_ssl: Option<bool>,
}

impl Configurable for PosgresConfigs {
    fn get() -> Self {
        get_env_vars_by_prefix("POSTGRES_")
    }
}

fn default_require_ssl() -> Option<bool> {
    Some(false)
}

impl PosgresConfigs {
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
