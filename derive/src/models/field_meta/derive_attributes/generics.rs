use crate::models::{derive_attributes::TableDeriveAttributes, GenericTypeExtractor};
use quote::quote;
use syn::{visit::Visit, *};

use super::MyFieldReceiver;

impl MyFieldReceiver {
    // This extracts generics metadata for field and from struct generics metadata.
    // This could come from the concrete rust field type or
    // as an attribute on the field from links which link to
    // other tables structs models i.e Edge, Node and Objects.
    // These are usually specified using the link_one, link_self
    // and link_many and relate attributes.
    // e.g
    // #[surreal_orm(link_one = User<'a, T, u32>)]
    // student: LinkOne<User<'a, T, u32>
    pub fn get_field_generics_meta<'a>(
        &self,
        table_attributes: TableDeriveAttributes,
    ) -> FieldGenericsMeta<'a> {
        let field_type = self
            .ty
            .replace_self_with_current_struct_ident(&table_attributes)
            .get_generics_meta(table_attributes);

        let (field_impl_generics, field_ty_generics, field_where_clause) =
            GenericTypeExtractor::extract_generics_for_complex_type(&table_attributes, &field_type)
                .split_for_impl();
        FieldGenericsMeta {
            field_impl_generics,
            field_ty_generics,
            field_where_clause,
        }
    }

    // fn has_generics(&self, table_attributes: TableDeriveAttributes) -> bool {
    //     let current_struct_generics = table_attributes.generics;
    //     match ty {
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

pub struct FieldGenericsMeta<'a> {
    pub(crate) field_impl_generics: syn::ImplGenerics<'a>,
    pub(crate) field_ty_generics: syn::TypeGenerics<'a>,
    pub(crate) field_where_clause: Option<&'a syn::WhereClause>,
}
