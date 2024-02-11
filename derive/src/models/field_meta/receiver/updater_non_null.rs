use proc_macro::TokenStream;
use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, FieldsMeta, RelationType},
};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn create_non_null_updater_struct_fields(
        &self,
        store: &mut FieldsMeta,
        table_derive_attributes: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let field_type = self.field_type_rust();
        let field_ident_normalized =
            self.field_ident_normalized(&table_derive_attributes.casing()?)?;

        match self.to_relation_type() {
            RelationType::LinkOne(_) => {
                store.non_null_updater_fields.push(
                    quote!(pub #field_ident_normalized: ::std::option::Option<#field_type>, )
                        .into(),
                );
            }
            RelationType::LinkSelf(_) => {
                store.non_null_updater_fields.push(
                    quote!(pub #field_ident_normalized: ::std::option::Option<#field_type>, )
                        .into(),
                );
            }
            RelationType::LinkMany(_) => {
                store.non_null_updater_fields.push(
                    quote!(pub #field_ident_normalized: ::std::option::Option<#field_type>, )
                        .into(),
                );
            }
            RelationType::NestObject(_) => {
                store.non_null_updater_fields(
                    quote!(pub #field_ident_normalized: ::std::option::Option<<#field_type as #crate_name::Object>::NonNullUpdater>, ).into(),
                );
            }
            RelationType::None | RelationType::List(_) | RelationType::NestArray(_) => {
                store.non_null_updater_fields.push(
                    quote!(pub #field_ident_normalized: ::std::option::Option<#field_type>, )
                        .into(),
                );
            }
            _ => {}
        }

        Ok(())
    }

    fn insert_non_null_updater_token(
        &self,
        store: &mut FieldsMeta,
        updater_field_token: TokenStream,
    ) {
        // let is_invalid =
        //     &["id", "in", "out"].contains(&field_ident_normalised_as_str.as_str());
        // if !is_invalid {
        //     store
        //         .non_null_updater_fields
        //         .push(updater_field_token.clone());
        // }
        store
            .non_null_updater_fields
            .push(updater_field_token.clone().into());
        // We dont care about the field type. We just use this struct to check for
        // renamed serialized field names at compile time by asserting that the a field
        // exist.
        store
            .renamed_serialized_fields
            .push(quote!(pub #field_ident_raw_to_underscore_suffix: &'static str, ).into());
    }
}
