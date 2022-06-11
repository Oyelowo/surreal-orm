use anyhow::Context;

use common::utils::get_config;
use redis::aio::ConnectionManager;

use redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use super::utils::{get_config, Configurable};

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub struct RedisConfigs {
    pub username: String,
    pub password: String,
    pub host: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl Configurable for RedisConfigs {
    fn get() -> Self {
        get_config("REDIS_")
    }
}

impl RedisConfigs {
    fn get_redis_connection_info(self) -> RedisConnectionInfo {
        RedisConnectionInfo {
            db: 0,
            username: None,
            password: Some(self.password),
        }
    }

    pub fn get_connection_info(self) -> ConnectionInfo {
        let addr = ConnectionAddr::Tcp(self.host.clone(), self.port);

        let redis = self.get_redis_connection_info();
        ConnectionInfo { addr, redis }
    }

    pub fn get_client(self) -> anyhow::Result<redis::Client> {
        let addr = ConnectionAddr::Tcp(self.host, self.port);

        let redis = RedisConnectionInfo {
            db: 0,
            username: None,
            password: Some(self.password),
        };
        let connection_info = ConnectionInfo { addr, redis };
        redis::Client::open(connection_info)?
    }

    pub fn get_url(&self) -> String {
        let Self {
            host,
            port,
            username,
            password,
            ..
        } = self;
        let db = 0;

        // format!("{host}:{port}")
        // redis://[<username>][:<password>@]<hostname>[:port][/<db>]
        format!("redis://{username}:{password}@{host}:{port}/{db}")
    }

    pub async fn get_connection_manager(self) -> anyhow::Result<ConnectionManager> {
        ConnectionManager::new(self.get_client()).await?
    }
}
