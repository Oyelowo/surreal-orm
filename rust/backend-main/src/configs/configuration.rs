use anyhow::{Result};
use typed_builder::TypedBuilder;

use std::{env};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvironmentVariableError {
    #[error("environment variable is not set")]
    NotSet,

    #[error("environment variable: `{name}` is invalid. Check that it is correctly spelt")]
    Invalid { name: String },
}

#[derive(PartialEq, Debug)]
pub enum Environemnt {
    LOCAL,
    DEVEVELOPMENT,
    STAGING,
    PRODUCTION,
}

impl TryFrom<String> for Environemnt {
    type Error = EnvironmentVariableError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use Environemnt::*;

        match value.to_lowercase().as_str() {
            "local" => Ok(LOCAL),
            "staging" => Ok(STAGING),
            "development" => Ok(DEVEVELOPMENT),
            "production" => Ok(PRODUCTION),
            "" => Err(EnvironmentVariableError::NotSet),
            err => Err(EnvironmentVariableError::Invalid { name: err.into() }),
        }
    }
}

#[derive(TypedBuilder, Clone, Debug)]
pub struct ApplicationConfigs {
    #[builder(default = 8000)]
    pub port: u16,

    #[builder(default="0.0.0.0".into())]
    pub host: String,

    pub domain: String,
}

#[derive(TypedBuilder, Clone, Debug)]
pub struct DatabaseConfigs {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,

    #[builder(default = false)]
    pub require_ssl: bool,
}

#[derive(Debug)]
pub struct Configs {
    pub environment: Environemnt,
    pub application: ApplicationConfigs,
    pub database: DatabaseConfigs,
}

impl Configs {
    pub fn init() -> Self {
        Self::new().unwrap_or_else(|e| panic!("Failed config. Error: {:?}", e))
        // .map_err(|e| {
        //     eprintln!("Failed to print, {:?}", e);
        //     e
        // })
        // .expect("Build failed")
    }

    fn new() -> anyhow::Result<Self> {
        // Using normal pattern
        // let application = ApplicationConfigs {
        //     host: env::var("HOST").unwrap_or("0.0.0.0".into()),
        //     port: env::var("PORT")?.parse().unwrap_or(8000),
        // };

        // let database = DatabaseConfigs {
        //     host: env::var("DB_HOST")?,
        //     username: env::var("DB_USERNAME")?,
        //     password: env::var("DB_PASSWORD")?,
        //     port: env::var("DB_PORT")?.parse()?,
        //     database_name: env::var("DB_NAME")?.parse()?,
        //     require_ssl: true
        // };

        // Using builder pattern which is checked for correctedness at compile time
        let host = env::var("HOST")?;
        let port = env::var("PORT")?.parse()?;

        let application = ApplicationConfigs::builder()
            .host(host.clone())
            .port(port)
            .domain(format!("{}:{}", &host, port))
            .build();

        let database = DatabaseConfigs::builder()
            .host(env::var("DB_HOST")?)
            .username(env::var("DB_USERNAME")?)
            .password(env::var("DB_PASSWORD")?)
            .port(env::var("DB_PORT")?.parse()?)
            .database_name(env::var("DB_NAME")?.parse()?)
            .require_ssl(env::var("DB_REQUIRE_SSL")?.parse()?)
            .build();

        Ok(Self {
            environment: env::var("APP_ENVIRONMENT")?.try_into()?,
            application,
            database,
        })
    }
}
