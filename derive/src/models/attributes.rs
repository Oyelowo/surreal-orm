#![allow(missing_docs)]
#![allow(dead_code)]
/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

use crate::models::replace_lifetimes_with_underscore;

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    errors::ExtractorResult,
    field_rust_type::{Attributes, FieldRustType},
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

    fn from_list(items: &[darling::ast::NestedMeta]) -> ::darling::Result<Self> {
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
    pub model: Option<Type>,
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
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRelate {
            model: Type,
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
    pub(crate) old_name: Option<Ident>,

    #[darling(default)]
    pub(crate) rename: Option<Rename>,

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) relate: Option<Relate>,

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) link_one: Option<Type>,

    // reference singular: LinkSelf<Account>
    #[darling(default)]
    pub(crate) link_self: Option<Type>,

    // reference plural: LinkMany<Account>
    #[darling(default)]
    pub(crate) link_many: Option<Type>,

    #[darling(default)]
    pub(crate) nest_array: Option<Type>,

    #[darling(default)]
    pub(crate) nest_object: Option<Type>,

    #[darling(default)]
    pub(crate) skip_serializing: bool,

    #[darling(default)]
    pub(crate) skip: bool,

    // #[darling(default)]
    // default: ::std::option::Option<syn::Expr>,
    // #[darling(default, rename = "type")]
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

    // #[darling(default)]
    // pub(crate) item_type: Option<FieldTypeWrapper>,
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

#[derive(Debug, Clone)]
pub struct FieldTypeDerived {
    pub(crate) field_type: TokenStream,
    pub(crate) static_assertion: TokenStream,
}

impl Default for FieldTypeDerived {
    fn default() -> Self {
        let crate_name = get_crate_name(false);
        Self {
            field_type: quote!(#crate_name::FieldType::Any),
            static_assertion: Default::default(),
        }
    }
}

impl MyFieldReceiver {
    pub fn get_type(
        &self,
        field_name_normalized: &String,
        model_type: &DataType,
        table: &String,
    ) -> ExtractorResult<Option<FieldTypeDerived>> {
        println!(
            "get_type1 fieldname {}; model_type {:?}; table {}",
            field_name_normalized, model_type, table
        );
        let mut static_assertions = vec![];
        let crate_name = get_crate_name(false);

        if let Some(type_) = &self.type_ {
            let field_type = type_.deref();
            println!(
                "get_type2 fieldname {}; field_type {:?};",
                field_name_normalized, field_type
            );
            // id: record<student>
            // in: record
            // out: record
            // link_one => record<book> = #crate_name::validators::assert_has_field(<Book as Node>::TableNameChecker, book);
            // link_self => record<student> = #crate_name::validators::assert_has_field(<Student as Node>::TableNameChecker, student);
            // link_many => Vec<Book> => array<record<book>> = #crate_name::validators::assert_has_field(<Book as Node>::TableNameChecker, book);
            // e.g names: Vec<T> => array || array<string> => names: array && names.* : string

            match self {
                MyFieldReceiver {
                    type_: Some(type_),
                    item_assert,
                    item_assert_fn,
                    ..
                } if !type_.is_array() & (item_assert.is_some() || item_assert_fn.is_some()) => {
                    return Err(syn::Error::new_spanned(
                        field_name_normalized,
                        "item_assert or item_assert_fn can only be used with array types",
                    )
                    .into());
                }
                MyFieldReceiver {
                    type_: Some(type_),
                    link_one,
                    link_self,
                    link_many,
                    ..
                } => {
                    let linked_node = link_one.clone().or(link_self.clone());
                    let field_type = type_.deref();
                    let ref_node_table_name_checker_ident =
                        format_ident!("I{field_name_normalized}RefChecker");

                    if let Some(link_single_ref_node) = linked_node {
                        // Validate that it is a type - record, when link_one or link_self used,
                        // since those attributes are used for record links. When record type
                        // provided, do static assertions validation to check the inner type e.g
                        // record<book>
                        match field_type {
                            FieldType::Record(link_table_names) => {
                                let link_table_name = format_ident!(
                                    "{}",
                                    link_table_names
                                        .first()
                                        .map(ToString::to_string)
                                        .unwrap_or_default()
                                );
                                // TODO: Remove
                                // let ref_node = NodeTypeName::from(&link_single_ref_node);
                                // let ref_node_token: TokenStream = ref_node.into();
                                // Generate validation for the record type content at compile
                                // time
                                // Check that the link name in the type is same used lin
                                // link_one attribute e.g record(book), when link_one="Book",
                                // which gives <Book as Node>::TableNameChecker
                                static_assertions.push(quote!(
                                type #ref_node_table_name_checker_ident = <#link_single_ref_node as #crate_name::Node>::TableNameChecker;
                                #crate_name::validators::assert_fields!(#ref_node_table_name_checker_ident: #link_table_name);
                                           ));
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    field_name_normalized,
                                    "when link_one or link_self attribute is used, type must be record or record(<ref_node_table_name>)",
                                ).into());
                            }
                        }
                    } else if let Some(link_many_ref_node) = link_many {
                        match field_type.clone() {
                            FieldType::Array(item_type, _) | FieldType::Set(item_type, _) => {
                                // Check content type if of array type. link_many is used for
                                // // array types. e.g link_many = "Blog"

                                match item_type.deref() {
                                    FieldType::Record(array_item_table_name) => {
                                        match array_item_table_name.len() {
                                            1 => {
                                                let array_item_table_name = format_ident!(
                                                    "{}",
                                                    array_item_table_name
                                                        .first()
                                                        .expect("Table should be present here. This is a bug if not so.")
                                                        .to_string()
                                                );
                                                let ref_node =
                                                    NodeTypeName::from(link_many_ref_node);
                                                let ref_node_token: TokenStream = ref_node.into();

                                                static_assertions.push(quote!(
                                            type #ref_node_table_name_checker_ident = <#ref_node_token as #crate_name::Node>::TableNameChecker;
                                            #crate_name::validators::assert_fields!(#ref_node_table_name_checker_ident: #array_item_table_name);
                                        ));
                                            }
                                            _ => {
                                                return Err(syn::Error::new_spanned(
                                                    field_name_normalized,
                                                    "when link_many attribute is provided, type_ should reference a single table in the format  - array<record<table>>",
                                                ).into());
                                            }
                                        }
                                    }
                                    _ => {
                                        let err = format!("when link_many attribute is provided, type_ must be of type array<record> or array<record<ref_node_table_name>>. Got - {}", item_type.deref());
                                        return Err(syn::Error::new_spanned(
                                            field_name_normalized,
                                            err,
                                        )
                                        .into());
                                    }
                                }
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    field_name_normalized,
                                    "when link_many attribute is used, type must be array",
                                )
                                .into());
                            }
                        }
                    }
                }
                _ => {}
            };

            // Gather assertions for all field types
            if let DataType::Edge = model_type {
                match field_name_normalized.as_str() {
                    "id" => {
                        if !field_type.is_record_of_the_table(table) && !field_type.is_record_any()
                        {
                            let err = format!(
                                "`id` field must be of type `record({})` or `record()`",
                                table
                            );
                            return Err(syn::Error::new_spanned(
                                field_name_normalized,
                                err.as_str(),
                            )
                            .into());
                        }
                    }
                    "in" | "out" => {
                        if !field_type.is_record() {
                            let err = format!(
                                "`{}` field must be of type `record()`",
                                field_name_normalized
                            );
                            return Err(syn::Error::new_spanned(
                                field_name_normalized,
                                err.as_str(),
                            )
                            .into());
                        }
                    }
                    _ => {}
                }
            }

            let raw_type = &self.ty;
            let delifed_raw_type = replace_lifetimes_with_underscore(&mut raw_type.clone());
            let static_assertion = match field_type {
                FieldType::Any => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                }
                FieldType::Null => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                }
                FieldType::Uuid => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Uuid>);)
                }
                FieldType::Bytes => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Bytes>);)
                }
                FieldType::Union(_) => {
                    // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                    quote!()
                }
                FieldType::Option(_) => {
                    // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                    quote!()
                }
                FieldType::String => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<::std::string::String>);)
                }
                FieldType::Int => {
                    quote!(
                        #crate_name::validators::is_int::<#delifed_raw_type>();
                        // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                    )
                }
                FieldType::Float => {
                    quote!(
                        #crate_name::validators::is_float::<#delifed_raw_type>();
                        // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                    )
                    // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                }
                FieldType::Bool => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<::std::primitive::bool>);)
                }
                FieldType::Array(_, _) => {
                    quote!(
                        #crate_name::validators::assert_is_vec::<#delifed_raw_type>();
                    )
                }
                FieldType::Set(_, _) => {
                    quote!(
                        #crate_name::validators::assert_is_vec::<#delifed_raw_type>();
                    )
                }
                FieldType::Datetime => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Datetime>);)
                }
                FieldType::Decimal => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                }
                FieldType::Duration => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Duration>);)
                }
                FieldType::Number => {
                    quote!(
                        #crate_name::validators::is_number::<#delifed_raw_type>();
                        // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                    )
                    // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
                }
                FieldType::Object => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Object>);)
                }
                FieldType::Record(_) => {
                    if let DataType::Edge = model_type {
                        quote!()
                    } else {
                        quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<Option<#crate_name::sql::Thing>>);)
                    }
                }
                FieldType::Geometry(_) => {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Geometry>);)
                }
            };

            println!(
                "get_type4 fieldname {}; field_type {:?};",
                field_name_normalized, field_type
            );

            static_assertions.push(static_assertion);

            // Get the field type
            // define_field_methods.push(quote!(.type_(#type_.parse::<#crate_name::FieldType>()
            //                                             .expect("Must have been checked at compile time. If not, this is a bug. Please report"))
            //                                  )
            //                           );
            // define_field_methods.push(quote!(.type_(#crate_name::FieldType::String)));
            // let content
            let ft_string = field_type.to_string();
            Ok(Some(FieldTypeDerived {
                field_type: quote!(#ft_string.parse::<#crate_name::FieldType>()
                                                            .expect("Must have been checked at compile time. If not, this is a bug. Please report")),
                static_assertion: quote!( # ( #static_assertions ) *),
            }))
        } else if self
            .rust_type()
            .type_is_inferrable(&field_name_normalized.to_string())
        {
            Ok(Some(self.rust_type().infer_surreal_type_heuristically(
                field_name_normalized,
                model_type,
            )))
        } else {
            println!("problem fieldname {}", field_name_normalized);
            return Err(syn::Error::new_spanned(field_name_normalized, format!(
                r#"Unable to infer database type for the field. Type must be provided for field - {}.\
            e.g use the annotation #[surreal_orm(type_="int")] to provide the type explicitly."#,
                field_name_normalized
            ).as_str()).into());
        }
    }

    pub fn get_fallback_array_item_concrete_type(&self) -> Result<TokenStream, &str> {
        let field_type = self.type_.clone().map_or(FieldType::Any, |t| t.0);

        let item_type = match field_type {
            FieldType::Array(item_type, _) => item_type,
            // TODO: Check if to error out here or just use Any
            _ => return Err("Must be array type"),
            // _ => Box::new(FieldType::Any),
        };

        let crate_name = get_crate_name(false);
        let value = match item_type.deref() {
            FieldType::Any => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Null => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Uuid => {
                quote!(#crate_name::sql::Uuid)
            }
            FieldType::Bytes => {
                quote!(#crate_name::sql::Bytes)
            }
            FieldType::Union(_) => {
                quote!(#crate_name::sql::Value)
            }
            FieldType::Option(_) => {
                quote!(::std::option::Option<#crate_name::sql::Value>)
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
            FieldType::Array(_, _) => {
                // quote!(::std::iter::IntoIterator)
                // quote!(::std::convert::Into<#crate_name::sql::Array>)
                quote!(::std::vec::Vec<#crate_name::sql::Value>)
            }
            FieldType::Set(_, _) => {
                quote!(::std::collections::HashSet<#crate_name::sql::Value>)
            }
            FieldType::Datetime => {
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
            }
            FieldType::Record(_) => {
                quote!(::std::convert::Option<#crate_name::sql::Thing>)
            }
            FieldType::Geometry(_) => {
                quote!(#crate_name::sql::Geometry)
            }
        };
        Ok(value)
    }

    pub fn is_numeric(&self) -> bool {
        let field_type = self.type_.clone().map_or(FieldType::Any, |t| t.0);
        let explicit_ty_is_numeric = matches!(
            field_type,
            FieldType::Int | FieldType::Float | FieldType::Decimal | FieldType::Number
        );
        explicit_ty_is_numeric || self.rust_type().is_numeric()
    }

    pub fn is_list(&self) -> bool {
        let field_type = self.type_.clone().map_or(FieldType::Any, |t| t.0);
        let explicit_ty_is_list =
            matches!(field_type, FieldType::Array(_, _) | FieldType::Set(_, _));
        explicit_ty_is_list
            || self.rust_type().is_list()
            || self.type_.as_ref().map_or(false, |t| t.deref().is_array())
            || self.link_many.is_some()
    }

    pub fn rust_type(&self) -> FieldRustType {
        let ty = if self.ident.to_token_stream().to_string() == "age" {
            //create u8 type for age
            syn::parse_str::<syn::Type>("u8").unwrap()
        } else {
            self.ty.clone()
        };
        let attrs = Attributes {
            link_one: self.link_one.as_ref(),
            link_self: self.link_self.as_ref(),
            link_many: self.link_many.as_ref(),
            nest_array: self.nest_array.as_ref(),
            nest_object: self.nest_object.as_ref(),
        };
        let rust_type = FieldRustType::new(ty.clone(), attrs);
        rust_type
    }
}

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
                let permissions =
                    parse_lit_to_tokenstream(permissions).expect("Unable to parse permissions");
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

#[derive(Debug, Clone)]
pub struct FieldTypeWrapper(FieldType);

impl Display for FieldTypeWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for FieldTypeWrapper {
    type Target = FieldType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for FieldTypeWrapper {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.parse::<FieldType>() {
            Ok(f) => Ok(Self(f)),
            Err(e) => Err(darling::Error::unknown_value(&e)),
        }
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
    ) -> ExtractorResult<Self> {
        println!(
            "with_field_definition1 fieldname {}; ",
            field_name_normalized,
        );
        let crate_name = get_crate_name(false);
        let mut define_field: Option<TokenStream> = None;
        let mut define_field_methods = vec![];
        let mut define_array_field_item_methods = vec![];
        let mut static_assertions = vec![];

        println!(
            "with_field_definition2 fieldname {}; ",
            field_name_normalized,
        );
        let type_inf = field_receiver.get_type(field_name_normalized, data_type, table)?;
        // println!("type_inf {}", type_inf.unwrap_or_default());

        println!(
            "with_field_definition3 fieldname {}; ",
            field_name_normalized,
        );
        let field_type_resolved = if let Some(type_data) = type_inf {
            let FieldTypeDerived {
                field_type,
                static_assertion,
            } = type_data;

            define_field_methods.push(quote!(.type_(#field_type)));
            static_assertions.push(static_assertion);

            // TODO: Check if this would be needed.
            // if let Some(field_item_type) = field_item_type {
            //     define_array_field_item_methods.push(quote!(.type_(#field_item_type)));
            // }
            // Return field_type for later overriding type information in define_fn/define
            // attributes in case user uses either of those attributes. This is cause the type
            // attribute should supersede as it is what is used to validate field data at compile
            // time. Doing that with the `define` function attributes at compile-time may be tricky/impossible.
            field_type
        } else {
            return Err(
                syn::Error::new_spanned(field_name_normalized, "Invalid type provided").into(),
            );
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
                        || item_assert.is_some()
                        || item_assert_fn.is_some()
                ) =>
            {
                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    r#"Invalid combination. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:
    assert,
    assert_fn,
    value,
    value_fn,
    permissions,
    permissions_fn,
    item_assert,
    item_assert_fn"#).into());
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
                        || item_assert.is_some()
                        || item_assert_fn.is_some()
                ) =>
            {
                //             return Err("Invalid combination. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:
                // define,
                // define_fn,
                // assert,
                // assert_fn,
                // value,
                // value_fn,
                // permissions,
                // permissions_fn,
                // item_assert,
                // item_assert_fn");

                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    r#"Invalid combination. When `relate`, the following attributes cannot be use in combination to prevent confusion:
    define,
    define_fn,
    assert,
    assert_fn,
    value,
    value_fn,
    permissions,
    permissions_fn,
    item_assert,
    item_assert_fn"#).into());
            }
            MyFieldReceiver {
                define: Some(_),
                define_fn: Some(_),
                ..
            } => {
                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    "define and define_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.").into());
            }
            MyFieldReceiver {
                define: Some(define),
                ..
            } => {
                let define = parse_lit_to_tokenstream(define).expect("Unable to parse define");
                if define.to_token_stream().to_string().chars().count() < 3 {
                    // If empty, we get only the `()` of the function, so we can assume that it is empty
                    // if there are less than 3 characters.
                    // return Err("define attribute is empty. Please provide a define_fn attribute.");
                    return Err(syn::Error::new_spanned(
                        field_name_normalized,
                        "define attribute is empty. Please provide a define_fn attribute.",
                    )
                    .into());
                }
                define_field = Some(
                    quote!(#define.on_table(Self::table_name()).type_(#field_type_resolved).to_raw()),
                );
            }
            MyFieldReceiver {
                define_fn: Some(define_fn),
                ..
            } => {
                if define_fn.to_token_stream().to_string().is_empty() {
                    return Err(syn::Error::new_spanned(
                        field_name_normalized,
                        "define_fn attribute is empty. Please provide a define_fn attribute.",
                    )
                    .into());
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
                return Err(syn::Error::new_spanned(
                    field_name_normalized,
                    "item_assert and item_assert_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into());
            }
            MyFieldReceiver {
                item_assert: Some(item_assert),
                ..
            } => {
                let item_assert =
                    parse_lit_to_tokenstream(item_assert).expect("Unable to parse item_assert");
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
                return Err(syn::Error::new_spanned(
                    field_name_normalized,
                    "value and value_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into());
            }
            MyFieldReceiver {
                value: Some(value),
                type_: Some(type_),
                ..
            } => {
                let value = parse_lit_to_tokenstream(value).expect("unable to parse value");
                let field_type = type_.deref();
                let static_assertion = match field_type {
                    FieldType::Duration => quote!(#crate_name::sql::Duration::from(#value)),
                    FieldType::Uuid => quote!(#crate_name::sql::Uuid::from(#value)),
                    FieldType::Bytes => quote!(#crate_name::sql::Bytes::from(#value)),
                    FieldType::Null => quote!(#crate_name::sql::Value::Null),
                    // FieldType::Union(_) => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Union(_) => quote!(),
                    // FieldType::Option(_) => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Option(_) => quote!(),
                    FieldType::String => quote!(#crate_name::sql::String::from(#value)),
                    FieldType::Int => quote!(#crate_name::sql::Number::from(#value)),
                    FieldType::Float => quote!(#crate_name::sql::Number::from(#value)),
                    FieldType::Bool => quote!(#crate_name::sql::Bool::from(#value)),
                    FieldType::Array(_, _) => quote!(),
                    FieldType::Set(_, _) => quote!(),
                    // FieldType::Array => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Datetime => quote!(#crate_name::sql::Datetime::from(#value)),
                    FieldType::Decimal => quote!(#crate_name::sql::Number::from(#value)),
                    FieldType::Number => quote!(#crate_name::sql::Number::from(#value)),
                    FieldType::Object => quote!(),
                    // FieldType::Object => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Record(_) => quote!(#crate_name::sql::Thing::from(#value)),
                    FieldType::Geometry(_) => quote!(#crate_name::sql::Geometry::from(#value)),
                    FieldType::Any => quote!(#crate_name::sql::Value::from(#value)),
                };

                static_assertions.push(quote!(let _ = #static_assertion;));

                define_field_methods
                    // .push(quote!(.value(#crate_name::sql::Value::from(#type_of))));
                    .push(quote!(.value(#crate_name::sql::to_value(&#value).unwrap())));
            }
            MyFieldReceiver {
                value_fn: Some(value_fn),
                type_: Some(type_),
                ..
            } => {
                let field_type = type_.deref();
                let static_assertion = match field_type {
                    FieldType::Bytes => quote!(#crate_name::sql::Bytes::from(#value_fn())),
                    FieldType::Null => quote!(#crate_name::sql::Value::Null),
                    // FieldType::Union(_) => quote!(#crate_name::sql::Value::from(#value_fn())),
                    FieldType::Union(_) => quote!(),
                    // FieldType::Option(_) => quote!(#crate_name::sql::Value::from(#value_fn())),
                    FieldType::Option(_) => quote!(),
                    FieldType::Uuid => quote!(#crate_name::sql::Uuid::from(#value_fn())),
                    FieldType::Duration => quote!(#crate_name::sql::Duration::from(#value_fn())),
                    FieldType::String => quote!(#crate_name::sql::String::from(#value_fn())),
                    FieldType::Int => quote!(#crate_name::sql::Number::from(#value_fn())),
                    FieldType::Float => quote!(#crate_name::sql::Number::from(#value_fn())),
                    FieldType::Bool => quote!(#crate_name::sql::Bool::from(#value_fn())),
                    FieldType::Array(_, _) => quote!(),
                    FieldType::Set(_, _) => quote!(),
                    // FieldType::Array => quote!(#crate_name::sql::Value::from(#value)),
                    FieldType::Datetime => quote!(#crate_name::sql::Datetime::from(#value_fn())),
                    FieldType::Decimal => quote!(#crate_name::sql::Number::from(#value_fn())),
                    FieldType::Number => quote!(#crate_name::sql::Number::from(#value_fn())),
                    FieldType::Object => quote!(),
                    // FieldType::Object => quote!(#crate_name::sql::Value::from(#value_fn())),
                    FieldType::Record(_) => quote!(#crate_name::sql::Thing::from(#value_fn())),
                    FieldType::Geometry(_) => quote!(#crate_name::sql::Geometry::from(#value_fn())),
                    FieldType::Any => quote!(#crate_name::sql::Value::from(#value_fn())),
                };
                static_assertions.push(quote!(let _ = #static_assertion;));

                define_field_methods
                    // .push(quote!(.value(#crate_name::sql::Value::from(#value_fn()))));
                    // .push(quote!(.value(#crate_name::sql::Value::from(#type_of))));
                    .push(quote!(.value(#crate_name::sql::to_value(&#value_fn()).unwrap())));
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
                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    "assert and assert_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into());
            }
            MyFieldReceiver {
                assert: Some(assert),
                ..
            } => {
                let assert = parse_lit_to_tokenstream(assert).expect("unable to parse assert");
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
                return Err(
                syn::Error::new_spanned(
                    field_name_normalized,
                    "permissions and permissions_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two.",
                ).into());
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

        Ok(self)
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

        let foreign_node_schema_import = if *struct_name_ident == node_type_name.to_string() {
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
                #crate_name::validators::assert_impl_one!(#schema_type_ident: #crate_name::Node);
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

        let foreign_node_schema_import = if *struct_name_ident == node_type_name.to_string() {
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
                #crate_name::validators::assert_impl_one!(#schema_type_ident: #crate_name::Object);
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
        let field_ident = field_receiver
            .ident
            .as_ref()
            .expect("Field ident is required");

        let field_ident_cased = FieldIdentCased::from(FieldIdentUnCased {
            uncased_field_name: field_ident.to_string(),
            casing: struct_level_casing,
        });

        // get the field's proper serialized format. Renaming should take precedence
        let original_field_name_normalised = &field_receiver.rename.as_ref().map_or_else(
            || field_ident_cased.into(),
            |renamed| renamed.clone().serialize,
        );
        let field_ident_normalised = &format_ident!("{original_field_name_normalised}");

        let (field_ident_normalised, field_ident_normalised_as_str) =
            if original_field_name_normalised.trim_start_matches("r#") == "in" {
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
    pub(crate) generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: Data<util::Ignored, self::MyFieldReceiver>,

    #[darling(default)]
    pub(crate) rename_all: ::std::option::Option<Rename>,

    #[darling(default)]
    pub(crate) table_name: ::std::option::Option<Ident>,

    #[darling(default)]
    pub(crate) relax_table_name: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) schemafull: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) drop: ::std::option::Option<bool>,

    #[darling(default)]
    pub(crate) flexible: ::std::option::Option<bool>,

    // #[darling(default, rename = "as_")]
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
    pub fn get_table_definition_token(&self) -> Result<TokenStream, &str> {
        let TableDeriveAttributes {
            ref drop,
            ref flexible,
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
                || flexible.is_some()
                || permissions.is_some()
                || permissions_fn.is_some())
        {
            return Err("Invalid combination. When `define` or `define_fn`, the following attributes cannot be use in combination to prevent confusion:
                            drop,
                            flexible,
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
                let define = parse_lit_to_tokenstream(define).expect("Unable to parse define attribute");
                define_table = Some(quote!(#define.to_raw()));
            },
            (None, Some(define_fn)) => {
                define_table = Some(quote!(#define_fn().to_raw()));
            },
            (Some(_), Some(_)) => return Err("define and define_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
            (None, None) => (),
        };

        if let Some(_drop) = drop {
            define_table_methods.push(quote!(.drop()))
        }

        if let Some(_flexible) = flexible {
            define_table_methods.push(quote!(.flexible()))
        }

        match (as_, as_fn){
            (Some(as_), None) => {
                let as_ = parse_lit_to_tokenstream(as_).expect("Unable to parse 'as' attribute");
                define_table_methods.push(quote!(.as_(#as_)))
            },
            (None, Some(as_fn)) => {
                    define_table_methods.push(quote!(.as_(#as_fn())));
            },
            (Some(_), Some(_)) => return Err("as and as_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
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
            (Some(_), Some(_)) => return Err("permissions and permissions_fn attribute cannot be provided at the same time to prevent ambiguity. Use either of the two."),
            (None, None) => (),
        };

        Ok(define_table.unwrap_or_else(|| {
            quote!(
                #crate_name::statements::define_table(Self::table_name())
                #( #define_table_methods) *
                .to_raw()
            )
        }))
    }
}
