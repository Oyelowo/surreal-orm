/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{FileMetadata, MigrationFileOneWay, MigrationFileTwoWayPair, MigrationResult};

#[derive(Clone, Debug)]
pub struct EmbeddedMigrationTwoWay {
    pub up: &'static FileMetadataStatic,
    pub down: &'static FileMetadataStatic,
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

    pub const fn get_migrations(&self) -> &'static [EmbeddedMigrationTwoWay] {
        self.migrations
    }

    pub fn to_migrations_two_way(&self) -> MigrationResult<Vec<MigrationFileTwoWayPair>> {
        let migs = self
            .migrations
            .iter()
            .map(|meta| {
                let up_name = meta
                    .up
                    .name
                    .to_string()
                    .try_into()
                    .expect("Invalid migration name");
                let up_content = meta.up.content.to_string().into();
                let down_name = meta
                    .up
                    .name
                    .to_string()
                    .try_into()
                    .expect("Invalid migration name");
                let down_content = meta.down.content.to_string().into();

                MigrationFileTwoWayPair {
                    up: FileMetadata::new(up_name, up_content),
                    down: FileMetadata::new(down_name, down_content),
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

    pub fn to_migrations_one_way(&self) -> MigrationResult<Vec<MigrationFileOneWay>> {
        let migs = self
            .migrations
            .iter()
            .map(|meta| {
                let name = meta
                    .name()
                    .to_string()
                    .try_into()
                    .expect("Invalid migration name");
                let content = meta.content().to_string().into();

                MigrationFileOneWay::new(FileMetadata { name, content })
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
pub struct EmbeddedMigrationOneWay(pub &'static FileMetadataStatic);

impl EmbeddedMigrationOneWay {
    pub fn name(&self) -> &'static str {
        self.0.name
    }

    pub fn content(&self) -> &'static str {
        self.0.content
    }
}

#[derive(Clone, Debug)]
pub struct FileMetadataStatic {
    pub name: &'static str,
    pub content: &'static str, // status: String,
}
