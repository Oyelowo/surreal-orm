use chrono::{DateTime, Duration, Utc};
use poem::{
    session::{CookieConfig, RedisStorage, ServerSession},
    web::cookie::{CookieKey, SameSite},
};
use redis::aio::ConnectionManager;
use std::process;

use crate::configurations::{
    application::Environment,
    redis::{RedisConfigError, RedisConfigs},
};

pub async fn get_session(
    redis_config: RedisConfigs,
    environment: &Environment,
) -> Result<ServerSession<RedisStorage<ConnectionManager>>, SessionError> {
    use Environment::*;

    let cookie_key = CookieKey::generate();
    // Alternative, if you want stable key to regenerate all cookies
    // Generate a random 32 byte key. Note that it is important to use a unique
    // private key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    // Generate key with the command `openssl rand -base64 32`
    // let cookie_key = CookieKey::from(
    //     "YN7sLNF+vsvAX+bYe5qNUtmCUOJSYuZFF9PasqO+b8w="
    //         .repeat(256)
    //         .as_bytes(),
    // );
    let cookie_config = CookieConfig::private(cookie_key)
        .name("oyelowo-session")
        .secure(matches!(environment, Production | Staging | Development))
        .same_site(SameSite::Lax)
        .max_age(Some(get_session_duration_std()));

    Ok(ServerSession::new(
        cookie_config,
        RedisStorage::new(redis_config.get_connection_manager().await?),
    ))
}

#[derive(thiserror::Error, Debug)]
pub enum SessionError {
    #[error("Failed to get redis client")]
    RedisCientFailure(#[from] RedisConfigError),
}

const EXPIRES_IN_DAY: u8 = 180;

pub fn get_session_duration() -> Duration {
    Duration::days(EXPIRES_IN_DAY.into())
}

pub fn get_session_duration_std() -> std::time::Duration {
    get_session_duration().to_std().unwrap_or_else(|e| {
        // "We are sure it's greater than zero"
        log::error!("Cannot be less than zero. Error: {e}");
        process::exit(-1)
    })
}

pub fn get_session_expiry() -> DateTime<Utc> {
    Utc::now() + Duration::days(EXPIRES_IN_DAY.into())
}
