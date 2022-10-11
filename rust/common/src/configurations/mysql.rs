use super::utils::get_env_vars_by_prefix;
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct MySqlConfigs {
    pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,

    #[serde(default = "default_require_ssl")]
    pub require_ssl: Option<bool>,
}

impl Default for MySqlConfigs {
    fn default() -> Self {
        get_env_vars_by_prefix("MYSQL_")
    }
}

fn default_require_ssl() -> Option<bool> {
    Some(false)
}

impl MySqlConfigs {
    pub fn with_db(&self) -> MySqlConnectOptions {
        self.without_db().database(&self.name)
    }

    pub fn without_db(&self) -> MySqlConnectOptions {
        let ssl_mode = match self.require_ssl {
            Some(true) => MySqlSslMode::Required,
            // Try an encrypted connection, fallback to unencrypted if it fails
            _ => MySqlSslMode::Preferred,
        };

        MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .ssl_mode(ssl_mode)
        // .application_name("my-app");
    }
}
