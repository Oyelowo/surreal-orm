/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

use quote::quote;

use crate::models::*;

use super::Codegen;

impl Codegen<'_> {
    pub fn create_table_id_type_token(&mut self) -> ExtractorResult<()> {
        let field_receiver = self.field_receiver();
        let table_derive_attrs = self.table_derive_attributes();
        let field_ident_serialized_fmt =
            field_receiver.db_field_name(&table_derive_attrs.casing()?)?;
        let field_type = &field_receiver.ty();

        if field_ident_serialized_fmt.is_id() {
            self.table_id_type = quote!(#field_type).into();
            // self.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::SurrealId<#struct_name_ident>);));
        }

        Ok(())
    }

    pub fn create_db_field_names_token(&mut self) -> ExtractorResult<()> {
        let db_field_name = self
            .field_receiver()
            .db_field_name(&self.table_derive_attributes().casing()?)?;

        self.serialized_field_names_normalised
            .push(db_field_name.to_token_stream().into());

        Ok(())
    }
}
