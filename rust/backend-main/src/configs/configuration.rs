use anyhow::Result;
use std::env;
use thiserror::Error;



#[derive(Error, Debug)]
pub enum EnvironmentVariableError {
    #[error("environment variable is not set")]
    NotSet,

    #[error("environment variable: `{name}` is invalid. Check that it is correctly spelt")]
    Invalid{name: String},

    #[error("unknown environment variable error. You are on your own. lol")]
    Unknown,
}

#[derive(PartialEq)]
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

        match value.as_str() {
            "local" => Ok(LOCAL),
            "staging" => Ok(STAGING),
            "development" => Ok(DEVEVELOPMENT),
            "production" => Ok(PRODUCTION),

            err => match err {
                "" => Err(EnvironmentVariableError::NotSet),
                _ => Err(EnvironmentVariableError::Invalid{name: err.into()}),
            },
        }
    }
}

