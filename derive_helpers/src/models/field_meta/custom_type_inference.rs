/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
*/

use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::{self, spanned::Spanned, Expr};

use crate::models::*;

use super::{field_name_serialized::DbFieldName, *};

pub struct FieldTypeInference<'a> {
    pub db_field_name: &'a DbFieldName,
    pub field_ty: &'a CustomType,
    pub relation_type: &'a RelationType,
    pub model_attrs: &'a ModelAttributes<'a>,
}

impl<'a> FieldTypeInference<'a> {
    pub fn infer_type(&self) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        let inferred = self
            .infer_type_by_priority()
            .map(Some)
            .unwrap_or_else(|_| None);

        inferred.ok_or_else(|| {
            syn::Error::new(
                self.field_ty.span(),
                "Could not infer the field type for the field",
            )
            .into()
        })
    }

    pub fn infer_type_by_priority(&self) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        let relation_type = self.relation_type;
        let model_attrs = self.model_attrs;
        let ty = self.field_ty;
        let field_name = self.db_field_name;

        // NOTE: The order of the inference approach is important. Type path should always be first
        let priority1 = || self.based_on_type_path_token_structure(ty, relation_type.clone());
        let priority2 = || self.based_on_db_field_name(ty, field_name, model_attrs);
        let priority3 = || self.based_on_field_link_type(ty, relation_type);

        let db_type = if let Ok(db_ty) = priority1() {
            Some(db_ty)
        } else if let Ok(db_ty) = priority2() {
            Some(db_ty)
        } else if let Ok(Some(db_ty)) = priority3() {
            Some(db_ty)
        } else {
            None
        };

        Ok(db_type)
    }

    /// This is done directly by morphologically analysing the field type path tokenstream
    /// using certain heuristics and is cross-referenced with other stuff in case our guess
    /// is wrong. Users can usually specify the exact db type as a field attribute for explicitness
    fn based_on_type_path_token_structure(
        &self,
        field_ty: &CustomType,
        relation_type: RelationType,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let crate_name = get_crate_name(false);
        let binding = field_ty
            // .remove_non_static_lifetime_and_reference()
            .replace_self_with_current_struct_concrete_type(self.model_attrs)?;
        let ty = &binding.into_inner_ref();

        let meta = if field_ty.raw_type_is_bool() {
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Bool,
                field_type_db_token: quote!(#crate_name::FieldType::Bool).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_bool::<#ty>();).into(),
            }
        } else if field_ty.raw_type_is_float() {
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Float,
                field_type_db_token: quote!(#crate_name::FieldType::Float).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_float::<#ty>();).into(),
            }
        } else if field_ty.raw_type_is_integer() {
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Int,
                field_type_db_token: quote!(#crate_name::FieldType::Int).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_int::<#ty>();).into(),
            }
        } else if field_ty.raw_type_is_string() {
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::String,
                field_type_db_token: quote!(#crate_name::FieldType::String).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_string::<#ty>();).into(),
            }
        } else if field_ty.raw_type_is_optional() {
            // let get_option_item_type = field_ty.get_option_item_type();
            let get_option_item_type = field_ty.inner_angle_bracket_type()?;
            let item = get_option_item_type
                .clone()
                .as_ref()
                .map(|ct| {
                    let ty = ct.clone();
                    // let item = CustomType::new(ty.into_inner());
                    let item = ty.into_inner();

                    self.based_on_type_path_token_structure(&item, relation_type)
                })
                .ok_or(syn::Error::new(
                    ty.span(),
                    "Could not infer type for the optional field",
                ))??;

            let inner_type = item.field_type_db_token;
            let item_static_assertion = item.static_assertion_token;

            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Option(Box::new(item.field_type_db_original)),
                field_type_db_token:
                    quote!(#crate_name::FieldType::Option(::std::boxed::Box::new(#inner_type)))
                        .into(),
                static_assertion_token: quote!(
                    #crate_name::validators::assert_type_is_option::<#ty>();
                    #item_static_assertion
                )
                .into(),
            }
        } else if field_ty.is_array() {
            // let inner_type = field_ty.get_array_inner_type();
            let inner_type = field_ty.inner_angle_bracket_type()?;
            let none_opt_token = || quote!(::std::option::Option::None);

            let array_len_token_stream =
                field_ty
                    .get_array_const_length()
                    .map_or(none_opt_token(), |expr| {
                        if expr.to_token_stream().is_empty() {
                            none_opt_token()
                        } else {
                            quote!(::std::option::Option::Some(#expr))
                        }
                    });

            let array_len_token = field_ty.get_array_const_length();
            let array_len_token_as_int = array_len_token
                .as_ref()
                .and_then(|expr| {
                    if let Expr::Lit(lit) = expr {
                        if let syn::Lit::Int(int) = &lit.lit {
                            Some(int.base10_parse::<u64>().unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });

            let inner_item = inner_type
                .map(|ct| self.based_on_type_path_token_structure(&ct, relation_type))
                .ok_or(syn::Error::new(
                    ty.span(),
                    "Could not infer type for the array inner type",
                ))??;

            let inner_type = inner_item.field_type_db_token;
            let inner_static_assertion = inner_item.static_assertion_token;
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Array(Box::new(inner_item.field_type_db_original), array_len_token_as_int),
                field_type_db_token: quote!(#crate_name::FieldType::Array(::std::boxed::Box::new(#inner_type), #array_len_token_stream)).into(),
                static_assertion_token: quote!(
                            #crate_name::validators::assert_type_is_array::<#ty>();
                            #inner_static_assertion
                ).into(),
            }
        } else if field_ty.raw_type_is_set() {
            // let inner_type = field_ty.get_set_inner_type();
            let inner_type = field_ty.inner_angle_bracket_type()?;

            let inner_item = inner_type
                .map(|ct| self.based_on_type_path_token_structure(&ct, relation_type))
                .ok_or(syn::Error::new(
                    ty.span(),
                    "Could not infer type for the set inner type",
                ))??;
            let inner_type = inner_item.field_type_db_token;
            let inner_static_assertion = inner_item.static_assertion_token;

            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Set(Box::new(inner_item.field_type_db_original), None),
                field_type_db_token: quote!(#crate_name::FieldType::Set(::std::boxed::Box::new(#inner_type), ::std::option::Option::None)).into(),
                static_assertion_token: quote!(
                    #crate_name::validators::assert_type_is_set::<#ty>();
                    #inner_static_assertion
                ).into(),
            }
        } else if field_ty.raw_type_is_object() {
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Object,
                field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                static_assertion_token: quote!(#crate_name::validators::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Object>);).into(),
            }
        } else if field_ty.raw_type_is_duration() {
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Duration,
                field_type_db_token: quote!(#crate_name::FieldType::Duration).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_duration::<#ty>();).into(),
            }
        } else if field_ty.raw_type_is_datetime() {
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Datetime,
                field_type_db_token: quote!(#crate_name::FieldType::Datetime).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_datetime::<#ty>();).into(),
            }
        }
        // else if field_ty.raw_type_is_geometry() {
        else if let Some(geo_kind) = field_ty.raw_type_geometry_kind() {
            // We are intentionally using debug string representation of the geometry kind
            let geo_kind_name_from_debug_print = format!("{:?}", &geo_kind);
            let geo_kind_ident = format_ident!("{geo_kind_name_from_debug_print}");

            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Geometry(vec![geo_kind]),
                field_type_db_token: quote!(#crate_name::FieldType::Geometry(::std::vec![#crate_name::GeometryType::#geo_kind_ident])).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_geometry::<#ty>();).into(),
            }
        } else if let Some(foreign_type) = self.refereces_a_nested_object(&relation_type)? {
            // Guess if its a foreign Table type by comparing the type path segment with the foreign
            // rust field type ident
            let current_segment_is_likely_foreign_type =
                field_ty.type_ident()? == foreign_type.type_ident()?;
            if current_segment_is_likely_foreign_type {
                DbFieldTypeAstMeta {
                    field_type_db_original: FieldType::Object,
                    field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                    static_assertion_token: quote!(
                    // #crate_name::validators::assert_type_eq_all!(#ty, #foreign_type);
                            #crate_name::validators::assert_type_is_object::<#ty>();
                            )
                    .into(),
                }
            } else {
                return Err(syn::Error::new(
                    ty.span(),
                    "Could not infer the database type for the field based on the field type in rust provided. Specify by using e.g ty = \"array\"",
                ).into());
            }
        } else {
            return Err(syn::Error::new(
                ty.span(),
                "Could not infer the database type for the field based on the field type in rust provided. Specify by using e.g ty = \"array\"",
            )
            .into());
        };

        Ok(meta)
    }

    /// Gets out the foreign/nested object type if the field references a foreign struct
    fn refereces_a_nested_object(
        &self,
        relation_type: &'a RelationType,
    ) -> ExtractorResult<Option<&'a CustomType>> {
        // NOTE 1:
        // Links are excluded from this because we are handling them separately in the inference
        // based on link type.
        // Nested objects are inluded here when we infer based on the field type path because
        // nested/embedded objects are just normal rust struct types with no relation nor special treatment
        // within the context of the database. But we are able to infer that because of the
        // nest_array/ nest_object field attributes we provide
        //
        // NOTE 2:
        // We are mainly interested in getting the foreign struct itself whether or not its a
        // single top-level struct, or one or deep lelve nested struct.
        let foreign_type = match relation_type {
            RelationType::NestObject(ref_object) => Some(ref_object.into_inner_ref()),
            RelationType::NestArray(foreign_array_object) => {
                Some(foreign_array_object.into_inner_ref())
            }
            RelationType::Relate(_)
            | RelationType::List(_)
            | RelationType::None
            | RelationType::LinkOne(_)
            | RelationType::LinkSelf(_)
            | RelationType::LinkManyInAndOutEdgeNodesInert(_)
            | RelationType::LinkMany(_) => None,
        };

        Ok(foreign_type)
    }

    /// based on field metadata such as reserved field names e.g `id`, `in` and `out`
    /// With these metadata, we can usually reliably derive/guess the field type as a whole
    fn based_on_db_field_name(
        &self,
        field_ty: &CustomType,
        db_field_name: &DbFieldName,
        model_attrs: &ModelAttributes,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let crate_name = get_crate_name(false);
        let model_type = model_attrs.to_data_type();
        let table = model_attrs.table()?;
        let ty = &field_ty.into_inner_ref();

        let meta = if db_field_name.is_id() {
            let table_name = match table {
                Some(t) => vec![t.to_token_stream().to_string().into()],
                None => vec![],
            };
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Record(table_name),
                field_type_db_token:
                    quote!(#crate_name::FieldType::Record(::std::vec![Self::table()])).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_thing::<#ty>();).into(),
            }
        } else if db_field_name.is_in_or_out_edge_node(&model_type) {
            // An edge might be shared by multiple In/Out nodes. So, default to any type of
            // record for edge in and out e.g Student->Plays->Football, Student->Plays->Instrument,
            // Teacher->Plays->Football
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Record(vec![]),
                field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![])).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_link_one::<#ty>();).into(),
            }
        } else {
            return Err(syn::Error::new(
                ty.span(),
                format!("Could not infer type for the field - {db_field_name}. Specify by using e.g ty = \"record<any>\"")
            )
            .into());
        };
        Ok(meta)
    }

    /// This is usually shallow and just infers the type as a whole
    /// based on field attributes such as relation type e.g link_one, link_fieldm link_many etc
    /// With these metadata, we can usually reliably derive/guess the field type as a whole
    fn based_on_field_link_type(
        &self,
        field_ty: &CustomType,
        relation_type: &RelationType,
    ) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        let crate_name = get_crate_name(false);
        let ty = &field_ty.into_inner_ref();

        let meta = match relation_type {
                RelationType::Relate(_ref_node) => {
                    // Relations are readonly and are not stored on nodes, but
                    // on edges. Just used on nodes for convenience
                    // during deserialization
                  return Ok(None)
                }

            RelationType::LinkOne(ref_node) => {
                let ref_node = ref_node.turbo_fishize()?;

                DbFieldTypeAstMeta {
                            field_type_db_original: FieldType::Record(vec![]),
                            field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![#ref_node::table()])).into(),
                            static_assertion_token: quote!(#crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkOne<#ref_node>);).into()
                        }
            },
            RelationType::LinkSelf(self_node) => {
                let current_struct_type = self.model_attrs.struct_no_bounds()?;
                let self_node = self_node.turbo_fishize()?;

                DbFieldTypeAstMeta {
                            field_type_db_original: FieldType::Record(vec![]),
                            field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![Self::table()])).into(),
                            static_assertion_token: quote!(
                                    #crate_name::validators::assert_type_eq_all!(#crate_name::LinkSelf<#current_struct_type>, #crate_name::LinkSelf<#self_node>);
                                    #crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkSelf<#self_node>);
                                ).into(),
                        }
            },
            RelationType::LinkMany(ref_node) | RelationType::LinkManyInAndOutEdgeNodesInert(ref_node) => {
                let ref_node = ref_node.turbo_fishize()?;

                
                DbFieldTypeAstMeta {
                            field_type_db_original: FieldType::Array(
                                ::std::boxed::Box::new(FieldType::Record(vec![])),
                                ::std::option::Option::None
                            ),
                            field_type_db_token: quote!(#crate_name::FieldType::Array(
                                ::std::boxed::Box::new(#crate_name::FieldType::Record(::std::vec![#ref_node::table()])),
                                ::std::option::Option::None
                            )).into(),
                            static_assertion_token: quote!(
                                #crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkMany<#ref_node>);
                        ).into(),
                        }
            },
                // TODO: Consider removing the concept of list altogether to 
                // avoid confusion/ambiguity
        // NOTE: Nested objects and nested arrays of objects are handled in the type path token
                RelationType::NestArray(_) | RelationType::NestObject(_) | RelationType::List(_) | RelationType::None => {
                    return Err(syn::Error::new(
                        ty.span(),
                        format!("Could not infer type for the field. Specify explicitly by using e.g ty = \"array<any>\". You can choose from one of these types: {}", FieldType::variants().join(", ")),
                    )
                    .into())
                }
            };

        Ok(Some(meta))
    }

    #[allow(unused)]
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

    #[allow(unused)]
    fn count_vec_nesting(field_type: &syn::Type) -> usize {
        match field_type {
            syn::Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_type)) =
                                angle_args.args.first()
                            {
                                1 + Self::count_vec_nesting(inner_type)
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}
