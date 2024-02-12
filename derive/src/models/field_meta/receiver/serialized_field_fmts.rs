use proc_macro::TokenStream;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, FieldsMeta, Relate, RelationType},
};

use super::{EdgeRelationModelSelectedIdent, MyFieldReceiver};

impl MyFieldReceiver {
    pub fn create_db_fields_for_links_and_loaders(
        &self,
        store: &mut FieldsMeta,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let relation_type = self.to_relation_type();
        let db_field_name = self.db_field_name(&table_derive_attrs.casing()?)?;

        let serialized_field_fmt = quote!(#crate_name::Field::new(#db_field_name));

        &mut store
            .serialized_fmt_db_field_names_instance
            .push(serialized_field_fmt.into());

        if !self.skip_serializing && !self.skip {
            match relation_type {
                RelationType::LinkOne(_) => {
                    store.link_one_fields.push(serialized_field_fmt.into());
                    store
                        .link_one_and_self_fields
                        .push(serialized_field_fmt.into());
                    store.linked_fields.push(serialized_field_fmt.into());
                }
                RelationType::LinkSelf(_) => {
                    store.link_self_fields.push(serialized_field_fmt.into());
                    store
                        .link_one_and_self_fields
                        .push(serialized_field_fmt.into());
                    store.linked_fields.push(serialized_field_fmt.into());
                }
                RelationType::LinkMany(_) => {
                    store.link_many_fields.push(serialized_field_fmt.into());
                    store.linked_fields.push(serialized_field_fmt.into());
                }
                _ => {}
            }
        }
        Ok(())
    }
}
