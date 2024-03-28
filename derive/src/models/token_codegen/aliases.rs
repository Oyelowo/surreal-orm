/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::models::*;

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn create_relation_aliases_struct_fields_types_kv(&mut self) -> ExtractorResult<()> {
        let crate_name = get_crate_name(false);
        let casing = self.table_derive_attributes().casing()?;
        let field_receiver = self.field_receiver();
        let db_field_name = field_receiver.db_field_name(&casing)?;
        let db_field_name_as_ident = db_field_name.as_ident();

        if let RelationType::Relate(_) = field_receiver.to_relation_type() {
            self.aliases_struct_fields_types_kv
                .push(quote!(pub #db_field_name_as_ident: #crate_name::AliasName, ).into());

            self.aliases_struct_fields_names_kv
                .push(quote!(pub #db_field_name_as_ident: #db_field_name.into(),).into());
        }
        Ok(())
    }
}
