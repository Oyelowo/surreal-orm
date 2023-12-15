/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{MigrationOneWay, MigrationResult, MigrationTwoWay};

#[derive(Clone, Debug)]
pub struct EmbeddedMigrationTwoWay {
    pub name: &'static str,
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
    pub const fn get_migrations(&self) -> &'static [EmbeddedMigrationTwoWay] {
        self.migrations
    }

    pub const fn new(migrations: &'static [EmbeddedMigrationTwoWay]) -> Self {
        Self { migrations }
    }

    pub fn to_migrations_two_way(&self) -> MigrationResult<Vec<MigrationTwoWay>> {
        let migs = self
            .migrations
            .iter()
            .map(|meta| {
                let name = meta.name.to_string();
                let up = meta.up.to_string().into();
                let down = meta.down.to_string().into();

                MigrationTwoWay {
                    name: name.to_string().try_into().expect("Invalid migration name"),
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
    migrations: &'static [EmbeddedMigrationOneWay],
}

impl EmbeddedMigrationsOneWay {
    pub const fn get_migrations(&self) -> &'static [EmbeddedMigrationOneWay] {
        self.migrations
    }

    pub fn to_migrations_one_way(&self) -> MigrationResult<Vec<MigrationOneWay>> {
        let migs = self
            .migrations
            .iter()
            .map(|meta| {
                let name = meta
                    .name
                    .to_string()
                    .try_into()
                    .expect("Invalid migration name");
                let content = meta.content.to_string().into();

                MigrationOneWay { name, content }
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
    pub name: &'static str,
    pub content: &'static str, // status: String,
}
