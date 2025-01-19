/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

// ---
use super::Codegen;
use crate::models::*;

impl Codegen<'_> {
    pub fn create_field_type_static_assertion_token(&mut self) -> ExtractorResult<()> {
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();

        let field_type_meta =
            field_receiver.field_type_db_with_static_assertions(table_derive_attrs)?;
        self.static_assertions.extend(vec![
            field_type_meta.unwrap_or_default().static_assertion_token,
        ]);

        Ok(())
    }
}
