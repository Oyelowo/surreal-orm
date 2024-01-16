/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
pub use proc_macros_helpers::{get_crate_name, parse_lit_to_tokenstream};
use quote::{format_ident, quote};
use syn::{
    visit::Visit, DeriveInput, GenericParam, Generics, PathArguments, Type, TypeArray, TypeBareFn,
    TypeGroup, TypeImplTrait, TypePath, TypePtr, TypeReference, TypeSlice, TypeTuple,
    WherePredicate,
};
use syn::{ReturnType, TypeMacro};

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

pub(crate) struct GenericTypeExtractor<'a> {
    struct_generics: &'a Generics,
    field_generics: Generics,
}

impl<'a> GenericTypeExtractor<'a> {
    pub fn new(struct_generics: &'a Generics) -> Self {
        Self {
            struct_generics,
            field_generics: Generics::default(),
        }
    }

    pub fn extract_generics_for_complex_type(&mut self, field_ty: &'a Type) -> &Generics {
        self.visit_type(field_ty);
        &self.field_generics
    }

    // pub fn extract_generics_for_complex_type(
    //     ty: &'a Type,
    //     struct_generics: &'a Generics,
    // ) -> Generics {
    //     let mut extractor = Self {
    //         struct_generics,
    //         field_generics: Generics::default(),
    //     };
    //     extractor.visit_type(ty);
    //     extractor.field_generics
    // }
}

impl<'a> Visit<'a> for GenericTypeExtractor<'a> {
    // Visit types and extract generics
    fn visit_type_path(&mut self, i: &'a TypePath) {
        for segment in &i.path.segments {
            // Check if segment matches a generic parameter of the struct
            if let Some(gen_param) = self.struct_generics.params.iter().find(|param| {
                matches!(param, GenericParam::Type(type_param) if segment.ident == type_param.ident)
            }) {
                self.field_generics.params.push(gen_param.clone());

                // Handle constraints on the generic parameter
                if let Some(where_clause) = &self.struct_generics.where_clause {
                    for predicate in &where_clause.predicates {
                        if let WherePredicate::Type(predicate_type) = predicate {
                            if let syn::Type::Path(type_path) = &predicate_type.bounded_ty {
                                if type_path.path.is_ident(&segment.ident) {
                                    self.field_generics.make_where_clause().predicates.push(predicate.clone());
                                }
                            }
                        }
                    }
                }
            }

            // Recursively visit nested generic arguments
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                for arg in &args.args {
                    match arg {
                        // Recursively visit the nested type
                        syn::GenericArgument::Type(ty) => self.visit_type(ty),
                        syn::GenericArgument::Lifetime(lt) => {
                            // Here we handle lifetime arguments
                            if !self.field_generics.params.iter().any(|param| matches!(param, GenericParam::Lifetime(lifetime_def) if lifetime_def.lifetime == *lt)) {
                                // Only add the lifetime if it's not already in the list
                                self.field_generics.params.push(GenericParam::Lifetime(syn::LifetimeParam {
                                    attrs: Vec::new(),
                                    lifetime: lt.clone(),
                                    colon_token: None,
                                    bounds: syn::punctuated::Punctuated::new(),
                                }));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // default visitation of this type path
        syn::visit::visit_type_path(self, i);
    }
    // Visit tuple types like (T, U, V)
    fn visit_type_tuple(&mut self, i: &'a TypeTuple) {
        for elem in &i.elems {
            self.visit_type(elem);
        }
        syn::visit::visit_type_tuple(self, i);
    }

    // Visit array types like [T; N]
    fn visit_type_array(&mut self, i: &'a TypeArray) {
        self.visit_type(&i.elem);
        syn::visit::visit_type_array(self, i);
    }

    // Visit slice types like [T]
    fn visit_type_slice(&mut self, i: &'a TypeSlice) {
        self.visit_type(&i.elem);
        syn::visit::visit_type_slice(self, i);
    }

    // Visit raw pointer types like *const T and *mut T
    fn visit_type_ptr(&mut self, i: &'a TypePtr) {
        self.visit_type(&i.elem);
        syn::visit::visit_type_ptr(self, i);
    }

    // Visit reference types like &T and &mut T
    fn visit_type_reference(&mut self, i: &'a TypeReference) {
        self.visit_type(&i.elem);
        syn::visit::visit_type_reference(self, i);
    }

    // Visit bare function types like fn(T) -> U
    fn visit_type_bare_fn(&mut self, i: &'a TypeBareFn) {
        for input in &i.inputs {
            self.visit_bare_fn_arg(input);
        }
        self.visit_return_type(&i.output);
        syn::visit::visit_type_bare_fn(self, i);
    }

    // Visit impl Trait types used in return position or as standalone types
    fn visit_type_impl_trait(&mut self, i: &'a TypeImplTrait) {
        for bound in &i.bounds {
            self.visit_type_param_bound(bound);
        }
        syn::visit::visit_type_impl_trait(self, i);
    }

    // Visit grouped types, which are used to control the order of evaluation in complex type expressions
    fn visit_type_group(&mut self, i: &'a TypeGroup) {
        self.visit_type(&i.elem);
        syn::visit::visit_type_group(self, i);
    }

    // Visit macro types. Handling macro types can be complex as their structure depends on the macro's expansion
    fn visit_type_macro(&mut self, i: &'a TypeMacro) {
        // Macro types require special handling based on the macro expansion
        syn::visit::visit_type_macro(self, i);
    }
}
