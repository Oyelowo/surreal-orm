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
    pub fn infer_type(&self) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        let inferred = self.infer_type_by_priority()
            .map(|res| Some(res))
            .unwrap_or_else(|_| None);

        inferred.ok_or_else(|| {
            syn::Error::new(
                self.field_ty.span(),
                "Could not infer the field type for the field",
            )
            .into()
        })
    }

    pub fn infer_type_by_priority_of_field_ty(
        &self,
    ) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        fn infer_db_ty_from_path_recursively(
            ty: &CustomType, 
            field_name: &DbFieldName, 
            model_attrs: &ModelAttributes, 
            relation_type: &RelationType,
            ass: &mut Vec<StaticAssertionToken>
        ) -> ExtractorResult<()> {
            let field_ty = ty.inner_angle_bracket_type()?;

            if let Some(fty) = field_ty  {
                let inner_db_ty_ast = FieldTypeInference {
                    db_field_name: field_name,
                    field_ty: &fty,
                    relation_type: &RelationType::None,
                    model_attrs,
                    // TODO: Change relation back to using references
                }.based_on_type_path_token_structure(fty.into_inner_ref(), relation_type.clone())?;
                // .infer_type_by(InferenceApproach::BasedOnTypePathToken)?;
                
                // if let Ok(meta) = inner_db_ty_ast {
                // }
                ass.push(inner_db_ty_ast.static_assertion_token);
                infer_db_ty_from_path_recursively(fty.into_inner_ref(), field_name, model_attrs, relation_type ,ass)?;
                
            }

            Ok(todo!())
        }

        todo!()
    }

    pub fn infer_type_by_priority(
        &self,
    ) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        let relation_type = self.relation_type;
        let model_attrs = self.model_attrs;
        let ty = self.field_ty;
        let field_name = self.db_field_name;

        // NOTE: The order of the inference approach is important. Type path should always be first
        let priority1 = || self.based_on_type_path_token_structure(&ty, relation_type.clone());
        let priority2 = || self.based_on_db_field_name(&ty, &field_name, &model_attrs);
        let priority3 = || self.based_on_field_relation_type(&ty, &relation_type);


        let db_type = if let Ok(db_ty) = priority1() {
             Some(db_ty)
        }
        else if let Ok(db_ty) = priority2() {
            Some(db_ty)
        } else if let Ok(Some(db_ty)) =  priority3() {
             Some(db_ty)
        }
        else {
            None
        };

        Ok(db_type)
    }

    pub fn infer_type_by(
        &self,
        approach: InferenceApproach,
    ) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        let relation_type = self.relation_type;
        let model_attrs = self.model_attrs;
        let ty = self.field_ty;
        let field_name = self.db_field_name;

        // TODO: Consider removing lifetime except static here rather than doing it 
        // within the helper functions.
        let db_type = match approach {
            InferenceApproach::BasedOnTypePathToken => Some(self.based_on_type_path_token_structure(&ty, relation_type.clone())?),
            InferenceApproach::BasedOnRelationTypeFieldAttr => {
                self.based_on_field_relation_type(&ty, &relation_type)?
            }
            InferenceApproach::BasedOnDbFieldName => {
                Some(self.based_on_db_field_name(&ty, &field_name, &model_attrs)?)
            }
        };
        Ok(db_type)
    }

    // More Speed, Less Haste. Momentum is the kye.
    fn based_on_type_path_token_structure(
        &self,
        field_ty: &CustomType,
        relation_type: RelationType
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
            let none_opt_token =|| quote!(::std::option::Option::None);

            let array_len_token_stream = field_ty.get_array_const_length().map_or(none_opt_token(), |expr| {
                if expr.to_token_stream().is_empty() {
                    none_opt_token()
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
        } else if field_ty.raw_type_is_geometry() {
            DbFieldTypeAstMeta {
                // TODO: check if to auto-infer more speicific geometry type?
                field_type_db_original: FieldType::Geometry(vec![]),
                field_type_db_token: quote!(#crate_name::FieldType::Geometry(::std::vec![])).into(),
                static_assertion_token:
                    quote!(#crate_name::validators::assert_type_is_geometry::<#ty>();).into(),
            }
        } 
        else if let Some(foreign_type) = self.refereces_a_nested_object(relation_type)? {
        // Guess if its a foreign Table type by comparing the type path segment with the foreign
        // rust field type ident
            let current_segment_is_likely_foreign_type = field_ty.type_ident()? == foreign_type.type_ident()?;
            if current_segment_is_likely_foreign_type {
                DbFieldTypeAstMeta {
                    field_type_db_original: FieldType::Object,
                    field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                    static_assertion_token: quote!(
                                #crate_name::validators::assert_type_eq_all!(#ty, #foreign_type);
                ).into(),
                }
            } else  {
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
    fn refereces_a_nested_object(&self, relation_type: RelationType) -> ExtractorResult<Option<CustomType>> {
        let foreign_type = match relation_type {
            // RelationType::LinkOne(ref_node) => {
            //     Some(ref_node.into_inner())
            // },
            // RelationType::LinkSelf(self_node) => {
            //     let replace_self_with_current_struct_concrete_type = self_node.replace_self_with_current_struct_concrete_type(self.model_attrs)?;
            //     Some(replace_self_with_current_struct_concrete_type.into_inner())
            // },
            // RelationType::LinkMany(ref_node) | RelationType::LinkManyInAndOutEdgeNodesInert(ref_node) => {
            //     Some(ref_node.into_inner())
            // },
            RelationType::NestObject(ref_object) => {
                Some(ref_object.into_inner())
            },
            RelationType::NestArray(foreign_array_object) => {
                Some(foreign_array_object.into_inner())
            },
            RelationType::Relate(_) 
            | RelationType::List(_) 
            | RelationType::None  
            | RelationType::LinkOne(_)  
            | RelationType::LinkSelf(_)  
            | RelationType::LinkManyInAndOutEdgeNodesInert(_)  
            | RelationType::LinkMany(_)=> {
                None
            }
        };

        Ok(foreign_type)
    }

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
                field_type_db_token: quote!(
                    #crate_name::FieldType::Record(::std::vec![])
                )
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

                let db_field_type_ast_meta = DbFieldTypeAstMeta {
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
                        };
                db_field_type_ast_meta
            },
            RelationType::NestObject(_ref_object) => DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Object,
                field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                static_assertion_token:
                quote!(#crate_name::validators::assert_type_is_object::<#ty>();).into(),
            },
            RelationType::NestArray(foreign_array_object) => {
                let foreign_array_object= foreign_array_object.turbo_fishize()?.to_custom_type();
                let nesting_level = Self::count_vec_nesting(field_ty.to_basic_type());
                        let nested_vec_type =
                            Self::generate_nested_vec_type(&foreign_array_object, nesting_level);

                        DbFieldTypeAstMeta {
                                    // provide the inner type for when the array part start recursing
                                    field_type_db_original: FieldType::Array(Box::new(FieldType::Any), None),
                                    field_type_db_token: quote!(#crate_name::FieldType::Array(
                                        ::std::boxed::Box::new(#crate_name::FieldType::Any),
                                        ::std::option::Option::None
                                    )).into(),
                                    static_assertion_token: quote!(
                                        // #crate_name::validators::assert_type_eq_all!(#foreign_array_object, #nested_vec_type);
                                        #crate_name::validators::assert_type_eq_all!(#foreign_array_object, #field_ty);
                                    ).into(),
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

        Ok(Some(meta))
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
