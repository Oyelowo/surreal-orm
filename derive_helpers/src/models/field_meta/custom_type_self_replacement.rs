/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

use syn::{
    parse_quote,
    punctuated::Punctuated,
    token::Comma,
    visit_mut::{self, VisitMut},
    AngleBracketedGenericArguments, ExprPath, GenericArgument, Ident, Path, PathArguments,
    PathSegment, Type, TypePath,
};

use crate::models::{CustomType, CustomTypeNoSelf};

pub struct ReplaceSelfVisitor {
    pub struct_ident: Ident,
    pub generics: AngleBracketedGenericArguments,
}

impl ReplaceSelfVisitor {
    #[allow(dead_code)]
    pub fn replace_self(&mut self, ty: &CustomType) -> CustomTypeNoSelf {
        let mut ty = ty.into_inner_ref().clone();
        self.visit_type_mut(&mut ty);
        CustomTypeNoSelf::new(ty.clone())
    }
}
impl VisitMut for ReplaceSelfVisitor {
    fn visit_type_mut(&mut self, i: &mut Type) {
        // Handle the top-level replacements and delegate deeper inspection to other visit_*_mut methods.
        visit_mut::visit_type_mut(self, i);
    }

    fn visit_path_segment_mut(&mut self, segment: &mut PathSegment) {
        if segment.ident == "Self" {
            segment.ident = self.struct_ident.clone();
            if !self.generics.args.is_empty() {
                segment.arguments = PathArguments::AngleBracketed(self.generics.clone());
            }
        } else {
            // Recursively handle the arguments of the segment.
            match &mut segment.arguments {
                PathArguments::AngleBracketed(angle_bracketed) => {
                    for arg in &mut angle_bracketed.args {
                        if let GenericArgument::Type(ty) = arg {
                            self.visit_type_mut(ty)
                        }
                    }
                }
                PathArguments::Parenthesized(parenthesized) => {
                    for input in &mut parenthesized.inputs {
                        self.visit_type_mut(input);
                    }
                    self.visit_return_type_mut(&mut parenthesized.output);
                }
                _ => {}
            }
        }
    }

    fn visit_angle_bracketed_generic_arguments_mut(
        &mut self,
        i: &mut AngleBracketedGenericArguments,
    ) {
        for arg in &mut i.args {
            self.visit_generic_argument_mut(arg);
        }
    }

    // Replace `Self` in paths
    fn visit_path_mut(&mut self, i: &mut Path) {
        // Example: Replace `Self` in `Self::method()` or just `Self`
        if i.segments.len() == 1 && i.segments[0].ident == "Self" {
            i.segments[0].ident = self.struct_ident.clone();
            if !self.generics.args.is_empty() {
                i.segments[0].arguments = PathArguments::AngleBracketed(self.generics.clone());
            }
        } else {
            visit_mut::visit_path_mut(self, i);
        }
    }

    // Replace `Self` in type paths
    fn visit_type_path_mut(&mut self, i: &mut TypePath) {
        // Example: Replace `Self` in type annotations, e.g., `let x: Self;`
        self.visit_path_mut(&mut i.path);
        visit_mut::visit_type_path_mut(self, i);
    }

    // Replace `Self` in expressions
    fn visit_expr_path_mut(&mut self, i: &mut ExprPath) {
        // Example: Replace `Self` in expressions, e.g., `Self::function()`
        if i.path.is_ident("Self") {
            let struct_ident = &self.struct_ident;
            i.path = parse_quote! { #struct_ident };
            if !self.generics.args.is_empty() {
                i.path
                    .segments
                    .last_mut()
                    .expect("Empty paths segments.")
                    .arguments = PathArguments::AngleBracketed(self.generics.clone());
            }
        }
        visit_mut::visit_expr_path_mut(self, i);
    }

    // Replace `Self` in reference types (&Self, &mut Self)
    fn visit_type_reference_mut(&mut self, i: &mut syn::TypeReference) {
        // Example: Replace `Self` in `&Self` and `&mut Self`
        syn::visit_mut::visit_type_reference_mut(self, i);
    }

    // Replace `Self` in tuple types (Self, Self)
    fn visit_type_tuple_mut(&mut self, i: &mut syn::TypeTuple) {
        // Example: Replace `Self` in `(Self, Self)`
        syn::visit_mut::visit_type_tuple_mut(self, i);
    }

    // Replace `Self` in slice types ([Self])
    fn visit_type_slice_mut(&mut self, i: &mut syn::TypeSlice) {
        // Example: Replace `Self` in `[Self]`
        syn::visit_mut::visit_type_slice_mut(self, i);
    }

    // Replace `Self` in array types ([Self; N])
    fn visit_type_array_mut(&mut self, i: &mut syn::TypeArray) {
        // Example: Replace `Self` in `[Self; 5]`
        syn::visit_mut::visit_type_array_mut(self, i);
    }

    // Replace `Self` in pointer types (*const Self, *mut Self)
    fn visit_type_ptr_mut(&mut self, i: &mut syn::TypePtr) {
        // Example: Replace `Self` in `*const Self` and `*mut Self`
        syn::visit_mut::visit_type_ptr_mut(self, i);
    }

    // Replace `Self` in group types (used in complex type manipulation)
    fn visit_type_group_mut(&mut self, i: &mut syn::TypeGroup) {
        // Example: Rarely used directly but can wrap other type constructs
        syn::visit_mut::visit_type_group_mut(self, i);
    }

    // Replace `Self` in "never" types (!Self)
    fn visit_type_never_mut(&mut self, i: &mut syn::TypeNever) {
        // Example: Replace `Self` in `!Self`, though `!Self` is a rare and somewhat theoretical use case
        syn::visit_mut::visit_type_never_mut(self, i);
    }

    // Replace `Self` in generic arguments (e.g., Vec<Self>)
    fn visit_generic_argument_mut(&mut self, i: &mut syn::GenericArgument) {
        if let syn::GenericArgument::Type(ty) = i {
            self.visit_type_mut(ty);
        }
        syn::visit_mut::visit_generic_argument_mut(self, i);
    }

    // Replace `Self` in trait objects (e.g., Box<dyn Trait<Self>>)
    fn visit_type_trait_object_mut(&mut self, i: &mut syn::TypeTraitObject) {
        // Example: Replace `Self` in trait objects
        for bound in &mut i.bounds {
            if let syn::TypeParamBound::Trait(trait_bound) = bound {
                for segment in &mut trait_bound.path.segments {
                    self.visit_path_segment_mut(segment);
                }
            }
        }
        syn::visit_mut::visit_type_trait_object_mut(self, i);
    }

    // Replace `Self` in impl Trait (e.g., impl Trait<Self>)
    fn visit_type_impl_trait_mut(&mut self, i: &mut syn::TypeImplTrait) {
        // Example: Replace `Self` in `impl Trait for Self`
        for bound in &mut i.bounds {
            if let syn::TypeParamBound::Trait(trait_bound) = bound {
                for segment in &mut trait_bound.path.segments {
                    self.visit_path_segment_mut(segment);
                }
            }
        }
        syn::visit_mut::visit_type_impl_trait_mut(self, i);
    }

    // Replace `Self` in function pointers (e.g., fn foo(x: Self) -> Self)
    fn visit_type_bare_fn_mut(&mut self, i: &mut syn::TypeBareFn) {
        // Example: Replace `Self` in function pointer types
        for input in &mut i.inputs {
            self.visit_type_mut(&mut input.ty);
        }
        if let syn::ReturnType::Type(_, ty) = &mut i.output {
            self.visit_type_mut(ty);
        }
        syn::visit_mut::visit_type_bare_fn_mut(self, i);
    }
}

#[allow(dead_code)]
// Convert a vector of generic arguments to a punctuated list of generic arguments
// e.g., vec![A, B, C] -> A, B, C
fn vec_to_punctuated(vec: Vec<GenericArgument>) -> Punctuated<GenericArgument, Comma> {
    let mut punctuated = Punctuated::new();
    for item in vec {
        punctuated.push_value(item);
        punctuated.push_punct(Comma::default());
    }
    punctuated
}

#[cfg(test)]
mod tests {
    use quote::{format_ident, quote};

    use super::*;

    #[test]
    fn replace_self() {
        let struct_ident = Ident::new("User", proc_macro2::Span::call_site());
        let generics = AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args: vec_to_punctuated(vec![
                parse_quote!('a),
                parse_quote!('b),
                parse_quote!(U),
                parse_quote!(T),
                // GenericArgument::Lifetime(syn::Lifetime::new("'a", proc_macro2::Span::call_site())),
                // GenericArgument::Lifetime(syn::Lifetime::new("'b", proc_macro2::Span::call_site())),
                // GenericArgument::Type(Type::Path(TypePath {
                //     qself: None,
                //     path: Path::from(Ident::new("U", proc_macro2::Span::call_site())),
                // })),
                // GenericArgument::Type(Type::Path(TypePath {
                //     qself: None,
                //     path: Path::from(Ident::new("V", proc_macro2::Span::call_site())),
                // })),
            ]),
            gt_token: Default::default(),
        };

        let ty_to_replace = Type::Path(TypePath {
            qself: None,
            path: Path::from(Ident::new("Self", proc_macro2::Span::call_site())),
        });

        let mut replacer = ReplaceSelfVisitor {
            struct_ident,
            generics,
        };
        let ty_to_replace = replacer.replace_self(&ty_to_replace.into());

        assert_eq!(
            quote!(#ty_to_replace).to_string(),
            quote!(User<'a, 'b, U, T , >).to_string()
        );
    }

    #[test]
    fn replace_self_using_quote() {
        let struct_ident = format_ident!("User");
        let generics = quote!(<'a, 'b, U, V>);
        let generics: AngleBracketedGenericArguments = syn::parse2(generics).unwrap();
        let type_with_self = quote!(Self<'a, 'b, U, V>);

        let mut replacer = ReplaceSelfVisitor {
            struct_ident,
            generics,
        };
        let type_with_self: syn::Type = syn::parse2(type_with_self).unwrap();
        let ty_to_replace = replacer.replace_self(&type_with_self.into());

        assert_eq!(
            quote::quote!(#ty_to_replace).to_string(),
            quote::quote!(User<'a, 'b, U, V>).to_string()
        );
    }
}
