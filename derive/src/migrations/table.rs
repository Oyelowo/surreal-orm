/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromDeriveInput;
use surreal_derive_helpers::migrations::table::TableMigrationSchemaDeriveAttributes;
use syn::parse_macro_input;
use quote::quote;

pub fn generate_table_resources_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = match TableMigrationSchemaDeriveAttributes::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
