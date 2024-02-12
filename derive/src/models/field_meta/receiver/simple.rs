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

    pub fn create_simple_meta(
        &self,
        store: &mut FieldsMeta,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        // TODO: Decide if to use table_derive_attrs or store.table_derive_attributes()
        // let table_derive_attrs = store.table_derive_attributes();
        let field_ident_normalized = self.field_ident_normalized(&table_derive_attrs.casing()?)?;
        let field_name_pascalized = self.field_name_pascalized(table_derive_attrs);
        let field_name_serialized = self.field_name_serialized(&table_derive_attrs.casing()?)?;
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
    }
}
