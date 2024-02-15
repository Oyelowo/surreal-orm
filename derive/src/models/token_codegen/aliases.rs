/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{errors::ExtractorResult, models::RelationType};

use super::Codegen;

impl Codegen {
    pub fn create_relation_aliases_struct_fields_types_kv(
        &mut self,
    ) -> ExtractorResult<Vec<String>> {
        let field_receiver = self.field_receiver();
        let casing = self.table_derive_attributes().casing()?;
        let field_ident_normalized = field_receiver.field_ident_normalized(&casing)?;
        let db_field_name = field_receiver.db_field_name(&casing)?;
        if let RelationType::Relate(_) = field_receiver.to_relation_type() {
            self.aliases_struct_fields_types_kv
                .push(quote!(pub #field_ident_normalized: #crate_name::AliasName, ).into());

            self.aliases_struct_fields_names_kv
                .push(quote!(pub #field_ident_normalized: #db_field_name.into(),).into());
        }
    }
}
