use std::process;

use serde::de::DeserializeOwned;

pub fn get_env_vars_by_prefix<T: DeserializeOwned>(config_prefix: &str) -> T {
    envy::prefixed(config_prefix)
        .from_env::<T>()
        .unwrap_or_else(|e| {
            // e.g if prefix is APP_ and error msg = "missing value for field port". ==> APP_PORT
            let error_msg = e.to_string();
            let env_variable = error_msg
                .split(" ")
                .last()
                .expect("Couldn't get it")
                .to_ascii_uppercase();
                
            let var_to_provide = format!("{config_prefix}{env_variable}");
            log::error!(
                "You are missing {var_to_provide}. Please provide as an environment variable.
                Error {e:?}",
            );
            process::exit(1);
        })
}
