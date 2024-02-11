use crate::models::{FieldsMeta, RelationType};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn create_relation_aliases_struct_fields_types_kv(
        &self,
        store: &mut FieldsMeta,
    ) -> Vec<String> {
        if let RelationType::Relate(_) = self.to_relation_type() {
            store.aliases_struct_fields_types_kv.push(
                quote!(pub #field_ident_raw_to_underscore_suffix: #crate_name::AliasName, ).into(),
            );

            store.aliases_struct_fields_names_kv.push(
                quote!(#field_ident_raw_to_underscore_suffix: #field_ident_serialized_fmt.into(),)
                    .into(),
            );
        }
    }
}
