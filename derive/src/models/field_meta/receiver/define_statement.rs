use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::quote;

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, MyFieldReceiver, StaticAssertionToken},
};

use super::MyFieldReceiver;

pub struct DefineFieldStatementToken(TokenStream);

impl MyFieldReceiver {
    pub fn get_db_field_defintion(
        &self,
        table_derive_attrs: &TableDeriveAttributes,
    ) -> ExtractorResult<Vec<DefineFieldStatementToken>> {
        let crate_name = get_crate_name(false);
        let field_receiver = self;
        let mut define_field_methods = vec![];
        let mut define_array_field_item_methods = vec![];
        let field_ident_normalized = self
            .normalized_ident(table_derive_attrs.struct_level_casing()?)
            .field_ident_serialized_fmt;
        let mut all_field_defintions = vec![];

        self.validate_field_attributes()?;

        if let Some(define) = field_receiver.define {
            quote!(#define.to_raw());
        }

        if let Some(assert) = field_receiver.assert {
            define_field_methods.push(quote!(.assert(#assert)));
        }

        if let Some(item_assert) = field_receiver.item_assert {
            define_array_field_item_methods.push(quote!(.assert(#item_assert)));
        }

        if let Some(value) = field_receiver.value {
            define_field_methods.push(quote!(.value(#value)));
        }

        if let Some(permissions) = field_receiver.permissions {
            define_field_methods.push(permissions);
        }

        let main_field_def = quote!(
            #crate_name::statements::define_field(#crate_name::Field::new(#field_name_normalized))
            .on_table(#crate_name::Table::from(Self::table_name()))
            #( # define_field_methods) *
            .to_raw()
        );
        all_field_defintions.push(main_field_def);

        if !define_array_field_item_methods.is_empty() {
            let array_field_item_str = format!("{field_ident_normalized}.*");
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

    pub fn get_value_static_assertion(&self) -> StaticAssertionToken {
        self.value.map_or(StaticAssertionToken::default(), |v| {
            v.get_static_assrtion(self.get_db_type().into_inner())
        })
    }

    fn validate_field_attributes(&self) -> ExtractorResult<ReferencedNodeMeta> {
        MyFieldReceiver {
            define,
            assert: assert_,
            value,
            permissions,
            item_assert,
            ident,
            ..
        } = &self;

        if (define.is_some()
            && (assert_.is_some()
                || value.is_some()
                || permissions.is_some()
                || item_assert.is_some()))
        {
            return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    r#"Invalid combination. When `define`, the following attributes cannot be use in combination to prevent confusion:
    assert,
    value,
    permissions,
    item_assert"#).into());
        }

        if (relate.is_some()
            && (define.is_some()
                || assert_.is_some()
                || value.is_some()
                || permissions.is_some()
                || item_assert.is_some()))
        {
            return (Err(syn::Error::new_spanned(
                ident,
                r#"This is a read-only relation field and does not allow the following attributes:
    define,
    assert,
    value,
    permissions,
    item_assert"#,
            )
            .into()));
        }

        Ok(())
    }
}
