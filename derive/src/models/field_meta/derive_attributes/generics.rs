/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::models::*;

use super::MyFieldReceiver;

impl MyFieldReceiver {
    // fn has_generics(&self, table_attributes: TableDeriveAttributes) -> bool {
    //     let current_struct_generics = table_attributes.generics;
    //     match self.ty() {
    //         Type::Path(TypePath { path, .. }) => {
    //             path.segments.iter().any(|segment| {
    //             if current_struct_generics.params.iter().any(|param| matches!(param, syn::GenericParam::Type(type_param) if segment.ident == type_param.ident)) {
    //                 return true;
    //             }
    //
    //             if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
    //                 args.args.iter().any(|arg| {
    //                     if let syn::GenericArgument::Type(ty) = arg {
    //                         is_generic_type(ty, current_struct_generics)
    //                     } else {
    //                         false
    //                     }
    //                 })
    //             } else {
    //                 false
    //             }
    //         })
    //         }
    //         _ => false,
    //     }
    // }
    //
}

#[derive(Clone, Debug)]
pub struct FieldGenericsMeta<'a> {
    pub(crate) field_impl_generics: syn::ImplGenerics<'a>,
    pub(crate) field_ty_generics: syn::TypeGenerics<'a>,
    pub(crate) field_where_clause: Option<&'a syn::WhereClause>,
}
