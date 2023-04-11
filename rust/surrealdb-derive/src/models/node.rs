/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

#![allow(dead_code)]

use convert_case::{Case, Casing};
use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{str::FromStr, ops::Deref};

use syn::{self, parse_macro_input, LitStr, Error};

use super::{
    attributes::TableDeriveAttributes,
    casing::CaseString,
    errors,
    parser::{SchemaFieldsProperties, SchemaPropertiesArgs},
    variables::VariablesModelMacro, parse_lit_to_tokenstream,
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surrealdb, serde), forward_attrs(allow, doc, cfg))]
struct NodeToken(TableDeriveAttributes);

impl Deref for NodeToken {
    type Target=TableDeriveAttributes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for NodeToken{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TableDeriveAttributes {
            ident: struct_name_ident,
            data,
            rename_all,
            table_name,
            relax_table_name,
            drop,
            schemafull,
            as_,
            as_fn,
            permissions,
            permissions_fn,
            define,
            define_fn,
            ..
        } = &self.0;

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
            __________connect_node_to_graph_traversal_string,
            ___________graph_traversal_string,
            ___________bindings,
            ___________errors,
            schema_instance,
            ..
        } = VariablesModelMacro::new();
        let schema_props_args = SchemaPropertiesArgs {
            data,
            struct_level_casing,
            struct_name_ident
        };

        let SchemaFieldsProperties {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            aliases_struct_fields_types_kv,
            aliases_struct_fields_names_kv,
            static_assertions,
            mut imports_referenced_node_schema,
            connection_with_field_appended,
            record_link_fields_methods,
            node_edge_metadata,
            schema_struct_fields_names_kv_empty,
            serialized_field_name_no_skip,
            field_definitions,
            ..
        } = SchemaFieldsProperties::from_receiver_data(schema_props_args);
        let node_edge_metadata_tokens = node_edge_metadata.generate_token_stream();
        // let imports_referenced_node_schema = imports_referenced_node_schema.dedup_by(|a, b| a.to_string() == b.to_string());
        let imports_referenced_node_schema = imports_referenced_node_schema
            .into_iter()
            .collect::<Vec<_>>();

        let node_edge_metadata_static_assertions = node_edge_metadata.generate_static_assertions();

        // imports_referenced_node_schema.dedup_by(|a, b| a.to_string().trim() == b.to_string().trim());

        let module_name = format_ident!("{}", struct_name_ident.to_string().to_lowercase());
        let aliases_struct_name = format_ident!("{}Aliases", struct_name_ident);
        let test_function_name = format_ident!("test_{module_name}_edge_name");

        
        let table_definitions = self.get_table_definition_token();

        // #[derive(SurrealdbModel, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
        // #[serde(rename_all = "camelCase")]
        // #[surrealdb(table_name = "student", drop, schemafull, permission, define="any_fnc")]
        // pub struct Student {
        //     #[serde(skip_serializing_if = "Option::is_none")]
        //     #[builder(default, setter(strip_option))]
        //     id: Option<String>,
        //     first_name: String,
        //
        //     #[surrealdb(link_one = "Book", skip_serializing)]
        //     course: LinkOne<Book>,
        //
        //     #[surrealdb(link_many = "Book", skip_serializing)]
        //     #[serde(rename = "lowo")]
        //     all_semester_courses: LinkMany<Book>,
        //
        //     #[surrealdb(relate(model = "StudentWritesBlog", connection = "->writes->Blog"))]
        //     written_blogs: Relate<Blog>,
        // }
        tokens.extend(quote!( 
            use #crate_name::{ToRaw as _};
            
            impl #crate_name::SurrealdbNode for #struct_name_ident {
                type TableNameChecker = #module_name::TableNameStaticChecker;
                type Schema = #module_name::#struct_name_ident;
                type Aliases = #module_name::#aliases_struct_name;

                fn with(clause: impl Into<#crate_name::NodeClause>) -> Self::Schema {
                    let clause: #crate_name::NodeClause = clause.into();
                    
                    #module_name::#struct_name_ident::#__________connect_node_to_graph_traversal_string(
                                // "".into(),
                                #module_name::#struct_name_ident::empty(),
                                clause.with_table(#table_name_str),
                                // true,
                                // #module_name::#struct_name_ident::new().get_bindings()
                                // vec![],
                                // vec![],
                    )
                }
                
                fn schema() -> Self::Schema {
                    #module_name::#struct_name_ident::new()
                }
                
                fn aliases() -> Self::Aliases {
                    #module_name::#aliases_struct_name::new()
                }
                
                fn get_key<T: From<#crate_name::RecordId>>(self) -> ::std::option::Option<T>{
                    let record_id = self.id.map(|id| #crate_name::RecordId::from(id).into());
                    record_id
                }
                
                fn get_table_name() -> #crate_name::Table {
                    #table_name_str.into()
                }
                
            }

            impl #crate_name::SurrealdbModel for #struct_name_ident {
                fn table_name() -> #crate_name::Table {
                    #table_name_str.into()
                }
                
                fn get_serializable_field_names() -> Vec<&'static str> {
                    return vec![#( #serialized_field_name_no_skip), *]
                }
                
                fn define_table() -> #crate_name::Raw {
                    #table_definitions
                }
                
                fn define_fields() -> Vec<#crate_name::Raw> {
                    vec![
                   #( #field_definitions), *
                    ]
                }
            }
            
            pub mod #module_name {
                use #crate_name::Parametric as _;
                use #crate_name::Buildable as _;
                use #crate_name::Erroneous as _;
                use #crate_name::Schemaful as _;

                pub struct TableNameStaticChecker {
                    pub #table_name_ident: String,
                }
                
               #( #imports_referenced_node_schema) *
                

                #[derive(Debug, Clone)]
                pub struct #struct_name_ident {
                   #( #schema_struct_fields_types_kv) *
                    #___________graph_traversal_string: ::std::string::String,
                    #___________bindings: #crate_name::BindingsList,
                    #___________errors: Vec<String>,
                }

                #[derive(Debug, Clone)]
                pub struct #aliases_struct_name {
                   #( #aliases_struct_fields_types_kv) *
                }
                
                impl #aliases_struct_name {
                    pub fn new() -> Self {
                        Self {
                           #( #aliases_struct_fields_names_kv) *
                        }
                    }
                }

                impl #crate_name::Aliasable for #struct_name_ident {}
            
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
            
                impl #crate_name::Buildable for #struct_name_ident {
                    fn build(&self) -> ::std::string::String {
                        self.#___________graph_traversal_string.to_string()
                    }
                }
                
                impl #crate_name::Erroneous for #struct_name_ident {
                    fn get_errors(&self) -> Vec<String> {
                        self.#___________errors.to_vec()
                    }
                }
                
                impl ::std::fmt::Display for #struct_name_ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_fmt(format_args!("{}", self.#___________graph_traversal_string))
                    }
                }
            
                impl #crate_name::Aliasable for &#struct_name_ident {}
            
                impl #crate_name::Schemaful for &#struct_name_ident {
                    fn get_connection(&self) -> String {
                        self.#___________graph_traversal_string.to_string()
                    }
                }
                
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
                    
                    pub fn #__________connect_node_to_graph_traversal_string(
                        connection: impl #crate_name::Buildable + #crate_name::Parametric + #crate_name::Erroneous,
                        clause: impl Into<#crate_name::NodeClause>,
                    ) -> Self {
                        let mut #schema_instance = Self::empty(); 
                        let clause: #crate_name::NodeClause = clause.into();
                        let bindings = [connection.get_bindings().as_slice(), clause.get_bindings().as_slice()].concat();
                        let bindings = bindings.as_slice();

                        schema_instance.#___________bindings = bindings.into();
                        
                        let errors = [connection.get_errors().as_slice(), clause.get_errors().as_slice()].concat();
                        let errors = errors.as_slice();

                        schema_instance.#___________errors = errors.into();
                        
                        let connection_str = format!("{}{}", connection.build(), clause.build());
                        #schema_instance.#___________graph_traversal_string.push_str(connection_str.as_str());
                        let #___________graph_traversal_string = &#schema_instance.#___________graph_traversal_string;
                        
                        #( #connection_with_field_appended) *
                        #schema_instance
                    }
                    
                    #( #record_link_fields_methods) *

                }
                
                #node_edge_metadata_tokens
            }

                
            #[test]
            fn #test_function_name() {
                #( #static_assertions) *
                #node_edge_metadata_static_assertions
                
            }
));
    }
}

pub fn generate_fields_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input);
    // let output = FieldsGetterOpts::from_derive_input(&input).expect("Wrong options");
    let output = match NodeToken::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
