use std::process;

use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionLength, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    http,
};
use anyhow::Context;

use common::my_time;
use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client, Database,
};

use redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Local,
    Development,
    Staging,
    Production,
}

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
    pub fn get_database(self) -> anyhow::Result<Database> {
        let credential = Credential::builder()
            .username(self.username)
            .password(self.password)
            .source(self.name.clone())
            .build();

        let hosts = vec![ServerAddress::Tcp {
            host: self.host,
            port: Some(self.port),
        }];

        let options = ClientOptions::builder()
            .app_name(Some("graphql-mongo".into()))
            .hosts(hosts)
            .credential(credential)
            .build();

        let db = Client::with_options(options)
            .context("Faulty db option")?
            .database(&self.name);
        Ok(db)
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub struct RedisConfigs {
    // pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,

    /// Generate a random 32 byte key. Note that it is important to use a unique
    /// private key for every project. Anyone with access to the key can generate
    /// authentication cookies for any user!
    /// Generate key with the command `openssl rand -base64 32`
    pub session_key: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl RedisConfigs {
    pub fn get_connection_info(&self) -> ConnectionInfo {
        let addr = ConnectionAddr::Tcp(self.host.clone(), self.port);

        let redis = RedisConnectionInfo {
            db: 0,
            username: None,
            password: Some(self.password.clone()),
        };
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

        redis::Client::open(connection_info).with_context(|| "Failed to open connection")
    }

    /// Generate a random 32 byte key. Note that it is important to use a unique
    /// private key for every project. Anyone with access to the key can generate
    /// authentication cookies for any user!
    /// Generate key with the command `openssl rand -base64 32`
    pub fn get_key(&self) -> Key {
        Key::from(self.session_key.repeat(256).as_bytes())
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
}

#[derive(Debug)]
pub struct Configs {
    pub application: ApplicationConfigs,
    pub database: DatabaseConfigs,
    pub redis: RedisConfigs,
}

impl Configs {
    pub fn get_app_config() -> ApplicationConfigs {
        Self::get_config("APP_")
    }

    pub fn get_db_config() -> DatabaseConfigs {
        Self::get_config("MONGODB_")
    }

    pub fn get_redis_config() -> RedisConfigs {
        Self::get_config("REDIS_")
    }

    fn get_config<T: DeserializeOwned>(config_prefix: &str) -> T {
        envy::prefixed(config_prefix)
            .from_env::<T>()
            .unwrap_or_else(|e| {
                log::error!(
                    "problem with {config_prefix:?} environment variables(s). 
                Check that the prefix is correctly spelt and the configs are complete. Error {e:?}"
                );
                process::exit(1);
            })
    }

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
        .cookie_secure(matches!(
            application.environment,
            Environment::Production | Environment::Staging
        ))
        .cookie_same_site(SameSite::Strict)
        .build()
    }

    /// https://javascript.info/fetch-crossorigin#cors-for-safe-requests
    /// http://www.ruanyifeng.com/blog/2016/04/cors.html
    pub fn get_cors() -> Cors {
        Cors::default()
            .allow_any_origin() // FIXME: // remove after testing.
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
    }
}
