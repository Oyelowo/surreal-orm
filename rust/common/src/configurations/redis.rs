use redis::{aio::ConnectionManager, RedisError};

use super::utils::get_env_vars_by_prefix;
use redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use thiserror;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub struct RedisConfigs {
    // pub username: String,
    pub password: String,
    pub host: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl RedisConfigs {
    pub fn get() -> Self {
        get_env_vars_by_prefix("REDIS_")
    }

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

    pub fn get_client(self) -> Result<redis::Client, RedisConfigError> {
        let addr = ConnectionAddr::Tcp(self.host, self.port);

        let redis = RedisConnectionInfo {
            db: 0,
            username: None,
            password: Some(self.password),
        };
        let connection_info = ConnectionInfo { addr, redis };
        redis::Client::open(connection_info).map_err(RedisConfigError::OpenConnectionFailure)
    }

    pub fn get_connection(self) -> Result<redis::Connection, RedisConfigError> {
        self.get_client()?.get_connection().map_err(|e| {
            log::error!("Problem getting connection. Error:{e:?}");
            RedisConfigError::ConnectionFailure(e)
        })
    }

    pub async fn get_async_connection(self) -> Result<redis::aio::Connection, RedisConfigError> {
        let con = self
            .get_client()?
            .get_async_connection()
            .await
            .map_err(|e| {
                log::error!("Problem getting connection. Error:{e:?}");
                RedisConfigError::ConnectionFailure(e)
            });
        con
    }

    pub async fn get_connection_manager(self) -> Result<ConnectionManager, RedisConfigError> {
        ConnectionManager::new(self.get_client()?)
            .await
            .map_err(RedisConfigError::RedisConnectionManagerFailed)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RedisConfigError {
    #[error("Failed to get connection manager")]
    RedisConnectionManagerFailed(#[source] RedisError),

    #[error("Failed to open connection")]
    OpenConnectionFailure(#[source] RedisError),

    #[error("Problem getting connection")]
    ConnectionFailure(#[source] RedisError),
}
