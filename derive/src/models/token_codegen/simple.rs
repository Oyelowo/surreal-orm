/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{
        count_vec_nesting, derive_attributes::TableDeriveAttributes, field_name_serialized,
        generate_nested_vec_type, variables::VariablesModelMacro, RelationType,
    },
};

use super::Codegen;

impl Codegen {
    pub fn create_table_id_type_token(&mut self) -> ExtractorResult<()> {
        let fr = self.field_receiver();
        let table_derive_attrs = self.table_derive_attributes();
        let field_ident_serialized_fmt = fr.db_field_name(&table_derive_attrs.casing()?)?;
        let field_type = &fr.field_type_rust();

        if field_ident_serialized_fmt.is_id() {
            self.table_id_type = quote!(#field_type);
            // self.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::SurrealId<#struct_name_ident>);));
        }

        Ok(())
    }

    pub fn create_db_field_names_token(&mut self) -> ExtractorResult<()> {
        let field_db_field_name = self
            .field_receiver()
            .db_field_name(&self.table_derive_attributes().casing()?)?;

        self.serialized_field_names_normalised
            .push(field_db_field_name.to_owned().into());

        Ok(())
    }
}
