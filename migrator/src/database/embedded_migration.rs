/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{MigrationOneWay, MigrationResult, MigrationTwoWay};

#[derive(Clone, Debug)]
pub struct EmbeddedMigrationTwoWay {
    pub id: &'static str,
    pub name: &'static str,
    pub timestamp: u64,
    pub up: &'static str,
    pub down: &'static str,
    // status: String,
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct EmbeddedMigrationsTwoWay {
    migrations: &'static [EmbeddedMigrationTwoWay],
}

impl EmbeddedMigrationsTwoWay {
    pub const fn new(migrations: &'static [EmbeddedMigrationTwoWay]) -> Self {
        Self { migrations }
    }

    pub(crate) fn to_migrations_two_way(&self) -> MigrationResult<Vec<MigrationTwoWay>> {
        let migs = self
            .migrations
            .iter()
            .map(|meta| {
                let name = meta.name.to_string();
                let up = meta.up.to_string();
                let down = meta.down.to_string();
                let timestamp = meta.timestamp;
                let id = meta.id.to_string();

                MigrationTwoWay {
                    id: id.try_into().unwrap(),
                    name,
                    timestamp,
                    up,
                    down,
                    directory: None,
                }
            })
            .collect::<Vec<_>>();
        Ok(migs)
    }
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct EmbeddedMigrationsOneWay {
    pub migrations: &'static [EmbeddedMigrationOneWay],
}

impl EmbeddedMigrationsOneWay {
    pub fn to_migrations_one_way(&self) -> MigrationResult<Vec<MigrationOneWay>> {
        let migs = self
            .migrations
            .iter()
            .map(|meta| {
                let name = meta.name.to_string();
                let content = meta.content.to_string();
                let timestamp = meta.timestamp;
                let id = meta.id.to_string();

                MigrationOneWay {
                    id: id.try_into().unwrap(),
                    name,
                    timestamp,
                    content,
                }
            })
            .collect::<Vec<_>>();
        Ok(migs)
    }
}

impl EmbeddedMigrationsOneWay {
    pub const fn new(migrations: &'static [EmbeddedMigrationOneWay]) -> Self {
        Self { migrations }
    }
}

#[derive(Clone, Debug)]
pub struct EmbeddedMigrationOneWay {
    pub id: &'static str,
    pub name: &'static str,
    pub timestamp: u64,
    pub content: &'static str, // status: String,
}
