/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use super::*;

#[derive(Debug, Copy, Clone)]
pub struct ListSimple;

macro_rules! create_custom_type_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name(pub crate::models::CustomType);

        ::paste::paste! {
            #[derive(Debug, Clone)]
            pub struct [<$name TurboFished>](pub crate::models::CustomTypeTurboFished);

            impl ::quote::ToTokens for [<$name TurboFished>] {
                fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                    self.0.to_tokens(tokens);
                }
            }
        }

        impl $name {
            pub fn into_inner(self) -> crate::models::CustomType {
                self.0
            }

            pub fn into_inner_ref(&self) -> &crate::models::CustomType {
                &self.0
            }

            ::paste::paste! {
                pub fn turbo_fishize(&self) -> crate::models::ExtractorResult<[<$name TurboFished>]> {
                    Ok([<$name TurboFished>](self.0.turbo_fishize()?))
                }

            }
        }

        impl ::darling::FromMeta for $name {
            fn from_meta(item: &::syn::Meta) -> ::darling::Result<Self> {
                let custom_type = CustomType::from_meta(item)?;
                Ok(Self(custom_type))
            }
        }

        impl ::std::convert::From<crate::models::CustomType> for $name {
            fn from(ty: CustomType) -> Self {
                Self(ty)
            }
        }

        impl ::std::convert::From<$name> for crate::models::CustomType {
            fn from(ty: $name) -> Self {
                ty.0
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = crate::models::CustomType;

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
create_custom_type_wrapper!(ArrayItemType);
