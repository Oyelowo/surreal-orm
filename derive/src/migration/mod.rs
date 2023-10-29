use migrator::{
    FileManager, MigrationError, MigrationFileName, MigrationFlag, MigrationOneWay,
    MigrationTwoWay, Mode,
};
use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, Expr, Result,
    Token,
};
use thiserror::Error;

fn generate_migration_code(file_manager: FileManager) -> proc_macro2::TokenStream {
    let crate_name = get_crate_name(false);

    let xx = match file_manager.migration_flag {
        MigrationFlag::OneWay => file_manager
            .get_oneway_migrations(false)
            .unwrap()
            .iter()
            .map(|meta| {
                let name = meta.name.to_string();
                let content = meta.content.to_string();
                let timestamp = meta.timestamp;
                let id = meta.id.to_string();

                quote!(#crate_name::migrator::EmbeddedMigrationOneWay {
                    id: #id,
                    name: #name,
                    timestamp: #timestamp,
                    content: #content,
                })
            })
            .collect::<Vec<_>>(),
        MigrationFlag::TwoWay => file_manager
            .get_two_way_migrations(false)
            .unwrap()
            .iter()
            .map(|meta| {
                let name = meta.name.to_string();
                let up = meta.up.to_string();
                let down = meta.down.to_string();
                let timestamp = meta.timestamp;
                let id = meta.id.clone().to_string();

                quote!(#crate_name::migrator::EmbeddedMigrationTwoWay {
                    id: #id,
                    name: #name,
                    timestamp: #timestamp,
                    up: #up,
                    down: #down,
                })
            })
            .collect::<Vec<_>>(),
    };

    let embedded_migration = match file_manager.migration_flag {
        MigrationFlag::OneWay => {
            quote!(#crate_name::migrator::EmbeddedMigrationsOneWay::new(&[#(#xx),*]))
        }
        MigrationFlag::TwoWay => quote!(
                #crate_name::migrator::EmbeddedMigrationsTwoWay::new(&[#(#xx),*])
        ),
    };
    embedded_migration
}

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
            _ => {
                // Handle other kinds of expressions
            }
        }
    }

    args
}

pub fn generate_embedded_migrations(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Args);

    if input.args.len() > 3 {
        panic!("Too many arguments. Expected 3 or less");
    }

    let mut args = parse_it(input).into_iter();

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

    let file_manager = FileManager {
        mode,
        custom_path: custom_path.as_ref().map(|x| x.to_string()),
        migration_flag: flag,
    };

    generate_migration_code(file_manager).into()
}

#[cfg(test)]
mod tests {}
