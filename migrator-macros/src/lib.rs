// use migrator::{MigrationFileName, MigrationOneWay, MigrationTwoWay};
// use proc_macro::TokenStream;
// // use proc_macro2::TokenStream;
// use quote::quote;
// use std::fs;
// use std::path::{Path, PathBuf};
// use thiserror::Error;
//
// #[derive(Error, Debug)]
// enum MigrationError {
//     InvalidMigrationDirectory(String),
//     NoMigrationDirectories,
//     NoMigrationsFound(String),
// }
//
// // Step 1: Custom Path Handling
// fn resolve_migration_directory(custom_path: Option<&str>) -> Result<PathBuf, MigrationError> {
//     let default_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
//     let path = custom_path.map_or(default_path, Path::new).to_owned();
//
//     if path.exists() && path.is_dir() {
//         Ok(path)
//     } else {
//         Err(MigrationError::InvalidMigrationDirectory(
//             path.to_string_lossy().to_string(),
//         ))
//     }
// }
//
// // Step 2: Search for Migrations
// fn find_migration_directories(path: &Path) -> Result<Vec<PathBuf>, MigrationError> {
//     let mut migration_directories = Vec::new();
//     for entry in fs::read_dir(path)? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_dir() {
//             migration_directories.push(path);
//         }
//     }
//     if migration_directories.is_empty() {
//         return Err(MigrationError::NoMigrationDirectories);
//     } else {
//         Ok(migration_directories)
//     }
// }
//
// enum MigrationContent {
//     OneWay(Vec<MigrationOneWay>),
//     TwoWay(Vec<MigrationTwoWay>),
// }
//
// // Step 3: Read Migrations
// fn read_migrations(directory: &Path) -> Result<MigrationContent, MigrationError> {
//     let mut migrations = Vec::new();
//     for entry in fs::read_dir(directory)? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_file() {
//             let x: MigrationFileName = path
//                 .file_name()
//                 .clone()
//                 .try_into()
//                 .expect("Problem converting migration file name");
//
//             match x {
//                 MigrationFileName::Up(meta) => {
//                     let up = fs::read_to_string(&path)?;
//
//                     let content = fs::read_to_string(&path)?;
//                     migrations.push(content);
//                 }
//                 MigrationFileName::Down(_) => {
//                     let content = fs::read_to_string(&path)?;
//                     migrations.push(content);
//                 }
//                 MigrationFileName::Unidirectional(_) => todo!(),
//             };
//             let content = fs::read_to_string(&path)?;
//             migrations.push(content);
//         }
//     }
//     if migrations.is_empty() {
//         Err(MigrationError::NoMigrationsFound(
//             directory.to_string_lossy().to_string(),
//         ))
//     } else {
//         Ok(migrations)
//     }
// }
//
// // Step 4: Checksum Calculation
// // fn calculate_checksum(path: &Path) -> Result<String, Box<dyn Error>> {
// //     use checksums::hash_file;
// //     let checksum = hash_file(path)?;
// //     Ok(checksum)
// // }
//
// enum MigrationType {
//     OneWay,
//     TwoWay,
// }
//
// // Step 7: Generate Rust Code
// fn generate_migration_code(
//     migrations: Vec<FileMeta>,
//     mig_type: MigrationType,
// ) -> proc_macro2::TokenStream {
//     let migration_code = migrations.iter().map(
//         |FileMeta {
//              file_path,
//              file_content,
//          }| {
//             let migration_name: MigrationFileName = file_path
//                 .clone()
//                 .try_into()
//                 .expect("Problem converting migration file name");
//
//             // let name_str = name.as_str();
//             // let content_str = content.as_str();
//             let two_way = migrator::MigrationTwoWay {
//                 id: migration_name.to_string(),
//                 name: migration_name.basename(),
//                 timestamp: migration_name.timestamp(),
//                 up: file_content.clone(),
//                 down: "test".to_string(),
//             };
//
//             let one_way = migrator::MigrationOneWay {
//                 id: 1,
//                 name: "test".to_string(),
//                 timestamp: "test".to_string(),
//                 content: todo!(),
//             };
//
//             quote! {
//                 diesel_migrations::EmbeddedMigration::new(
//                     #content_str,
//                     None, // Implement "down" migrations if needed
//                     diesel_migrations::EmbeddedName::new(#name_str),
//                     diesel_migrations::TomlMetadataWrapper::new(false), // Adjust as needed
//                 ),
//             }
//         },
//     );
//
//     quote! {
//         diesel_migrations::EmbeddedMigrations::new(&[
//             #(#migration_code)*
//         ])
//     }
// }
//
// struct FileMeta {
//     file_path: PathBuf,
//     file_content: String,
// }
//
// // Your procedural macro entry point
// #[proc_macro]
// pub fn embed_migrations(input: TokenStream) -> TokenStream {
//     // Step 1: Resolve custom path
//     let custom_path = input.to_string();
//     let migration_directory = match resolve_migration_directory(Some(&custom_path)) {
//         Ok(path) => path,
//         Err(e) => {
//             return e.to_compile_error().into();
//         }
//     };
//
//     // Step 2: Find migration directories
//     let migration_directories = match find_migration_directories(&migration_directory) {
//         Ok(directories) => directories,
//         Err(e) => {
//             return e.to_compile_error().into();
//         }
//     };
//
//     // Step 3: Read migrations
//     let mut migrations = Vec::new();
//     for directory in &migration_directories {
//         match read_migrations(&directory) {
//             Ok(migration_files) => {
//                 for file_content in migration_files {
//                     migrations.push(FileMeta {
//                         file_path: directory.file_name().unwrap().to_string_lossy().to_string(),
//                         file_content,
//                     });
//                 }
//             }
//             Err(e) => {
//                 return e.to_compile_error().into();
//             }
//         }
//     }
//
//     // Step 7: Generate Rust code
//     let code = generate_migration_code(migrations, MigrationType::OneWay);
//
//     code.into()
// }
//
// #[cfg(test)]
// mod tests {}
