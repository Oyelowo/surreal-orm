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
pub struct AppUrl {
    pub port: u16,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationConfigs {
    #[serde(flatten)]
    pub url: AppUrl,
    pub environment: Environemnt,
}

impl From<AppUrl> for String {
    fn from(url: AppUrl) -> String {
        format!("{}:{}", url.host, url.port)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DbUrl {
    pub port: u16,
    pub host: String,
    pub username: String,
    pub password: String,
}

impl From<DbUrl> for String {
    fn from(
        DbUrl {
            port: _, host: _, ..
        }: DbUrl,
    ) -> String {
        // format!("mongodb://{host}:{port}/")
        "mongodb://localhost:27017/".into()
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct DatabaseConfigs {
    pub database_name: String,

    #[serde(flatten)]
    pub url: DbUrl,

    #[serde(default = "default_require_ssl")]
    pub require_ssl: Option<bool>,
}

fn default_require_ssl() -> Option<bool> {
    Some(false)
}

#[derive(Debug)]
pub struct Configs {
    pub application: ApplicationConfigs,
    pub database: DatabaseConfigs,
}

impl Configs {
    pub fn init() -> Self {
        let application = envy::from_env::<ApplicationConfigs>()
            .unwrap_or_else(|e| panic!("Failed config. Error: {:?}", e));
        // FIXME: Use as above once docker/kube is properly setup
        let database = envy::from_env::<DatabaseConfigs>().unwrap_or_default();

        Self {
            application,
            database,
        }
    }
}
