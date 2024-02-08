/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::FromMeta;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use syn::{
    self,
    parse::{Parse, ParseStream, Parser},
    parse_quote,
    spanned::Spanned,
    visit_mut::VisitMut,
    GenericArgument, Ident, Lifetime, Path, PathArguments, PathSegment, Type, TypeReference,
};

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, DataType},
};

use super::{field_name_serialized::FieldNameSerialized, *};

#[derive(Debug, Clone)]
pub struct CustomTypeNoSelf(CustomType);

impl CustomTypeNoSelf {
    pub fn new(ty: Type) -> Self {
        Self(CustomType(ty))
    }

    pub fn type_name(&self) -> ExtractorResult<Ident> {
        self.0.type_name()
    }

    pub fn to_basic_type(self) -> Type {
        self.0.to_basic_type()
    }

    pub fn get_generics_meta<'a>(
        &self,
        table_attributes: TableDeriveAttributes,
    ) -> FieldGenericsMeta<'a> {
        self.0.get_generics_meta(&table_attributes)
    }
}

#[derive(Debug, Clone, FromMeta)]
pub struct CustomType(Type);

impl Parse for CustomType {
    // TODO: Handle type parsing if frommeta does not work or manually implement fromMeta
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

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

impl CustomType {
    pub fn new(ty: Type) -> Self {
        Self(ty)
    }

    pub fn to_basic_type(self) -> Type {
        self.0
    }

    pub fn type_name(&self) -> ExtractorResult<Ident> {
        match self.0 {
            Type::Path(type_path) => {
                let last_segment = type_path
                    .path
                    .segments
                    .last()
                    .ok_or_else(|| darling::Error::custom("Expected a type. Make sure there are no typos and you are using a proper struct as the linked Node."))?;
                Ok(last_segment.ident.clone())
            }
            _ => Err(syn::Error::new(self.to_type().span(), "Expected a struct type").into()),
        }
    }

    pub fn get_generics_meta<'a>(
        &self,
        table_attributes: &TableDeriveAttributes,
    ) -> FieldGenericsMeta<'a> {
        let (field_impl_generics, field_ty_generics, field_where_clause) =
            GenericTypeExtractor::extract_generics_for_complex_type(
                table_attributes,
                &self.to_basic_type(),
            )
            .split_for_impl();
        FieldGenericsMeta {
            field_impl_generics,
            field_ty_generics,
            field_where_clause,
        }
    }

    pub fn replace_self_with_current_struct_ident(
        &self,
        table_def: &TableDeriveAttributes,
    ) -> CustomTypeNoSelf {
        let ty = &self.to_basic_type();
        let replacement_path_from_current_struct = table_def.struct_as_path();

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
                // TODO: Extend to handle other types like Tuple, Array, etc.
                _ => ty.clone(),
            }
        }

        CustomTypeNoSelf::new(replace_type(ty, &replacement_path_from_current_struct))
    }

    fn strip_bounds_from_generics(&self) -> Self {
        let stripped_ty = match self.into_inner() {
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
                                        let ident = &tp.path.get_ident().unwrap();
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
            _ => self.into_inner().clone(),
        };
        Self(stripped_ty)
    }

    pub fn replace_lifetimes_with_underscore(&self) -> Self {
        let ty = &self.0;
        let mut ty = ty.clone();
        struct ReplaceLifetimesVisitor;

        impl VisitMut for ReplaceLifetimesVisitor {
            fn visit_lifetime_mut(&mut self, i: &mut Lifetime) {
                *i = Lifetime::new("'_", i.apostrophe);
            }
        }

        let mut visitor = ReplaceLifetimesVisitor;
        visitor.visit_type_mut(&mut ty);
        ty.into()
    }

    pub fn is_numeric(&self) -> bool {
        let ty = &self.into_inner();
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
        match self.into_inner() {
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
        match self.into_inner() {
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
        match &self.into_inner() {
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
        match self.into_inner() {
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

    pub fn is_list(&self) -> bool {
        self.is_list()
        // || self.into_inner()pe_.as_ref().map_or(false, |t| t.deref().is_array())
        // || self.link_many.is_some()
    }

    pub fn is_set(&self) -> bool {
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();
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
        let ty = &self.into_inner();

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
        let ty = &self.into_inner();

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
        field_name: &FieldNameSerialized,
        relation_type: &RelationType,
        model_type: &DataType,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let crate_name = get_crate_name(false);
        let ty = &self.into_inner();

        let meta = if self.raw_type_is_bool() {
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Bool),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<::std::primitive::bool>);),
            }
        } else if self.raw_type_is_float() {
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Float),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Number>);),
            }
        } else if self.raw_type_is_integer() {
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Int),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Number>);),
            }
        } else if self.raw_type_is_string() {
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::String),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Strand>);),
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

            let inner_type = item.field_type_db;
            let item_static_assertion = item.static_assertion;

            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Option(::std::boxed::Box::new(#inner_type))),
                static_assertion: quote!(
                    #crate_name::validators::assert_option::<#ty>();
                    #item_static_assertion
                ),
            }
        } else if self.is_list() {
            let inner_type = self.get_array_inner_type();
            let inner_item = inner_type
                .map(|ct| {
                    Self::new(ct).infer_surreal_type_heuristically(
                        field_name,
                        relation_type,
                        model_type,
                    )
                })
                .ok_or(syn::Error::new(
                    ty.span(),
                    "Could not infer type for the field",
                ))??;

            let inner_type = inner_item.field_type_db;
            let inner_static_assertion = inner_item.static_assertion;
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Array(::std::boxed::Box::new(#inner_type), ::std::option::Option::None)),
                static_assertion: quote!(
                            #crate_name::validators::assert_is_vec::<#ty>();
                            #inner_static_assertion
                ),
            }
        } else if self.raw_type_is_hash_set() {
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Set(::std::boxed::Box::new(#crate_name::FieldType::Any), ::std::option::Option::None)),
                static_assertion: quote!(#crate_name::validators::assert_is_vec::<#ty>();),
            }
        } else if self.raw_type_is_object() {
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Object),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Object>);),
            }
        } else if self.raw_type_is_duration() {
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Duration),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Duration>);),
            }
        } else if self.raw_type_is_datetime() {
            DbFieldTypeAstMeta {
                field_type_db: quote!(#crate_name::FieldType::Datetime),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Datetime>);),
            }
        } else if self.raw_type_is_geometry() {
            DbFieldTypeAstMeta {
                // TODO: check if to auto-infer more speicific geometry type?
                field_type_db: quote!(#crate_name::FieldType::Geometry(::std::vec![])),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Geometry>);),
            }
        } else {
            if field_name.is_id() {
                DbFieldTypeAstMeta {
                    field_type_db: quote!(#crate_name::FieldType::Record(::std::vec![Self::table_name()])),
                    static_assertion: quote!(),
                }
            } else if field_name.is_orig_or_dest_edge_node(model_type) {
                // An edge might be shared by multiple In/Out nodes. So, default to any type of
                // record for edge in and out
                DbFieldTypeAstMeta {
                    field_type_db: quote!(#crate_name::FieldType::Record(::std::vec![])),
                    static_assertion: quote!(),
                }
            } else if relation_type.is_some() {
                match relation_type {
                    RelationType::Relate(ref_node) => {
                        // Relation are not stored on nodes, but
                        // on edges. Just used on nodes for convenience
                        // during deserialization
                        DbFieldTypeAstMeta {
                            field_type_db: quote!(),
                            static_assertion: quote!(),
                        }
                    }
                    RelationType::LinkOne(ref_node) => DbFieldTypeAstMeta {
                        field_type_db: quote!(#crate_name::FieldType::Record(::std::vec![#ref_node::table_name()])),
                        static_assertion: quote!(),
                    },
                    RelationType::LinkSelf(self_node) => DbFieldTypeAstMeta {
                        field_type_db: quote!(#crate_name::FieldType::Record(::std::vec![Self::table_name()])),
                        static_assertion: quote!(),
                    },
                    RelationType::LinkMany(ref_node) => DbFieldTypeAstMeta {
                        field_type_db: quote!(#crate_name::FieldType::Array(
                            ::std::boxed::Box::new(#crate_name::FieldType::Record(::std::vec![#ref_node::table_name()])),
                            ::std::option::Option::None
                        )),
                        static_assertion: quote!(),
                    },
                    RelationType::NestObject(ref_object) => DbFieldTypeAstMeta {
                        field_type_db: quote!(#crate_name::FieldType::Object),
                        static_assertion: quote!(),
                    },
                    RelationType::NestArray(ref_array) => DbFieldTypeAstMeta {
                        // provide the inner type for when the array part start recursing
                        field_type_db: quote!(#crate_name::FieldType::Object),
                        // db_field_type: quote!(#crate_name::FieldType::Array(
                        //     ::std::boxed::Box::new(#crate_name::FieldType::Object),
                        //     ::std::option::Option::None
                        // )),
                        static_assertion: quote!(),
                    },
                    RelationType::List(list_simple) => DbFieldTypeAstMeta {
                        // provide the inner type for when the array part start recursing
                        field_type_db: quote!(#crate_name::FieldType::Array(
                            ::std::boxed::Box::new(#crate_name::FieldType::Any),
                            ::std::option::Option::None
                        )),
                        // db_field_type: quote!(#crate_name::FieldType::Array(
                        //     ::std::boxed::Box::new(#crate_name::FieldType::Object),
                        //     ::std::option::Option::None
                        // )),
                        static_assertion: quote!(),
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
                return Err(
                    syn::Error::new(ty.span(), "Could not infer type for the field").into(),
                );
            }
        };
        Ok(meta)
    }

    pub fn type_is_inferrable(
        &self,
        field_name: &FieldIdentSerialized,
        model_type: &DataType,
    ) -> bool {
        self.relation_type.is_some()
            || field_name.is_id()
            || field_name.is_orig_or_dest_edge_node(model_type)
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
