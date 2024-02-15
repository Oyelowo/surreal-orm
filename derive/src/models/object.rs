/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(dead_code)]

use darling::{ast, util, FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::str::FromStr;
use surreal_query_builder::Object;

use convert_case::{Case, Casing};
use syn::{self, parse_macro_input};

use super::{
    attributes::{MyFieldReceiver, Rename},
    casing::CaseString,
    derive_attributes::{ModelAttributes, StructIdent},
    parser::{DataType, FieldsMeta, SchemaPropertiesArgs},
    token_codegen::{Codegen, CommonIdents},
    variables::VariablesModelMacro,
    DataType, MyFieldReceiver, Rename, StructGenerics,
};

// #[derive(Debug, FromDeriveInput)]
// #[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
// struct ObjectToken(TableDeriveAttributes);
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct ObjectToken {
    pub(crate) ident: StructIdent,
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: StructGenerics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: ast::Data<util::Ignored, MyFieldReceiver>,

    #[darling(default)]
    pub(crate) rename_all: ::std::option::Option<Rename>,
}

impl ModelAttributes for ObjectToken {
    fn rename_all(&self) -> Option<super::Rename> {
        self.rename_all.clone()
    }

    fn ident(&self) -> super::derive_attributes::StructIdent {
        self.ident.clone()
    }

    fn generics(&self) -> &super::StructGenerics {
        &self.0.generics
    }
}

impl ToTokens for ObjectToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_name = get_crate_name(false);
        let table_derive_attributes = self.deref();
        let struct_name_ident = &table_derive_attributes.ident;
        let (struct_impl_generics, struct_ty_generics, struct_where_clause) =
            generics.split_for_impl();
        let table_name_ident = match table_derive_attributes.table_name() {
            Ok(table_name) => table_name,
            Err(err) => return tokens.extend(err.write_errors()),
        };
        let table_name_str = table_name_ident.as_string();
        let VariablesModelMacro {
            __________connect_object_to_graph_traversal_string,
            ___________graph_traversal_string,
            ___________bindings,
            ___________errors,
            _____field_names,
            schema_instance,
            ..
        } = VariablesModelMacro::new();
        let code_gen = match Codegen::parse_fields(self, DataType::Object) {
            Ok(props) => props,
            Err(err) => return tokens.extend(err.write_errors()),
        };

        let Codegen {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            schema_struct_fields_names_kv_prefixed,
            field_wrapper_type_custom_implementations,
            static_assertions,
            imports_referenced_node_schema,
            connection_with_field_appended,
            record_link_fields_methods,
            schema_struct_fields_names_kv_empty,
            non_null_updater_fields,
            ..
        } = code_gen;

        let imports_referenced_node_schema = imports_referenced_node_schema
            .into_iter()
            .collect::<Vec<_>>();
        let CommonIdents {
            module_name_internal,
            module_name_rexported,
            test_function_name,
            non_null_updater_struct_name,
            _____schema_def,
            ..
        } = code_gen.common_idents();

        // #[derive(Object, Serialize, Deserialize, Debug, Clone)]
        // #[serde(rename_all = "camelCase")]
        // pub struct Student {
        //     first_name: String,
        //     last_name: String,
        //     age: u8,
        // }
        tokens.extend(quote!(
            use #crate_name::{ToRaw as _};

            impl #impl_generics #crate_name::SchemaGetter for #struct_name_ident #ty_generics #where_clause {
                type Schema = #module_name_internal::#struct_name_ident;

                fn schema() -> #module_name_rexported::Schema {
                    #module_name_rexported::Schema::new()
                }

                fn schema_prefixed(prefix: impl ::std::convert::Into<#crate_name::ValueLike>) -> #module_name_rexported::Schema {
                    #module_name_rexported::Schema::new_prefixed(prefix)
                }
            }

            impl #impl_generics #crate_name::Object for #struct_name_ident #ty_generics #where_clause {
                // type Schema = #module_name::#struct_name_ident;
                // type NonNullUpdater = #module_name::#non_null_updater_struct_name;
                type NonNullUpdater = #non_null_updater_struct_name #ty_generics;

                // fn schema() -> Self::Schema {
                //     #module_name::#struct_name_ident::new()
                // }
            }

            #[allow(non_snake_case)]
            #[derive(#crate_name::serde::Serialize, #crate_name::serde::Deserialize, Debug, Clone, Default)]
            pub struct #non_null_updater_struct_name #impl_generics #where_clause {
               #(
                    #[serde(skip_serializing_if = "Option::is_none")]
                    #non_null_updater_fields
                ) *
            }

            #[allow(non_snake_case)]
            pub mod #module_name_rexported {
                pub use super::#module_name_internal::#_____schema_def::Schema;
            }


            #[allow(non_snake_case)]
            mod #module_name_internal {
                use #crate_name::Parametric as _;
                use #crate_name::Buildable as _;
                use #crate_name::Erroneous as _;

               #( #imports_referenced_node_schema) *

                pub(super) mod #_____field_names {
                    use super::super::*;
                    use #crate_name::Parametric as _;
                    use #crate_name::Buildable as _;

                    #( #field_wrapper_type_custom_implementations) *
                }

                pub mod #_____schema_def {
                    use super::#_____field_names;

                    #[allow(non_snake_case)]
                    #[derive(Debug, Clone)]
                    pub struct Schema {
                       #( #schema_struct_fields_types_kv) *
                        pub(super) #___________graph_traversal_string: ::std::string::String,
                        pub(super) #___________bindings: #crate_name::BindingsList,
                        pub(super) #___________errors: ::std::vec::Vec<::std::string::String>,
                    }
                }
                pub type #struct_name_ident = #_____schema_def::Schema;


                impl #crate_name::Parametric for #struct_name_ident {
                    fn get_bindings(&self) -> #crate_name::BindingsList {
                        self.#___________bindings.to_vec()
                    }
                }

                impl #crate_name::Buildable for #struct_name_ident {
                    fn build(&self) -> ::std::string::String {
                        self.#___________graph_traversal_string.to_string()
                    }
                }

                impl #crate_name::Erroneous for #struct_name_ident {
                    fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
                        self.#___________errors.to_vec()
                    }
                }

                impl ::std::fmt::Display for #struct_name_ident #ty_generics #where_clause {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_fmt(format_args!("{}", self.#___________graph_traversal_string))
                    }
                }

                impl #crate_name::Aliasable for &#struct_name_ident {}

                impl #crate_name::Parametric for &#struct_name_ident {
                    fn get_bindings(&self) -> #crate_name::BindingsList {
                        self.#___________bindings.to_vec()
                    }
                }

                impl #crate_name::Buildable for &#struct_name_ident {
                    fn build(&self) -> ::std::string::String {
                        self.#___________graph_traversal_string.to_string()
                    }
                }

                impl #crate_name::Erroneous for &#struct_name_ident {
                    fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
                        self.#___________errors.to_vec()
                    }
                }


                impl #struct_name_ident {
                    pub fn new() -> Self {
                        Self {
                           #( #schema_struct_fields_names_kv) *
                            #___________graph_traversal_string: "".into(),
                            #___________bindings: vec![],
                            #___________errors: vec![],
                        }
                    }

                    pub fn new_prefixed(prefix: impl ::std::convert::Into<#crate_name::ValueLike>) -> Self {
                        let prefix: #crate_name::ValueLike = prefix.into();
                        Self {
                           #( #schema_struct_fields_names_kv_prefixed) *
                            #___________graph_traversal_string: prefix.build(),
                            #___________bindings: prefix.get_bindings(),
                            #___________errors: vec![],
                        }
                    }

                    pub fn empty() -> Self {
                        Self {
                           #( #schema_struct_fields_names_kv_empty) *
                            #___________graph_traversal_string: "".into(),
                            #___________bindings: vec![],
                            #___________errors: vec![],
                        }
                    }

                    pub fn #__________connect_object_to_graph_traversal_string(
                        // store: ::std::string::String,
                        connection: impl #crate_name::Buildable + #crate_name::Parametric + #crate_name::Erroneous,
                        clause: impl ::std::convert::Into<#crate_name::ObjectClause>,
                        // use_table_name: bool,
                        // existing_bindings: #crate_name::BindingsList,
                        // existing_errors: ::std::vec::Vec<String>,
                    ) -> Self {
                        let mut #schema_instance = Self::empty();
                        let clause: #crate_name::ObjectClause = clause.into();
                        let bindings = [connection.get_bindings().as_slice(), clause.get_bindings().as_slice()].concat();
                        let bindings = bindings.as_slice();

                        schema_instance.#___________bindings = bindings.into();

                        let errors = [connection.get_errors().as_slice(), clause.get_errors().as_slice()].concat();
                        let errors = errors.as_slice();

                        schema_instance.#___________errors = errors.into();


                    // let connection = if use_table_name {
                    //     // format!("{}{}", store, clause.format_with_model(#table_name_str))
                    //     format!("{}{}", store, clause)
                    // }else{
                    //     format!("{}{}", store, clause) 
                    // };

                        // let connection_str = format!("{}{}", store, clause.build());
                        let connection_str = format!("{}{}", connection.build(), clause.build());
                        #schema_instance.#___________graph_traversal_string.push_str(connection_str.as_str());
                        let #___________graph_traversal_string = &#schema_instance.#___________graph_traversal_string;

                        #( #connection_with_field_appended) *
                        #schema_instance
                    }

                    #( #record_link_fields_methods) *

                }
            }


            // #[test]
            #[allow(non_snake_case)]
            fn #test_function_name() {
                #( #static_assertions) *
            }
        ));
    }
}

pub fn generate_fields_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = match ObjectToken::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
