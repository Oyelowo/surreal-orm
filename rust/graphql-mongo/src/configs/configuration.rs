use anyhow::Context;
use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client, Database,
};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Local,
    Development,
    Staging,
    Production,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct ApplicationConfigs {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub environment: Environment,
}

impl ApplicationConfigs {
    pub fn get_url(&self) -> String {
        let Self { host, port, .. } = self;
        // Url::parse(format!("http://{host}:{port}").as_ref()).expect("Problem parsing application uri")
        format!("{host}:{port}")
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub struct DatabaseConfigs {
    pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,

    #[serde(default = "default_require_ssl")]
    pub require_ssl: Option<bool>,
}

fn default_require_ssl() -> Option<bool> {
    Some(false)
}

impl DatabaseConfigs {
    pub fn get_database(self) -> anyhow::Result<Database> {
        let credential = Credential::builder()
            .username(self.username)
            .password(self.password)
            .source(self.name.clone())
            .build();

        let hosts = vec![ServerAddress::Tcp {
            host: self.host,
            port: Some(self.port),
        }];

        let options = ClientOptions::builder()
            .app_name(Some("graphql-mongo".into()))
            .hosts(hosts)
            .credential(credential)
            .build();

        let db = Client::with_options(options)
            .context("Faulty db option")?
            .database(&self.name);
        Ok(db)
    }

    // pub fn get_url(&self) -> String {
    //     let Self {
    //         host,
    //         port,
    //         username,
    //         password,
    //         ..
    //     } = self;

    //     Url::parse(format!("mongodb://{username}:{password}@{host}:{port}/").as_str())
    //         .expect("Problem passing mongodb uri")
    //         .into()
    // }
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub struct RedisConfigs {
    // pub name: String,
    // pub username: String,
    // pub password: String,
    pub host: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl RedisConfigs {
    pub fn get_url(&self) -> String {
        // let Self { host, port, .. } = self;
        // Url::parse(format!("http://{host}:{port}").as_ref()).expect("Problem parsing application uri")
        // format!("{host}:{port}")
        "redis-database-master.development:6379".into()
    }
}

#[derive(Debug)]
pub struct Configs {
    pub application: ApplicationConfigs,
    pub database: DatabaseConfigs,
    pub redis: RedisConfigs,
}

impl Configs {
    pub fn init() -> Self {
        let application = envy::prefixed("APP_")
            .from_env::<ApplicationConfigs>()
            .unwrap_or_else(|e| panic!("Failed config. Error: {:?}", e));
        // FIXME: Use as above once docker/kube is properly setup
        let database = envy::prefixed("MONGODB_")
            .from_env::<DatabaseConfigs>()
            .expect("problem with mongo db environment variables(s)");

        // let redis = envy::prefixed("REDIS")
        //     .from_env::<RedisConfigs>()
        //     .expect("problem with redis environment variables(s)");

        Self {
            application,
            database,
            redis: RedisConfigs {
                // TODO: change
                host: "redis".into(),
                port: 6370,
            },
        }
    }
}
