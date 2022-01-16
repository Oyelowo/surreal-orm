use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environemnt {
    Local,
    Development,
    Staging,
    Production,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationConfigs {
    pub port: u16,
    pub host: String,
    pub environment: Environemnt,
}

impl From<ApplicationConfigs> for String {
    fn from(app_config: ApplicationConfigs) -> String {
        format!("{}:{}", app_config.host, app_config.port)
    }
}


impl ApplicationConfigs {
    pub fn get_url(self) -> String {
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
