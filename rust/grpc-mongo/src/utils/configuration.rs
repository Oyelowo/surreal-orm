use std::net::SocketAddr;

use common::utils::get_config;
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
    pub fn get_url(&self) -> SocketAddr {
        let Self { host, port, .. } = self;
        // Url::parse(format!("http://{host}:{port}").as_ref()).expect("Problem parsing application uri")
        // "[::1]:50051"
        format!("{host}:{port}")
            .parse()
            .expect("Problem parsing address")
        //"[::1]:50051".parse().expect("Problem parsing address")
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
        let Self { host, port, .. } = self;
        Url::parse(format!("mongodb://{host}:{port}/").as_str())
            .expect("Problem pasing mongodb uri")
            .into()
    }
}

pub fn get_app_config() -> ApplicationConfigs {
    get_config("APP_")
}

pub fn get_db_config() -> DatabaseConfigs {
    get_config("MONGODB_")
}
