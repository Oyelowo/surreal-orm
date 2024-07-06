/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{Display, Formatter},
    ops::Deref,
    str::FromStr,
};

use darling::FromMeta;
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::spanned::Spanned;

use crate::models::*;

create_tokenstream_wrapper!(=> FieldTypeDbToken);

impl Default for FieldTypeDbToken {
    fn default() -> Self {
        let crate_name = get_crate_name(false);
        quote!(#crate_name::FieldType::Any).into()
    }
}

#[derive(Debug, Clone)]
pub struct DbFieldTypeAstMeta {
    pub field_type_db_original: FieldType,
    pub field_type_db_token: FieldTypeDbToken,
    pub static_assertion_token: StaticAssertionToken,
}

impl DbFieldTypeAstMeta {
    pub fn array_item_type_path<'a>(&'a self) -> Option<&'a FieldTypeDbToken> {
        if self.field_type_db_original.is_array() {
            return Some(&self.field_type_db_token);
        }
        return None
    }

    pub fn set_item_type_path<'a>(&'a self) -> Option<&'a FieldTypeDbToken> {
        if self.field_type_db_original.is_set() {
            return Some(&self.field_type_db_token);
        }
        return None
    }
}

impl Default for DbFieldTypeAstMeta {
    fn default() -> Self {
        let crate_name = get_crate_name(false);
        Self {
            field_type_db_original: Default::default(),
            field_type_db_token: quote!(#crate_name::FieldType::Any).into(),
            static_assertion_token: Default::default(),
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct FieldTypeDb(pub FieldType);

impl From<FieldType> for FieldTypeDb {
    fn from(field_type: FieldType) -> Self {
        Self(field_type)
    }
}

impl ToTokens for FieldTypeDb {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_type_str = self.0.to_string();
        let crate_name = get_crate_name(false);
        let as_token = quote!(#crate_name::FieldType::from_str(#field_type_str).expect("Failed to parse field type during tokenization. This is a bug. Please report it."));

        as_token.to_tokens(tokens);
    }
}

impl FieldTypeDb {
    pub fn into_inner(self) -> FieldType {
        self.0
    }

    pub fn into_inner_ref(&self) -> &FieldType {
        &self.0
    }

    pub fn array_item_type(&self) -> Option<Self> {
        match self.0 {
            FieldType::Array(ref ft, _) => Some(Self(ft.deref().clone())),
            _ => None,
        }
    }

    pub fn set_item_type(&self) -> Option<Self> {
        match self.0 {
            FieldType::Set(ref ft, _) => Some(Self(ft.deref().clone())),
            _ => None,
        }
    }

    pub fn as_db_sql_value_tokenstream(&self) -> SqlValueTokenStream {
        let crate_name = get_crate_name(false);
        let value = self.into_inner_ref().clone().to_string();
        let value = quote!(#crate_name::FieldType::from_str(#value).unwrap());
        value.into()
    }

    pub fn generate_static_assertion(&self, field_type: &CustomType) -> TokenStream {
        let crate_name = get_crate_name(false);

        match self.0 {
            FieldType::Any => quote!(#crate_name::validators::assert_type_is_any::<#field_type>();),
            FieldType::Null => {
                quote!(#crate_name::validators::assert_type_is_null::<#field_type>();)
            }
            FieldType::Uuid => {
                quote!(#crate_name::validators::assert_type_is_uuid::<#field_type>();)
            }
            FieldType::Bytes => {
                quote!(#crate_name::validators::assert_type_is_bytes::<#field_type>();)
            }
            FieldType::Union(_) => quote!(),
            FieldType::Option(_) => {
                quote!(#crate_name::validators::assert_type_is_option::<#field_type>();)
            }
            FieldType::String => {
                quote!(#crate_name::validators::assert_type_is_string::<#field_type>();)
            }
            FieldType::Int => quote!(#crate_name::validators::assert_type_is_int::<#field_type>();),
            FieldType::Float => {
                quote!(#crate_name::validators::assert_type_is_float::<#field_type>();)
            }
            FieldType::Bool => {
                quote!(#crate_name::validators::assert_type_is_bool::<#field_type>();)
            }
            FieldType::Array(_, _) => {
                quote!(#crate_name::validators::assert_type_is_array::<#field_type>();)
            }
            FieldType::Set(_, _) => {
                quote!(#crate_name::validators::assert_type_is_set::<#field_type>();)
            }
            FieldType::Datetime => {
                quote!(#crate_name::validators::assert_type_is_datetime::<#field_type>();)
            }
            FieldType::Decimal => {
                quote!(#crate_name::validators::assert_type_is_decimal::<#field_type>();)
            }
            FieldType::Duration => {
                quote!(#crate_name::validators::assert_type_is_duration::<#field_type>();)
            }
            FieldType::Number => {
                quote!(#crate_name::validators::assert_type_is_number::<#field_type>();)
            }
            FieldType::Object => {
                quote!(#crate_name::validators::assert_type_is_object::<#field_type>();)
            }
            FieldType::Record(_) => {
                quote!(#crate_name::validators::assert_type_is_thing::<#field_type>();)
            }
            FieldType::Geometry(_) => {
                quote!(#crate_name::validators::assert_type_is_geometry::<#field_type>();)
            }
        }
    }
}

impl Display for FieldTypeDb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for FieldTypeDb {
    type Target = FieldType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for FieldTypeDb {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        let field_type = match expr {
            syn::Expr::Lit(expr_lit) => {
                if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                    lit_str.value()
                } else {
                    return Err(darling::Error::custom("Expected a string literal for ty")
                        .with_span(&expr.span()));
                }
            }
            syn::Expr::Path(expr_path) => expr_path
                .path
                .get_ident()
                .map(|ident| ident.to_string())
                .ok_or_else(|| {
                    darling::Error::custom("Expected an identifier for ty").with_span(&expr.span())
                })?,
            syn::Expr::Verbatim(expr_verbatim) => expr_verbatim.to_string(),
            _ => {
                return Err(darling::Error::custom("Expected a string literal for ty")
                    .with_span(&expr.span()))
            }
        };
        let field_type = FieldType::from_str(&field_type).map_err(|_| {
            darling::Error::custom(format!(
                "Invalid db_field_type: {field_type}. Must be one of these: {}",
                FieldType::variants().join(", ")
            ))
            .with_span(&expr.span())
        })?;
        Ok(Self(field_type))
    }
}
