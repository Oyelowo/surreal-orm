/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{FileManager, MigrationFlag};
use quote::{format_ident, quote};

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
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct EmbeddedMigrationsOneWay {
    pub migrations: &'static [EmbeddedMigrationOneWay],
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

// pub fn generate_migration_code(
//     file_manager: FileManager,
//     path: &String,
// ) -> proc_macro2::TokenStream {
//     let crate_name = format_ident!("surreal_orm");
//     // let crate_name = get_crate_name(false);
//
//     let embedded_migrations = match file_manager.migration_flag {
//         MigrationFlag::OneWay => file_manager
//             .get_oneway_migrations(false)
//             .unwrap()
//             .iter()
//             .map(|meta| {
//                 let name = meta.name.to_string();
//                 let content = meta.content.to_string();
//                 let timestamp = meta.timestamp;
//                 let id = meta.id.to_string();
//
//                 quote!(#crate_name::migrator::EmbeddedMigrationOneWay {
//                     id: #id,
//                     name: #name,
//                     timestamp: #timestamp,
//                     content: #content,
//                 })
//             })
//             .collect::<Vec<_>>(),
//         MigrationFlag::TwoWay => file_manager
//             .get_two_way_migrations(false)
//             .unwrap()
//             .iter()
//             .map(|meta| {
//                 let name = meta.name.to_string();
//                 let up = meta.up.to_string();
//                 let down = meta.down.to_string();
//                 let timestamp = meta.timestamp;
//                 let id = meta.id.clone().to_string();
//
//                 quote!(#crate_name::migrator::EmbeddedMigrationTwoWay {
//                     id: #id,
//                     name: #name,
//                     timestamp: #timestamp,
//                     up: #up,
//                     down: #down,
//                 })
//             })
//             .collect::<Vec<_>>(),
//     };
//
//     let embedded_migration = match file_manager.migration_flag {
//         MigrationFlag::OneWay => {
//             quote!(#crate_name::migrator::EmbeddedMigrationsOneWay::new(&[#(#embedded_migrations),*]))
//         }
//         MigrationFlag::TwoWay => {
//             quote!(#crate_name::migrator::EmbeddedMigrationsTwoWay::new(&[#(#embedded_migrations),*]))
//         }
//     };
//     embedded_migration
// }
