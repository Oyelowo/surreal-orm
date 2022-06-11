use poem::{
    session::{CookieConfig, RedisStorage, ServerSession, Session},
    web::cookie::SameSite,
};
use redis::{aio::ConnectionManager, Client};

use crate::{
    my_time,
    utils::configuration::{Environment, RedisConfigs},
};

pub async fn get_session(
    redis_config: RedisConfigs,
    environment: Environment,
) -> ServerSession<RedisStorage<ConnectionManager>> {
    use Environment::*;

    let cookie_config = CookieConfig::default()
        .name("oyelowo-session")
        .secure(matches!(environment, Production | Staging | Development))
        .same_site(SameSite::Strict)
        .max_age(my_time::get_session_duration());

    ServerSession::new(cookie_config, get_redis_storage(redis_config).await)
}

async fn get_redis_storage(
    redis_config: RedisConfigs,
) -> anyhow::Result<RedisStorage<ConnectionManager>> {
    let k = ConnectionManager::new(redis_config.get_client());
    // RedisStorage::new(ConnectionManager::new(redis_config.get_client()).await.unwrap())
    todo!()
}

// #[error(transparent)]
// UnexpectedError(#[from] anyhow::Error),
#[derive(thiserror::Error, Debug)]
pub enum SessionError {
    RedisConnectionManagerFailed(),
}
