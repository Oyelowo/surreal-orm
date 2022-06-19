use std::process;

use serde::de::DeserializeOwned;

pub fn get_env_vars_by_prefix<T: DeserializeOwned>(config_prefix: &str) -> T {
    envy::prefixed(config_prefix)
        .from_env::<T>()
        .map_err(|e| match e {
            envy::Error::MissingValue(value) => {
                format!("{config_prefix}{}", value.to_ascii_uppercase())
            }
            envy::Error::Custom(e) => e,
        })
        .unwrap_or_else(|e| {
            log::error!("You are missing {e}. Please provide it as an environment variable",);
            process::exit(1);
        })
}
