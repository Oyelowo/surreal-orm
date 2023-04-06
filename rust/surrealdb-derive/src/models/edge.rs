/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(dead_code)]

use convert_case::{Case, Casing};
use darling::{ast, util, FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{str::FromStr, ops::Deref};

use syn::{self, parse_macro_input};

use super::{
    attributes::{MyFieldReceiver, Rename, TableDeriveAttributes},
    casing::CaseString,
    errors,
    parser::{SchemaFieldsProperties, SchemaPropertiesArgs},
    variables::VariablesModelMacro,
};

// #[derive(Debug, FromDeriveInput)]
// #[darling(attributes(surrealdb, serde), forward_attrs(allow, doc, cfg))]
// pub struct FieldsGetterOpts {
//     ident: syn::Ident,
//     attrs: Vec<syn::Attribute>,
//     generics: syn::Generics,
//     /// Receives the body of the struct or enum. We don't care about
//     /// struct fields because we previously told darling we only accept structs.
//     data: ast::Data<util::Ignored, MyFieldReceiver>,
//
//     #[darling(default)]
//     rename_all: ::std::option::Option<Rename>,
//
//     #[darling(default)]
//     pub(crate) table_name: ::std::option::Option<String>,
//
//     #[darling(default)]
//     pub(crate) relax_table_name: ::std::option::Option<bool>,
// }

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surrealdb, serde), forward_attrs(allow, doc, cfg))]
struct EdgeToken(TableDeriveAttributes);

impl Deref for EdgeToken {
    type Target=TableDeriveAttributes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for EdgeToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TableDeriveAttributes {
            ident: struct_name_ident,
            table_name,
            data,
            rename_all,
            relax_table_name,
            ..
        } = &self.0;
        let table_definitions = self.get_table_definition_token();

        let expected_table_name = struct_name_ident.to_string().to_case(Case::Snake);
        let ref table_name_ident = format_ident!("{}", table_name.as_ref().unwrap());
        let table_name_str =
            errors::validate_table_name(struct_name_ident, table_name, relax_table_name).as_str();

        let struct_level_casing = rename_all.as_ref().map(|case| {
            CaseString::from_str(case.serialize.as_str()).expect("Invalid casing, The options are")
        });

        let binding = struct_name_ident.to_string();
        let struct_name_ident_as_str = binding.as_str();
        let schema_mod_name = format_ident!("{}", struct_name_ident.to_string().to_lowercase());
        let crate_name = super::get_crate_name(false);

        let VariablesModelMacro {
            __________connect_edge_to_graph_traversal_string,
            ___________graph_traversal_string,
            ___________model,
            schema_instance_edge_arrow_trimmed,
            schema_instance,
            ___________in_marker,
            ___________out_marker,
            ___________bindings,
            ____________update_many_bindings,
            bindings,
            ___________errors,
        ..
        } = VariablesModelMacro::new();
        let schema_props_args = SchemaPropertiesArgs {
            data,
            struct_level_casing,
            struct_name_ident,
            // table_name_ident,
        };

        let SchemaFieldsProperties {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            serialized_field_names_normalised,
            static_assertions,
            mut imports_referenced_node_schema,
            connection_with_field_appended,
            record_link_fields_methods,
            schema_struct_fields_names_kv_empty,
            serialized_field_name_no_skip,
            field_definitions,
            ..
        } = SchemaFieldsProperties::from_receiver_data(schema_props_args);
        // if serialized_field_names_normalised.conta("")
        if !serialized_field_names_normalised.contains(&"in".into())
            || !serialized_field_names_normalised.contains(&"out".into())
        {
            panic!("Vector does not contain both 'in' and 'out'");
        }
        let imports_referenced_node_schema = Vec::from_iter(imports_referenced_node_schema);
        // imports_referenced_node_schema.dedup_by(|a,
        //                                         b| a.to_string() == b.to_string());
        // schema_struct_fields_names_kv.dedup_by(same_bucket)

        let test_name = format_ident!("test_{schema_mod_name}_edge_name");

        // let field_names_ident = format_ident!("{struct_name_ident}Fields");
        let module_name = format_ident!("{}_schema", struct_name_ident.to_string().to_lowercase());
        

        tokens.extend(quote!( 
                use #crate_name::{ToRaw as _, Raw};
                        
                impl<In: #crate_name::SurrealdbNode, Out: #crate_name::SurrealdbNode> #crate_name::SurrealdbEdge for #struct_name_ident<In, Out> {
                    type In = In;
                    type Out = Out;
                    type TableNameChecker = #module_name::TableNameStaticChecker;
                    type Schema = #module_name::#struct_name_ident;

                    fn schema() -> Self::Schema {
                        #module_name::#struct_name_ident::new()
                    }
                    
                    fn get_key<T: From<#crate_name::RecordId>>(self) -> ::std::option::Option<T>{
                        let record_id = self.id.map(|id| #crate_name::RecordId::from(id).into());
                        record_id
                    }
                    
                fn get_table_name() -> #crate_name::Table {
                        #table_name_str.into()
                    }
                }
        
                impl<In: #crate_name::SurrealdbNode, Out: #crate_name::SurrealdbNode> #crate_name::SurrealdbModel for #struct_name_ident<In, Out> {
                    fn table_name() -> #crate_name::Table {
                        #table_name_str.into()
                    }
                    
                    fn get_serializable_field_names() -> Vec<&'static str> {
                        return vec![#( #serialized_field_name_no_skip), *]
                    }
                    
                    fn define_table() -> #crate_name::Raw{
                        #table_definitions
                    }
                    
                    fn define_fields() -> Vec<#crate_name::Raw> {
                        vec![
                       #( #field_definitions), *
                        ]
                    }
                }
                
                pub mod #module_name {
                    use #crate_name::SurrealdbNode;
                    use #crate_name::Parametric as _;
                    use #crate_name::Erroneous as _;
                    use #crate_name::Schemaful as _;
                    
                    pub struct TableNameStaticChecker {
                        pub #table_name_ident: String,
                    }

                
                    #( #imports_referenced_node_schema) *

                    #[derive(Debug)]
                    pub struct #struct_name_ident {
                       #( #schema_struct_fields_types_kv) *
                        #___________graph_traversal_string: ::std::string::String,
                        #___________bindings: #crate_name::BindingsList,
                        #___________errors: Vec<String>,
                    }
                    
                    impl #crate_name::Schemaful for #struct_name_ident {
                        fn get_connection(&self) -> String {
                            self.#___________graph_traversal_string.to_string()
                        }
                    }

                    impl #crate_name::Parametric for #struct_name_ident {
                        fn get_bindings(&self) -> #crate_name::BindingsList {
                            self.#___________bindings.to_vec()
                        }
                    }
                    
                    impl #crate_name::Erroneous for #struct_name_ident {
                        fn get_errors(&self) -> Vec<String> {
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

                        pub fn empty() -> Self {
                            Self {
                               #( #schema_struct_fields_names_kv_empty) *
                                #___________graph_traversal_string: "".into(),
                                #___________bindings: vec![],
                                #___________errors: vec![],
                            }
                        }
                        
                        pub fn #__________connect_edge_to_graph_traversal_string(
                            store: ::std::string::String,
                            clause: impl Into<#crate_name::Clause>,
                            arrow_direction: &str,
                            destination_table_name: ::std::string::String,
                            existing_bindings: #crate_name::BindingsList,
                            existing_errors: Vec<String>,
                        ) -> Self {
                            let mut schema_instance = Self::empty();
                            let clause: #crate_name::Clause = clause.into();
                            let bindings = [&existing_bindings[..], &clause.get_bindings()[..]].concat();
                            let bindings = bindings.as_slice();
                            schema_instance.#___________bindings = bindings.into();
                            
                            let clause_errors = clause.get_errors(#table_name_str.into());
                            let errors = [&existing_errors[..], &clause_errors[..]].concat();
                            let errors = errors.as_slice();
                            schema_instance.#___________errors = errors.into();
                            let origin_table_name = #table_name_str.to_string();
                        
                            let schema_edge_str_with_arrow = format!(
                                "{}{}{}{}{}{}",
                                store.as_str(),
                                arrow_direction,
                                origin_table_name,
                                clause.format_with_model(#table_name_str),
                                arrow_direction,
                                "",
                                // destination_table_name,
                            );
                            
                            #schema_instance.#___________graph_traversal_string.push_str(schema_edge_str_with_arrow.as_str());

                            let #___________graph_traversal_string = &#schema_instance
                                .#___________graph_traversal_string
                                .replace(arrow_direction, "");

                            #( #connection_with_field_appended) *
                            
                            #schema_instance
                        }
                        
                        #( #record_link_fields_methods) *
                    }
                }
                
            fn #test_name() {
                #( #static_assertions) *
            }
        ));
    }
}

pub fn generate_fields_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input);
    // let output = FieldsGetterOpts::from_derive_input(&input).expect("Wrong options");
    let output = match EdgeToken::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
