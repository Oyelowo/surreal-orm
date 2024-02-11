use proc_macro::TokenStream;
use proc_macro2::TokenStream;
use quote::quote;

use crate::models::{FieldsMeta, Relate, RelationType};

use super::{EdgeRelationModelSelectedIdent, MyFieldReceiver};

impl MyFieldReceiver {
    pub fn create_serialized_fields(&self, store: &mut FieldsMeta) {
        let relation_type = self.to_relation_type();

        let serialized_field_fmt = quote!(#crate_name::Field::new(#field_ident_serialized_fmt));

        &mut store.serializable_fields.push(serialized_field_fmt.into());

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
    }
}
