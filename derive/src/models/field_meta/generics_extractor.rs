/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::models::*;
use darling::FromGenerics;
use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, visit::Visit, *};

use super::CustomType;

#[derive(Debug)]
pub struct StrippedBoundsGenerics(pub Generics);

#[derive(Clone, Debug, Default)]
pub struct CustomGenericsWithoutStaticInImpl(pub Generics);

create_tokenstream_wrapper!(=>ImplGenerics);
create_tokenstream_wrapper!(=>TypeGenerics);
create_tokenstream_wrapper!(=>WhereClause);

#[derive(Clone, Debug)]
struct GenericsMeta(ImplGenerics, TypeGenerics, Option<WhereClause>);

#[derive(Clone, Debug, Default)]
pub struct CustomGenerics {
    original_generics: Generics,
    generics_without_static_in_impl: Generics,
}

impl CustomGenerics {
    pub fn new(original_generics: Generics) -> Self {
        let generics_without_static_in_impl = original_generics.clone();
        let filter_static = |param: &GenericParam| match param {
            GenericParam::Lifetime(lifetime_def) => lifetime_def.lifetime.ident != "static",
            _ => true,
        };

        let generics_without_static_in_impl_params = generics_without_static_in_impl
            .params
            .into_iter()
            .filter(|param| filter_static(param))
            .collect();

        let gen = Generics {
            params: generics_without_static_in_impl_params,
            ..original_generics.clone()
        };

        CustomGenerics {
            original_generics,
            generics_without_static_in_impl: gen,
        }
    }

    pub fn into_inner(self) -> Generics {
        self.original_generics
    }

    pub fn into_inner_ref(&self) -> &Generics {
        &self.original_generics
    }

    pub fn into_inner_ref_mut(&mut self) -> &mut Generics {
        &mut self.original_generics
    }

    pub fn params(&self) -> &Punctuated<syn::GenericParam, Token![,]> {
        &self.original_generics.params
    }

    pub fn remove_static_lifetime(&mut self) {
        let filter_static = |param: &GenericParam| match param {
            GenericParam::Lifetime(lifetime_def) => lifetime_def.lifetime.ident != "static",
            _ => true,
        };

        let generics_without_static_in_impl_params = self
            .original_generics
            .params
            .iter()
            .filter(|param| filter_static(param))
            .cloned()
            .collect();

        self.original_generics.params = generics_without_static_in_impl_params;
    }

    pub fn split_for_impl(&self) -> (ImplGenerics, TypeGenerics, WhereClause) {
        let binding = self.strip_static_from_generics();
        let (impl_gen, ty_gen, wh) = binding.generics_without_static_in_impl.split_for_impl();
        (
            quote!(#impl_gen).into(),
            quote!(#ty_gen).into(),
            quote!(#wh).into(),
        )
    }

    pub fn strip_static_from_generics(&self) -> Self {
        let generics_without_static_in_impl = self.original_generics.clone();
        let filter_static = |param: &GenericParam| match param {
            GenericParam::Lifetime(lifetime_def) => lifetime_def.lifetime.ident != "static",
            _ => true,
        };

        let generics_without_static_in_impl_params = generics_without_static_in_impl
            .params
            .into_iter()
            .filter(|param| filter_static(param))
            .collect();

        let gen = Generics {
            params: generics_without_static_in_impl_params,
            ..self.original_generics.clone()
        };

        Self {
            original_generics: self.original_generics.clone(),
            generics_without_static_in_impl: gen,
        }
    }

    pub fn to_basic_generics(&self) -> &Generics {
        &self.into_inner_ref()
    }

    pub fn strip_bounds_from_generics(original_generics: &Generics) -> StrippedBoundsGenerics {
        let stripped_params = original_generics
            .params
            .iter()
            .map(|param| {
                match param {
                    GenericParam::Type(type_param) => {
                        // Keep only the type identifier
                        let ident = &type_param.ident;
                        let new_type_param: GenericParam = parse_quote!(#ident);
                        new_type_param
                    }
                    GenericParam::Lifetime(lifetime_def) => {
                        // Keep only the lifetime identifier
                        let lifetime = &lifetime_def.lifetime;
                        let new_lifetime_def: GenericParam = parse_quote!(#lifetime);
                        new_lifetime_def
                    }
                    GenericParam::Const(const_param) => {
                        // Keep only the const parameter
                        let ident = &const_param.ident;
                        let ty = &const_param.ty;
                        let new_const_param: GenericParam = parse_quote! { const #ident: #ty };
                        new_const_param
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

    pub fn to_angle_bracketed(&self) -> AngleBracketedGenericArguments {
        let args = self
            .to_basic_generics()
            .params
            .iter()
            .map(|param| match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = &type_param.ident;
                    let ty: Type = syn::parse_quote!(#ident);
                    GenericArgument::Type(ty)
                }
                syn::GenericParam::Lifetime(lifetime_def) => {
                    let lifetime = &lifetime_def.lifetime;
                    GenericArgument::Lifetime(Lifetime::new(&lifetime.to_string(), lifetime.span()))
                }
                syn::GenericParam::Const(const_param) => {
                    let ident = &const_param.ident;
                    let ty = &const_param.ty;
                    let expr: Expr = syn::parse_quote!(#ident as #ty);
                    GenericArgument::Const(expr)
                }
            })
            .collect();

        AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args,
            gt_token: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StructGenerics(pub CustomGenerics);

impl FromGenerics for StructGenerics {
    fn from_generics(generics: &Generics) -> darling::Result<Self> {
        Ok(Self(CustomGenerics::new(generics.clone())))
    }
}

impl std::ops::Deref for StructGenerics {
    type Target = CustomGenerics;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// PhantomData<(&'a dyn std::any::Any, &'b dyn std::any::Any, T, U, V)>,
#[derive(Clone, Debug)]
pub struct PhantomDataType(Type);

impl ToTokens for PhantomDataType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.to_token_stream());
    }
}

impl StructGenerics {
    pub fn phantom_marker_type(&self) -> PhantomDataType {
        let args = self
            .to_basic_generics_ref()
            .params
            .iter()
            .map(|param| match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = &type_param.ident;
                    let ty: Type = syn::parse_quote!(#ident);
                    // GenericArgument::Type(ty)
                    quote!(#ty)
                }
                syn::GenericParam::Lifetime(lifetime_def) => {
                    let lifetime = &lifetime_def.lifetime;
                    // GenericArgument::Lifetime(Lifetime::new(&lifetime.to_string(), lifetime.span()))
                    quote!(& #lifetime dyn ::std::any::Any)
                }
                syn::GenericParam::Const(const_param) => {
                    let ident = &const_param.ident;
                    let ty = &const_param.ty;
                    let expr: Expr = syn::parse_quote!(#ident as #ty);
                    // GenericArgument::Const(expr)
                    quote!(#expr)
                }
            })
            .collect::<Vec<TokenStream>>();

        let marker = quote! (::std::marker::PhantomData<( #( #args ),* )>);
        let marker: Type = parse_quote!(#marker);
        // let marker: Type = syn::parse2(marker).unwrap();
        PhantomDataType(marker)
    }

    pub fn to_basic_generics_ref(&self) -> &Generics {
        &self.0.into_inner_ref()
    }

    pub fn split_for_impl(&self) -> (ImplGenerics, TypeGenerics, WhereClause) {
        self.0.split_for_impl()
    }

    pub fn to_angle_bracketed(&self) -> AngleBracketedGenericArguments {
        self.0.to_angle_bracketed()
    }
}

#[derive(Clone, Debug, Default)]
pub struct FieldGenerics(pub CustomGenerics);

impl FieldGenerics {
    pub fn to_basic_generics_ref(&self) -> &Generics {
        &self.0.into_inner_ref()
    }

    pub fn to_basic_generics_ref_mut(&mut self) -> &mut Generics {
        &mut self.0.original_generics
    }
}

pub struct GenericTypeExtractor<'a> {
    pub struct_generics: &'a StructGenerics,
    pub field_generics: FieldGenerics,
}

impl<'a> GenericTypeExtractor<'a> {
    pub fn sync_field_type_to_current_struct_generics(
        model_attributes: &'a ModelAttributes,
        field_ty: &CustomType,
    ) -> CustomGenerics {
        let mut generics = Self {
            struct_generics: model_attributes.generics(),
            field_generics: Default::default(),
        };
        generics.visit_type(field_ty.into_inner_ref());
        generics.field_generics.0.strip_static_from_generics()
    }

    fn add_lifetime_if_not_exists(&mut self, lt: &Lifetime) {
        let lifetime_exists = self
            .field_generics.to_basic_generics_ref()
            .params
            .iter()
            .any(|param| matches!(param, GenericParam::Lifetime(lifetime_def) if lifetime_def.lifetime == *lt));

        if !lifetime_exists {
            self.field_generics
                .to_basic_generics_ref_mut()
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
            if let Some(gen_param) = self.struct_generics.params().iter().find(|param| {
                matches!(param, GenericParam::Type(type_param) if segment.ident == type_param.ident)
            }) {
                self.field_generics.to_basic_generics_ref_mut().params.push(gen_param.clone());

                // Handle constraints on the generic parameter
                if let Some(where_clause) = &self.struct_generics.to_basic_generics_ref().where_clause {
                    for predicate in &where_clause.predicates {
                        if let WherePredicate::Type(predicate_type) = predicate {
                            if let syn::Type::Path(type_path) = &predicate_type.bounded_ty {
                                if type_path.path.is_ident(&segment.ident) {
                                    self.field_generics.to_basic_generics_ref_mut().make_where_clause().predicates.push(predicate.clone());
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
            if let PathArguments::AngleBracketed(_args) = &segment.arguments {
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
