/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};

use super::Codegen;
use crate::models::*;

pub struct DefineFieldStatementToken(TokenStream);

impl<'a> Codegen<'a> {
    pub fn create_field_definitions(&mut self) -> ExtractorResult<()> {
        self.validate_field_attributes()?;

        self.field_definitions.extend(self.field_defintion_db()?);
        Ok(())
    }

    pub fn field_defintion_db(&self) -> ExtractorResult<Vec<DefineFieldStatementToken>> {
        let crate_name = get_crate_name(false);
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let db_field_name = field_receiver.db_field_name(&table_derive_attrs.casing()?);
        let casing = table_derive_attrs.casing()?;
        let field_name_serialized = field_receiver.field_ident_normalized(&casing)?;

        let mut define_field_methods = vec![];
        let mut define_array_field_item_methods = vec![];
        let mut all_field_defintions = vec![];

        if let Some(define) = self.define {
            quote!(#define.to_raw());
        }

        if let Some(assert) = self.assert {
            define_field_methods.push(quote!(.assert(#assert)));
        }

        if let Some(item_assert) = self.item_assert {
            define_array_field_item_methods.push(quote!(.assert(#item_assert)));
        }

        if let Some(value) = self.value {
            define_field_methods.push(quote!(.value(#value)));
        }

        if let Some(permissions) = self.permissions {
            define_field_methods.push(permissions.into_token_stream());
        }

        let main_field_def = quote!(
            #crate_name::statements::define_field(#crate_name::Field::new(#db_field_name))
            .on_table(#crate_name::Table::from(Self::table_name()))
            #( # define_field_methods) *
            .to_raw()
        );
        all_field_defintions.push(main_field_def);

        if !define_array_field_item_methods.is_empty() {
            let array_field_item_str = format!("{field_name_serialized}.*");
            let array_item_definition = quote!(
                #crate_name::statements::define_field(#crate_name::Field::new(#array_field_item_str))
                                        .on_table(#crate_name::Table::from(Self::table_name()))
                                        #( # define_array_field_item_methods) *
                                        .to_raw()

            );
            all_field_defintions.push(array_item_definition);
        };

        Ok(DefineFieldStatementToken(all_field_defintions))
    }

    pub fn static_assertion_field_value(
        &self,
        table_derive_attrs: &TableDeriveAttributes,
        model_type: &DataType,
    ) -> ExtractorResult<StaticAssertionToken> {
        let field_type = self
            .field_type_db(table_derive_attrs, model_type)?
            .into_inner();

        let static_assertion = self.value.map_or(StaticAssertionToken::default(), |v| {
            v.get_static_assertion(field_type)
        });

        Ok(static_assertion)
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
        let db_field_name = field_receiver.db_field_name(&self.table_derive_attributes().casing()?);

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
