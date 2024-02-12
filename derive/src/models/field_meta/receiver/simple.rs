use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{
        count_vec_nesting, derive_attributes::TableDeriveAttributes, field_name_serialized,
        generate_nested_vec_type, variables::VariablesModelMacro, FieldsMeta, RelationType,
    },
};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn create_table_id_type_token(
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

        Ok(())
    }
    pub fn create_db_field_names_token(
        &self,
        store: &mut FieldsMeta,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let field_db_field_names = self.field_name_serialized(&table_derive_attrs.casing()?)?;

        store
            .serialized_field_names_normalised
            .push(field_db_field_names.to_owned().into());

        Ok(())
    }
}
