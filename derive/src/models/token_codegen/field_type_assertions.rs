/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use quote::quote;

use crate::models::*;

use super::Codegen;

impl<'a> Codegen<'a> {
    pub fn create_field_type_static_assertion_token(&mut self) -> ExtractorResult<()> {
        let crate_name = &get_crate_name(false);
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let field_type = &field_receiver.ty();

        match field_receiver.to_relation_type() {
            RelationType::Relate(relate) => {}
            RelationType::LinkOne(foreign_node) => {
                self.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkOne<#foreign_node>);).into());
            }
            RelationType::LinkSelf(self_node) => {
                let current_struct_type = table_derive_attrs.struct_as_path_no_bounds()?;
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
                let nesting_level = Self::count_vec_nesting(&field_type.to_basic_type());
                let nested_vec_type =
                    Self::generate_nested_vec_type(&foreign_array_object, nesting_level);

                self.static_assertions.push(quote! {
                        #crate_name::validators::assert_type_eq_all!(#foreign_array_object, #nested_vec_type);
                    }.into());
            }
            RelationType::List(_) => {}
            RelationType::None => {}
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
