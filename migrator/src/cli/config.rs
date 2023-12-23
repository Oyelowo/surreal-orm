use clap::{ArgAction, Parser};
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use surrealdb::dbs::Capabilities;
use surrealdb::opt::Config;
use typed_builder::TypedBuilder;

use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::{Database, Root};
use surrealdb::Surreal;

use crate::Mode;

#[derive(Parser, Debug, Default, Clone, TypedBuilder)]
pub struct SharedAll {
    /// Optional custom migrations dir
    #[clap(short, long, help = "Optional custom migrations dir")]
    #[builder(default, setter(strip_option))]
    pub migrations_dir: Option<PathBuf>,

    /// Sets the level of verbosity e.g -v, -vv, -vvv, -vvvv
    #[clap(short, long, action = ArgAction::Count, default_value="3")]
    pub(crate) verbose: u8,
}

#[derive(Clone, Debug)]
pub enum UrlDb {
    Memory,
    Others(String),
}

impl Display for UrlDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlDb::Memory => write!(f, "mem://"),
            UrlDb::Others(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for UrlDb {
    type Err = String;

    // Can be one of memory, file:<path>, tikv:<addr>, file://<path>, tikv://<addr>
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        if s == "memory" {
            Ok(UrlDb::Memory)
        } else {
            Ok(UrlDb::Others(s))
            // Err("Invalid path".to_string())
        }
    }
}

#[derive(Parser, Debug, Clone, TypedBuilder)]
pub struct RuntimeConfig {
    /// URL or path to connect to a database instance. Supports various backends.
    /// Examples:
    /// - Local WebSocket: `ws://localhost:8000`
    /// - Remote WebSocket: `wss://cloud.surrealdb.com`
    /// - HTTP: `http://localhost:8000`
    /// - HTTPS: `https://cloud.surrealdb.com`
    /// - In-Memory: `mem://`
    /// - File-Backend: `file://temp.db`
    /// - IndxDB-Backend: `indxdb://MyDatabase`
    /// - TiKV-Backend: `tikv://localhost:2379`
    /// - FoundationDB-Backend: `fdb://fdb.cluster`
    #[clap(
            long,
            // value_name = "URL",
            default_value = "ws://localhost:8000",
            help = "Example:\n\
                    - ws://localhost:8000\n\
                    - wss://cloud.surrealdb.com\n\
                    - http://localhost:8000\n\
                    - https://cloud.surrealdb.com\n\
                    - mem://\n\
                    - file://temp.db\n\
                    - indxdb://MyDatabase\n\
                    - tikv://localhost:2379\n\
                    - fdb://fdb.cluster"
        )]
    pub(crate) url: UrlDb,

    #[clap(long, default_value = "test", help = "Database name")]
    #[builder(setter(strip_option))]
    pub(crate) db: Option<String>,

    #[clap(long, default_value = "test", help = "Namespace name")]
    #[builder(setter(strip_option))]
    pub(crate) ns: Option<String>,

    /// users scope
    #[clap(long, help = "Scope")]
    #[builder(default, setter(strip_option))]
    pub(crate) sc: Option<String>,

    #[clap(short, long, default_value = "root", help = "User name")]
    #[builder(setter(strip_option))]
    pub(crate) user: Option<String>,

    #[clap(short, long, default_value = "root", help = "Password")]
    #[builder(setter(strip_option))]
    pub(crate) pass: Option<String>,

    #[clap(
        long,
        help = "If to be strict or lax. Strictness validates the migration files against the database e.g doing checksum checks to make sure.\
            that file contents and valid and also checking filenames. Lax does not.",
        default_value = "strict"
    )]
    #[builder(setter(strip_option))]
    pub(crate) mode: Option<Mode>,

    #[clap(
        long,
        help = "If to prune migration files after rollback",
        default_value = "false"
    )]
    pub(crate) prune: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig::builder()
            .db("test".into())
            .ns("test".into())
            .user("root".into())
            .pass("root".into())
            .mode(Mode::Strict)
            .prune(false)
            .url(UrlDb::Memory)
            .build()
    }
}

pub struct SetupDb {
    db: Surreal<Any>,
}

impl SetupDb {
    // pub async fn new(runtime_config: RuntimeConfig) -> Self {
    //     let db = Self::setup_db(&runtime_config).await;
    //     Self { db }
    // }

    pub fn db(&self) -> Surreal<Any> {
        self.db.clone()
    }

    pub(crate) async fn new(runtime_config: &RuntimeConfig) -> Surreal<Any> {
        let cli_db_url = &runtime_config.url;
        let database = runtime_config.db.clone().unwrap_or_default();
        let database = database.as_str();
        let namespace = runtime_config.ns.clone().unwrap_or_default();
        let namespace = namespace.as_str();
        let username = runtime_config.user.clone().unwrap_or_default();
        let username = username.as_str();
        let password = runtime_config.pass.clone().unwrap_or_default();
        let password = password.as_str();
        let config = Config::new().capabilities(Capabilities::all());

        let config = match cli_db_url {
            UrlDb::Memory => {
                let creds = Root { username, password };
                config.user(creds)
            }
            UrlDb::Others(_s) => config,
        };

        let db = connect((cli_db_url.to_string(), config)).await.unwrap();

        let db_creds = Database {
            username,
            password,
            database,
            namespace,
        };

        // TODO: Support scope signin
        db.signin(db_creds).await.expect("Failed to signin");
        db.use_db(runtime_config.clone().db.unwrap_or_default())
            .await
            .expect("Failed to use db");
        db.use_ns(runtime_config.clone().sc.unwrap_or_default())
            .await
            .expect("Failed to use ns");
        db
        // Self::init_db(db.clone(), &runtime_config).await
    }

    pub async fn init_db(db: Surreal<Any>, shared: &RuntimeConfig) -> Surreal<Any> {
        match (&shared.user, &shared.pass) {
            (Some(u), Some(p)) => {
                let signin = db
                    .signin(Root {
                        username: u.as_str(),
                        password: p.as_str(),
                    })
                    .await;
                if let Err(e) = signin {
                    println!("Failed to signin: {e}");
                    log::error!("Failed to signin: {e}");
                    panic!();
                }
                log::info!("Signed in successfully");
            }
            (Some(_), None) => {
                log::error!("Password not provided");
                panic!();
            }
            (None, Some(_)) => {
                log::error!("User not provided");
                panic!();
            }
            _ => {
                log::warn!("User and password not provided, using root default");
                // db.signin(Root {
                //     username: "root",
                //     password: "root",
                // })
                // .await
                // .expect("Failed to signin");
            }
        };

        if let Some(db_name) = &shared.db {
            log::info!("Using db {}", db_name);
            let db = db.use_db(db_name).await;
            if let Err(e) = db {
                log::error!("Failed to use db: {}", e.to_string());
                panic!();
            }
        }

        if let Some(ns_name) = &shared.ns {
            log::info!("Using ns {}", ns_name);
            let ns = db.use_ns(ns_name).await;
            if let Err(e) = ns {
                log::error!("Failed to use ns: {}", e.to_string());
                panic!();
            }
        }
        db
    }
}
