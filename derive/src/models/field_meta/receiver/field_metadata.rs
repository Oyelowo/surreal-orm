use quote::quote;

use crate::models::{
    derive_attributes::TableDeriveAttributes, field_name_serialized,
    variables::VariablesModelMacro, FieldsMeta,
};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn create_field_metada_token(
        &self,
        store: &mut FieldsMeta,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let field_definition = self.field_defintion_db(table_derive_attrs)?;
        let field_name_serialized = self.db_field_name(&table_derive_attrs.casing()?)?;
        let old_field_name = match self.old_name.as_ref() {
            Some(old_name) if !old_name.to_string().is_empty() => {
                let old_name = old_name.to_string();
                quote!(::std::option::Some(#old_name.into()))
            }
            _ => quote!(::std::option::Option::None),
        };

        if !field_definition.is_empty() {
            store
                .field_metadata
                .push(quote!(#crate_name::FieldMetadata {
                    name: #field_name_serialized.into(),
                    old_name: #old_field_name,
                    definition: ::std::vec![ #field_definition ]
                }));
        }

        Ok(())
    }
}
