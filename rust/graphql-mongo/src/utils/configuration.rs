use std::process;

use anyhow::Context;

use common::utils::get_config;
use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client, Database,
};
use poem::session::{CookieConfig, RedisStorage, ServerSession};
use redis::aio::ConnectionManager;

use redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ExternalApiConfigs {
    pub github_secret: Stirng,
    pub github_client_id: String,
}

impl ApplicationConfigs {
    pub fn get() -> ApplicationConfigs {
        get_config("EXTERNAL_API_")
    }
}
