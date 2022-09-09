use serde::de::DeserializeOwned;

pub fn get_env_vars_by_prefix<T: DeserializeOwned>(config_prefix: impl AsRef<str>) -> T {
    let config_prefix = config_prefix.as_ref();
    envy::prefixed(config_prefix)
        .from_env::<T>()
        .map_err(|e| match e {
            envy::Error::MissingValue(value) => {
                let e = format!("{config_prefix}{}", value.to_ascii_uppercase());
                format!("You are missing {e}. Please provide it as an environment variable")
            }
            envy::Error::Custom(e) => {
                format!("{e}. Check that all your environment variables have the right types.")
            }
        })
        .unwrap_or_else(|e| {
            panic!("{e}");
        })
}
