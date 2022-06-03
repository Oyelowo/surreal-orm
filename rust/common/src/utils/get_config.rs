use std::process;

use serde::de::DeserializeOwned;

pub fn get_config<T: DeserializeOwned>(config_prefix: &str) -> T {
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
