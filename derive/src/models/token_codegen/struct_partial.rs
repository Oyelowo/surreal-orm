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
        let model_attributes = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let field_type = field_receiver.ty();
        let field_ident_original = field_receiver.ident()?;
        let db_field_name = field_receiver.db_field_name(&model_attributes.casing()?)?;

        let maybe_fn_path = format!("{crate_name}::Maybe::is_none");
        match field_receiver.to_relation_type(model_attributes) {
            RelationType::None
            | RelationType::NestArray(_)
            | RelationType::LinkOne(_)
            | RelationType::LinkSelf(_)
            | RelationType::LinkMany(_)
            | RelationType::LinkManyInAndOutEdgeNodesInert(_)
            | RelationType::List(_) => {
                let field_type = field_type.replace_self_with_current_struct_concrete_type(model_attributes)?;
                let optionalized_field_type = quote!(#crate_name:: Maybe<#field_type>);
                self.insert_struct_partial_field_type_def_meta(quote!(
                    #[serde(skip_serializing_if = #maybe_fn_path, rename = #db_field_name)]
                    pub #field_ident_original: #optionalized_field_type
                ))?;

                self.insert_struct_partial_builder_fields_methods(
                    field_type.to_token_stream().into(),
                )?;

                self.insert_renamed_serialized_fields_kv()?;
                self.insert_struct_partial_init_fields()?;
            }
            RelationType::NestObject(nested_object) => {
                let inner_field_type =
                    quote!(<#nested_object as #crate_name::PartialUpdater>::StructPartial);

                let optionalized_field_type = quote!(#crate_name::Maybe<#inner_field_type>);

                self.insert_struct_partial_field_type_def_meta(quote!(
                        #[serde(skip_serializing_if = #maybe_fn_path, rename = #db_field_name)]
                        pub #field_ident_original: #optionalized_field_type
                ))?;
                self.insert_struct_partial_builder_fields_methods(inner_field_type.into())?;
                self.insert_renamed_serialized_fields_kv()?;
                self.insert_struct_partial_init_fields()?;
            }
            RelationType::Relate(_) => {}
        }

        Ok(())
    }

    fn insert_struct_partial_builder_fields_methods(
        &mut self,
        struct_partial_field_type: StructPartialFieldType,
    ) -> ExtractorResult<()> {
        let field_receiver = self.field_receiver();
        let field_ident_original = field_receiver.ident()?;
        let crate_name = get_crate_name(false);

        let ass_functions = quote! {
            pub fn #field_ident_original(mut self, value: #struct_partial_field_type) -> Self {
                    self.0.#field_ident_original = #crate_name::Maybe::Some(value);
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
        self.struct_partial_fields.push(updater_field_token.into());
        Ok(())
    }

    fn insert_struct_partial_init_fields(&mut self) -> ExtractorResult<()> {
        let fr = &self.field_receiver();
        let ident = fr.ident()?;
        // let db_field_name =
        //     fr
        //     .db_field_name(&self.table_derive_attributes().casing()?)?;
        //
        // // NOTE: Check in latest 2.0  version of surrealdb if in and out fields of
        // // edges are not updateable.Currently, they are readonly once created.
        // // Id field should remain immutable, hence this check.
        // if !db_field_name.is_id() {
        //     self.serialized_ident_struct_partial_init_fields
        //         .push(SerializedIdentStructPartialInitFields::new(
        //             quote!(#ident),
        //         ));
        // }
        self.serialized_ident_struct_partial_init_fields
            .push(SerializedIdentStructPartialInitFields::new(quote!(#ident)));
        Ok(())
    }
    fn insert_renamed_serialized_fields_kv(&mut self) -> Result<(), ExtractorError> {
        let table_derive_attributes = self.table_derive_attributes();
        let fr = self.field_receiver();
        // We dont care about the field type. We just use this struct to check for
        // renamed serialized field names at compile time by asserting that the a field
        // exist.
        let field_ident = fr.field_ident_normalized(&table_derive_attributes.casing()?)?;
        self.renamed_serialized_fields_kv
            .push(quote!(pub #field_ident: &'static str ).into());
        Ok(())
    }
}
