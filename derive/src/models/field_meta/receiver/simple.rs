use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{
        derive_attributes::TableDeriveAttributes, field_name_serialized,
        variables::VariablesModelMacro, FieldsMeta,
    },
};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn create_field_metada_token(
        &self,
        store: &mut FieldsMeta,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let field_definition = self.field_defintion_db(table_derive_attrs)?;
        let field_name_serialized = self.field_name_serialized(&table_derive_attrs.casing()?)?;
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

    pub fn create_simple_tokens(
        &self,
        store: &mut FieldsMeta,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let field_ident_serialized_fmt =
            self.field_name_serialized(&table_derive_attrs.casing()?)?;
        let field_type = &self.field_type_rust();

        if field_ident_serialized_fmt.is_id() {
            store.table_id_type = quote!(#field_type);
            // store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::SurrealId<#struct_name_ident>);));
        }

        store
            .serialized_field_names_normalised
            .push(field_ident_serialized_fmt.to_owned().into());

        Ok(())
    }
}
