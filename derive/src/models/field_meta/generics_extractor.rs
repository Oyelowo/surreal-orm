/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::models::*;
use darling::ast::GenericParam;
use syn::{punctuated::Punctuated, visit::Visit, *};

use super::CustomType;

struct StrippedBoundsGenerics(pub Generics);

struct CustomGenerics(pub Generics);

impl CustomGenerics {
    pub fn params(&self) -> &Punctuated<GenericParam, Token![,]> {
        &self.0.params
    }

    fn strip_bounds_from_generics(original_generics: &Generics) -> StrippedBoundsGenerics {
        let stripped_params = original_generics
            .params
            .iter()
            .map(|param| {
                match param {
                    GenericParam::Type(type_param) => {
                        // Keep only the type identifier
                        let ident = &type_param.ident;
                        parse_quote!(#ident)
                    }
                    GenericParam::Lifetime(lifetime_def) => {
                        // Keep only the lifetime identifier
                        let lifetime = &lifetime_def.lifetime;
                        parse_quote!(#lifetime)
                    }
                    GenericParam::Const(const_param) => {
                        // Keep only the const parameter
                        let ident = &const_param.ident;
                        parse_quote!(const #ident: usize)
                    }
                }
            })
            .collect();

        StrippedBoundsGenerics(Generics {
            params: stripped_params,
            where_clause: None,
            ..*original_generics
        })
    }

    pub fn split_for_impl(&self) -> (ImplGenerics, TypeGenerics, Option<&WhereClause>) {
        self.0.split_for_impl()
    }
}

#[derive(Debug)]
pub struct StructGenerics(pub CustomGenerics);

impl std::ops::Deref for StructGenerics {
    type Target = CustomGenerics;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StructGenerics {
    pub fn to_basic_generics(&self) -> &Generics {
        &self.0 .0
    }

    pub fn split_for_impl(&self) -> (ImplGenerics, TypeGenerics, Option<&WhereClause>) {
        self.0.split_for_impl()
    }
}

pub struct FieldGenerics(pub CustomGenerics);

pub(crate) struct GenericTypeExtractor<'a> {
    struct_generics: &'a StructGenerics,
    field_generics: FieldGenerics,
}

impl<'a> GenericTypeExtractor<'a> {
    pub fn extract_generics_for_complex_type(
        model_attributes: &impl ModelAttributes,
        field_ty: &'a CustomType,
    ) -> &'a CustomGenerics {
        let generics = Self {
            struct_generics: &model_attributes.generics,
            field_generics: Generics::default(),
        };
        generics.visit_type(&field_ty.to_basic_type());
        &generics.field_generics.0.into()
    }

    fn add_lifetime_if_not_exists(&mut self, lt: &Lifetime) {
        let lifetime_exists = self
            .field_generics
            .0
            .params
            .iter()
            .any(|param| matches!(param, GenericParam::Lifetime(lifetime_def) if lifetime_def.lifetime == *lt));

        if !lifetime_exists {
            self.field_generics
                .0
                .params
                .push(GenericParam::Lifetime(LifetimeParam {
                    attrs: Vec::new(),
                    lifetime: lt.clone(),
                    colon_token: None,
                    bounds: syn::punctuated::Punctuated::new(),
                }));
        }
    }
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
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                for arg in &args.args {
                    if let syn::GenericArgument::Lifetime(lt) = arg {
                        self.add_lifetime_if_not_exists(lt);
                    }
                }
            }
            // Recursively visit nested generic arguments
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                // for arg in &args.args {
                //     match arg {
                //         // Recursively visit the nested type
                //         syn::GenericArgument::Type(ty) => self.visit_type(ty),
                //         syn::GenericArgument::Lifetime(lt) => {
                //             // Here we handle lifetime arguments
                //             if !self.field_generics.params.iter().any(|param| matches!(param, GenericParam::Lifetime(lifetime_def) if lifetime_def.lifetime == *lt)) {
                //                 // Only add the lifetime if it's not already in the list
                //                 self.field_generics.params.push(GenericParam::Lifetime(syn::LifetimeParam {
                //                     attrs: Vec::new(),
                //                     lifetime: lt.clone(),
                //                     colon_token: None,
                //                     bounds: syn::punctuated::Punctuated::new(),
                //                 }));
                //             }
                //         }
                //         _ => {}
                //     }
                // }
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
        // self.visit_type(&i.elem);
        // syn::visit::visit_type_reference(self, i);
        if let Some(lifetime) = &i.lifetime {
            self.add_lifetime_if_not_exists(lifetime);
        }
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
