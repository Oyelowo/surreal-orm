/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromDeriveInput;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};

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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_table_migration_schema_derive_attributes() {
        let input = parse_quote! {
            struct TableMigrationSchema;
        };

        let derive_input = TableMigrationSchemaDeriveAttributes::from_derive_input(&input).unwrap();
        let mut tokens = proc_macro2::TokenStream::new();
        derive_input.to_tokens(&mut tokens);

        let expected = quote! {
            impl surreal_orm::TableResources for TableMigrationSchema {}
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
