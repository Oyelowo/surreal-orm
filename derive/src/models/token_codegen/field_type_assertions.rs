/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro2::TokenStream;
use quote::quote;
use surreal_query_builder::FieldType;

use crate::models::*;

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn create_field_type_static_assertion_token(&mut self) -> ExtractorResult<()> {
        let crate_name = &get_crate_name(false);
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let field_type = &field_receiver
            .ty()
            .remove_non_static_lifetime_and_reference()
            .replace_self_with_current_struct_concrete_type(table_derive_attrs)?;

        let static_assertions = match field_receiver.to_relation_type() {
            RelationType::Relate(_relate) => {
                vec![]
            }
            RelationType::LinkOne(foreign_node) => {
                vec![
                    quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkOne<#foreign_node>);),
                ]
            }
            RelationType::LinkSelf(self_node) => {
                let current_struct_type = table_derive_attrs.struct_no_bounds()?;
                vec![
                    quote!(#crate_name::validators::assert_type_eq_all!(#current_struct_type, #crate_name::LinkSelf<#self_node>);),
                    quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkSelf<#self_node>);),
                ]
            }
            RelationType::LinkMany(foreign_node) => {
                vec![
                    quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkMany<#foreign_node>);),
                ]
            }
            RelationType::NestObject(foreign_object) => {
                vec![
                    quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #foreign_object);),
                ]
            }
            RelationType::NestArray(foreign_array_object) => {
                let nesting_level = Self::count_vec_nesting(field_type.to_basic_type());
                let nested_vec_type =
                    Self::generate_nested_vec_type(&foreign_array_object, nesting_level);

                vec![quote! {
                    #crate_name::validators::assert_type_eq_all!(#foreign_array_object, #nested_vec_type);
                }]
            }
            RelationType::None | RelationType::List(_) => {
                self.get_field_type_static_assertions()?
            }
        };
        self.static_assertions
            .extend(static_assertions.into_iter().map(Into::into));

        Ok(())
    }

    fn get_field_type_static_assertions(&self) -> ExtractorResult<Vec<proc_macro2::TokenStream>> {
        let crate_name = &get_crate_name(false);
        fn get_field_type_static_assertions(
            field_type: &CustomType,
            db_field_type: &FieldType,
            crate_name: &TokenStream,
        ) -> Vec<TokenStream> {
            match db_field_type {
                FieldType::Any => {
                    vec![
                        quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Value>);),
                    ]
                }
                FieldType::Null => {
                    vec![quote!()]
                }
                FieldType::Uuid => {
                    vec![
                        quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Uuid>);),
                    ]
                }
                FieldType::Bytes => {
                    vec![
                        quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Bytes>);),
                    ]
                }
                FieldType::Union(_) => {
                    vec![quote!()]
                }
                FieldType::Option(_) => {
                    vec![quote!(#crate_name::validators::assert_type_is_option::<#field_type>;)]
                }
                FieldType::String => {
                    vec![quote!(#crate_name::validators::assert_type_is_string::<#field_type>();)]
                }
                FieldType::Int => {
                    vec![quote!(
                        #crate_name::validators::assert_type_is_int::<#field_type>();
                    )]
                }
                FieldType::Float => {
                    vec![quote!(
                        #crate_name::validators::assert_type_is_float::<#field_type>();
                    )]
                }
                FieldType::Bool => {
                    vec![quote!(#crate_name::validators::assert_type_is_bool::<#field_type>();)]
                }
                FieldType::Array(_, _) => {
                    vec![quote!(
                        #crate_name::validators::assert_type_is_array::<#field_type>();
                    )]
                }
                FieldType::Set(_, _) => {
                    vec![quote!(
                        #crate_name::validators::assert_type_is_set::<#field_type>();
                    )]
                }
                FieldType::Datetime => {
                    vec![quote!(#crate_name::validators::assert_type_is_datetime::<#field_type>();)]
                }
                FieldType::Decimal => {
                    vec![quote!(
                        #crate_name::validators::assert_type_is_decimal::<#field_type>();
                    )]
                }
                FieldType::Duration => {
                    vec![quote!(#crate_name::validators::assert_type_is_duration::<#field_type>();)]
                }
                FieldType::Number => {
                    vec![quote!(
                        #crate_name::validators::assert_type_is_number::<#field_type>();
                    )]
                }
                FieldType::Object => {
                    vec![
                        quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Object>);),
                    ]
                }
                FieldType::Record(_) => {
                    vec![
                        quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Thing>);),
                    ]
                }
                FieldType::Geometry(_) => {
                    vec![
                        quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Geometry>);),
                    ]
                }
            }
        }
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let field_type = &field_receiver
            .ty()
            .remove_non_static_lifetime_and_reference()
            .replace_self_with_current_struct_concrete_type(table_derive_attrs)?;

        let db_field_type = field_receiver.field_type_db(table_derive_attrs)?;
        let db_field_type = db_field_type.into_inner_ref();

        let mut top_level_check = get_field_type_static_assertions(
            field_type.into_inner_ref(),
            db_field_type,
            crate_name,
        );

        fn static_assertions_for_inner_type(
            inner_rust_ty: Option<&CustomTypeInnerAngleBracket>,
            inner_db_field_type: &FieldType,
            field_receiver: &MyFieldReceiver,
            table_derive_attrs: &ModelAttributes,
            model_type: &DataType,
            assertion_accumulator: &mut Vec<TokenStream>,
        ) -> ExtractorResult<()> {
            let crate_name = &get_crate_name(false);

            if let Some(inner_rust_ty) = inner_rust_ty.clone() {
                let inner = get_field_type_static_assertions(
                    inner_rust_ty.into_inner_ref(),
                    inner_db_field_type,
                    crate_name,
                );
                assertion_accumulator.extend(inner);
            }

            // Checks inner type when autoinferring the field type
            if field_receiver.field_type_db.is_none() {
                if let Some(inner_rust_ty) = inner_rust_ty {
                    if inner_rust_ty
                        .type_is_inferrable_primitive(field_receiver, table_derive_attrs)
                    {
                        let field_name =
                            &field_receiver.db_field_name(&table_derive_attrs.casing()?)?;
                        let relation_type = field_receiver.to_relation_type();
                        let db_type_meta = inner_rust_ty.infer_surreal_type_heuristically(
                            field_name,
                            &relation_type,
                            &model_type,
                        )?;

                        if let Some(ft) = db_type_meta.field_type_db_original {
                            let inner = get_field_type_static_assertions(
                                inner_rust_ty.into_inner_ref(),
                                &ft,
                                crate_name,
                            );
                            assertion_accumulator.extend(inner);
                        }
                    }
                }
            }
            Ok(())
        }

        fn static_check_inner_type(
            inner_rust_field_type: Option<&CustomTypeInnerAngleBracket>,
            db_field_type: &FieldType,
            field_receiver: &MyFieldReceiver,
            table_derive_attrs: &ModelAttributes,
            model_type: &DataType,
            assertion_accumulator: &mut Vec<TokenStream>,
        ) -> ExtractorResult<()> {
            if inner_rust_field_type.is_none() {
                return Ok(());
            }

            let inner = match db_field_type {
                FieldType::Option(inner) => inner,
                FieldType::Set(inner, _) => inner,
                FieldType::Array(inner, _) => inner,
                _ => return Ok(()),
            };
            static_assertions_for_inner_type(
                inner_rust_field_type,
                inner,
                field_receiver,
                table_derive_attrs,
                model_type,
                assertion_accumulator,
            )?;

            static_check_inner_type(
                inner_rust_field_type
                    .map(|ft| ft.inner_angle_bracket_type().ok())
                    .flatten()
                    .flatten()
                    .as_ref(),
                inner,
                field_receiver,
                table_derive_attrs,
                model_type,
                assertion_accumulator,
            )?;

            Ok(())
        }

        static_check_inner_type(
            field_type
                .inner_angle_bracket_type()
                .ok()
                .flatten()
                .as_ref(),
            db_field_type,
            self.field_receiver(),
            table_derive_attrs,
            &self.data_type(),
            &mut top_level_check,
        )?;

        Ok(top_level_check)
    }

    fn generate_nested_vec_type(
        foreign_node: &CustomType,
        nesting_level: usize,
    ) -> proc_macro2::TokenStream {
        if nesting_level == 0 {
            quote!(#foreign_node)
        } else {
            let inner_type = Self::generate_nested_vec_type(foreign_node, nesting_level - 1);
            quote!(::std::vec::Vec<#inner_type>)
        }
    }

    fn count_vec_nesting(field_type: &syn::Type) -> usize {
        match field_type {
            syn::Type::Path(type_path) => {
                // Check if the outermost type is a `Vec`.
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        // It's a Vec, now look at the inner type.
                        if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_type)) =
                                angle_args.args.first()
                            {
                                // Recursively count nesting inside the Vec.
                                1 + Self::count_vec_nesting(inner_type)
                            } else {
                                0 // No type inside Vec's angle brackets.
                            }
                        } else {
                            0 // Vec has no angle brackets, which should not happen for valid Vec usage.
                        }
                    } else {
                        0 // The outermost type is not a Vec.
                    }
                } else {
                    0 // No segments in the type path.
                }
            }
            _ => 0, // Not a type path, so it can't be a Vec.
        }
    }
}
