


/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]


// pub(crate) mod casing;
// mod parser;
// pub(crate) mod relations;
// pub(crate) mod serialize_skipper;
// mod trait_generator;
// use super:{
//     casing::CaseString,
//     get_crate_name,
//     parser::{EdgeModelAttr, ModelAttributesTokensDeriver},
// };
use darling::{ast, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::str::FromStr;

use syn::{self, parse_macro_input};

use super::{edge_parser::SchemaFieldsProperties, casing::CaseString};

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

    // reference singular: LinkOne<Account>
    #[darling(default)]
    pub(crate) link_one: ::std::option::Option<String>,

    // reference singular: LinkSelf<Account>
    #[darling(default)]
    pub(crate) link_self: ::std::option::Option<String>,
    
    // reference plural: LinkMany<Account>
    #[darling(default)]
    pub(crate) link_many: ::std::option::Option<String>,

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

        let binding = struct_name_ident.to_string();
        let struct_name_ident_as_str = binding.as_str();
        let schema_mod_name = format_ident!("{}", struct_name_ident.to_string().to_lowercase());
        let crate_name = super::get_crate_name(false);

        let SchemaFieldsProperties {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            serialized_field_names_normalised,
            static_assertions,
            referenced_node_schema_import,
            referenced_field_record_link_method,
            connection_with_field_appended,
        }: SchemaFieldsProperties  = SchemaFieldsProperties::from_receiver_data(
            data,
            struct_level_casing,
            struct_name_ident,
        );
        // schema_struct_fields_names_kv.dedup_by(same_bucket)

        let test_name = format_ident!("test_{schema_mod_name}_edge_name");

        let field_names_ident = format_ident!("{struct_name_ident}DbFields");
        let module_name = format_ident!("{}_schema", struct_name_ident.to_string().to_lowercase());
        
        let schema_alias = format_ident!("{}Schema", struct_name_ident.to_string().to_lowercase());
        
        tokens.extend(quote!( 
                        
                impl<In: #crate_name::SurrealdbNode, Out: #crate_name::SurrealdbNode> #crate_name::SurrealdbEdge for #struct_name_ident<In, Out> {
                    type In = In;
                    type Out = Out;
                    type TableNameChecker = #module_name::TableNameStaticChecker;

                    // type Schema = writes_schema::Writes;
                    //
                    // fn get_schema() -> Self::Schema {
                    //     todo!()
                    // }
                    // fn get_key(&self) -> ::std::option::Option<String> {self.id.as_ref().map(::std::string::String::clone) } 
                }

                use #module_name::#struct_name_ident as #schema_alias;
                pub mod #module_name {
                    
                    pub struct TableNameStaticChecker {
                        #struct_name_ident: String,
                    }

                    // use super::{
                    //     blog_schema::Blog, book_schema::Book, format_clause, student_schema::Student, Clause,
                    //     DbField, EdgeDirection,
                    // };

                    use #crate_name::{DbField, EdgeDirection, format_clause};
                    // type Book = <super::Book as SurrealdbNode>::Schema;
                    // type Student = <super::Student as SurrealdbNode>::Schema;
                    #( #referenced_node_schema_import); *

                    #[derive(Debug, Default)]
                        pub struct #struct_name_ident<Model: ::serde::Serialize + Default> {
                        // Even though it's possible to have full object when in and out are loaded,
                        // in practise, we almost never want to do this, since edges are rarely
                        // accessed directly but only via nodes and they are more like bridges
                        // between two nodes. So, we make that trade-off of only allowing DbField
                        // - which is just a surrealdb id , for both in and out nodes.
                        // Still, we can get access to in and out nodes via the origin and destination nodes
                        // e.g User->Eats->Food. We can get User and Food without accessing Eats directly.
                       #( #schema_struct_fields_types_kv), *
                        pub __________store: String,
                        ___________model: ::std::marker::PhantomData<Model>,
                        // ___________outer: PhantomData<Out>,
                    }

                    impl<Model: ::serde::Serialize + Default> #struct_name_ident<Model> {
                        pub fn new() -> Self {
                            Self {
                               #( #schema_struct_fields_names_kv), *
                                __________store: "".into(),
                                ___________model: ::std::marker::PhantomData,
                                // ___________outer: PhantomData,
                            }
                        }

                        pub fn __________update_edge(
                            store: &::std::string::String,
                            clause: #crate_name::Clause,
                            arrow_direction: #crate_name::EdgeDirection,
                        ) -> Self<Model> {
                            let mut schema_instance = Self::<Model>::default();
                            // e.g ExistingConnection->writes[WHERE id = "person:lowo"]->
                            // note: clause could also be empty
                            let current_edge = format!(
                                "{}{arrow_direction}{}{arrow_direction}{}",
                                store.as_str(),
                                #struct_name_ident,
                                format_clause(clause, #struct_name_ident_as_str)
                            );
                            schema_instance.__________store.push_str(current_edge.as_str());

                            let store_without_end_arrow = schema_instance
                                .__________store
                                .trim_end_matches(arrow_direction.to_string().as_str());

                            // schema_instance.time_written
                            //     .push_str(format!("{}.time_written", store_without_end_arrow).as_str());
                            #( #connection_with_field_appended); *
                            
                            schema_instance
                        }
                        
                        // pub fn student(&self, clause: Clause) -> Student {
                        //     Student::__________update_connection(&self.__________store, clause)
                        // }
                            #( #referenced_field_record_link_method) *
                    }
                }
                
            fn #test_name() {
                #( #static_assertions); *

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
