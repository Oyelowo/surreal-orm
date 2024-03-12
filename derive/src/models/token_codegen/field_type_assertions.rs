/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use quote::quote;
use surreal_query_builder::FieldType;

use crate::models::*;

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn create_field_type_static_assertion_token(&mut self) -> ExtractorResult<()> {
        let crate_name = &get_crate_name(false);
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let field_type = &field_receiver.ty();

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
                let nesting_level = Self::count_vec_nesting(field_type.as_basic_type_ref());
                let nested_vec_type =
                    Self::generate_nested_vec_type(&foreign_array_object, nesting_level);

                vec![quote! {
                    #crate_name::validators::assert_type_eq_all!(#foreign_array_object, #nested_vec_type);
                }]
            }
            RelationType::None | RelationType::List(_) => {
                let db_field_type = field_receiver.field_type_db(table_derive_attrs)?;
                let db_type_static_checker = match db_field_type.into_inner_ref() {
                    FieldType::Any => {
                        vec![
                            quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Value>);),
                        ]
                    }
                    FieldType::Null => {
                        vec![
                            quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Value>);),
                        ]
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
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                        vec![quote!()]
                    }
                    FieldType::Option(ft) => {
                        let x = field_type.raw_type_is_optional();
                        let xx = field_type.inner_angle_bracket_type()?;
                        // .map(|x| x.type_is_inferrable(field_receiver, model_attributes));
                        // .map(|x| x.type_is_inferrable(field_receiver, model_attributes));
                        // option<bool>  => assert_impl_one!(Option<bool>: Into<Option<bool>>);
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                        // TODO: Should I do a recursive option check?
                        vec![quote!(#crate_name::validators::assert_is_option::<#field_type>;)]
                    }
                    FieldType::String => {
                        vec![
                            quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<::std::string::String>);),
                        ]
                    }
                    FieldType::Int => {
                        vec![quote!(
                            #crate_name::validators::is_int::<#field_type>();
                            // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                        )]
                    }
                    FieldType::Float => {
                        vec![quote!(
                            #crate_name::validators::is_float::<#field_type>();
                            // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                        )]
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                    }
                    FieldType::Bool => {
                        vec![
                            quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<::std::primitive::bool>);),
                        ]
                    }
                    FieldType::Array(_, _) => {
                        vec![quote!(
                            #crate_name::validators::assert_is_array::<#field_type>();
                        )]
                    }
                    FieldType::Set(_, _) => {
                        vec![quote!(
                            #crate_name::validators::assert_is_set::<#field_type>();
                        )]
                    }
                    FieldType::Datetime => {
                        vec![
                            quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Datetime>);),
                        ]
                    }
                    FieldType::Decimal => {
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                        vec![quote!(
                            #crate_name::validators::is_number::<#field_type>();
                            // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                        )]
                    }
                    FieldType::Duration => {
                        vec![
                            quote!(#crate_name::validators::assert_impl_one!(#field_type: ::std::convert::Into<#crate_name::sql::Duration>);),
                        ]
                    }
                    FieldType::Number => {
                        vec![quote!(
                            #crate_name::validators::is_number::<#field_type>();
                            // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                        )]
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
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
                };

                self.static_assertions.push(x);
            }
        };

        Ok(())
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
