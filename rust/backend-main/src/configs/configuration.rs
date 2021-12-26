use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub enum Environemnt {
    LOCAL,
    DEVEVELOPMENT,
    STAGING,
    PRODUCTION,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationConfigs {
    pub port: u16,
    pub host: String,
    pub environment: Environemnt,
}

impl ApplicationConfigs {
    pub fn derive_domain(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfigs {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,

    #[serde(default = "default_require_ssl")]
    pub require_ssl: Option<bool>,
}

fn default_require_ssl() -> Option<bool> {
    Some(false)
}

#[derive(Debug)]
pub struct Configs {
    pub application: ApplicationConfigs,
    // pub database: DatabaseConfigs,
}

impl Configs {
    pub fn init() -> Self {
        let application = envy::from_env::<ApplicationConfigs>()
            .unwrap_or_else(|e| panic!("Failed config. Error: {:?}", e));

        Self {
            application,
            // database,
        }
    }
}
