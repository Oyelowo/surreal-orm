// use std::error::Error;
// use std::fs;
// use std::path::{Path, PathBuf};
//
// use proc_macro::TokenStream;
// use quote::quote;
// use std::error::Error;
// use std::fs;
// use std::path::{Path, PathBuf};
//
// // Step 1: Custom Path Handling
// fn resolve_migration_directory(custom_path: Option<&str>) -> Result<PathBuf, Box<dyn Error>> {
//     let default_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
//     let path = match custom_path {
//         Some(p) => Path::new(p).to_owned(),
//         None => default_path,
//     };
//
//     if path.exists() && path.is_dir() {
//         Ok(path)
//     } else {
//         Err(Box::new(MigrationError::InvalidMigrationDirectory(
//             path.to_string_lossy().to_string(),
//         )))
//     }
// }
//
// // Step 2: Search for Migrations
// fn find_migration_directories(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
//     let mut migration_directories = Vec::new();
//     for entry in fs::read_dir(path)? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_dir() {
//             migration_directories.push(path);
//         }
//     }
//     if migration_directories.is_empty() {
//         Err(Box::new(MigrationError::NoMigrationDirectories))
//     } else {
//         Ok(migration_directories)
//     }
// }
//
// // Step 3: Read Migrations
// fn read_migrations(directory: &Path) -> Result<Vec<String>, Box<dyn Error>> {
//     let mut migrations = Vec::new();
//     for entry in fs::read_dir(directory)? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_file() {
//             let content = fs::read_to_string(&path)?;
//             migrations.push(content);
//         }
//     }
//     if migrations.is_empty() {
//         Err(Box::new(MigrationError::NoMigrationsFound(
//             directory.to_string_lossy().to_string(),
//         )))
//     } else {
//         Ok(migrations)
//     }
// }
//
// // Step 4: Checksum Calculation
// fn calculate_checksum(path: &Path) -> Result<String, Box<dyn Error>> {
//     use checksums::hash_file;
//     let checksum = hash_file(path)?;
//     Ok(checksum)
// }
//
// // Step 7: Generate Rust Code
// fn generate_migration_code(migrations: Vec<(String, String)>) -> proc_macro2::TokenStream {
//     let migration_code = migrations.iter().map(|(name, content)| {
//         let name_str = name.as_str();
//         let content_str = content.as_str();
//         quote! {
//             diesel_migrations::EmbeddedMigration::new(
//                 #content_str,
//                 None, // Implement "down" migrations if needed
//                 diesel_migrations::EmbeddedName::new(#name_str),
//                 diesel_migrations::TomlMetadataWrapper::new(false), // Adjust as needed
//             ),
//         }
//     });
//
//     quote! {
//         diesel_migrations::EmbeddedMigrations::new(&[
//             #(#migration_code)*
//         ])
//     }
// }
//
// // Your procedural macro entry point
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
//                     migrations.push((
//                         directory.file_name().unwrap().to_string_lossy().to_string(),
//                         file_content,
//                     ));
//                 }
//             }
//             Err(e) => {
//                 return e.to_compile_error().into();
//             }
//         }
//     }
//
//     // Step 7: Generate Rust code
//     let code = generate_migration_code(migrations);
//
//     code.into()
// }
//
// #[derive(Debug)]
// enum MigrationError {
//     InvalidMigrationDirectory(String),
//     NoMigrationDirectories,
//     NoMigrationsFound(String),
// }
// // Your procedural macro entry point
// pub fn embed_migrations(input: TokenStream) -> TokenStream {
//     // Step 1: Resolve custom path
//     let custom_path = input.to_string(); // Extract the custom path from input
//     let migration_directory = resolve_migration_directory(Some(&custom_path)).unwrap();
//
//     // Step 2: Find migration directories
//     let migration_directories = find_migration_directories(&migration_directory).unwrap();
//
//     // Step 3: Read migrations
//     let mut migrations = Vec::new();
//     for directory in &migration_directories {
//         let migration_files = read_migrations(&directory).unwrap();
//         for file_content in migration_files {
//             migrations.push((
//                 directory.file_name().unwrap().to_string_lossy().to_string(),
//                 file_content,
//             ));
//         }
//     }
//
//     // Step 7: Generate Rust code
//     let code = generate_migration_code(migrations);
//
//     code.to_string().parse().unwrap()
// }
