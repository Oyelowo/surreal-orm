#![allow(missing_docs)]
#![allow(dead_code)]
/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{ops::Deref, str::FromStr};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    get_crate_name, parse_lit_to_tokenstream,
    parser::DataType,
    relations::NodeTypeName,
    variables::VariablesModelMacro,
};
use darling::{ast::Data, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use surreal_query_builder::FieldType;
use syn::{Ident, Lit, LitStr, Path, Type};

#[derive(Debug, Clone)]
pub struct Rename {
    pub(crate) serialize: String,
}

/// This enables us to handle potentially nested values i.e
///   #[serde(rename = "simple_name")]
///    or
///   #[serde(rename(serialize = "age"))]
///  #[serde(rename(serialize = "ser_name_nested", deserialize = "deser_name_nested"))]
/// However, We dont care about deserialized name from serde, so we just ignore that.
impl FromMeta for Rename {
    fn from_string(value: &str) -> ::darling::Result<Self> {
        Ok(Self {
            serialize: value.into(),
        })
    }

    fn from_list(items: &[syn::NestedMeta]) -> ::darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRename {
            serialize: String,

            #[darling(default)]
            _deserialize: util::Ignored, // Ignore deserialize since we only care about the serialized string
        }

        impl From<FullRename> for Rename {
            fn from(v: FullRename) -> Self {
                let FullRename { serialize, .. } = v;
                Self { serialize }
            }
        }
        FullRename::from_list(items).map(Rename::from)
    }
}

#[derive(Debug, Clone)]
pub struct Relate {
    /// e.g ->writes->book
    pub connection_model: String,
    // #[darling(default)]
    /// e.g StudentWritesBook,
    /// derived from: type StudentWritesBook = Writes<Student, Book>;
    pub model: Option<String>,
}
//#[rename(se)]
impl FromMeta for Relate {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self {
            connection_model: value.into(),
            model: None,
        })
    }
    //TODO: Check to maybe remove cos I probably dont need this
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRelate {
            model: String,
            connection: String,
        }

        impl From<FullRelate> for Relate {
            fn from(v: FullRelate) -> Self {
                let FullRelate {
                    connection, model, ..
                } = v;
                Self {
                    connection_model: connection,
                    model: Some(model),
                }
            }
        }
        FullRelate::from_list(items).map(Relate::from)
    }
}

// #[derive(Debug, Clone)]
// struct Content {
//     #[darling(default, rename = "type")]
//     pub(crate) type_: Option<FieldTypeWrapper>,
// }
//
// #[derive(Debug, Clone)]
// struct FullContent {
//     #[darling(default, rename = "type")]
//     pub(crate) type_: Option<FieldTypeWrapper>,
//
//     #[darling(default)]
//     pub(crate) assert: Option<syn::LitStr>,
//
//     #[darling(default)]
//     pub(crate) assert_fn: Option<syn::Path>,
// }
// impl FromMeta for Content {
//     fn from_string(value: &str) -> ::darling::Result<Self> {
//         let value = match value.parse::<FieldType>() {
//             Ok(f) => Ok(Self {
//                 type_: Some(FieldTypeWrapper(value.to_string())),
//             }),
//             Err(e) => Err(darling::Error::unknown_value(&e)),
//         };
//     }
//
//     fn from_list(items: &[syn::NestedMeta]) -> ::darling::Result<Self> {
//         #[derive(FromMeta)]
//         struct FullContent {
//             serialize: String,
//
//             #[darling(default)]
//             deserialize: util::Ignored, // Ignore deserialize since we only care about the serialized string
//         }
//
//         impl From<FullRename> for Rename {
//             fn from(v: FullRename) -> Self {
//                 let FullRename { serialize, .. } = v;
//                 Self { serialize }
//             }
//         }
//         FullRename::from_list(items).map(Rename::from)
//     }
// }
// struct Oyelowo {
//     #[orm(type="list")]
//     names: List<names>,
//
//     #[orm(type="list", content="string"))]
//     names: List<names>,
//
//     #[orm(type="list", content=(type="string"))]
//     names: List<names>,
//
//     #[orm(type="list", content=(type="string", assert_fn="my_assert()"))]
//     names: List<names>,
//
//     #[orm(type="list", item_type="string", item_assert="my_assert()")]
//     names2: List<names>
// }
// fn my_assert() {
//     value > 5
//
// }

#[derive(Debug, FromField)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub(crate) ident: Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    pub(crate) ty: syn::Type,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub(crate) rename: Option<Rename>,

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) relate: Option<Relate>,

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) link_one: Option<String>,

    // reference singular: LinkSelf<Account>
    #[darling(default)]
    pub(crate) link_self: Option<String>,

    // reference plural: LinkMany<Account>
    #[darling(default)]
    pub(crate) link_many: Option<String>,

    #[darling(default)]
    pub(crate) nest_array: Option<String>,

    #[darling(default)]
    pub(crate) nest_object: Option<String>,

    #[darling(default)]
    pub(crate) skip_serializing: bool,

    #[darling(default)]
    pub(crate) skip: bool,

    // #[darling(default)]
    // default: ::std::option::Option<syn::Expr>,
    #[darling(default, rename = "type")]
    pub(crate) type_: Option<FieldTypeWrapper>,

    #[darling(default)]
    pub(crate) assert: Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) assert_fn: Option<syn::Path>,

    #[darling(default)]
    pub(crate) define: Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) define_fn: Option<syn::Path>,

    #[darling(default)]
    pub(crate) value: Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) value_fn: Option<syn::Path>,

    #[darling(default)]
    pub(crate) permissions: Option<Permissions>,

    #[darling(default)]
    pub(crate) permissions_fn: Option<PermissionsFn>,

    #[darling(default)]
    pub(crate) item_type: Option<FieldTypeWrapper>,

    #[darling(default)]
    item_assert: Option<syn::LitStr>,

    #[darling(default)]
    item_assert_fn: Option<syn::Path>,

    #[darling(default)]
    skip_serializing_if: ::darling::util::Ignored,

    #[darling(default)]
    with: ::darling::util::Ignored,
    #[darling(default)]
    deserialize_with: ::darling::util::Ignored,
    #[darling(default)]
    default: ::darling::util::Ignored,
}

pub struct FieldTypeDerived {
    field_type: TokenStream,
    field_item_type: Option<TokenStream>,
    static_assertion: TokenStream,
}

impl MyFieldReceiver {
    pub fn get_type(
        &self,
        field_name_normalized: &String,
        model_type: &DataType,
        table: &String,
    ) -> Option<FieldTypeDerived> {
        let mut static_assertions = vec![];
        let crate_name = get_crate_name(false);

        if let Some(type_) = &self.type_ {
            let type_ = type_.0.to_string();
            // id: record(student)
            // in: record
            // out: record
            // link_one => record(book) = static_assertions::assert_has_field(<Book as Node>::TableNameChecker, book);
            // link_self => record(student) = static_assertions::assert_has_field(<Student as Node>::TableNameChecker, student);
            // link_many => Vec<Book> => array(record(book)) = static_assertions::assert_has_field(<Book as Node>::TableNameChecker, book);
            // e.g names: Vec<T> => array || array(string) => names: array && names.* : string

            match self {
                MyFieldReceiver {
                    type_: Some(type_),
                    item_type,
                    item_assert,
                    item_assert_fn,
                    ..
                } if !type_.0.trim().to_string().starts_with("array")
                    & (item_type.is_some()
                        || item_assert.is_some()
                        || item_assert_fn.is_some()) =>
                {
                    panic!("attributes `item_type`, `item_assert`, or `item_assert_fn` can only be used when type is array.")
                }
                MyFieldReceiver {
                    type_: Some(type_),
                    link_one,
                    link_self,
                    link_many,
                    item_type,
                    ..
                } => {
                    let linked_node = link_one.clone().or(link_self.clone());

                    let error = format!(
                        "Invalid type. Expected one of - `{:?}`",
                        FieldType::variants()
                    );
                    let error = error.as_str();
                    let field_type = FieldType::from_str(type_.to_string()).expect(error);
                    // .expect("Field type should have been validated here. If not, report bug");
                    let ref_node_table_name_checker_ident =
                        format_ident!("I{field_name_normalized}RefChecker");

                    if let Some(link_single_ref_node) = linked_node {
                        // Validate that it is a type - record, when link_one or link_self used,
                        // since those attributes are used for record links. When record type
                        // provided, do static assertions validation to check the inner type e.g
                        // record(book)
                        match field_type {
                            FieldType::Record(link_table_name) => {
                                let link_table_name =
                                    format_ident!("{}", link_table_name.to_string());
                                let ref_node = NodeTypeName::from(&link_single_ref_node);
                                let ref_node_token: TokenStream = ref_node.into();
                                // Generate validation for the record type content at compile
                                // time
                                // Check that the link name in the type is same used lin
                                // link_one attribute e.g record(book), when link_one="Book",
                                // which gives <Book as Node>::TableNameChecker
                                static_assertions.push(quote!(
                                type #ref_node_table_name_checker_ident = <#ref_node_token as #crate_name::Node>::TableNameChecker;
                                ::static_assertions::assert_fields!(#ref_node_table_name_checker_ident: #link_table_name);
                                           ));
                            }
                            _ => {
                                panic!("when link_one or link_self attribute is used, type must be record or record(<ref_node_table_name>)");
                            }
                        }
                    } else if let Some(link_many_ref_node) = link_many {
                        match field_type {
                            FieldType::Array => {
                                if let Some(item_type) = item_type {
                                    // Check content type if of array type. link_many is used for
                                    // array types. e.g link_many = "Blog"
                                    let item_type =
                                        FieldType::from_str(&item_type.0.to_string()).unwrap();

                                    match item_type {
                                        FieldType::Record(array_item_table_name) => {
                                            let array_item_table_name = format_ident!(
                                                "{}",
                                                array_item_table_name.to_string()
                                            );
                                            let ref_node = NodeTypeName::from(link_many_ref_node);
                                            let ref_node_token: TokenStream = ref_node.into();

                                            static_assertions.push(quote!(
                                                            type #ref_node_table_name_checker_ident = <#ref_node_token as #crate_name::Node>::TableNameChecker;
                                                            ::static_assertions::assert_fields!(#ref_node_table_name_checker_ident: #array_item_table_name);
                                                       ));
                                        }
                                        _ => {
                                            panic!("when link_many attribute is provided, item_type must be of type record or record(<ref_node_table_name>)");
                                        }
                                    }
                                }
                            }
                            _ => {
                                panic!("type must be `array` when link_many attribute is used")
                            }
                        }
                    } else {
                        if let FieldType::Array = field_type {
                            if self.item_type.is_none()
                                && !self.type_is_inferrable(field_name_normalized)
                            {
                                panic!(
                                    "Not able to infer array content type. Content type must 
be provided when type is array and the compiler cannot infer the type. 
Please, provide `item_type` for the field - {}. 
e.g `#[surreal_orm(type=array, item_type=\"int\")]`",
                                    &field_name_normalized
                                );
                            }
                        }
                    }
                }
                _ => {}
            };

            // Gather assertions for all field types
            let raw_type = &self.ty;
            let field_type = FieldType::from_str(&type_.to_string()).unwrap();

            if let DataType::Edge = model_type {
                match field_name_normalized.as_str() {
                    "id" => {
                        if !field_type.is_record_of_the_table(table) && !field_type.is_record_any()
                        {
                            panic!(
                                "`id` field must be of type `record({})` or `record()`",
                                table
                            );
                        }
                    }
                    "in" | "out" => {
                        if !field_type.is_record() {
                            panic!(
                                "`{}` field must be of type `record()`",
                                field_name_normalized
                            );
                        }
                    }
                    _ => {}
                }
            }

            let static_assertion = match field_type {
                FieldType::Any => {
                    quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                }
                FieldType::String => {
                    quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<::std::string::String>);)
                }
                FieldType::Int => {
                    quote!(
                        #crate_name::validators::is_int::<#raw_type>();
                        // ::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                    )
                }
                FieldType::Float => {
                    quote!(
                        #crate_name::validators::is_float::<#raw_type>();
                        // ::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                    )
                    // quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                }
                FieldType::Bool => {
                    quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<::std::primitive::bool>);)
                }
                FieldType::Array => {
                    quote!(
                        #crate_name::validators::assert_is_vec::<#raw_type>();
                    )
                }
                FieldType::DateTime => {
                    quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Datetime>);)
                }
                FieldType::Decimal => {
                    quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                }
                FieldType::Duration => {
                    quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Duration>);)
                }
                FieldType::Number => {
                    quote!(
                        #crate_name::validators::is_number::<#raw_type>();
                        // ::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                    )
                    // quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                }
                FieldType::Object => {
                    quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Object>);)
                }
                FieldType::Record(_) => {
                    if let DataType::Edge = model_type {
                        quote!()
                    } else {
                        quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<Option<#crate_name::sql::Thing>>);)
                    }
                }
                FieldType::RecordAny => {
                    if let DataType::Edge = model_type {
                        quote!()
                    } else {
                        quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<Option<#crate_name::sql::Thing>>);)
                    }
                }
                FieldType::Geometry(_) => {
                    quote!(::static_assertions::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Geometry>);)
                }
            };

            static_assertions.push(static_assertion);

            // Get the field type
            // define_field_methods.push(quote!(.type_(#type_.parse::<#crate_name::FieldType>()
            //                                             .expect("Must have been checked at compile time. If not, this is a bug. Please report"))
            //                                  )
            //                           );
            // define_field_methods.push(quote!(.type_(#crate_name::FieldType::String)));
            // let content
            Some(FieldTypeDerived {
                field_type: quote!(#type_.parse::<#crate_name::FieldType>()
                                                            .expect("Must have been checked at compile time. If not, this is a bug. Please report")),

                field_item_type: self.item_type.as_ref().map(|item_type| {
                    let ref item_type = item_type.0;
                    quote!(#item_type.parse::<#crate_name::FieldType>()
                        .expect("Must have been checked at compile time. If not, this is a bug. Please report"))
                }),
                static_assertion: quote!( # ( #static_assertions ) *),
                // #( # define_array_field_item_methods) *
            })
        } else if self.type_is_inferrable(&field_name_normalized.to_string()) {
            Some(self.infer_surreal_type_heuristically(field_name_normalized))
        } else {
            None
        }
    }

    pub fn infer_surreal_type_heuristically(
        &self,
        // struct_name_ident_str: &String,
        field_name_normalized: &String,
    ) -> FieldTypeDerived {
        let crate_name = get_crate_name(false);
        let ty = &self.ty;

        if self.raw_type_is_bool() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Bool),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<::std::primitive::bool>);),
            }
        } else if self.raw_type_is_float() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Float),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Number>);),
            }
        } else if self.raw_type_is_integer() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Int),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Number>);),
            }
        } else if self.raw_type_is_string() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::String),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Strand>);),
            }
        } else if self.raw_type_is_list() || self.item_type.is_some() {
            let item_type = self.item_type.as_ref().map(|ct| {
                let ct = ct.to_string();
                quote!(#ct.to_string().parse::<#crate_name::FieldType>().expect("Invalid db type"))
            });
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Array),
                field_item_type: item_type,
                static_assertion: quote!(#crate_name::validators::assert_is_vec::<#ty>();),
                // quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Array>);),
            }
        } else if self.raw_type_is_object() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Object),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Object>);),
            }
        } else if self.raw_type_is_duration() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Duration),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Duration>);),
            }
        } else if self.raw_type_is_datetime() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::DateTime),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Datetime>);),
            }
        } else if self.raw_type_is_geometry() {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Geometry(::std::vec![#crate_name::GeometryType::Feature])),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Geometry>);),
            }
        } else if let MyFieldReceiver {
            type_: None,
            link_one,
            link_self,
            link_many,
            nest_array,
            nest_object,
            ..
        } = self
        {
            let field_name_normalized = field_name_normalized.as_str();

            if field_name_normalized == "id" {
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Record(Self::table_name())),
                    field_item_type: None,
                    static_assertion: quote!(),
                }
                // TODO: Only do this for Edge
            } else if field_name_normalized == "out" || field_name_normalized == "in" {
                // An edge might be shared by multiple In/Out nodes. So, default to any type of
                // record for edge in and out
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::RecordAny),
                    field_item_type: None,
                    static_assertion: quote!(),
                }
            } else if let Some(ref_node_type) = link_one.clone().or(link_self.clone()) {
                let ref_node_type = format_ident!("{ref_node_type}");

                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Record(#ref_node_type::table_name())),
                    field_item_type: None,
                    static_assertion: quote!(),
                }
            } else if let Some(ref_node_type) = link_many {
                let ref_struct_name = format_ident!("{ref_node_type}");
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Array),
                    field_item_type: Some(
                        quote!(#crate_name::FieldType::Record(#ref_struct_name::table_name())),
                    ),
                    static_assertion: quote!(),
                }
            } else if let Some(_ref_node_type) = nest_object {
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Object),
                    field_item_type: None,
                    static_assertion: quote!(),
                }
            } else if let Some(_ref_node_type) = nest_array {
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Array),
                    field_item_type: None,
                    static_assertion: quote!(),
                }
            } else if let Some(_ref_node_type) = link_one {
                FieldTypeDerived {
                    // #crate_name::SurrealId<#foreign_node>
                    field_type: quote!(#crate_name::FieldType::Record(_ref_node_type::table_name())),
                    field_item_type: None,
                    static_assertion: quote!(),
                }
            } else if let Some(_ref_node_type) = link_self {
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Record(_ref_node_type::table_name())),
                    field_item_type: None,
                    static_assertion: quote!(),
                }
            } else {
                FieldTypeDerived {
                    field_type: quote!(#crate_name::FieldType::Any),
                    field_item_type: None,
                    static_assertion: quote!(),
                }
            }
        } else {
            FieldTypeDerived {
                field_type: quote!(#crate_name::FieldType::Any),
                field_item_type: None,
                static_assertion: quote!(::static_assertions::assert_impl_one!(#ty: ::std::convert::Into<#crate_name::sql::Value>);),
            }
        }
    }

    pub fn type_is_inferrable(&self, field_name_normalized_str: &String) -> bool {
        self.link_one.is_some()
            || self.link_self.is_some()
            || self.link_many.is_some()
            || self.nest_object.is_some()
            || self.nest_array.is_some()
            || field_name_normalized_str == "id"
            || field_name_normalized_str == "in"
            || field_name_normalized_str == "out"
            || self.raw_type_is_float()
            || self.raw_type_is_integer()
            || self.raw_type_is_string()
            || self.raw_type_is_bool()
            || self.raw_type_is_list()
            || self.item_type.is_some()
            || self.raw_type_is_object()
            || self.raw_type_is_duration()
            || self.raw_type_is_datetime()
            || self.raw_type_is_geometry()
    }
    pub fn is_numeric(&self) -> bool {
        let ty = &self.ty;
        let surreal_field_type = match &self.type_ {
            Some(x) => FieldType::from_str(x.0.as_str()).unwrap_or(FieldType::Any),
            None => FieldType::Any,
        };

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
            || matches!(
                surreal_field_type,
                FieldType::Int | FieldType::Number | FieldType::Float
            )
        // let is_numeric = match quote! {#ty}.to_string().as_str() {
        //     "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128"
        //     | "f32" | "f64" => true,
        //     _ => false,
        // };
        // is_numeric
    }

    pub fn raw_type_is_float(&self) -> bool {
        // let ty = &self.ty;
        // let is_float = match quote! {#ty}.to_string().as_str() {
        //     "f32" | "f64" => true,
        //     _ => false,
        // };
        // is_float
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
        match self.ty {
            syn::Type::Path(ref p) => {
                let path = &p.path;
                path.leading_colon.is_none() && path.segments.len() == 1 && {
                    let ident = &path.segments[0].ident.to_string();
                    ["String"].iter().any(|&x| x == ident)
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
            || self.type_.as_ref().map_or(false, |t| {
                t.parse::<FieldType>().unwrap_or(FieldType::Any).is_array()
            })
            || self.link_many.is_some()
    }

    pub fn raw_type_is_list(&self) -> bool {
        let ty = &self.ty;
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path.path.segments.last().unwrap();
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        if let syn::Type::Infer(_) = inner_type {
                            // if let syn::Type::Infer(_) = inner_type.as_ref() {
                            // The list type should have a specified type parameter
                            return false;
                        }
                    }
                    last_seg.ident.to_string() == "Vec"
                } else {
                    false
                }
            }
            syn::Type::Array(_) => true,
            _ => false,
        }
    }

    pub fn raw_type_is_object(&self) -> bool {
        let ty = &self.ty;
        // let is_object = match quote! {#ty}.to_string().as_str() {
        //     "HashMap" | "std::collections::HashMap" => true,
        //     "BTreeMap" | "std::collections::BTreeMap" => true,
        //     _ => false,
        // };
        // is_object
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path.path.segments.last().unwrap();
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        if let syn::Type::Infer(_) = inner_type {
                            // if let syn::Type::Infer(_) = inner_type.as_ref() {
                            // The list type should have a specified type parameter
                            return false;
                        }
                    }
                    last_seg.ident.to_string() == "HashMap"
                        || last_seg.ident.to_string() == "BTreeMap"
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
        // let is_datetime = match quote! {#ty}.to_string().as_str() {
        //     "std::time::Duration" | "chrono::Duration" => true,
        //     "chrono::NaiveDateTime" | "chrono::DateTime<chrono::Utc>" => true,
        //     _ => false,
        // };
        // is_datetime
        // match ty {
        //     syn::Type::Path(path) => {
        //         let last_seg = path.path.segments.last().unwrap();
        //         if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
        //             // if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
        //             //     if let syn::Type::Infer(_) = inner_type {
        //             //         // if let syn::Type::Infer(_) = inner_type.as_ref() {
        //             //         // The list type should have a specified type parameter
        //             //         return false;
        //             //     }
        //             // }
        //             last_seg.ident.to_string() == "DateTime"
        //         } else {
        //             false
        //         }
        //     }
        //     syn::Type::Array(_) => true,
        //     _ => false,
        // }
        match ty {
            syn::Type::Path(type_path) => {
                let last_segment = type_path.path.segments.last().unwrap();
                if last_segment.ident.to_string().to_lowercase() == "datetime" {
                    return true;
                } else {
                    return false;
                }
            }
            _ => false,
        }
    }

    pub fn raw_type_is_duration(&self) -> bool {
        let ty = &self.ty;
        // let is_duration = match quote! {#ty}.to_string().as_str() {
        //     "Duration" => true,
        //     _ => false,
        // };
        // is_duration
        // match ty {
        //     syn::Type::Path(path) => {
        //         let last_seg = path.path.segments.last().unwrap();
        //         if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
        //             if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
        //                 if let syn::Type::Infer(_) = inner_type {
        //                     // if let syn::Type::Infer(_) = inner_type.as_ref() {
        //                     // The list type should have a specified type parameter
        //                     return false;
        //                 }
        //             }
        //             last_seg.ident.to_string() == "Duration"
        //         } else {
        //             false
        //         }
        //     }
        //     syn::Type::Array(_) => true,
        //     _ => false,
        // }
        match ty {
            syn::Type::Path(type_path) => {
                let last_segment = type_path.path.segments.last().unwrap();
                if last_segment.ident == "Duration" {
                    return true;
                } else {
                    return false;
                }
            }
            _ => false,
        }
    }

    pub fn raw_type_is_geometry(&self) -> bool {
        let ty = &self.ty;
        // let is_geometry = match quote! {#ty}.to_string().as_str() {
        //     "Point" | "LineString" | "Polygon" | "MultiPoint" | "MultiLineString"
        //     | "MultiPolygon" | "GeometryCollection" | "Geometry" => true,
        //     _ => false,
        // };
        // is_geometry
        match ty {
            syn::Type::Path(path) => {
                let last_seg = path.path.segments.last().unwrap();
                last_seg.ident.to_string() == "Geometry"
                    || last_seg.ident.to_string() == "Point"
                    || last_seg.ident.to_string() == "LineString"
                    || last_seg.ident.to_string() == "Polygon"
                    || last_seg.ident.to_string() == "MultiPoint"
                    || last_seg.ident.to_string() == "MultiLineString"
                    || last_seg.ident.to_string() == "MultiPolygon"
                    || last_seg.ident.to_string() == "GeometryCollection"
                // if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                //     // if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                //     //     if let syn::Type::Infer(_) = inner_type {
                //     //         // if let syn::Type::Infer(_) = inner_type.as_ref() {
                //     //         // The list type should have a specified type parameter
                //     //         return false;
                //     //     }
                //     // }
                //     last_seg.ident.to_string() == "Geometry"
                //         || last_seg.ident.to_string() == "Point"
                //         || last_seg.ident.to_string() == "LineString"
                //         || last_seg.ident.to_string() == "Polygon"
                //         || last_seg.ident.to_string() == "MultiPoint"
                //         || last_seg.ident.to_string() == "MultiLineString"
                //         || last_seg.ident.to_string() == "MultiPolygon"
                //         || last_seg.ident.to_string() == "GeometryCollection"
                // } else {
                //     false
                // }
            }
            syn::Type::Array(_) => true,
            _ => false,
        }
    }

    pub fn get_array_item_type(&self) -> Option<TokenStream> {
        let ty = &self.ty;
        // if let Some(links_type) = self.link_many.clone() {
        //     let links_type = syn::parse_str::<syn::Type>(&links_type).unwrap();
        //     // let links_type = format_ident!("{links_type}");
        //     return Some(quote!(#links_type));
        // }

        get_vector_item_type(ty).map(|t| t.into_token_stream())
        // match ty {
        //     syn::Type::Array(array) => {
        //         dbg!(&array.elem);
        //         Some(*array.elem.clone())
        //     }
        //     _ => None,
        // }
    }

    pub fn get_fallback_array_item_concrete_type(&self) -> TokenStream {
        let field_type =
            FieldType::from_str(&self.item_type.clone().unwrap_or("any".into()).to_string())
                .unwrap();
        let crate_name = get_crate_name(false);
        match field_type {
            FieldType::Any => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::String => {
                quote!(::std::string::String)
            }
            FieldType::Int => {
                // quote!(#crate_name::validators::Int)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Float => {
                // quote!(#crate_name::validators::Float)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Bool => {
                quote!(::std::convert::Into<::std::primitive::bool>)
            }
            FieldType::Array => {
                // quote!(::std::iter::IntoIterator)
                // quote!(::std::convert::Into<#crate_name::sql::Array>)
                quote!(::std::vec::Vec<#crate_name::sql::Value>)
            }
            FieldType::DateTime => {
                quote!(#crate_name::sql::Datetime)
            }
            FieldType::Decimal => {
                quote!(#crate_name::validators::Float)
            }
            FieldType::Duration => {
                quote!(#crate_name::sql::Duration)
            }
            FieldType::Number => {
                // quote!(#crate_name::validators::Num)
                quote!(#crate_name::sql::Number)
            }
            FieldType::Object => {
                quote!(#crate_name::sql::Object)
                // quote!(#crate_name::Object)
            }
            FieldType::Record(_) => {
                quote!(::std::convert::Option<#crate_name::sql::Thing>)
            }
            FieldType::RecordAny => {
                quote!(::std::convert::OOption<#crate_name::sql::Thing>)
            }
            FieldType::Geometry(_) => {
                quote!(#crate_name::sql::Geometry)
            }
        }
    }
}

fn get_vector_item_type(ty: &Type) -> Option<Type> {
    let item_ty = match ty {
        syn::Type::Path(type_path) => {
            let last_segment = type_path.path.segments.last().unwrap();
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
// #[derive(Debug, Clone)]
// pub struct ValueWrapper(syn::LitStr);
//
// impl FromMeta for ValueWrapper {
//     fn from_value(value: &Lit) -> darling::Result<Self> {
//         match value {
//             Lit::Int(i) => i,
//             Lit::Float(f) => f,
//             Lit::Bool(b) => b,
//             Lit::Str(str_lit) => ,
//             Lit::ByteStr(_) => todo!(),
//             Lit::Byte(_) => todo!(),
//             Lit::Char(_) => todo!(),
//             Lit::Verbatim(_) => todo!(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum Permissions {
    Full,
    None,
    FnName(LitStr),
}

impl Permissions {
    pub fn get_token_stream(&self) -> TokenStream {
        match self {
            Self::Full => {
                quote!(.permissions_full())
            }
            Self::None => {
                quote!(.permissions_none())
            }
            Self::FnName(permissions) => {
                let permissions = parse_lit_to_tokenstream(permissions).unwrap();
                quote!(.permissions(#permissions.to_raw()))
            }
        }
    }
}

impl FromMeta for Permissions {
    fn from_value(value: &Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(str_lit) => {
                let value_str = str_lit.value();

                if value_str.to_lowercase() == "none" {
                    Ok(Self::None)
                } else if value_str.to_lowercase() == "full" {
                    Ok(Self::Full)
                } else {
                    Ok(Self::FnName(LitStr::new(&value_str, str_lit.span())))
                    // Ok(Self::FnName(str_lit.to_owned()))
                }
                // Ok(Self::FnName(LitStr::new(&value_str, str_lit.span())))
            }
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FieldTypeWrapper(pub String);

impl From<&str> for FieldTypeWrapper {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl FieldTypeWrapper {
    fn to_string(&self) -> &String {
        &self.0
    }
}

#[derive(Default, Clone)]
pub(crate) struct ReferencedNodeMeta {
    pub(crate) foreign_node_type: TokenStream,
    pub(crate) foreign_node_schema_import: TokenStream,
    pub(crate) record_link_default_alias_as_method: TokenStream,
    pub(crate) foreign_node_type_validator: TokenStream,
    pub(crate) field_definition: TokenStream,
    pub(crate) field_type_validation_asserts: Vec<TokenStream>,
}

impl ReferencedNodeMeta {
    pub fn with_field_definition(
        mut self,
        field_receiver: &MyFieldReceiver,
        _struct_name_ident: &Ident,
        field_name_normalized: &String,
        data_type: &DataType,
        table: &String,
    ) -> Self {
        let crate_name = get_crate_name(false);
        let mut define_field: Option<TokenStream> = None;
        let mut define_field_methods = vec![];
        let mut define_array_field_item_methods = vec![];
        let mut static_assertions = vec![];

        let field_type_resolved = if let Some(type_data) =
            field_receiver.get_type(field_name_normalized, data_type, table)
        {
            let FieldTypeDerived {
                field_type,
                field_item_type,
                static_assertion,
            } = type_data;

            define_field_methods.push(quote!(.type_(#field_type)));
            static_assertions.push(static_assertion);

            if let Some(field_item_type) = field_item_type {
                define_array_field_item_methods.push(quote!(.type_(#field_item_type)));
            }
            // Return field_type for later overriding type information in define_fn/define
            // attributes in case user uses either of those attributes. This is cause the type
            // attribute should supersede as it is what is used to validate field data at compile
            // time. Doing that with the `define` function attributes at compile-time may be tricky/impossible.
            field_type
        } else {
            panic!("Invalid type provided");
        };

        match field_receiver {
            MyFieldReceiver {
                define,
                define_fn,
                // type_,
                assert,
                assert_fn,
                value,
                value_fn,
                permissions,
                permissions_fn,
                // item_type,
                item_assert,
                item_assert_fn,
                ..
            } if (define_fn.is_some() || define.is_some())
                && (
                    // I think type should be allowed in addition to define or define_fn but will
                    // override whatever is defined in define or define_fn, so we can use it for
                    // code inference and generation.
                    // type_.is_some()
                    assert.is_some()
                        || assert_fn.is_some()
                        || value.is_some()
                        || value_fn.is_some()
                        || permissions.is_some()
                        || permissions_fn.is_some()
                        // || item_type.is_some()
                        || item_assert.is_some()
                        || item_assert_fn.is_some()
                ) =>
            {
                panic!("Invalid combinationation. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:                 
    assert,
    assert_fn,
    value,
    value_fn,
    permissions,
    permissions_fn,
    item_assert,
    item_assert_fn");
            }
            MyFieldReceiver {
                define,
                define_fn,
                // type_,
                assert,
                assert_fn,
                value,
                value_fn,
                permissions,
                permissions_fn,
                // item_type,
                item_assert,
                item_assert_fn,
                relate,
                ..
            } if (relate.is_some())
                && (
                    // type_.is_some()
                    define.is_some()
                        || define_fn.is_some()
                        || assert.is_some()
                        || assert_fn.is_some()
                        || value.is_some()
                        || value_fn.is_some()
                        || permissions.is_some()
                        || permissions_fn.is_some()
                        // || item_type.is_some()
                        || item_assert.is_some()
                        || item_assert_fn.is_some()
                ) =>
            {
                panic!("Invalid combinationation. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:                 
    define,
    define_fn,
    assert,
    assert_fn,
    value,
    value_fn,
    permissions,
    permissions_fn,
    item_assert,
    item_assert_fn");
            }
            MyFieldReceiver {
                define: Some(_),
                define_fn: Some(_),
                ..
            } => {
                panic!("define and define_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                define: Some(define),
                ..
            } => {
                let define = parse_lit_to_tokenstream(define).unwrap();
                if define.to_token_stream().to_string().chars().count() < 3 {
                    // If empty, we get only the `()` of the function, so we can assume that it is empty
                    // if there are less than 3 characters.
                    panic!("define attribute is empty. Please provide a define_fn attribute.");
                }
                define_field = Some(quote!(#define.type_(#field_type_resolved).to_raw()));
            }
            MyFieldReceiver {
                define_fn: Some(define_fn),
                ..
            } => {
                if define_fn.to_token_stream().to_string().is_empty() {
                    panic!("define_fn attribute is empty. Please provide a define_fn attribute.");
                }

                define_field = Some(quote!(#define_fn().type_(#field_type_resolved).to_raw()));
            }
            _ => {}
        };

        match field_receiver {
            MyFieldReceiver {
                item_assert: Some(_),
                item_assert_fn: Some(_),
                ..
            } => {
                panic!("item_assert and item_assert_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                item_assert: Some(item_assert),
                ..
            } => {
                let item_assert = parse_lit_to_tokenstream(item_assert).unwrap();
                define_array_field_item_methods.push(quote!(.assert(#item_assert)));
            }
            MyFieldReceiver {
                item_assert_fn: Some(item_assert_fn),
                ..
            } => {
                define_array_field_item_methods.push(quote!(.assert(#item_assert_fn())));
            }
            _ => {}
        };

        // Gather default values
        match field_receiver {
            MyFieldReceiver {
                value: Some(_value),
                value_fn: Some(_value_fn),
                ..
            } => {
                panic!("value and value_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                value: Some(value), ..
            } => {
                let value = parse_lit_to_tokenstream(value).unwrap();
                define_field_methods.push(quote!(.value(#crate_name::sql::Value::from(#value))));
            }
            MyFieldReceiver {
                value_fn: Some(value_fn),
                ..
            } => {
                define_field_methods
                    .push(quote!(.value(#crate_name::sql::Value::from(#value_fn()))));
            }
            _ => {}
        };

        // Gather assertions
        match field_receiver {
            MyFieldReceiver {
                assert: Some(_),
                assert_fn: Some(_),
                ..
            } => {
                panic!("assert and assert_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                assert: Some(assert),
                ..
            } => {
                let assert = parse_lit_to_tokenstream(assert).unwrap();
                define_field_methods.push(quote!(.assert(#assert)));
            }
            MyFieldReceiver {
                assert_fn: Some(assert_fn),
                ..
            } => {
                define_field_methods.push(quote!(.assert(#assert_fn())));
            }
            _ => {}
        };

        // Gather permissions
        match field_receiver {
            MyFieldReceiver {
                permissions: Some(_),
                permissions_fn: Some(_),
                ..
            } => {
                panic!("permissions and permissions_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.");
            }
            MyFieldReceiver {
                permissions: Some(permissions),
                ..
            } => {
                define_field_methods.push(permissions.get_token_stream());
            }
            MyFieldReceiver {
                permissions_fn: Some(permissions_fn),
                ..
            } => {
                define_field_methods.push(permissions_fn.get_token_stream());
            }
            _ => {}
        };

        // Helps to define the schema definition of the content
        let array_field_item_str = format!("{field_name_normalized}.*");
        // Im putting coma before this to separate from the top field array type definition in case
        // it is present
        let array_item_definition = if define_array_field_item_methods.is_empty() {
            quote!()
        } else {
            quote!(
                    ,
                #crate_name::statements::define_field(#crate_name::Field::new(#array_field_item_str))
                                        .on_table(#crate_name::Table::from(Self::table_name()))
                                        #( # define_array_field_item_methods) *
                                        .to_raw()

            )
        };

        self.field_definition = define_field.unwrap_or_else(||quote!(
                    #crate_name::statements::define_field(#crate_name::Field::new(#field_name_normalized))
                                            .on_table(#crate_name::Table::from(Self::table_name()))
                                            #( # define_field_methods) *
                                            .to_raw()
                    #array_item_definition
            ));

        self.field_type_validation_asserts.extend(static_assertions);

        self
    }

    pub(crate) fn from_simple_array(normalized_field_name: &::syn::Ident) -> Self {
        let normalized_field_name_str = normalized_field_name.to_string();
        let crate_name = get_crate_name(false);

        let record_link_default_alias_as_method = quote!(
            pub fn #normalized_field_name(&self, clause: impl Into<#crate_name::NodeAliasClause>) -> #crate_name::Field {
                let clause: #crate_name::NodeAliasClause = clause.into();
                let clause: #crate_name::NodeClause = clause.into_inner();

                let normalized_field_name_str = if self.build().is_empty(){
                    #normalized_field_name_str.to_string()
                }else {
                    format!(".{}", #normalized_field_name_str)
                };

                let clause: #crate_name::NodeClause = clause.into();
                let bindings = self.get_bindings().into_iter().chain(clause.get_bindings().into_iter()).collect::<Vec<_>>();

                let errors = self.get_errors().into_iter().chain(clause.get_errors().into_iter()).collect::<Vec<_>>();

                let field = #crate_name::Field::new(format!("{normalized_field_name_str}{}", clause.build()))
                            .with_bindings(bindings)
                            .with_errors(errors);
                field

            }
        );

        Self {
            foreign_node_schema_import: quote!(),

            foreign_node_type_validator: quote!(),

            record_link_default_alias_as_method,
            foreign_node_type: quote!(schema_type_ident),
            field_definition: quote!(),
            field_type_validation_asserts: vec![],
        }
    }

    pub(crate) fn from_record_link(
        node_type_name: &NodeTypeName,
        normalized_field_name: &::syn::Ident,
        struct_name_ident: &::syn::Ident,
        is_list: bool,
    ) -> Self {
        let VariablesModelMacro {
            ___________graph_traversal_string,
            __________connect_node_to_graph_traversal_string,
            ..
        } = VariablesModelMacro::new();
        let normalized_field_name_str = normalized_field_name.to_string();

        let schema_type_ident = format_ident!("{node_type_name}");
        let crate_name = get_crate_name(false);

        let foreign_node_schema_import = if node_type_name.to_string()
            == struct_name_ident.to_string()
        {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #schema_type_ident = <super::#schema_type_ident as #crate_name::SchemaGetter>::Schema;)
        };

        let record_link_default_alias_as_method = if is_list {
            quote!(
                pub fn #normalized_field_name(&self, clause: impl Into<#crate_name::NodeAliasClause>) -> #schema_type_ident {
                     let clause: #crate_name::NodeAliasClause = clause.into();
                     let clause: #crate_name::NodeClause = clause.into_inner();

                    let normalized_field_name_str = if self.build().is_empty(){
                        #normalized_field_name_str.to_string()
                    }else {
                        format!(".{}", #normalized_field_name_str)
                    };


                    #schema_type_ident::#__________connect_node_to_graph_traversal_string(
                        self,
                        clause.with_field(normalized_field_name_str)
                    )

                }
            )
        } else {
            quote!(
                pub fn #normalized_field_name(&self) -> #schema_type_ident {
                    let clause = #crate_name::Clause::from(#crate_name::Empty);

                    let normalized_field_name_str = if self.build().is_empty(){
                        #normalized_field_name_str.to_string()
                    }else {
                        format!(".{}", #normalized_field_name_str)
                    };

                    #schema_type_ident::#__________connect_node_to_graph_traversal_string(
                        self,
                        clause.with_field(normalized_field_name_str)
                    )

                }
            )
        };

        Self {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SchemaGetter>::Schema;
            foreign_node_schema_import,

            foreign_node_type_validator: quote!(
                ::static_assertions::assert_impl_one!(#schema_type_ident: #crate_name::Node);
            ),

            record_link_default_alias_as_method,
            foreign_node_type: quote!(schema_type_ident),
            field_definition: quote!(),
            field_type_validation_asserts: vec![],
        }
    }

    pub(crate) fn from_nested(
        node_type_name: &NodeTypeName,
        normalized_field_name: &::syn::Ident,
        struct_name_ident: &::syn::Ident,
        is_list: bool,
    ) -> Self {
        let VariablesModelMacro {
            __________connect_object_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();

        let schema_type_ident = format_ident!("{node_type_name}");
        let normalized_field_name_str = normalized_field_name.to_string();
        let crate_name = get_crate_name(false);

        let foreign_node_schema_import = if node_type_name.to_string()
            == struct_name_ident.to_string()
        {
            // Dont import for current struct since that already exists in scope
            quote!()
        } else {
            quote!(type #schema_type_ident = <super::#schema_type_ident as #crate_name::SchemaGetter>::Schema;)
        };

        let record_link_default_alias_as_method = if is_list {
            quote!(
                pub fn #normalized_field_name(&self, clause: impl Into<#crate_name::ObjectClause>) -> #schema_type_ident {
                    let clause: #crate_name::ObjectClause = clause.into();
                    let normalized_field_name_str = if self.build().is_empty(){
                        #normalized_field_name_str.to_string()
                    }else {
                        format!(".{}", #normalized_field_name_str)
                    };


                    #schema_type_ident::#__________connect_object_to_graph_traversal_string(
                        self,
                        clause.with_field(normalized_field_name_str)
                    )

                }
            )
        } else {
            quote!(
                pub fn #normalized_field_name(&self) -> #schema_type_ident {
                    let clause = #crate_name::Clause::from(#crate_name::Empty);

                    let normalized_field_name_str = if self.build().is_empty(){
                        #normalized_field_name_str.to_string()
                    }else {
                        format!(".{}", #normalized_field_name_str)
                    };


                    #schema_type_ident::#__________connect_object_to_graph_traversal_string(
                        self,
                        clause.with_field(normalized_field_name_str)
                    )

                }
            )
        };

        Self {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SchemaGetter>::Schema;
            foreign_node_schema_import,

            foreign_node_type_validator: quote!(
                ::static_assertions::assert_impl_one!(#schema_type_ident: #crate_name::Object);
            ),

            record_link_default_alias_as_method,
            foreign_node_type: quote!(schema_type_ident),
            field_definition: quote!(),
            field_type_validation_asserts: vec![],
        }
    }
}

pub(crate) struct NormalisedField {
    pub(crate) field_ident_normalised: Ident,
    pub(crate) field_ident_normalised_as_str: String,
}

impl NormalisedField {
    pub(crate) fn from_receiever(
        field_receiver: &MyFieldReceiver,
        struct_level_casing: Option<CaseString>,
    ) -> Self {
        let field_ident = field_receiver.ident.as_ref().unwrap();

        let field_ident_cased = FieldIdentCased::from(FieldIdentUnCased {
            uncased_field_name: field_ident.to_string(),
            casing: struct_level_casing,
        });

        // get the field's proper serialized format. Renaming should take precedence
        let original_field_name_normalised = &field_receiver.rename.as_ref().map_or_else(
            || field_ident_cased.into(),
            |renamed| renamed.clone().serialize,
        );
        let ref field_ident_normalised = format_ident!("{original_field_name_normalised}");

        let (field_ident_normalised, field_ident_normalised_as_str) =
            if original_field_name_normalised.trim_start_matches("r#") == "in".to_string() {
                (format_ident!("in_"), "in".to_string())
            } else {
                (
                    field_ident_normalised.to_owned(),
                    field_ident_normalised.to_string(),
                )
            };

        Self {
            field_ident_normalised,
            field_ident_normalised_as_str,
        }
    }
}

impl Deref for FieldTypeWrapper {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for FieldTypeWrapper {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.parse::<FieldType>() {
            Ok(f) => Ok(Self(f.to_string())),
            Err(e) => Err(darling::Error::unknown_value(&e)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PermissionsFn {
    Full,
    None,
    FnPath(Path),
}

impl PermissionsFn {
    pub fn get_token_stream(&self) -> TokenStream {
        match self {
            Self::Full => {
                quote!(.permissions_full())
            }
            Self::None => {
                quote!(.permissions_none())
            }
            Self::FnPath(permissions_fn) => {
                quote!(.permissions(#permissions_fn().to_raw()))
            }
        }
    }
}

impl FromMeta for PermissionsFn {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "full" => Ok(Self::Full),
            _ => Err(darling::Error::unexpected_type(value)),
        }
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(str) => Ok(Self::FnPath(syn::parse_str::<Path>(&str.value())?)),
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct TableDeriveAttributes {
    pub(crate) ident: syn::Ident,
    // pub(crate) attrs: Vec<syn::Attribute>,
    // pub(crate) generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: Data<util::Ignored, self::MyFieldReceiver>,

    #[darling(default)]
    pub(crate) rename_all: ::std::option::Option<Rename>,

    #[darling(default)]
    pub(crate) table_name: ::std::option::Option<String>,

    #[darling(default)]
    pub(crate) relax_table_name: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) schemafull: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) drop: ::std::option::Option<bool>,

    #[darling(default, rename = "as")]
    pub(crate) as_: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) as_fn: ::std::option::Option<syn::Path>,

    #[darling(default)]
    pub(crate) permissions: ::std::option::Option<Permissions>,

    #[darling(default)]
    pub(crate) permissions_fn: ::std::option::Option<PermissionsFn>,

    #[darling(default)]
    pub(crate) define: ::std::option::Option<syn::LitStr>,

    #[darling(default)]
    pub(crate) define_fn: ::std::option::Option<syn::Path>,
}

impl TableDeriveAttributes {
    pub fn get_table_definition_token(&self) -> TokenStream {
        let TableDeriveAttributes {
            ref drop,
            ref schemafull,
            ref as_,
            ref as_fn,
            ref permissions,
            ref permissions_fn,
            ref define,
            ref define_fn,
            ..
        } = *self;

        let crate_name = super::get_crate_name(false);

        if (define_fn.is_some() || define.is_some())
            && (drop.is_some()
                || as_.is_some()
                || as_fn.is_some()
                || schemafull.is_some()
                || permissions.is_some()
                || permissions_fn.is_some())
        {
            panic!("Invalid combinationation. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:
                            drop,
                            as,
                            as_fn,
                            schemafull,
                            permissions,
                            permissions_fn");
        }

        let mut define_table: Option<TokenStream> = None;
        let mut define_table_methods = vec![];

        match (define, define_fn){
            (Some(define), None) => {
                let define = parse_lit_to_tokenstream(define).unwrap();
                define_table = Some(quote!(#define.to_raw()));
            },
            (None, Some(define_fn)) => {
                define_table = Some(quote!(#define_fn().to_raw()));
            },
            (Some(_), Some(_)) => panic!("define and define_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
            (None, None) => (),
        };

        if let Some(_drop) = drop {
            define_table_methods.push(quote!(.drop()))
        }

        match (as_, as_fn){
            (Some(as_), None) => {
                let as_ = parse_lit_to_tokenstream(as_).unwrap();
                define_table_methods.push(quote!(.as_(#as_)))
            },
            (None, Some(as_fn)) => {
                    define_table_methods.push(quote!(#as_fn()));
            },
            (Some(_), Some(_)) => panic!("as and as_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
            (None, None) => (),
        };

        if let Some(_schemafull) = schemafull {
            define_table_methods.push(quote!(.schemafull()))
        }

        match (permissions, permissions_fn){
            (None, Some(p_fn)) => {
                    define_table_methods.push(p_fn.get_token_stream());
            },
            (Some(p), None) => {
                    define_table_methods.push(p.get_token_stream());
            },
            (Some(_), Some(_)) => panic!("permissions and permissions_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
            (None, None) => (),
        };

        define_table.unwrap_or_else(|| {
            quote!(
                #crate_name::statements::define_table(Self::table_name())
                #( #define_table_methods) *
                .to_raw()
            )
        })
    }
}
