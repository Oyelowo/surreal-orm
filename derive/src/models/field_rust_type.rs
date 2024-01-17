/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use quote::{format_ident, quote};
use syn::{self, Type};

use crate::models::replace_lifetimes_with_underscore;

use super::{attributes::FieldTypeDerived, get_crate_name, parser::DataType, TypeStripper};

#[derive(Debug, Default)]
pub struct Attributes<'a> {
    pub(crate) link_one: Option<&'a String>,
    pub(crate) link_self: Option<&'a String>,
    pub(crate) link_many: Option<&'a String>,
    pub(crate) nest_array: Option<&'a String>,
    pub(crate) nest_object: Option<&'a String>,
}

pub struct FieldRustType<'a> {
    pub(crate) ty: Type,
    pub(crate) attributes: Attributes<'a>,
}

impl<'a> FieldRustType<'a> {
    pub fn new(ty: Type, attributes: Attributes<'a>) -> Self {
        // let ty = TypeStripper::strip_references_and_lifetimes(&ty);
        Self { ty, attributes }
    }

    pub fn is_numeric(&self) -> bool {
        let ty = &self.ty;
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
        match self.ty {
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
        match self.ty {
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
        match &self.ty {
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
        match self.ty {
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
        self.raw_type_is_list()
        // || self.type_.as_ref().map_or(false, |t| t.deref().is_array())
        // || self.link_many.is_some()
    }

    pub fn raw_type_is_list(&self) -> bool {
        let ty = &self.ty;
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
        let ty = &self.ty;
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
        let ty = &self.ty;
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
        let ty = &self.ty;
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
        let ty = &self.ty;
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
        let ty = &self.ty;
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
        let ty = &self.ty;
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

    pub fn get_array_inner_type(&self) -> Option<Type> {
        let ty = &self.ty;

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
            _ => return None,
        };
        Some(item_ty.clone())
    }

    pub fn get_option_item_type(&self) -> Option<Type> {
        let ty = &self.ty;

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
        field_name_normalized: &str,
        model_type: &DataType,
    ) -> FieldTypeDerived {
        println!("infer_surreal_type_heuristically");
        let crate_name = get_crate_name(false);
        let ty = &self.ty;
        let delifed_type_for_static_assert = replace_lifetimes_with_underscore(&mut ty.clone());

        if self.raw_type_is_bool() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Bool),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#delifed_type_for_static_assert: ::std::convert::Into<::std::primitive::bool>);),
            }
        } else if self.raw_type_is_float() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Float),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#delifed_type_for_static_assert: ::std::convert::Into<#crate_name::sql::Number>);),
            }
        } else if self.raw_type_is_integer() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Int),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#delifed_type_for_static_assert: ::std::convert::Into<#crate_name::sql::Number>);),
            }
        } else if self.raw_type_is_string() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::String),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#delifed_type_for_static_assert: ::std::convert::Into<#crate_name::sql::Strand>);),
            }
        } else if self.raw_type_is_optional() {
            let get_option_item_type = self.get_option_item_type();
            let item = get_option_item_type
                .clone()
                .as_ref()
                .map(|ct| {
                    let ty = ct.clone();
                    let item = Self {
                        ty,
                        attributes: Default::default(),
                    };

                    item.infer_surreal_type_heuristically(field_name_normalized, model_type)
                })
                .unwrap_or_default();

            let inner_type = item.field_type;
            let item_static_assertion = item.static_assertion;

            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Option(::std::boxed::Box::new(#inner_type))),
                static_assertion: quote!(
                    #crate_name::validators::assert_option::<#delifed_type_for_static_assert>();
                    #item_static_assertion
                ),
            }
        } else if self.raw_type_is_list() {
            let inner_type = self.get_array_inner_type();
            let inner_item = inner_type
                .clone()
                .as_ref()
                .map(|ct| {
                    let ty = ct.clone();
                    let item = Self {
                        ty,
                        attributes: Attributes {
                            nest_array: self.attributes.nest_array,
                            nest_object: self.attributes.nest_object,
                            ..Default::default()
                        },
                    };

                    item.infer_surreal_type_heuristically(field_name_normalized, model_type)
                })
                .unwrap_or_default();

            let inner_type = inner_item.field_type;
            let inner_static_assertion = inner_item.static_assertion;
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Array(::std::boxed::Box::new(#inner_type), ::std::option::Option::None)),
                static_assertion: quote!(
                            #crate_name::validators::assert_is_vec::<#delifed_type_for_static_assert>();
                            #inner_static_assertion
                ),
            }
        } else if self.raw_type_is_hash_set() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Set(::std::boxed::Box::new(#crate_name::FieldType::Any), ::std::option::Option::None)),
                static_assertion: quote!(#crate_name::validators::assert_is_vec::<#delifed_type_for_static_assert>();),
            }
        } else if self.raw_type_is_object() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Object),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#delifed_type_for_static_assert: ::std::convert::Into<#crate_name::sql::Object>);),
            }
        } else if self.raw_type_is_duration() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Duration),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#delifed_type_for_static_assert: ::std::convert::Into<#crate_name::sql::Duration>);),
            }
        } else if self.raw_type_is_datetime() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Datetime),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#delifed_type_for_static_assert: ::std::convert::Into<#crate_name::sql::Datetime>);),
            }
        } else if self.raw_type_is_geometry() {
            FieldTypeDerived {
                // TODO: check if to auto-infer more speicific geometry type?
                field_type: quote!(#crate_name::FieldType::Geometry(::std::vec![])),
                static_assertion: quote!(#crate_name::validators::assert_impl_one!(#delifed_type_for_static_assert: ::std::convert::Into<#crate_name::sql::Geometry>);),
            }
        } else {
            let Attributes {
                link_one,
                link_self,
                link_many,
                nest_array,
                nest_object,
            } = self.attributes;

            if field_name_normalized == "id" {
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Record(::std::vec![Self::table_name()])),
                    static_assertion: quote!(),
                }
            } else if (field_name_normalized == "out" || field_name_normalized == "in")
                && matches!(model_type, DataType::Edge)
            {
                // An edge might be shared by multiple In/Out nodes. So, default to any type of
                // record for edge in and out
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Record(::std::vec![])),
                    static_assertion: quote!(),
                }
            } else if let Some(ref_node_type) = link_one.or(link_self) {
                let ref_node_type = format_ident!("{ref_node_type}");

                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Record(::std::vec![#ref_node_type::table_name()])),
                    static_assertion: quote!(),
                }
            } else if let Some(ref_node_type) = link_many {
                let ref_struct_name = format_ident!("{ref_node_type}");
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Array(
                        ::std::boxed::Box::new(#crate_name::FieldType::Record(::std::vec![#ref_struct_name::table_name()])),
                        ::std::option::Option::None
                    )),
                    static_assertion: quote!(),
                }
            } else if let Some(_ref_node_type) = nest_object {
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Object),
                    static_assertion: quote!(),
                }
            } else if let Some(_ref_node_type) = nest_array {
                FieldTypeDerived {
                    // provide the inner type for when the array part start recursing
                    field_type: quote!(#crate_name::FieldType::Object),
                    // field_type: quote!(#crate_name::FieldType::Array(
                    //     ::std::boxed::Box::new(#crate_name::FieldType::Object),
                    //     ::std::option::Option::None
                    // )),
                    static_assertion: quote!(),
                }
            } else if let Some(_ref_node_type) = link_one {
                FieldTypeDerived {
                    // #crate_name::SurrealId<#foreign_node>
                    field_type: quote!(#crate_name::FieldType::Record(::std::vec![_ref_node_type::table_name()])),
                    static_assertion: quote!(),
                }
            } else if let Some(_ref_node_type) = link_self {
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Record(::std::vec![_ref_node_type::table_name()])),
                    static_assertion: quote!(),
                }
            } else {
                // FieldTypeDerived {
                //     field_type: quote!(#crate_name::FieldType::Any),
                //     static_assertion: quote!(),
                // }
                panic!(
                    "Could not infer type for the field {}",
                    field_name_normalized
                );
            }
        }
    }

    pub fn type_is_inferrable(&self, field_name_normalized_str: &String) -> bool {
        self.attributes.link_one.is_some()
            || self.attributes.link_self.is_some()
            || self.attributes.link_many.is_some()
            || self.attributes.nest_object.is_some()
            || self.attributes.nest_array.is_some()
            || field_name_normalized_str == "id"
            || field_name_normalized_str == "in"
            || field_name_normalized_str == "out"
            || self.raw_type_is_float()
            || self.raw_type_is_integer()
            || self.raw_type_is_string()
            || self.raw_type_is_bool()
            || self.raw_type_is_list()
            || self.raw_type_is_hash_set()
            || self.raw_type_is_object()
            || self.raw_type_is_optional()
            || self.raw_type_is_duration()
            || self.raw_type_is_datetime()
            || self.raw_type_is_geometry()
    }
}
