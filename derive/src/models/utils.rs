/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub use proc_macros_helpers::{get_crate_name, parse_lit_to_tokenstream};
use quote::quote;
use syn::{Generics, Type, TypePath};

pub fn generate_nested_vec_type(
    foreign_node: &syn::Ident,
    nesting_level: usize,
) -> proc_macro2::TokenStream {
    if nesting_level == 0 {
        quote!(#foreign_node)
    } else {
        let inner_type = generate_nested_vec_type(foreign_node, nesting_level - 1);
        quote!(::std::vec::Vec<#inner_type>)
    }
}

pub fn count_vec_nesting(field_type: &syn::Type) -> usize {
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
                            1 + count_vec_nesting(inner_type)
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

// pub fn is_generic_type(ty: &Type, generics: &Generics) -> bool {
//     match ty {
//         Type::Path(TypePath { path, .. }) => {
//             // Check if the type matches any of the generic type parameters
//             generics.params.iter().any(|param| match param {
//                 syn::GenericParam::Type(type_param) => path.is_ident(&type_param.ident),
//                 _ => false,
//             })
//         }
//         _ => false,
//     }
// }

pub fn is_generic_type(ty: &Type, generics: &Generics) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            // Check each segment of the path for generic parameters
            path.segments.iter().any(|segment| {
                // Check if this segment is a generic parameter itself
                if generics.params.iter().any(|param| matches!(param, syn::GenericParam::Type(type_param) if segment.ident == type_param.ident)) {
                    return true;
                }

                // Check if this segment has arguments that are generic parameters
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    args.args.iter().any(|arg| {
                        if let syn::GenericArgument::Type(ty) = arg {
                            is_generic_type(ty, generics)
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            })
        }
        // You can extend this match to handle other types like tuples, slices, etc.
        _ => false,
    }
}

use syn::{visit::Visit, GenericParam};

struct GenericTypeChecker<'a> {
    generics: &'a Generics,
    found: bool,
}

impl<'a> Visit<'a> for GenericTypeChecker<'a> {
    fn visit_type_path(&mut self, i: &'a TypePath) {
        if self.generics.params.iter().any(|param| {
            matches!(param, GenericParam::Type(type_param) if i.path.is_ident(&type_param.ident))
        }) {
            self.found = true;
        }
        // Continue walking down the tree
        syn::visit::visit_type_path(self, i);
    }

    // Implement other visit_* methods as needed to handle different type constructs
}

fn is_generic_type2(ty: &Type, generics: &Generics) -> bool {
    let mut checker = GenericTypeChecker {
        generics,
        found: false,
    };
    checker.visit_type(ty);
    checker.found
}
