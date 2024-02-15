/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromDeriveInput;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[derive(Debug, FromDeriveInput)]
pub struct TableMigrationSchemaDeriveAttributes {
    pub(crate) ident: syn::Ident,
    pub(crate) generics: syn::Generics,
}

impl ToTokens for TableMigrationSchemaDeriveAttributes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_name = get_crate_name(false);
        let TableMigrationSchemaDeriveAttributes {
            ident: struct_name_ident,
            generics,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        tokens.extend(quote! {
            impl #impl_generics #crate_name::TableResources for #struct_name_ident #ty_generics #where_clause {}
        });
    }
}

pub fn generate_table_resources_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = match TableMigrationSchemaDeriveAttributes::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
