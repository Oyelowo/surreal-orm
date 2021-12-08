use anyhow::Result;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvironmentVariableError {
    #[error("environment variable is not set")]
    NotSet,

    #[error("environment variable: `{0}` is invalid. Check that it is correctly spelled")]
    Invalid(String),

    #[error("unknown environment variable error. You are on your own. lol")]
    Unknown,
}

#[derive(PartialEq)]
enum Environemnt {
    Local,
    Development,
    Production,
}

impl TryFrom<String> for Environemnt {
    type Error = EnvironmentVariableError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Environemnt::Local),
            "development" => Ok(Environemnt::Development),
            "production" => Ok(Environemnt::Production),
            err => match err {
                "" => Err(EnvironmentVariableError::NotSet),
                _ => Err(EnvironmentVariableError::Invalid(err.into())),
            },
        }
    }
}



fn main() -> Result<()> {

    let env = Environemnt::try_from(env::var("RUST_ENV")?)?;

    if env == Environemnt::Development {
        println!("Rust is the coolest thing dev");
    }
    if env == Environemnt::Local {
        println!("Rust is the coolest thing local");
    }
    if env == Environemnt::Production {
        println!("Rust is the coolest thing prod");
    }
    Ok(())
}

// use std::env;

// #[derive(PartialEq)]
// enum Environemnt {
//     Local,
//     Development,
//     Production,
// }

// impl From<String> for Environemnt {
//     fn from(string: String) -> Self {
//         match string.as_str() {
//             "local" => Environemnt::Local,
//             "development" => Environemnt::Development,
//             "production" => Environemnt::Production,
//             _ => {
//                 println!("doesnt make sense. Fallback to local");
//                 Environemnt::Local
//             }
//         }
//     }
// }

// fn main() {
//     let env: Environemnt = env::var("RUST_ENV")
//         .unwrap_or("development".to_string())
//         .into();

//     if env == Environemnt::Development {
//         println!("Rust is the coolest thing dev");
//     }
//     if env == Environemnt::Local{
//         println!("Rust is the coolest thing local");
//     }
//     if env == Environemnt::Production {
//         println!("Rust is the coolest thing prod");
//     }
// }
