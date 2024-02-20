/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;

use super::*;

#[derive(Debug, Clone)]
pub struct ListSimple;

macro_rules! create_custom_type_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, FromMeta)]
        pub struct $name(pub CustomType);

        impl ::std::convert::Into<CustomType> for $name {
            fn into(self) -> CustomType {
                self.0
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
    };
}

pub(crate) use create_custom_type_wrapper;

create_custom_type_wrapper!(LinkSelfAttrType);
create_custom_type_wrapper!(LinkSelfAttrTypeReplaceSelfWithCurrentStructIdent);
create_custom_type_wrapper!(LinkOneAttrType);
create_custom_type_wrapper!(LinkManyAttrType);
create_custom_type_wrapper!(NestObjectAttrType);
create_custom_type_wrapper!(NestArrayAttrType);
