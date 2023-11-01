use migrator::{MigrationConfig, MigrationError, MigrationFlag, Mode};
use proc_macro::TokenStream;
use quote::quote;

use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, Expr, Lit, Result,
    Token,
};

fn generate_migration_code(
    flag: MigrationFlag,
    custom_path: Option<String>,
    mode: Mode,
) -> proc_macro2::TokenStream {
    let mut files_config = MigrationConfig::new().mode(mode);
    if let Some(custom_path) = custom_path {
        files_config = files_config.custom_path(custom_path);
    }

    let crate_name = get_crate_name(false);
    let xx = match flag {
        MigrationFlag::OneWay => files_config
            .one_way()
            .get_migrations()
            .expect("Failed to get migrations")
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
        MigrationFlag::TwoWay => files_config
            .two_way()
            .get_migrations()
            .expect("Failed to get migrations")
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

    match flag {
        MigrationFlag::OneWay => {
            quote!(#crate_name::migrator::EmbeddedMigrationsOneWay::new(&[#(#xx),*]))
        }
        MigrationFlag::TwoWay => quote!(
                #crate_name::migrator::EmbeddedMigrationsTwoWay::new(&[#(#xx),*])
        ),
    }
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
                let ident = &expr_path
                    .path
                    .segments
                    .last()
                    .expect("Custom path must be path or None if you want to use the default.")
                    .ident;
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
                .expect("Invalid flag")
        });

    let mode = args.next().flatten().map_or(Mode::default(), |md| {
        md.try_into()
            .map_err(|e: MigrationError| e.to_string())
            .expect("Invalid mode")
    });

    generate_migration_code(flag, custom_path, mode).into()
}

#[cfg(test)]
mod tests {}