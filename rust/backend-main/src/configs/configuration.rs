use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use url::Url;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environemnt {
    Local,
    Development,
    Staging,
    Production,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppUrl {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationConfigs {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub environment: Environemnt,
}

impl ApplicationConfigs {
    pub fn get_url(&self) -> anyhow::Result<Url, url::ParseError> {
        let url_str = format!("{}:{}", self.host, self.port);
        Url::parse(url_str.as_str())
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct DatabaseConfigs {
    pub database_name: String,
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
    pub fn get_url(&self) -> anyhow::Result<Url, url::ParseError> {
        // Url::parse("mongodb://{self.host}:{self.port}/")
        Url::parse("mongodb://localhost:27017/")
    }
}

#[derive(Debug)]
pub struct Configs {
    pub application: ApplicationConfigs,
    pub database: DatabaseConfigs,
}

impl Configs {
    pub fn init() -> Self {
        let application = envy::from_env::<ApplicationConfigs>()
            .unwrap_or_else(|e| panic!("Failed config. Error: {:?}", e));
        // FIXME: Use as above once docker/kube is properly setup
        let database = envy::from_env::<DatabaseConfigs>().unwrap_or_default();

        Self {
            application,
            database,
        }
    }
}
