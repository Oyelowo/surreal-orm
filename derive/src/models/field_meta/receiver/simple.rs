use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{
        derive_attributes::TableDeriveAttributes, field_name_serialized,
        variables::VariablesModelMacro, FieldsMeta, RelationType,
    },
};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    pub fn create_field_metadata_token(
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

    pub fn create_field_type_static_assertion_token(
        &self,
        store: &mut FieldsMeta,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<()> {
        let field_type = &self.field_type_rust();

        match self.to_relation_type() {
            RelationType::Relate(relate) => {}
            RelationType::LinkOne(foreign_node) => {
                store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkOne<#foreign_node>);).into());
            }
            RelationType::LinkSelf(self_node) => {
                let current_struct_type = table_derive_attrs.struct_as_path_no_bounds();
                store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#current_struct_type, #crate_name::LinkSelf<#foreign_node>);).into());

                store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkSelf<#foreign_node>);).into());
            }
            RelationType::LinkMany(foreign_node) => {
                store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkMany<#foreign_node>);).into());
            }
            RelationType::NestObject(foreign_object) => {
                store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #foreign_object);).into());
            }
            RelationType::NestArray(foreign_array_object) => {
                let nesting_level = count_vec_nesting(field_type);
                let nested_vec_type = generate_nested_vec_type(&foreign_node, nesting_level);

                store.static_assertions.push(quote! {
                        #crate_name::validators::assert_type_eq_all!(#foreign_array_object, #nested_vec_type);
                    });
            }
            RelationType::List(_) => {}
            RelationType::None => {}
        }

        Ok(())
    }
}
