/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
*/

use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use surreal_query_builder::FieldType;
use syn::{self, spanned::Spanned, Expr};

use crate::models::*;

use super::{field_name_serialized::DbFieldName, *};

pub enum InferenceApproach {
    /// This is done directly by morphologically analysing the field type path tokenstream
    /// using certain heuristics and is cross-referenced with other stuff in case our guess
    /// is wrong. Users can usually specify the exact db type as a field attribute for explicitness
    BasedOnTypePathToken,
    ///
    /// This is usually shallow and just infers the type as a whole
    /// based on field attributes such as relation type e.g link_one, link_field_ty. link_many etc
    /// With these metadata, we can usually reliably derive/guess the field type as a whole
    BasedOnRelationTypeFieldAttr,

    /// based on field metadata such as reserved field names e.g `id`, `in` and `out`
    /// With these metadata, we can usually reliably derive/guess the field type as a whole
    BasedOnDbFieldName,
}

impl InferenceApproach {
    pub fn all() -> Vec<Self> {
        vec![
            InferenceApproach::BasedOnTypePathToken,
            InferenceApproach::BasedOnRelationTypeFieldAttr,
            InferenceApproach::BasedOnDbFieldName,
        ]
    }
}

pub struct FieldTypeInference<'a> {
    // field_receiver: &'a MyFieldReceiver,
    pub db_field_name: &'a DbFieldName,
    pub field_ty: &'a CustomType,
    pub relation_type: &'a RelationType,
    // table_attrs: &'a TableDeriveAttributes,
    pub model_attrs: &'a ModelAttributes<'a>,
}

impl<'a> FieldTypeInference<'a> {
    pub fn infer_type(&self) -> ExtractorResult<DbFieldTypeAstMeta> {
        let inferred = InferenceApproach::all().iter().find_map(|approach| {
            self.infer_type_by(approach)
                .map(|res| Some(res))
                .unwrap_or_else(|_| None)
        });

        inferred.ok_or_else(|| {
            syn::Error::new(
                // self.field_receiver.ty().span(),
                self.field_ty.span(),
                "Could not infer the field type for the field",
            )
            .into()
        })
    }

    pub fn infer_type_by(
        &self,
        approach: &InferenceApproach,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let relation_type = self.relation_type;
        // let relation_type = self.field_receiver.to_relation_type();
        let model_attrs = self.model_attrs;
        let ty = self.field_ty;
        // let field_type = self.field_receiver.ty();
        let field_name = self.db_field_name;
        // let field_name = self.field_receiver.db_field_name(&model_attrs.casing()?)?;

        let res = match approach {
            InferenceApproach::BasedOnTypePathToken => self.based_on_type_path_token(&ty)?,
            InferenceApproach::BasedOnRelationTypeFieldAttr => {
                self.based_on_field_relation_type(&ty, &relation_type)?
            }
            InferenceApproach::BasedOnDbFieldName => {
                self.based_on_db_field_name(&ty, &field_name, &model_attrs.to_data_type())?
            }
        };
        Ok(res)
    }

    fn based_on_type_path_token(
        &self,
        field_ty: &CustomType,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let crate_name = get_crate_name(false);
        let binding = field_ty
            .remove_non_static_lifetime_and_reference()
            .replace_self_with_current_struct_concrete_type(self.model_attrs)?;
        let ty = &binding
            .into_inner_ref();

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

                    self.based_on_type_path_token(&item)
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
            let array_len_token_stream = field_ty.get_array_const_length().map(|expr| {
                if expr.to_token_stream().is_empty() {
                    quote!(::std::option::Option::None)
                } else {
                    quote!(::std::option::Option::Some(#expr))
                }
            });
            let array_len_token = field_ty.get_array_const_length();
            let array_len_token_as_int = array_len_token
                .as_ref()
                .map(|expr| {
                    if let Expr::Lit(lit) = expr {
                        if let syn::Lit::Int(int) = &lit.lit {
                            Some(int.base10_parse::<u64>().unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .flatten();

            let inner_item = inner_type
                .map(|ct| self.based_on_type_path_token(&ct))
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
                .map(|ct| self.based_on_type_path_token(&ct))
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
        } else if field_ty.raw_type_is_geometry() {
            DbFieldTypeAstMeta {
                // TODO: check if to auto-infer more speicific geometry type?
                field_type_db_original: FieldType::Geometry(vec![]),
                field_type_db_token: quote!(#crate_name::FieldType::Geometry(::std::vec![])).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_geometry::<#ty>();).into(),
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

    fn based_on_db_field_name(
        &self,
        field_ty: &CustomType,
        db_field_name: &DbFieldName,
        model_type: &DataType,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let crate_name = get_crate_name(false);
        let ty = &field_ty.into_inner_ref();

        let meta = if db_field_name.is_id() {
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Record(vec![]),
                field_type_db_token:
                    quote!(#crate_name::FieldType::Record(::std::vec![Self::table()])).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_thing::<#ty>();).into(),
            }
        } else if db_field_name.is_in_or_out_edge_node(model_type) {
            // An edge might be shared by multiple In/Out nodes. So, default to any type of
            // record for edge in and out e.g Student->Plays->Football, Student->Plays->Instrument,
            // Teacher->Plays->Football
            DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Record(vec![]),
                field_type_db_token: quote!((
                    ::std::boxed::Box::new(#crate_name::FieldType::Record(::std::vec![])),
                    ::std::option::Option::None
                ))
                .into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_link_many::<#ty>();).into(),
            }
        } else {
            return Err(syn::Error::new(
                ty.span(),
                format!("Could not infer type for the field - {db_field_name}. Specify by using e.g ty = \"array\"")
            )
            .into());
        };
        Ok(meta)
    }

    fn based_on_field_relation_type(
        &self,
        field_ty: &CustomType,
        relation_type: &RelationType,
    ) -> ExtractorResult<DbFieldTypeAstMeta> {
        let crate_name = get_crate_name(false);
        let ty = &field_ty.into_inner_ref();

        let meta = match relation_type {
                RelationType::Relate(_ref_node) => {
                    // Relation are not stored on nodes, but
                    // on edges. Just used on nodes for convenience
                    // during deserialization
                  return Err(syn::Error::new(
                      ty.span(),
                      "This is a readonly field. It cannot be stored in the database, therefore, no database type needed. Use it for deserialization only",
                  ))?
                }
            RelationType::LinkOne(ref_node) => DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Record(vec![]),
                field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![#ref_node::table()])).into(),
                static_assertion_token: quote!(
#crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkOne<#ref_node>);
            ).into(),
            },
            RelationType::LinkSelf(self_node) => {
                let current_struct_type = self.model_attrs.struct_no_bounds()?;
                DbFieldTypeAstMeta {
                            field_type_db_original: FieldType::Record(vec![]),
                            field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![Self::table()])).into(),
                            static_assertion_token: quote!(
                    quote!(#crate_name::validators::assert_type_eq_all!(#current_struct_type, #crate_name::LinkSelf<#self_node>);),
                    quote!(#crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkSelf<#self_node>);),

                        ).into(),
                        }
            },
            RelationType::LinkMany(ref_node) => DbFieldTypeAstMeta {
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
            },
            RelationType::NestObject(_ref_object) => DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Object,
                field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                static_assertion_token:
                quote!(#crate_name::validators::assert_type_is_object::<#ty>();).into(),
            },
            RelationType::NestArray(foreign_array_object) => {

        let nesting_level = Self::count_vec_nesting(field_ty.to_basic_type());
                let nested_vec_type =
                    Self::generate_nested_vec_type(&foreign_array_object, nesting_level);

                DbFieldTypeAstMeta {
                            // provide the inner type for when the array part start recursing
                            field_type_db_original: FieldType::Object,
                            field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                            // db_field_type: quote!(#crate_name::FieldType::Array(
                            //     ::std::boxed::Box::new(#crate_name::FieldType::Object),
                            //     ::std::option::Option::None
                            // )),
                            static_assertion_token: quote!().into(),
                            // static_assertion_token:
                            //     quote!(#crate_name::validators::assert_type_is_array::<#ty>();).into(),
                        }
            },
                // We already did for list/array/set earlier. 
                // TODO: Consider removing the concept of list altogether to 
                // avoid confusion/ambiguity
                RelationType::List(_) | RelationType::None => {
                    return Err(syn::Error::new(
                        ty.span(),
                        format!("Could not infer type for the field. Specify explicitly by using e.g ty = \"array<any>\". You can choose from one of these types: {}", FieldType::variants().join(", ")),
                    )
                    .into())
                }
            };

        Ok(meta)
    }

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
