/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::quote;

use crate::models::*;

use super::Codegen;
create_tokenstream_wrapper!(=> StructPartialFieldType);

impl<'a> Codegen<'a> {
    pub fn create_struct_partial_metadata(&mut self) -> ExtractorResult<()> {
        let crate_name = get_crate_name(false);
        let table_derive_attributes = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let field_type = field_receiver.ty();
        let field_ident_normalized =
            field_receiver.field_ident_normalized(&table_derive_attributes.casing()?)?;
        let db_field_name = field_receiver.db_field_name(&table_derive_attributes.casing()?)?;

        if db_field_name.is_id() {
            return Ok(());
        }

        let is_option = field_receiver
            .field_type_db(table_derive_attributes)?
            .into_inner()
            .is_option();
        let maybe_fn_path = format!("{crate_name}::Maybe::is_none");
        match field_receiver.to_relation_type() {
            RelationType::None
            | RelationType::NestArray(_)
            | RelationType::LinkOne(_)
            | RelationType::LinkSelf(_)
            | RelationType::LinkMany(_)
            | RelationType::List(_) => {
                let optionalized_field_type = quote!(#crate_name:: Maybe<#field_type>);
                self.insert_struct_partial_field_type_def_meta(quote!(
                    #[serde(skip_serializing_if = #maybe_fn_path, rename = #db_field_name)]
                    pub #field_ident_normalized: #optionalized_field_type,
                ))?;

                self.insert_struct_partial_builder_fields_methods(
                    field_type.to_token_stream().into(),
                )?;
            }
            RelationType::NestObject(nested_object) => {
                let inner_field_type =
                    quote!(<#nested_object as #crate_name::Object>::PartialBuilder);

                let optionalized_field_type = quote!(#crate_name::Maybe<#inner_field_type>);

                self.insert_struct_partial_field_type_def_meta(quote!(
                        #[serde(skip_serializing_if = #maybe_fn_path, rename = #db_field_name)]
                        pub #field_ident_normalized: #optionalized_field_type,
                ))?;
                self.insert_struct_partial_builder_fields_methods(inner_field_type.into())?;
            }
            RelationType::Relate(_) => {}
        }

        Ok(())
    }

    fn insert_struct_partial_builder_fields_methods(
        &mut self,
        struct_partial_field_type: StructPartialFieldType,
    ) -> ExtractorResult<()> {
        let table_derive_attributes = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let original_field_ident = field_receiver.ident()?;
        let crate_name = get_crate_name(false);

        let ass_functions = quote! {
            pub fn #original_field_ident(mut self, value: #struct_partial_field_type) -> Self {
                    self.0.#original_field_ident = #crate_name::Maybe::Some(value);
                    self
             }
        };

        self.struct_partial_associated_functions
            .push(ass_functions.into());
        Ok(())
    }

    fn insert_struct_partial_field_type_def_meta(
        &mut self,
        updater_field_token: TokenStream,
    ) -> ExtractorResult<()> {
        let table_derive_attributes = self.table_derive_attributes();
        let fr = self.field_receiver();
        let db_field_name = fr.db_field_name(&table_derive_attributes.casing()?)?;
        if db_field_name.is_updateable_by_default(&self.data_type()) {
            self.struct_partial_fields.push(updater_field_token.into());
        }
        // We dont care about the field type. We just use this struct to check for
        // renamed serialized field names at compile time by asserting that the a field
        // exist.
        let field_ident = fr.field_ident_normalized(&table_derive_attributes.casing()?)?;
        self.renamed_serialized_fields_kv
            .push(quote!(pub #field_ident: &'static str, ).into());
        Ok(())
    }
}
