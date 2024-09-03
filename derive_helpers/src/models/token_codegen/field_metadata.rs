/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

use quote::quote;

use crate::models::*;

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn create_field_metadata_token(&mut self) -> ExtractorResult<()> {
        let crate_name = get_crate_name(false);
        let field_receiver = self.field_receiver();
        let table_derive_attrs = self.table_derive_attributes();
        let field_definition = self.field_defintion_db()?;
        let field_name_serialized = field_receiver.db_field_name(&table_derive_attrs.casing()?)?;

        let old_field_name = match field_receiver.old_name.as_ref() {
            Some(old_name) if !old_name.to_string().is_empty() => {
                let old_name = old_name.to_string();
                quote!(::std::option::Option::Some(#old_name.into()))
            }
            _ => quote!(::std::option::Option::None),
        };

        if !field_definition.is_empty() {
            self.field_metadata.push(
                quote!(#crate_name::FieldMetadata {
                    name: #field_name_serialized.into(),
                    old_name: #old_field_name,
                    definition: ::std::vec![ #( #field_definition ),*]
                })
                .into(),
            );
        }

        Ok(())
    }
}
