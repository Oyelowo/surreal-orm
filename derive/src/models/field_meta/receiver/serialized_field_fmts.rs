use proc_macro2::TokenStream;

use crate::models::{FieldsMeta, Relate, RelationType};

use super::{EdgeRelationModelSelectedIdent, MyFieldReceiver};

impl MyFieldReceiver {
    pub fn gather_serialized_fields(&self, store: &mut FieldsMeta) {
        let relation_type = self.to_relation_type();

        self.update_ser_field_type(&mut store.serializable_fields);

        match relation_type {
            RelationType::LinkOne(_) => {
                self.update_ser_field_type(&mut store.link_one_fields);
                self.update_ser_field_type(&mut store.link_one_and_self_fields);
                self.update_ser_field_type(&mut store.linked_fields);
            }
            RelationType::LinkSelf(_) => {
                self.update_ser_field_type(&mut store.link_self_fields);
                self.update_ser_field_type(&mut store.link_one_and_self_fields);
                self.update_ser_field_type(&mut store.linked_fields);
            }
            RelationType::LinkMany(_) => {
                self.update_ser_field_type(&mut store.link_many_fields);
                self.update_ser_field_type(&mut store.linked_fields);
            }
            _ => {}
        }
    }

    fn update_ser_field_type(&self, serializable_field_type: &mut Vec<TokenStream>) {
        if !self.skip_serializing && !self.skip {
            serializable_field_type
                .push(quote!(#crate_name::Field::new(#field_ident_serialized_fmt)));
        };
    }
}
