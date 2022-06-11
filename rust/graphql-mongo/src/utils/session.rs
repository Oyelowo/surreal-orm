use actix_web::cookie::SameSite;

use super::configuration::Environment::*;
use actix_session::{storage::RedisActorSessionStore, SessionLength, SessionMiddleware};
use common::my_time;

use super::configuration::{ApplicationConfigs, RedisConfigs};

pub fn get_session_middleware(
    redis: &RedisConfigs,
    application: &ApplicationConfigs,
) -> SessionMiddleware<RedisActorSessionStore> {
    // https://javascript.info/cookie#:~:text=Cookies%20are%20usually%20set%20by,using%20the%20Cookie%20HTTP%2Dheader.
    SessionMiddleware::builder(
        RedisActorSessionStore::new(redis.get_url()),
        redis.get_key(),
    )
    .cookie_name("oyelowo-session".into())
    .session_length(SessionLength::Predetermined {
        max_session_length: Some(my_time::get_session_duration()),
    })
    .cookie_secure(matches!(application.environment, Production | Staging))
    .cookie_same_site(SameSite::Strict)
    .build()
}



use poem::{
    get, handler,
    listener::TcpListener,
    session::{CookieConfig, RedisStorage, ServerSession, Session},
    EndpointExt, Route, Server,
};
use redis::{aio::ConnectionManager, Client};

