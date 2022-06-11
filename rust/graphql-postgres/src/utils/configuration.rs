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


pub fn get_app_config() -> ApplicationConfigs {
    get_config("APP_")
}

pub fn get_postgres_config() -> DatabaseConfigs {
    get_config("POSTGRES_")
}

// pub fn get_redis_config() -> RedisConfigs {
//     get_config("REDIS_")
// }
