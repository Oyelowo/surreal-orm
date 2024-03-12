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

        match field_receiver.to_relation_type() {
            RelationType::Relate(_relate) => {}
            RelationType::LinkOne(foreign_node) => {
                self.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkOne<#foreign_node>);).into());
            }
            RelationType::LinkSelf(self_node) => {
                let current_struct_type = table_derive_attrs.struct_no_bounds()?;
                self.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#current_struct_type, #crate_name::LinkSelf<#self_node>);).into());

                self.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkSelf<#self_node>);).into());
            }
            RelationType::LinkMany(foreign_node) => {
                self.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkMany<#foreign_node>);).into());
            }
            RelationType::NestObject(foreign_object) => {
                self.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #foreign_object);).into());
            }
            RelationType::NestArray(foreign_array_object) => {
                let nesting_level = Self::count_vec_nesting(field_type.as_basic_type_ref());
                let nested_vec_type =
                    Self::generate_nested_vec_type(&foreign_array_object, nesting_level);

                self.static_assertions.push(quote! {
                        #crate_name::validators::assert_type_eq_all!(#foreign_array_object, #nested_vec_type);
                    }.into());
            }
            RelationType::None | RelationType::List(_) => {
                // TODO: Add attribute, skip type check
                surreal_query_builder::validators::assert_impl_one!(
                    &'static str: Into<std::string::String>
                );
                let x = 45u32;
                x.to_string();
                // &'static str
                // surreal_query_builder::validators::assert_impl_one!(&String: Into<String>);
                // surreal_query_builder::validators::assert_impl_one!(Option<&String>: Into<Option<String>>);
                // surreal_query_builder::validators::assert_impl_one!(u32: Into<String>);
                // surreal_query_builder::validators::assert_impl_one!(Option<u32>: Into<Option<String>>);
                // let x: surreal_query_builder::sql::String = 2.into();
                let table_derive_attrs = self.table_derive_attributes();
                let ft = field_receiver.field_type_db(table_derive_attrs)?;
                let is_simple = ft.is_primitive();
                let raw_type = field_type;
                let db_type_static_checker = match ft.into_inner_ref() {
                    FieldType::Any => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                    }
                    FieldType::Null => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                    }
                    FieldType::Uuid => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Uuid>);)
                    }
                    FieldType::Bytes => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Bytes>);)
                    }
                    FieldType::Union(_) => {
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                        quote!()
                    }
                    FieldType::Option(_) => {
                        // option<bool>  => assert_impl_one!(Option<bool>: Into<Option<bool>>);
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                        // TODO: Should I do a recursive option check?
                        quote!()
                    }
                    FieldType::String => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<::std::string::String>);)
                    }
                    FieldType::Int => {
                        quote!(
                            #crate_name::validators::is_int::<#raw_type>();
                            // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                        )
                    }
                    FieldType::Float => {
                        quote!(
                            #crate_name::validators::is_float::<#raw_type>();
                            // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                        )
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                    }
                    FieldType::Bool => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<::std::primitive::bool>);)
                    }
                    FieldType::Array(_, _) => {
                        quote!(
                            #crate_name::validators::assert_is_vec::<#raw_type>();
                        )
                    }
                    FieldType::Set(_, _) => {
                        // TODO, create is_set. Set field should use hashset or something
                        quote!(
                            #crate_name::validators::assert_is_vec::<#raw_type>();
                        )
                    }
                    FieldType::Datetime => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Datetime>);)
                    }
                    FieldType::Decimal => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                    }
                    FieldType::Duration => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Duration>);)
                    }
                    FieldType::Number => {
                        quote!(
                            #crate_name::validators::is_number::<#raw_type>();
                            // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                        )
                        // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                    }
                    FieldType::Object => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Object>);)
                    }
                    FieldType::Record(_) => {
                        if let DataType::Edge = model_type {
                            quote!()
                        } else {
                            quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<Option<#crate_name::sql::Thing>>);)
                        }
                    }
                    FieldType::Geometry(_) => {
                        quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Geometry>);)
                    }
                };

                self.static_assertions.push(x);
            }
        }

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
