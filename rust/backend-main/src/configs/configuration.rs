use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environemnt {
    Local,
    Development,
    Staging,
    Production,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct URL {
    pub port: u16,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationConfigs {
    #[serde(flatten)]
    pub url: URL,
    pub environment: Environemnt,
}

impl From<URL> for String {
    fn from(url: URL) -> String {
        format!("{}:{}", url.host, url.port)
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
