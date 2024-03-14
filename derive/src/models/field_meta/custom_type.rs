/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::{
    self, parse_quote, spanned::Spanned, visit::Visit, visit_mut::VisitMut, GenericArgument, Ident,
    Lifetime, Path, PathArguments, PathSegment, Token, Type, TypeReference,
};

use crate::models::*;

use super::{field_name_serialized::DbFieldName, *};

#[derive(Debug, Clone)]
pub struct CustomTypeNoSelf(CustomType);

impl CustomTypeNoSelf {
    pub fn new(ty: Type) -> Self {
        Self(CustomType(ty))
    }

    pub fn into_inner(self) -> CustomType {
        self.0
    }

    pub fn as_custom_type_ref(&self) -> &CustomType {
        &self.0
    }

    pub fn type_name(&self) -> ExtractorResult<Ident> {
        self.0.type_name()
    }

    pub fn to_basic_type(&self) -> &Type {
        self.0.into_inner_ref()
    }

    pub fn to_path(&self) -> ExtractorResult<Path> {
        match &self.to_basic_type() {
            Type::Path(type_path) => Ok(type_path.path.clone()),
            _ => Err(
                syn::Error::new(self.0.to_token_stream().span(), "Expected a struct type").into(),
            ),
        }
    }

    pub fn inner_angle_bracket_type(&self) -> ExtractorResult<Option<CustomTypeInnerAngleBracket>> {
        self.0.inner_angle_bracket_type()
    }
}

impl ToTokens for CustomTypeNoSelf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

// takes type as path and stringified also e.g User<'a, T, u32> or "User<'a, T, u32>"
#[derive(Debug, Clone)]
pub struct CustomType(Type);

create_custom_type_wrapper!(CustomTypeInnerAngleBracket);

impl FromMeta for CustomType {
    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        // panic!("Item: {:?}", item);
        // Type::from_meta(item).map(Self)
        let ty = match item {
            syn::Meta::Path(ref path) => {
                let ty = Type::Path(syn::TypePath {
                    qself: None,
                    path: path.clone(),
                });
                ty
            }
            syn::Meta::NameValue(ref name_value) => {
                // panic!("Name value: {:?}", name_value);
                let ty = match &name_value.value {
                    syn::Expr::Lit(lit_str) => match lit_str.lit {
                        syn::Lit::Str(ref lit_str) => {
                            let ty = syn::parse_str::<Type>(&lit_str.value())?;
                            // panic!("Parsed type: {:?}", ty);
                            ty
                        }
                        _ => {
                            return Err(darling::Error::custom(
                                "Unable to parse stringified type. Expected a valid Rust path or a stringified type",
                            ));
                        }
                    },
                    syn::Expr::Path(ref path) => {
                        let ty = Type::Path(syn::TypePath {
                            qself: None,
                            path: path.path.clone(),
                        });
                        ty
                    }
                    _ => {
                        return Err(darling::Error::custom(
                            "Expected a valid Rust path or a stringified type",
                        ));
                    }
                };
                ty
            }
            _ => {
                return Err(darling::Error::unsupported_shape(
                    "Expected a path or a name-value pair",
                ));
            }
        };
        Ok(Self(ty))
    }
}

// impl Parse for CustomType {
//     // TODO: Handle type parsing if frommeta does not work or manually implement fromMeta
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         todo!()
//     }
// }

impl From<Type> for CustomType {
    fn from(ty: Type) -> Self {
        Self(ty)
    }
}

impl ToTokens for CustomType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

#[derive(Debug, Clone)]
pub struct CustomTypeTurboFished(Type);

impl ToTokens for CustomTypeTurboFished {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl CustomType {
    pub fn new(ty: Type) -> Self {
        Self(ty)
    }

    pub fn into_inner(self) -> Type {
        self.0
    }

    pub fn into_inner_ref(&self) -> &Type {
        &self.0
    }

    pub fn remove_lifetime_and_reference(&self) -> Self {
        let ty = &self.0;
        let ty = match ty {
            Type::Reference(type_reference) => {
                let elem = type_reference.elem.as_ref();
                elem
            }
            _ => ty,
        };
        Self(ty.clone())
    }

    // e.g User if Option<User> or ::std::option::Option<User>
    pub fn inner_angle_bracket_type(&self) -> ExtractorResult<Option<CustomTypeInnerAngleBracket>> {
        match self.into_inner_ref() {
            Type::Path(type_path) => {
                let last_segment = type_path
                    .path
                    .segments
                    .last()
                    .ok_or_else(|| darling::Error::custom("Expected a type. Make sure there are no typos and you are using a proper struct as the linked Node."))?;
                if let PathArguments::AngleBracketed(angle_bracketed) = &last_segment.arguments {
                    let first_arg = angle_bracketed.args.first();
                    match first_arg {
                        Some(GenericArgument::Type(ty)) => Ok(Some(ty.clone().into())),
                        _ => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            }
            _ => {
                Err(syn::Error::new(self.to_token_stream().span(), "Expected a struct type").into())
            }
        }
    }

    // e.g from User<T> to User::<T>
    pub fn turbo_fishize(&self) -> ExtractorResult<CustomTypeTurboFished> {
        match self.into_inner_ref() {
            Type::Path(type_path) => {
                let mut path = type_path.path.clone();

                if let Some(last) = path.segments.last_mut() {
                    let arguments = std::mem::replace(&mut last.arguments, PathArguments::None);
                    match arguments {
                        PathArguments::AngleBracketed(angle_bracketed) => {
                            let colon2_token = Some(Token![::](angle_bracketed.span()));
                            last.arguments = PathArguments::AngleBracketed(
                                syn::AngleBracketedGenericArguments {
                                    colon2_token,
                                    ..angle_bracketed
                                },
                            );
                        }
                        _ => last.arguments = arguments,
                    }
                }

                let ty = Type::Path(syn::TypePath {
                    qself: type_path.qself.clone(),
                    path,
                });
                Ok(CustomTypeTurboFished(ty))
            }
            _ => {
                return Err(syn::Error::new(
                    self.to_token_stream().span(),
                    "Unsupported type for turbofishing",
                )
                .into());
            }
        }
    }

    pub fn type_name(&self) -> ExtractorResult<Ident> {
        match self.into_inner_ref() {
            Type::Path(type_path) => {
                let last_segment = type_path
                    .path
                    .segments
                    .last()
                    .ok_or_else(|| darling::Error::custom("Expected a type. Make sure there are no typos and you are using a proper struct as the linked Node."))?;
                Ok(last_segment.ident.clone())
            }
            _ => {
                Err(syn::Error::new(self.to_token_stream().span(), "Expected a struct type").into())
            }
        }
    }

    // This extracts generics metadata for field and from struct generics metadata.
    // This could come from the concrete rust field type or
    // as an attribute on the field from links which link to
    // other tables structs models i.e Edge, Node and Objects.
    // These are usually specified using the link_one, link_self
    // and link_many and relate attributes.
    // e.g
    // #[surreal_orm(link_one = User<'a, T, u32>)]
    // student: LinkOne<User<'a, T, u32>
    pub fn get_generics_from_current_struct(
        &self,
        model_attributes: &ModelAttributes,
    ) -> CustomGenerics {
        GenericTypeExtractor::sync_field_type_to_current_struct_generics(model_attributes, self)
    }

    pub fn extract_generics_for_complex_type(
        &self,
        model_attributes: &ModelAttributes,
        // field_ty: &CustomType,
    ) -> CustomGenerics {
        let mut generics = GenericTypeExtractor {
            struct_generics: model_attributes.generics(),
            field_generics: Default::default(),
        };
        generics.visit_type(self.into_inner_ref());
        // generics.visit_type(&field_ty.to_basic_type());
        generics.field_generics.0
    }

    // pub fn get_generics_meta<'a>(
    //     &self,
    //     model_attributes: &'a ModelAttributes,
    // ) -> FieldGenericsMeta<'a> {
    //     let binding =
    //         GenericTypeExtractor::extract_generics_for_complex_type(model_attributes, &self);
    //     let (field_impl_generics, field_ty_generics, field_where_clause) =
    //         &binding.split_for_impl();
    //     FieldGenericsMeta {
    //         field_impl_generics,
    //         field_ty_generics,
    //         field_where_clause: field_where_clause.cloned(),
    //     }
    // }

    #[allow(clippy::items_after_statements)]
    pub fn replace_self_with_current_struct_concrete_type(
        &self,
        model_attributes: &ModelAttributes,
    ) -> ExtractorResult<CustomTypeNoSelf> {
        // TODO: Consider using the declarative replacer over the more imperative approach
        // let replacer = ReplaceSelfVisitor {
        //     struct_ident: model_attributes.struct_as_path_no_bounds(),
        //     generics: model_attributes.generics().to_basic_generics(),
        // };
        // let x = replacer.replace_self(self.to_basic_type().clone());
        let ty = &self.into_inner_ref();
        let replacement_path_from_current_struct =
            model_attributes.struct_no_bounds()?.to_path()?;

        fn replace_self_in_segment(segment: &mut PathSegment, replacement_path: &Path) {
            if segment.ident == "Self" {
                if let Some(first_segment) = replacement_path.segments.first() {
                    *segment = first_segment.clone();
                }
            } else if let PathArguments::AngleBracketed(angle_args) = &mut segment.arguments {
                for arg in angle_args.args.iter_mut() {
                    if let GenericArgument::Type(t) = arg {
                        *t = replace_type(t, replacement_path);
                    }
                }
            }
        }

        // handle replacement within types
        fn replace_type(ty: &Type, replacement_path: &Path) -> Type {
            match ty {
                Type::Path(type_path) => {
                    let mut new_type_path = type_path.clone();
                    for segment in &mut new_type_path.path.segments {
                        replace_self_in_segment(segment, replacement_path);
                    }
                    Type::Path(new_type_path)
                }
                Type::Reference(type_reference) => {
                    let elem = Box::new(replace_type(&type_reference.elem, replacement_path));
                    Type::Reference(TypeReference {
                        and_token: type_reference.and_token,
                        lifetime: type_reference.lifetime.clone(),
                        mutability: type_reference.mutability,
                        elem,
                    })
                }
                Type::Paren(type_paren) => {
                    let elem = Box::new(replace_type(&type_paren.elem, replacement_path));
                    Type::Paren(syn::TypeParen {
                        paren_token: type_paren.paren_token,
                        elem,
                    })
                }
                Type::Group(type_group) => {
                    let elem = Box::new(replace_type(&type_group.elem, replacement_path));
                    Type::Group(syn::TypeGroup {
                        group_token: type_group.group_token,
                        elem,
                    })
                }
                Type::Array(type_array) => {
                    let elem = Box::new(replace_type(&type_array.elem, replacement_path));
                    Type::Array(syn::TypeArray {
                        bracket_token: type_array.bracket_token,
                        elem,
                        semi_token: type_array.semi_token,
                        len: type_array.len.clone(),
                    })
                }
                Type::Tuple(type_tuple) => {
                    let elems = type_tuple
                        .elems
                        .iter()
                        .map(|elem| replace_type(elem, replacement_path))
                        .collect();
                    Type::Tuple(syn::TypeTuple {
                        paren_token: type_tuple.paren_token,
                        elems,
                    })
                }
                // Type::BareFn(type_bare_fn) => {
                //     let inputs = type_bare_fn
                //         .inputs
                //         .iter()
                //         .map(|input| replace_type(&input.ty, replacement_path))
                //         .collect();
                //     let output = type_bare_fn
                //         .output
                //         .as_ref()
                //         .map(|output| replace_type(output, replacement_path));
                //     Type::BareFn(syn::TypeBareFn {
                //         lifetimes: type_bare_fn.lifetimes.clone(),
                //         unsafety: type_bare_fn.unsafety,
                //         abi: type_bare_fn.abi.clone(),
                //         fn_token: type_bare_fn.fn_token,
                //         paren_token: type_bare_fn.paren_token,
                //         inputs,
                //         variadic: type_bare_fn.variadic,
                //         output,
                //     })
                // }
                // Type::Never(type_never) => Type::Never(type_never),
                // Type::TraitObject(type_trait_object) => {
                //     let bounds = type_trait_object
                //         .bounds
                //         .iter()
                //         .map(|bound| replace_type(bound, replacement_path))
                //         .collect();
                //     Type::TraitObject(syn::TypeTraitObject {
                //         dyn_token: type_trait_object.dyn_token,
                //         bounds,
                //     })
                // }
                // Type::ImplTrait(type_impl_trait) => {
                //     let bounds = type_impl_trait
                //         .bounds
                //         .iter()
                //         .map(|bound| replace_type(bound, replacement_path))
                //         .collect();
                //     Type::ImplTrait(syn::TypeImplTrait {
                //         impl_token: type_impl_trait.impl_token,
                //         bounds,
                //     })
                // }
                // TODO: Extend to handle other types like Tuple, Array, etc.
                _ => ty.clone(),
            }
        }

        Ok(CustomTypeNoSelf::new(replace_type(
            ty,
            &replacement_path_from_current_struct,
        )))
    }

    fn _strip_bounds_from_generics(&self) -> Self {
        let stripped_ty = match self.into_inner_ref() {
            Type::Path(type_path) => {
                let mut new_type_path = type_path.clone();

                // Iterate through the path segments
                for segment in &mut new_type_path.path.segments {
                    if let PathArguments::AngleBracketed(angle_bracketed) = &mut segment.arguments {
                        // Collect only the generic identifiers, dropping bounds
                        let modified_args = angle_bracketed
                            .args
                            .iter()
                            .map(|arg| {
                                match arg {
                                    GenericArgument::Type(Type::Path(tp)) => {
                                        // Keep only the type identifier
                                        let ident = &tp
                                            .path
                                            .get_ident()
                                            .expect("Problem getting type path as ident.");
                                        parse_quote!(#ident)
                                    }
                                    GenericArgument::Lifetime(lifetime) => {
                                        // Keep only the lifetime identifier
                                        parse_quote!(#lifetime)
                                    }
                                    GenericArgument::Const(const_param) => {
                                        // Keep only the const parameter
                                        parse_quote!(#const_param)
                                    }
                                    _ => arg.clone(), // Other types of arguments are left as is
                                }
                            })
                            .collect();

                        // Replace the arguments with the modified ones
                        angle_bracketed.args = modified_args;
                    }
                }

                Type::Path(new_type_path)
            }
            _ => self.into_inner_ref().clone(),
        };
        Self(stripped_ty)
    }

    pub fn replace_lifetimes_with_underscore(&self) -> Self {
        struct ReplaceLifetimesVisitor;
        impl VisitMut for ReplaceLifetimesVisitor {
            fn visit_lifetime_mut(&mut self, i: &mut Lifetime) {
                *i = Lifetime::new("'_", i.apostrophe);
            }
        }

        let ty = &self.0;
        let mut ty = ty.clone();
        let mut visitor = ReplaceLifetimesVisitor;

        visitor.visit_type_mut(&mut ty);
        ty.into()
    }

    pub fn is_numeric(&self) -> bool {
        let ty = &self.into_inner_ref();
        let type_is_numeric = match ty {
            syn::Type::Path(ref p) => {
                let path = &p.path;
                path.leading_colon.is_none() && path.segments.len() == 1 && {
                    let ident = &path.segments[0].ident.to_string();
                    [
                        "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64",
                        "i128", "isize", "f32", "f64",
                    ]
                    .iter()
                    .any(|&x| x == ident)
                }
            }
            _ => false,
        };

        type_is_numeric
    }

    pub fn raw_type_is_float(&self) -> bool {
        match self.into_inner_ref() {
            syn::Type::Path(ref p) => {
                let path = &p.path;
                path.leading_colon.is_none() && path.segments.len() == 1 && {
                    let ident = &path.segments[0].ident.to_string();
                    ["f32", "f64"].iter().any(|&x| x == ident)
                }
            }
            _ => false,
        }
    }

    pub fn raw_type_is_integer(&self) -> bool {
        match self.into_inner_ref() {
            syn::Type::Path(ref p) => {
                let path = &p.path;
                path.leading_colon.is_none() && path.segments.len() == 1 && {
                    let ident = &path.segments[0].ident.to_string();
                    [
                        "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64",
                        "i128", "isize",
                    ]
                    .iter()
                    .any(|&x| x == ident)
                }
            }
            _ => false,
        }
    }

    pub fn raw_type_is_string(&self) -> bool {
        match &self.into_inner_ref() {
            syn::Type::Path(ref p) => {
                let path = &p.path;
                path.leading_colon.is_none() && path.segments.len() == 1 && {
                    let ident = &path.segments[0].ident.to_string();
                    ["String", "str"].contains(&ident.as_str())
                }
            }
            syn::Type::Reference(ref r) => {
                if let syn::Type::Path(ref p) = *r.elem {
                    let path = &p.path;
                    path.leading_colon.is_none() && path.segments.len() == 1 && {
                        let ident = &path.segments[0].ident.to_string();
                        ["String", "str"].contains(&ident.as_str())
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn raw_type_is_bool(&self) -> bool {
        match self.into_inner_ref() {
            syn::Type::Path(ref p) => {
                let path = &p.path;
                path.leading_colon.is_none() && path.segments.len() == 1 && {
                    let ident = &path.segments[0].ident.to_string();
                    ["bool"].iter().any(|&x| x == ident)
                }
            }
            _ => false,
        }
    }

    pub fn is_set(&self) -> bool {
        let ty = &self.into_inner_ref();
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Infer(_))) = args.args.first()
                    {
                        return false;
                    }
                    last_seg.ident.to_string().to_lowercase() == "hashset"
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        let ty = &self.into_inner_ref();
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Infer(_))) = args.args.first()
                    {
                        return false;
                    }
                    last_seg.ident == "Vec"
                } else {
                    false
                }
            }
            syn::Type::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        let ty = self.into_inner_ref();
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Infer(_))) = args.args.first()
                    {
                        return false;
                    }
                    last_seg.ident == "Vec"
                } else {
                    false
                }
            }
            syn::Type::Array(_) => true,
            _ => false,
        }
    }

    pub fn raw_type_is_optional(&self) -> bool {
        let ty = self.into_inner_ref();
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Infer(_))) = args.args.first()
                    {
                        return false;
                    }
                    last_seg.ident == "Option"
                } else {
                    false
                }
            }
            syn::Type::Array(_) => true,
            _ => false,
        }
    }

    pub fn raw_type_is_hash_set(&self) -> bool {
        let ty = self.into_inner_ref();
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Infer(_))) = args.args.first()
                    {
                        return false;
                    }
                    last_seg.ident == "HashSet"
                } else {
                    false
                }
            }
            syn::Type::Array(_) => false,
            _ => false,
        }
    }

    pub fn raw_type_is_object(&self) -> bool {
        let ty = self.into_inner_ref();
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(syn::Type::Infer(_))) = args.args.first()
                    {
                        return false;
                    }
                    last_seg.ident == "HashMap" || last_seg.ident == "BTreeMap"
                } else {
                    false
                }
            }
            syn::Type::Array(_) => true,
            _ => false,
        }
    }

    pub fn raw_type_is_datetime(&self) -> bool {
        let ty = self.into_inner_ref();
        match ty {
            syn::Type::Path(type_path) => {
                let last_segment = type_path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                last_segment.ident.to_string().to_lowercase() == "datetime"
            }
            _ => false,
        }
    }

    pub fn raw_type_is_duration(&self) -> bool {
        let ty = self.into_inner_ref();
        match ty {
            syn::Type::Path(type_path) => {
                let last_segment = type_path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                last_segment.ident == "Duration"
            }
            _ => false,
        }
    }

    pub fn raw_type_is_geometry(&self) -> bool {
        let ty = &self.into_inner_ref();
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                last_seg.ident == "Geometry"
                    || last_seg.ident == "Point"
                    || last_seg.ident == "LineString"
                    || last_seg.ident == "Polygon"
                    || last_seg.ident == "MultiPoint"
                    || last_seg.ident == "MultiLineString"
                    || last_seg.ident == "MultiPolygon"
                    || last_seg.ident == "GeometryCollection"
            }
            syn::Type::Array(_) => true,
            _ => false,
        }
    }

    pub fn get_array_inner_type(&self) -> Option<CustomType> {
        let ty = &self.into_inner_ref();

        let item_ty = match ty {
            syn::Type::Path(type_path) => {
                let last_segment = type_path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                if last_segment.ident != "Vec" {
                    return None;
                }
                let item_ty = match last_segment.arguments {
                    syn::PathArguments::AngleBracketed(ref args) => args.args.first(),
                    _ => None,
                };
                match item_ty {
                    Some(syn::GenericArgument::Type(ty)) => ty,
                    _ => return None,
                }
            }
            // syn:Type::Array(array) => {
            //     array.elem.as_ref()
            // },
            _ => return None,
        };
        Some(item_ty.clone().into())
    }

    pub fn get_option_item_type(&self) -> Option<Type> {
        let ty = &self.into_inner_ref();

        let item_ty = match ty {
            syn::Type::Path(type_path) => {
                let last_segment = type_path
                    .path
                    .segments
                    .last()
                    .expect("Must have at least one segment");
                if last_segment.ident != "Option" {
                    return None;
                }
                let item_ty = match last_segment.arguments {
                    syn::PathArguments::AngleBracketed(ref args) => args.args.first(),
                    _ => None,
                };
                match item_ty {
                    Some(syn::GenericArgument::Type(ty)) => ty,
                    _ => return None,
                }
            }
            _ => return None,
        };
        Some(item_ty.clone())
    }

    pub fn infer_surreal_type_heuristically(
        &self,
        field_name: &DbFieldName,
        relation_type: &RelationType,
        model_type: &DataType,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let crate_name = get_crate_name(false);
        let ty = &self.into_inner_ref();

        let meta = if self.raw_type_is_bool() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Bool),
                field_type_db_token: quote!(#crate_name::FieldType::Bool).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<::std::primitive::bool>);).into(),
            }
        } else if self.raw_type_is_float() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Float),
                field_type_db_token: quote!(#crate_name::FieldType::Float).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Number>);).into(),
            }
        } else if self.raw_type_is_integer() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Int),
                field_type_db_token: quote!(#crate_name::FieldType::Int).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Number>);).into(),
            }
        } else if self.raw_type_is_string() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::String),
                field_type_db_token: quote!(#crate_name::FieldType::String).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Strand>);).into(),
            }
        } else if self.raw_type_is_optional() {
            let get_option_item_type = self.get_option_item_type();
            let item = get_option_item_type
                .clone()
                .as_ref()
                .map(|ct| {
                    let ty = ct.clone();
                    let item = Self::new(ty);

                    item.infer_surreal_type_heuristically(field_name, relation_type, model_type)
                })
                .ok_or(syn::Error::new(
                    ty.span(),
                    "Could not infer type for the field",
                ))??;

            let inner_type = item.field_type_db_token;
            let item_static_assertion = item.static_assertion_token;

            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Option(Box::new(
                    item.field_type_db_original.unwrap_or(FieldType::Any),
                ))),
                field_type_db_token:
                    quote!(#crate_name::FieldType::Option(::std::boxed::Box::new(#inner_type)))
                        .into(),
                static_assertion_token: quote!(
                    #crate_name::validators::assert_option::<#ty>();
                    #item_static_assertion
                )
                .into(),
            }
        } else if self.is_list() {
            let inner_type = self.get_array_inner_type();
            let inner_item = inner_type
                .map(|ct| {
                    ct.infer_surreal_type_heuristically(field_name, relation_type, model_type)
                })
                .ok_or(syn::Error::new(
                    ty.span(),
                    "Could not infer type for the field",
                ))??;

            let inner_type = inner_item.field_type_db_token;
            let inner_static_assertion = inner_item.static_assertion_token;
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Array(Box::new(inner_item.field_type_db_original.unwrap_or(FieldType::Any)), None)),
                field_type_db_token: quote!(#crate_name::FieldType::Array(::std::boxed::Box::new(#inner_type), ::std::option::Option::None)).into(),
                static_assertion_token: quote!(
                            #crate_name::validators::assert_is_vec::<#ty>();
                            #inner_static_assertion
                ).into(),
            }
        } else if self.raw_type_is_hash_set() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Set(Box::new(FieldType::Any), None)),
                field_type_db_token: quote!(#crate_name::FieldType::Set(::std::boxed::Box::new(#crate_name::FieldType::Any), ::std::option::Option::None)).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_is_vec::<#ty>();).into(),
            }
        } else if self.raw_type_is_object() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Object),
                field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Object>);).into(),
            }
        } else if self.raw_type_is_duration() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Duration),
                field_type_db_token: quote!(#crate_name::FieldType::Duration).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Duration>);).into(),
            }
        } else if self.raw_type_is_datetime() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Datetime),
                field_type_db_token: quote!(#crate_name::FieldType::Datetime).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Datetime>);).into(),
            }
        } else if self.raw_type_is_geometry() {
            DbFieldTypeAstMeta {
                // TODO: check if to auto-infer more speicific geometry type?
                field_type_db_original: Some(FieldType::Geometry(vec![])),
                field_type_db_token: quote!(#crate_name::FieldType::Geometry(::std::vec![])).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Geometry>);).into(),
            }
        } else if field_name.is_id() {
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Record(vec![])),
                field_type_db_token:
                    quote!(#crate_name::FieldType::Record(::std::vec![Self::table_name()])).into(),
                static_assertion_token: quote!().into(),
            }
        } else if field_name.is_orig_or_dest_edge_node(model_type) {
            // An edge might be shared by multiple In/Out nodes. So, default to any type of
            // record for edge in and out
            DbFieldTypeAstMeta {
                field_type_db_original: Some(FieldType::Record(vec![])),
                field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![])).into(),
                static_assertion_token: quote!().into(),
            }
        } else if relation_type.is_some() {
            match relation_type {
                RelationType::Relate(_ref_node) => {
                    // Relation are not stored on nodes, but
                    // on edges. Just used on nodes for convenience
                    // during deserialization
                    DbFieldTypeAstMeta {
                        field_type_db_original: None,
                        field_type_db_token: quote!().into(),
                        static_assertion_token: quote!().into(),
                    }
                }
                RelationType::LinkOne(ref_node) => DbFieldTypeAstMeta {
                    field_type_db_original: Some(FieldType::Record(vec![])),
                    field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![#ref_node::table_name()])).into(),
                    static_assertion_token: quote!().into(),
                },
                RelationType::LinkSelf(_self_node) => DbFieldTypeAstMeta {
                    field_type_db_original: Some(FieldType::Record(vec![])),
                    field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![Self::table_name()])).into(),
                    static_assertion_token: quote!().into(),
                },
                RelationType::LinkMany(ref_node) => DbFieldTypeAstMeta {
                    field_type_db_original: Some(FieldType::Array(
                        ::std::boxed::Box::new(FieldType::Record(vec![])),
                        ::std::option::Option::None
                    )),
                    field_type_db_token: quote!(#crate_name::FieldType::Array(
                        ::std::boxed::Box::new(#crate_name::FieldType::Record(::std::vec![#ref_node::table_name()])),
                        ::std::option::Option::None
                    )).into(),
                    static_assertion_token: quote!().into(),
                },
                RelationType::NestObject(_ref_object) => DbFieldTypeAstMeta {
                    field_type_db_original: Some(FieldType::Object),
                    field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                    static_assertion_token: quote!().into(),
                },
                RelationType::NestArray(_ref_array) => DbFieldTypeAstMeta {
                    // provide the inner type for when the array part start recursing
                    field_type_db_original: Some(FieldType::Object),
                    field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                    // db_field_type: quote!(#crate_name::FieldType::Array(
                    //     ::std::boxed::Box::new(#crate_name::FieldType::Object),
                    //     ::std::option::Option::None
                    // )),
                    static_assertion_token: quote!().into(),
                },
                RelationType::List(_list_simple) => DbFieldTypeAstMeta {
                    // provide the inner type for when the array part start recursing
                    field_type_db_original: Some(FieldType::Array(
                        ::std::boxed::Box::new(FieldType::Any),
                        ::std::option::Option::None
                    )),
                    field_type_db_token: quote!(#crate_name::FieldType::Array(
                        ::std::boxed::Box::new(#crate_name::FieldType::Any),
                        ::std::option::Option::None
                    )).into(),
                    // db_field_type: quote!(#crate_name::FieldType::Array(
                    //     ::std::boxed::Box::new(#crate_name::FieldType::Object),
                    //     ::std::option::Option::None
                    // )),
                    static_assertion_token: quote!().into(),
                },
                RelationType::None => {
                    return Err(syn::Error::new(
                        ty.span(),
                        "Could not infer type for the field",
                    )
                    .into())
                }
            }
        } else {
            return Err(syn::Error::new(ty.span(), "Could not infer type for the field").into());
        };
        Ok(meta)
    }

    pub fn type_is_inferrable(
        &self,
        field_receiver: &MyFieldReceiver,
        model_attributes: &ModelAttributes,
    ) -> bool {
        let is_db_field = model_attributes.casing().map_or(false, |casing| {
            field_receiver.db_field_name(&casing).map_or(false, |dfn| {
                dfn.is_id() || dfn.is_orig_or_dest_edge_node(&model_attributes.to_data_type())
            })
        });

        field_receiver.to_relation_type().is_some()
            || is_db_field
            || self.raw_type_is_float()
            || self.raw_type_is_integer()
            || self.raw_type_is_string()
            || self.raw_type_is_bool()
            || self.is_list()
            || self.raw_type_is_hash_set()
            || self.raw_type_is_object()
            || self.raw_type_is_optional()
            || self.raw_type_is_duration()
            || self.raw_type_is_datetime()
            || self.raw_type_is_geometry()
    }
}
