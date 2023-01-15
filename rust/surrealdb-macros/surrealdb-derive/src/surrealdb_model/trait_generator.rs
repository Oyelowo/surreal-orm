/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use super::{
    casing::CaseString,
    get_crate_name,
    parser::{EdgeModelAttr, ModelAttributesTokensDeriver},
};
use darling::{ast, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::str::FromStr;

use syn::{self, parse_macro_input};

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
            deserialize: util::Ignored, // Ignore deserialize since we only care about the serialized string
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
    pub link: String,
    // #[darling(default)]
    pub edge: Option<String>,
}
//#[rename(se)]
impl FromMeta for Relate {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self {
            link: value.into(),
            edge: None,
        })
    }
    //TODO: Check to maybe remove cos I probably dont need this
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRelate {
            edge: String,
            link: String,
        }

        impl From<FullRelate> for Relate {
            fn from(v: FullRelate) -> Self {
                let FullRelate { link, edge, .. } = v;
                Self {
                    link,
                    edge: Some(edge),
                }
            }
        }
        FullRelate::from_list(items).map(Relate::from)
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(surrealdb, serde), forward_attrs(allow, doc, cfg))]
pub(crate) struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub(crate) ident: ::std::option::Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    pub(crate) ty: syn::Type,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub(crate) rename: ::std::option::Option<Rename>,

    // graph relation: e.g ->has->Account
    #[darling(default)]
    pub(crate) relate: ::std::option::Option<Relate>,

    // reference singular: Foreign<Account>
    #[darling(default)]
    pub(crate) reference_one: ::std::option::Option<String>,

    // reference plural: Foreign<Vec<Account>>
    #[darling(default)]
    pub(crate) reference_many: ::std::option::Option<String>,

    #[darling(default)]
    pub(crate) skip_serializing: bool,

    #[darling(default)]
    skip_serializing_if: ::darling::util::Ignored,

    #[darling(default)]
    with: ::darling::util::Ignored,

    #[darling(default)]
    default: ::darling::util::Ignored,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surrealdb, serde), forward_attrs(allow, doc, cfg))]
pub struct FieldsGetterOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<util::Ignored, self::MyFieldReceiver>,

    #[darling(default)]
    rename_all: ::std::option::Option<Rename>,

    #[darling(default)]
    pub(crate) relation_name: ::std::option::Option<String>,
}

impl ToTokens for FieldsGetterOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldsGetterOpts {
            ident: ref struct_name_ident,
            ref data,
            ref rename_all,
            ref relation_name,
            ..
        } = *self;

        let struct_level_casing = rename_all.as_ref().map(|case| {
            CaseString::from_str(case.serialize.as_str()).expect("Invalid casing, The options are")
        });

        let schema_mod_name = format_ident!("{}", struct_name_ident.to_string().to_lowercase());
        let crate_name = get_crate_name(false);

        let ModelAttributesTokensDeriver {
            all_model_imports,
            all_model_schema_fields,
            all_static_assertions,
            edge_metadata, .. } = ModelAttributesTokensDeriver::from_receiver_data(
            data,
            struct_level_casing,
            struct_name_ident,
        );
        let EdgeModelAttr {
            in_node_type,
            out_node_type,
        } = edge_metadata;
        let all_model_imports = all_model_imports
            .into_iter()
            .map(Into::into)
            .collect::<Vec<TokenStream>>();
        let test_name = format_ident!("test_{schema_mod_name}_edge_name");

        let edge_model_tokens = match relation_name {
            Some(relation) => {
                let relation_name_ident = format_ident!("{relation}");
                let mod_name_ident = format_ident!("{struct_name_ident}{relation}_relation");

                match (in_node_type, out_node_type) {
                    (Some(in_node_type), Some(out_node_type)) => {
                        quote!(
                            pub mod #mod_name_ident {
                                pub struct EdgeChecker {
                                    pub #relation_name_ident: String,
                                }

                            }

                            impl #crate_name::Edge for #struct_name_ident {
                                type EdgeChecker = #mod_name_ident::EdgeChecker;
                                type InNode = #in_node_type;
                                type OutNode = #out_node_type;
                            }

                        )
                    }
                    _ => {
                        panic!("`in` and `out` fields must be provided for the edge model struct.")
                    }
                }
            }
            None => quote!(),
        };

        tokens.extend(quote!( 
                        pub mod #schema_mod_name {
                            #( #all_model_imports) *

                            ::surreal_simple_querybuilder::prelude::model!(
                             #struct_name_ident {
                                #( #all_model_schema_fields) *
                            }
                         );
                        }

                        impl #crate_name::SurrealdbModel for #struct_name_ident {
                            // e.g type Schema = account::schema::Account<0>;
                            type Schema<const T: usize> = #schema_mod_name::schema::#struct_name_ident<T>;
                            fn get_schema() -> Self::Schema<0> {
                                #schema_mod_name::schema::#struct_name_ident::<0>::new()
                            }

                            // fn get_key(&self) -> #crate_name::Id {#crate_name::Id(self.id.unwrap()) };
                            
                            fn get_key(&self) -> ::std::option::Option<String> {self.id.as_ref().map(::std::string::String::clone) } 
                            // fn get_key<E>(&self) -> ::std::result::Result<String, E>
                            //     where
                            //         E: ::serde::ser::Error
                            //     {
                            //         self
                            //         .id
                            //         .as_ref()
                            //         .map(::std::string::String::clone)
                            //         .ok_or(::serde::ser::Error::custom("The model has no ID"))
                            //     }
                        }
                        // impl #struct_name_ident {
                        //     // type Schema = account::schema::Account<0>;
                        //     // type Schema = #schema_mod_name::schema::#my_struct<0>;
                        //     const SCHEMA: #schema_mod_name::schema::#struct_name_ident<0> = #schema_mod_name::schema::#struct_name_ident::<0>::new();
                        //     const fn get_schema() -> <Self as #crate_name::SurrealdbModel>::Schema<0> {
                        //         // project::schema::model
                        //         //  account::schema::Account<0>::new()
                        //         // e.g: account::schema::Account::<0>::new()
                        //         #schema_mod_name::schema::#struct_name_ident::<0>::new()
                        //     }
                        //     // fn own_schema(&self) -> #schema_type_alias_name<0> {
                        //     //     // project::schema::model
                        //     //     //  account::schema::Account<0>::new()
                        //     //     // e.g: account::schema::Account::<0>::new()
                        //     //     #schema_mod_name::schema::#my_struct::<0>::new()
                        //     // }
                        // }

                        impl ::surreal_simple_querybuilder::prelude::IntoKey<::std::string::String> for #struct_name_ident {
                            fn into_key<E>(&self) -> ::std::result::Result<String, E>
                                where
                                    E: ::serde::ser::Error
                                {
                                    self
                                    .id
                                    .as_ref()
                                    .map(::std::string::String::clone)
                                    .ok_or(::serde::ser::Error::custom("The project has no ID"))
                                }
                        }
             #edge_model_tokens
            #[test]
            fn #test_name() {
                #( #all_static_assertions) *

                // ::static_assertions::assert_type_eq_all!(<AccountManageProject as Edge>::InNode, Account);
                // ::static_assertions::assert_type_eq_all!(<AccountManageProject as Edge>::OutNode, Project);
                // ::static_assertions::assert_fields!(Modax: manage);
            }
));
    }
}

pub fn generate_fields_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input);
    // let output = FieldsGetterOpts::from_derive_input(&input).expect("Wrong options");
    let output = match FieldsGetterOpts::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
