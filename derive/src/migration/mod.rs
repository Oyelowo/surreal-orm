use migrator::{
    EmbeddedMigrations, FileManager, MigrationError, MigrationFileName, MigrationFlag,
    MigrationOneWay, MigrationTwoWay, Mode,
};
use proc_macro::TokenStream;
// use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

// #[derive(Error, Debug)]
// enum MigrationError {
//     InvalidMigrationDirectory(String),
//     NoMigrationDirectories,
//     NoMigrationsFound(String),
// }

// Step 4: Checksum Calculation
// fn calculate_checksum(path: &Path) -> Result<String, Box<dyn Error>> {
//     use checksums::hash_file;
//     let checksum = hash_file(path)?;
//     Ok(checksum)
// }

enum MigrationType {
    OneWay,
    TwoWay,
}

// Step 7: Generate Rust Code
fn generate_migration_code(file_manager: FileManager, path: &String) -> proc_macro2::TokenStream {
    let crate_name = get_crate_name(false);

    let xx = match file_manager.migration_flag {
        MigrationFlag::OneWay => file_manager
            .get_oneway_migrations()
            .unwrap()
            .iter()
            .map(|x| {
                let name = x.name.to_string();
                let content = x.content.to_string();
                let timestamp = x.timestamp;
                let id = x.id.to_string();
                    // id: #id.to_string().try_into().expect("Invalid filename as format. Must be in format <timestamp>_<name>.<up|down|<None>>.surql"),
                quote!(#crate_name::migrator::EmbeddedMigrationOneWay {
                    id: #id,
                    name: #name,
                    timestamp: #timestamp,
                    content: #content,
                    // content: include_str!(#path),
                })
            })
            .collect::<Vec<_>>(),
        MigrationFlag::TwoWay => file_manager
            .get_two_way_migrations()
            .unwrap()
            .iter()
            .map(|x| {
                let name = x.name.clone();
                let up = x.up.clone();
                let down = x.down.clone();
                let timestamp = x.timestamp.clone();
                let id = x.id.clone().to_string();
                quote!(#crate_name::migrator::MigrationTwoWay {
                    id: #id.to_string().try_into().expect("Invalid filename as format. Must be in format <timestamp>_<name>.<up|down|<None>>.surql"),
                    name: #name.into(),
                    timestamp: #timestamp.into(),
                    up: #up.into(),
                    down: #down.into(),
                })
            })
            .collect::<Vec<_>>(),
    };

    let pp = match file_manager.migration_flag {
        // EmbeddedMigrationsTwoWay
        MigrationFlag::OneWay => {
            quote!(#crate_name::migrator::EmbeddedMigrationsOneWay::new(&[#(#xx),*]))
        }
        MigrationFlag::TwoWay => quote!(
                #crate_name::migrator::EmbeddedMigrationsTwoWay {
                    migrations: &[#(#xx),*]
                }
        ),
    };
    // let xxv = quote!(::std::vec![#(#xx),*]);
    // // panic!("{}", xxv.to_string());
    // quote!(::std::vec![#(#xx),*])
    pp
}

use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, Expr, Result,
    Token,
};

use crate::models::get_crate_name;

struct Args {
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = Punctuated::parse_terminated_with(input, Expr::parse)?;
        Ok(Self { args })
    }
}

fn parse_it(input: Args) -> Vec<Option<String>> {
    use syn::{Expr, Lit};

    // This is pseudo-code and may require adaptation.
    let mut args: Vec<Option<String>> = Vec::new();
    for arg in &input.args {
        match arg {
            Expr::Lit(expr_lit) => {
                match &expr_lit.lit {
                    Lit::Str(lit_str) => {
                        // Remove quotes from string literals
                        args.push(Some(lit_str.value()));
                    }
                    _ => {
                        // Handle other literal types, if necessary
                    }
                }
            }
            Expr::Path(expr_path) => {
                // Here arg is an identifier like an enum variant
                let ident = &expr_path.path.segments.last().unwrap().ident;
                let ident_str = ident.to_string();

                if ident_str == "None" {
                    args.push(None);
                } else {
                    args.push(Some(ident_str));
                }
            }
            // ... Other Expr variants
            _ => {
                // Handle other kinds of expressions
            }
        }
    }

    args
}

// Your procedural macro entry point
pub fn generate_embedded_migrations(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Args);

    if input.args.len() > 3 {
        panic!("Too many arguments. Expected 3 or less");
    }

    // let mut args = input.args.iter().map(|arg| quote! {#arg}.to_string());
    let mut args = parse_it(input).into_iter();
    // let custom_path = args
    //     .next()
    //     .map_or(None, |path| if path == "None" { None } else { Some(path) });

    let custom_path = args.next().flatten().clone();
    let flag = args
        .next()
        .flatten()
        .map_or(MigrationFlag::default(), |fl| {
            fl.try_into()
                .map_err(|e: MigrationError| e.to_string())
                .unwrap()
        });

    let mode = args.next().flatten().map_or(Mode::default(), |md| {
        md.try_into()
            .map_err(|e: MigrationError| e.to_string())
            .unwrap()
    });

    // let custom_path = match args.next() {
    //     Some(path) if path == "None" || path.is_empty() => None,
    //     Some(path) => {
    //         // panic!("{}", path);
    //         Some(path.to_string())
    //     }
    //     None => None,
    // };
    //
    // let flag = args.next().map_or(MigrationFlag::default(), |fl| {
    //     fl.try_into()
    //         .map_err(|e: MigrationError| e.to_string())
    //         .unwrap()
    // });
    // let mode = args.next().map_or(Mode::default(), |md| {
    //     md.try_into()
    //         .map_err(|e: MigrationError| e.to_string())
    //         .unwrap()
    // });

    let file_manager = FileManager {
        mode,
        custom_path: custom_path.as_ref().map(|x| x.to_string()),
        migration_flag: flag,
    };

    generate_migration_code(file_manager, custom_path.as_ref().unwrap()).into()
}

#[cfg(test)]
mod tests {}
