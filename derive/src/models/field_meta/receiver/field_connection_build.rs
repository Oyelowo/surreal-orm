use quote::quote;

use crate::models::{
    derive_attributes::TableDeriveAttributes, field_name_serialized,
    variables::VariablesModelMacro, FieldsMeta,
};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn create_field_connection_builder_token(
        &self,
        store: &mut FieldsMeta,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        if self.to_relation_type().is_relate_graph() {
            // Relate graph fields are readonly and derived and only used in
            // our rust code but not in the database schema
            return Ok(());
        };

        // TODO: Decide if to use table_derive_attrs or store.table_derive_attributes()
        // let table_derive_attrs = store.table_derive_attributes();
        let field_ident_normalized = self.field_ident_normalized(&table_derive_attrs.casing()?)?;
        let field_name_serialized = self.db_field_name(&table_derive_attrs.casing()?)?;
        let field_name_pascalized = self.field_name_pascalized(table_derive_attrs);
        let VariablesModelMacro {
            _____field_names,
            schema_instance,
            ..
        } = VariablesModelMacro::new();

        store.schema_struct_fields_types_kv.push(
            quote!(pub #field_ident_normalized: #_____field_names::#field_name_pascalized, ).into(),
        );

        store
            .schema_struct_fields_names_kv
            .push(quote!(#field_ident_normalized: #field_name_serialized.into(),).into());

        store.schema_struct_fields_names_kv_prefixed
                        .push(quote!(#field_ident_normalized:
                                            #crate_name::Field::new(format!("{}.{}", prefix.build(), #field_name_serialized))
                                            .with_bindings(prefix.get_bindings()).into(),).into());

        store
            .schema_struct_fields_names_kv_empty
            .push(quote!(#field_ident_normalized: "".into(),).into());

        store.connection_with_field_appended
           .push(quote!(
                                #schema_instance.#field_ident_normalized = #schema_instance.#field_ident_normalized
                                  .set_graph_string(format!("{}.{}", #___________graph_traversal_string, #field_name_serialized))
                                        .#____________update_many_bindings(#bindings).into();
                            ).into());

        Ok(())
    }
}
