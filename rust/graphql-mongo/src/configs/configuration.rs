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
#[serde(rename_all = "lowercase")]
pub struct ApplicationConfigs {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub environment: Environemnt,
}

impl ApplicationConfigs {
    pub fn get_url(&self) -> String {
        let Self { host, port, .. } = self;
        // Url::parse(format!("http://{host}:{port}").as_ref()).expect("Problem parsing application uri")
        format!("{host}:{port}")
    }
}

#[derive(Deserialize, Debug, Default)]
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
    pub fn get_url(&self) -> String {
        let Self {
            host,
            port,
            username,
            password,
            ..
        } = self;

        // Url::parse(format!("mongodb://{username}:{password}@{host}:{port}/").as_str())
        Url::parse(format!("mongodb://{host}:{port}/").as_str())
            .expect("Problem passing mongodb uri")
            .into()
    }
}

#[derive(Debug)]
pub struct Configs {
    pub application: ApplicationConfigs,
    pub database: DatabaseConfigs,
}

impl Configs {
    pub fn init() -> Self {
        let application = envy::prefixed("APP_")
            .from_env::<ApplicationConfigs>()
            .unwrap_or_else(|e| panic!("Failed config. Error: {:?}", e));
        // FIXME: Use as above once docker/kube is properly setup
        let database = envy::prefixed("MONGODB_")
            .from_env::<DatabaseConfigs>()
            .expect("problem with mongo db environment variables(s)");

        Self {
            application,
            database,
        }
    }
}
