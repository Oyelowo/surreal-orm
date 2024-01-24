/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use syn::{
    self, parse_quote, spanned::Spanned, visit_mut::VisitMut, GenericArgument, Ident, Lifetime,
    Path, PathArguments, PathSegment, Type, TypeReference,
};

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, DataType},
};

use super::*;

#[derive(Debug, Clone)]
pub struct ListSimple;

macro_rules! create_link_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, FromMeta)]
        pub struct $name(CustomType);

        impl $name {}

        impl ::std::ops::Deref for $name {
            type Target = CustomType;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

create_link_wrapper!(LinkSelfAttrType);
create_link_wrapper!(LinkSelfAttrTypeReplaceSelfWithCurrentStructIdent);
create_link_wrapper!(LinkOneAttrType);
create_link_wrapper!(LinkManyAttrType);
create_link_wrapper!(NestObjectAttrType);
create_link_wrapper!(NestArrayAttrType);
