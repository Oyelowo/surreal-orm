use proc_macro::TokenStream;
use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, FieldsMeta, RelationType},
};

impl FieldsMeta {
    pub fn create_non_null_updater_struct_fields(&mut self) -> ExtractorResult<()> {
        let table_derive_attributes = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let field_type = field_receiver.field_type_rust();
        let field_ident_normalized =
            field_receiver.field_ident_normalized(&table_derive_attributes.casing()?)?;

        match field_receiver.to_relation_type() {
            RelationType::None
            | RelationType::NestArray(_)
            | RelationType::LinkOne(_)
            | RelationType::LinkSelf(_)
            | RelationType::LinkMany(_)
            | RelationType::List(_) => {
                self.insert_non_null_updater_token(
                    quote!(pub #field_ident_normalized: ::std::option::Option<#field_type>, )
                        .into(),
                );
            }
            RelationType::NestObject(nested_object) => {
                store.non_null_updater_fields(
                    quote!(pub #field_ident_normalized: ::std::option::Option<<#nested_object as #crate_name::Object>::NonNullUpdater>, ).into(),
                );
            }
            RelationType::Relate(_) => {}
        }

        Ok(())
    }

    fn insert_non_null_updater_token(
        &mut self,
        updater_field_token: TokenStream,
    ) -> ExtractorResult<()> {
        let table_derive_attributes = self.table_derive_attributes();
        let db_field_name = self
            .field_receiver()
            .db_field_name(&table_derive_attributes.casing()?)?;
        if db_field_name.is_updateable_by_default(self.data_type()) {
            self.non_null_updater_fields
                .push(updater_field_token.into());
        }
        // We dont care about the field type. We just use this struct to check for
        // renamed serialized field names at compile time by asserting that the a field
        // exist.
        self.renamed_serialized_fields
            .push(quote!(pub #field_ident_raw_to_underscore_suffix: &'static str, ).into());
        Ok(())
    }
}
