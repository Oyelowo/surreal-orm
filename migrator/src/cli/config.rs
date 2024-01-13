/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use clap::Args;
use std::fmt::Display;
use std::str::FromStr;
use surrealdb::dbs::Capabilities;
use surrealdb::opt::Config;
use typed_builder::TypedBuilder;

use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::{Database, Root};
use surrealdb::Surreal;

#[derive(Clone, Debug)]
pub enum UrlDb {
    Memory,
    Others(String),
}

impl Display for UrlDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlDb::Memory => write!(f, "mem://"),
            UrlDb::Others(s) => write!(f, "{s}"),
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
        }
    }
}

#[derive(Args, Debug, Clone, TypedBuilder)]
pub struct DatabaseConnection {
    /// URL or path to connect to a database instance. Supports various backends.
    /// Examples:
    /// - Local WebSocket: `ws://localhost:8000`
    /// - Remote WebSocket: `wss://cloud.example.com`
    /// - HTTP: `http://localhost:8000`
    /// - HTTPS: `https://cloud.example.com`
    /// - In-Memory: `mem://`
    /// - File-Backend: `file://temp.db`
    /// - IndxDB-Backend: `indxdb://MyDatabase`
    /// - TiKV-Backend: `tikv://localhost:2379`
    /// - FoundationDB-Backend: `fdb://fdb.cluster`
    #[arg(
            global = true,
            long,
            // value_name = "URL",
            default_value = "ws://localhost:8000",
            // default_value = "memory",
            help = "Example:\n\
                    - ws://localhost:8000\n\
                    - wss://cloud.example.com\n\
                    - http://localhost:8000\n\
                    - https://cloud.example.com\n\
                    - mem://\n\
                    - file://temp.db\n\
                    - indxdb://MyDatabase\n\
                    - tikv://localhost:2379\n\
                    - fdb://fdb.cluster"
        )]
    pub(crate) url: UrlDb,

    #[arg(global = true, long, default_value_t = String::from("test"), help = "Database name")]
    #[builder(default = "test".into())]
    pub(crate) db: String,

    #[arg(global = true, long, default_value_t = String::from("test"), help = "Namespace name")]
    #[builder(default = "test".into())]
    pub(crate) ns: String,

    /// users scope
    #[arg(global = true, long, help = "Scope")]
    #[builder(default, setter(strip_option))]
    pub(crate) sc: Option<String>,

    #[arg(
        global = true,
        short,
        long,
        default_value_t = String::from("root"),
        help = "User name"
    )]
    #[builder(default = "root".into())]
    pub(crate) user: String,

    #[arg(global = true, short, long, default_value_t = String::from("root"), help = "Password")]
    #[builder(default = "root".into())]
    pub(crate) pass: String,

    #[arg(skip)]
    #[builder(default, setter(strip_option))]
    pub(crate) db_connection: Option<Surreal<Any>>,
}

impl DatabaseConnection {
    pub async fn setup(&mut self) -> &mut Self {
        let cli_db_url = &self.url;
        let database = self.db.clone();
        let database = database.as_str();
        let namespace = self.ns.clone();
        let namespace = namespace.as_str();
        let username = self.user.clone();
        let username = username.as_str();
        let password = self.pass.clone();
        let password = password.as_str();
        let config = Config::new().capabilities(Capabilities::all());

        let config = match cli_db_url {
            UrlDb::Memory => {
                let creds = Root { username, password };
                config.user(creds)
            }
            UrlDb::Others(_s) => config,
        };

        let db_instance = connect((cli_db_url.to_string(), config)).await.unwrap();

        let db_creds = Database {
            username,
            password,
            database,
            namespace,
        };

        // TODO: Support scope signin
        db_instance
            .signin(db_creds)
            .await
            .expect("Failed to signin");
        db_instance
            .use_db(self.clone().db)
            .await
            .expect("Failed to use db");
        db_instance
            .use_ns(self.clone().sc.unwrap_or_default())
            .await
            .expect("Failed to use ns");

        self.db_connection = Some(db_instance);

        self
    }

    pub fn db(&self) -> Option<Surreal<Any>> {
        self.db_connection.clone()
    }
}

impl Default for DatabaseConnection {
    fn default() -> Self {
        DatabaseConnection::builder()
            .db("test".into())
            .ns("test".into())
            .user("root".into())
            .pass("root".into())
            .url(UrlDb::Memory)
            .build()
    }
}
