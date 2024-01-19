/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub struct FieldGenericsMeta<'a> {
    pub(crate) field_impl_generics: syn::ImplGenerics<'a>,
    pub(crate) field_ty_generics: syn::TypeGenerics<'a>,
    pub(crate) field_where_clause: Option<&'a syn::WhereClause>,
}

impl<'a> FieldGenericsMeta<'a> {
    // This extracts generics metadata for field and from struct generics metadata.
    // This could come from the concrete rust field type or
    // as an attribute on the field from links which link to
    // other tables structs models i.e Edge, Node and Objects.
    // These are usually specified using the link_one, link_self
    // and link_many and relate attributes.
    // e.g
    // #[surreal_orm(link_one = User<'a, T, u32>)]
    // student: LinkOne<User<'a, T, u32>
    pub fn new(
        &self,
        struct_name_ident: &Ident,
        struct_generics: &Generics,
        field_type: &Type,
    ) -> FieldGenericsMeta<'a> {
        let (_, struct_ty_generics, _) = struct_generics.split_for_impl();
        let field_type =
            &replace_self_in_type_str(&field_type, struct_name_ident, &struct_ty_generics);
        let mut field_extractor = GenericTypeExtractor::new(struct_generics);
        let (field_impl_generics, field_ty_generics, field_where_clause) = field_extractor
            .extract_generics_for_complex_type(&field_type)
            .split_for_impl();
        FieldGenericsMeta {
            field_impl_generics,
            field_ty_generics,
            field_where_clause,
        }
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

    fn add_lifetime_if_not_exists(&mut self, lt: &Lifetime) {
        let lifetime_exists = self
            .field_generics
            .params
            .iter()
            .any(|param| matches!(param, GenericParam::Lifetime(lifetime_def) if lifetime_def.lifetime == *lt));

        if !lifetime_exists {
            self.field_generics
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

fn construct_replacement_segment(
    struct_name: &syn::Ident,
    ty_generics: &syn::TypeGenerics,
) -> PathSegment {
    // Generate the replacement tokens
    let replacement_tokens: proc_macro2::TokenStream = quote!(#struct_name #ty_generics);

    // Parse the tokens into a PathSegment
    let replacement_segment: PathSegment =
        parse2(replacement_tokens).expect("Failed to parse replacement segment");

    replacement_segment
}

// use quote::quote;
// use syn::parse_str;

// pub fn replace_self_in_type_str(
//     ty: &Type,
//     struct_name: &syn::Ident,
//     ty_generics: &syn::TypeGenerics,
// ) -> Result<Type, syn::Error> {
//     // Convert the type to a string
//     let ty_str = quote!(#ty).to_string();
//
//     // Construct the replacement string
//     let replacement = format!("{}{}", struct_name, ty_generics.into_token_stream());
//
//     // Replace "Self" with the desired type
//     let modified_ty_str = ty_str.replace("Self", &replacement);
//     println!("rara---ty_str: {}", ty_str);
//     println!("rara---replacement: {}", replacement);
//     println!("rara---modified_ty_str: {}", modified_ty_str);
//
//     // Parse the string back into a Type
//     let parsed = parse_str::<Type>(&modified_ty_str).map_err(|e| syn::Error::new_spanned(ty, e));
//     println!(
//         "rara---parsed: {}",
//         parsed.clone().unwrap().into_token_stream()
//     );
//     parsed
// }

// Example usage
// let ty: Type = /* ... */; // Your Type here
// let struct_name = "MyStruct";
// let ty_generics_str = "<T, U>"; // Generics as a string

// match replace_self_in_type_str(&ty, struct_name, ty_generics_str) {
//     Ok(replaced_type) => {
//         // Use the replaced type
//     }
//     Err(e) => {
//         // Handle the error
//     }
// }
