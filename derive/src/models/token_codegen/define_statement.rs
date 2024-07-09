/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};

use super::Codegen;
use crate::models::*;

impl<'a> Codegen<'a> {
    pub fn create_field_definitions(&mut self) -> ExtractorResult<()> {
        self.validate_field_attributes()?;

        self.static_assertions
            .push(self.static_assertion_field_value(self.table_derive_attributes())?);
        self.field_definitions.extend(self.field_defintion_db()?);
        Ok(())
    }

    pub fn field_defintion_db(&self) -> ExtractorResult<Vec<DefineFieldStatementToken>> {
        let field_receiver = self.field_receiver();
        let model_attributes = self.table_derive_attributes();
        let relation_type = field_receiver.to_relation_type(model_attributes);
        if relation_type.is_relate_graph() {
            return Ok(vec![]);
        }

        let crate_name = get_crate_name(false);
        let casing = model_attributes.casing()?;
        let db_field_name = field_receiver.db_field_name(&casing)?;
        let field_type_in_db_token = field_receiver.field_type_db_token(model_attributes)?;

        let mut define_field_methods = vec![];
        let mut define_array_field_item_methods = vec![];
        let mut all_field_defintions: Vec<DefineFieldStatementToken> = vec![];

        if let Some(assert) = field_receiver.assert.as_ref() {
            define_field_methods.push(quote!(.assert(#assert)));
        }

        if let Some(item_assert) = field_receiver.item_assert.as_ref() {
            define_array_field_item_methods.push(quote!(.assert(#item_assert)));
        }

        if let Some(value) = field_receiver.value.as_ref() {
            define_field_methods.push(quote!(.value(#value)));
        }

        if let Some(permissions) = field_receiver.permissions.as_ref() {
            define_field_methods.push(permissions.into_token_stream());
        }

        let main_field_def = quote!(
            #crate_name::statements::define_field(#crate_name::Field::new(#db_field_name))
            .on_table(#crate_name::Table::from(Self::table()))
            .type_(#field_type_in_db_token)
            #( # define_field_methods) *
            .to_raw()
        );

        all_field_defintions.push(main_field_def.into());

        if !define_array_field_item_methods.is_empty() {
            let array_field_item_str = quote!(#db_field_name.*).to_string();
            let array_item_definition = quote!(
                #crate_name::statements::define_field(#crate_name::Field::new(#array_field_item_str))
                                        .on_table(#crate_name::Table::from(Self::table()))
                                        #( # define_array_field_item_methods) *
                                        .to_raw()

            );
            all_field_defintions.push(array_item_definition.into());
        };

        if let Some(define) = field_receiver.define.as_ref() {
            all_field_defintions.push(quote!(#define.to_raw()).into());
        }

        Ok(all_field_defintions)
    }

    fn static_assertion_field_value(
        &self,
        model_attrs: &ModelAttributes,
    ) -> ExtractorResult<StaticAssertionToken> {
        let field_receiver = self.field_receiver();
        let field_type = field_receiver.field_type_db_original(model_attrs)?;

        match field_type {
            Some(field_type) => {
                let static_assertion = field_receiver
                    .value
                    .as_ref()
                    .map_or(StaticAssertionToken::default(), |v| {
                        v.get_default_value_static_assertion(field_type.into_inner())
                    });
                Ok(static_assertion)
            }
            None => Ok(StaticAssertionToken::default()),
        }
    }

    fn validate_field_attributes(&self) -> ExtractorResult<()> {
        let field_receiver = self.field_receiver();
        let MyFieldReceiver {
            define,
            assert: assert_,
            value,
            permissions,
            item_assert,
            relate,
            ..
        } = field_receiver;
        let ident = field_receiver.ident()?;
        let db_field_name =
            field_receiver.db_field_name(&self.table_derive_attributes().casing()?)?;

        if define.is_some()
            && (assert_.is_some()
                || value.is_some()
                || permissions.is_some()
                || item_assert.is_some())
        {
            return Err(
                syn::Error::new_spanned(
                    db_field_name,
                    r#"Invalid combination. When `define`, the following attributes cannot be use in combination to prevent confusion:
    assert,
    value,
    permissions,
    item_assert"#).into());
        }

        if relate.is_some()
            && (define.is_some()
                || assert_.is_some()
                || value.is_some()
                || permissions.is_some()
                || item_assert.is_some())
        {
            return Err(syn::Error::new_spanned(
                ident,
                r#"This is a read-only relation field and does not allow the following attributes:
    define,
    assert,
    value,
    permissions,
    item_assert"#,
            )
            .into());
        }

        Ok(())
    }
}
