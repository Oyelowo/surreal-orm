/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;

use super::*;

#[derive(Debug, Copy, Clone)]
pub struct ListSimple;

macro_rules! create_custom_type_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, FromMeta)]
        pub struct $name(pub CustomType);

        impl $name {
            pub fn into_inner(self) -> CustomType {
                self.0
            }

            pub fn as_custom_type_ref(&self) -> &CustomType {
                &self.0
            }
        }

        impl ::std::convert::From<CustomType> for $name {
            fn from(ty: CustomType) -> Self {
                Self(ty)
            }
        }

        impl ::std::convert::From<$name> for CustomType {
            fn from(ty: $name) -> Self {
                ty.0
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = CustomType;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::convert::From<::syn::Type> for $name {
            fn from(ty: ::syn::Type) -> Self {
                Self(CustomType::new(ty))
            }
        }

        impl ::quote::ToTokens for $name {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                self.0.to_tokens(tokens);
            }
        }
    };
}

pub(crate) use create_custom_type_wrapper;

create_custom_type_wrapper!(LinkSelfAttrType);
create_custom_type_wrapper!(LinkOneAttrType);
create_custom_type_wrapper!(LinkManyAttrType);
create_custom_type_wrapper!(NestObjectAttrType);
create_custom_type_wrapper!(NestArrayAttrType);
