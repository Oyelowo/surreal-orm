/* 
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use super::{get_crate_name, parser::ModelAttributesTokensDeriver, casing::CaseString,};
use darling::{ast, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
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

pub trait Edge {
    const EDGE_RELATION: &'static str;
    fn to(&self) -> ::proc_macro2::TokenStream;
    fn from(&self) -> ::proc_macro2::TokenStream;
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
            edge: None
        })
    }
//TODO: Check to maybe remove cos I probably dont need this
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        // pub trait Edge {
        //     const edge_relation: &'static str;
        //     fn to(&self) -> ::proc_macro2::TokenStream;
        //     fn from(&self) -> ::proc_macro2::TokenStream;
        // }

        #[derive(FromMeta)]
        struct FullRelate {
            edge: String,
            link: String
        }

        impl From<FullRelate> for Relate {
            fn from(v: FullRelate) -> Self {
                let FullRelate {  link,edge, .. } = v;
                Self { link, edge: Some(edge)}
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
    ty: syn::Type,
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
}

impl ToTokens for FieldsGetterOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldsGetterOpts {
            ident: ref my_struct,
            ref data,
            ref rename_all,
            ..
        } = *self;

        let struct_level_casing = rename_all.as_ref().map(|case| {
            CaseString::from_str(case.serialize.as_str()).expect("Invalid casing, The options are")
        });

        let ModelAttributesTokensDeriver {
           // all_schema_reexported_aliases,
           all_model_imports,
           // all_schema_names_basic,
           all_fields,
           ..
        } = ModelAttributesTokensDeriver::from_receiver_data(data, struct_level_casing);

        let schema_type_alias_name = ::quote::format_ident!("{my_struct}Schema");
        
        let schema_mod_name = format_ident!("{}", my_struct.to_string().to_lowercase());
        let _crate_name = get_crate_name(false);
        
        tokens.extend(quote! {
            use ::surreal_simple_querybuilder::prelude::*;
            // #struct_type
            
            mod #schema_mod_name {
                #( #all_model_imports) *
                
                
                use surreal_simple_querybuilder::prelude::*;
                
                ::surreal_simple_querybuilder::prelude::model!(
                 #my_struct {
                    #( #all_fields) *
                }
             );
            }

            // e.g: type alias: type AccountSchema<const N: usize> = account::schema::Account<N>;
            type #schema_type_alias_name<const N: usize> = #schema_mod_name::schema::#my_struct<N>;
            // use #schema_mod_name::schema::model as #schema_type_alias_name;

            impl #my_struct {
                // type Schema = account::schema::Account<0>;
                // type Schema = #schema_mod_name::schema::#my_struct<0>;
                const SCHEMA: #schema_mod_name::schema::#my_struct<0> = #schema_mod_name::schema::#my_struct::<0>::new();
                const fn get_schema() -> #schema_type_alias_name<0> {
                    // project::schema::model
                    //  account::schema::Account<0>::new()
                    // e.g: account::schema::Account::<0>::new()
                    #schema_mod_name::schema::#my_struct::<0>::new()
                }
                fn own_schema(&self) -> #schema_type_alias_name<0> {
                    // project::schema::model
                    //  account::schema::Account<0>::new()
                    // e.g: account::schema::Account::<0>::new()
                    #schema_mod_name::schema::#my_struct::<0>::new()
                }
            }

            impl ::surreal_simple_querybuilder::prelude::IntoKey<::std::string::String> for #my_struct {
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

            // }      
            // impl #crate_name::SurrealdbModel for #my_struct {
            //     type Fields = #fields_getter_struct_name;
            //     fn get_fields_serialized() -> Self::Fields {
            //         #fields_getter_struct_name {
            //             #( #struct_values_fields), *
            //         }
            //     }
            // }
        });
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
