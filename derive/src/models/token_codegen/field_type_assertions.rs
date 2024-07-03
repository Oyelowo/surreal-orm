/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// ---
use crate::models::*;
use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn create_field_type_static_assertion_token(&mut self) -> ExtractorResult<()> {
        let crate_name = &get_crate_name(false);
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let field_type = &field_receiver
            .ty()
            .remove_non_static_lifetime_and_reference()
            .replace_self_with_current_struct_concrete_type(table_derive_attrs)?;

        let field_type_meta = field_receiver.field_type_db_with_static_assertions(table_derive_attrs)?;
        self.static_assertions
            .extend(vec![field_type_meta.unwrap_or_default().static_assertion_token]);

        Ok(())
    }

}
