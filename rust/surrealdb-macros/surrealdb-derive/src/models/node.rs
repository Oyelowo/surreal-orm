#![allow(dead_code)]

use convert_case::{Casing, Case};
use darling::{ToTokens, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::str::FromStr;

use syn::{self, parse_macro_input};

use super::{
    casing::CaseString,
    parser::{ SchemaFieldsProperties, SchemaPropertiesArgs}, attributes::FieldsGetterOpts,
    variables::VariablesModelMacro
};



/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/



impl ToTokens for FieldsGetterOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldsGetterOpts {
            ident: ref struct_name_ident,
            ref data,
            ref rename_all,
            ref table_name,
            ref strict,
            ..
        } = *self;

        // let table_name = table_name;
        let expected_table_name = struct_name_ident.to_string().to_case(Case::Snake);
        let ref table_name_ident = format_ident!("{}", table_name.as_ref().unwrap());
        if table_name.as_ref().unwrap() != &expected_table_name {panic!("E don happen");}
    
        let struct_level_casing = rename_all.as_ref().map(|case| {
            CaseString::from_str(case.serialize.as_str()).expect("Invalid casing, The options are")
        });
        
        let binding = struct_name_ident.to_string();
        let struct_name_ident_as_str = binding.as_str();
        let schema_mod_name = format_ident!("{}", struct_name_ident.to_string().to_lowercase());
        let crate_name = super::get_crate_name(false);

        let  VariablesModelMacro { 
            __________connect_to_graph_traversal_string, 
            ___________graph_traversal_string,
            schema_instance, .. 
        } = VariablesModelMacro::new();
        let schema_props_args = SchemaPropertiesArgs{  data, struct_level_casing, struct_name_ident, table_name_ident};

        let SchemaFieldsProperties {
                relate_edge_schema_struct_type_alias,
                relate_edge_schema_struct_type_alias_impl,
                relate_edge_schema_method_connection,
                relate_node_alias_method,
                schema_struct_fields_types_kv,
                schema_struct_fields_names_kv,
                serialized_field_names_normalised,
                static_assertions,
                mut imports_referenced_node_schema,
                connection_with_field_appended,
                record_link_fields_methods,
            node_edge_metadata,
        } = SchemaFieldsProperties::from_receiver_data(
            schema_props_args,
        );
       let node_edge_metadata = node_edge_metadata.generate_token_stream() ; 
        imports_referenced_node_schema.dedup_by(|a,
                                                b| a.to_string() == b.to_string());
        // schema_struct_fields_names_kv.dedup_by(same_bucket)

        let test_name = format_ident!("test_{schema_mod_name}_edge_name");

        let field_names_ident = format_ident!("{struct_name_ident}DbFields");
        let module_name = format_ident!("{}", struct_name_ident.to_string().to_lowercase());
        // let module_name = format_ident!("{}_schema", struct_name_ident.to_string().to_lowercase());
        
        let schema_alias = format_ident!("{}Schema", struct_name_ident.to_string().to_lowercase());
        
            // #[derive(SurrealdbModel, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
            // #[serde(rename_all = "camelCase")]
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
            //     #[surrealdb(relate(edge = "StudentWritesBlog", link = "->writes->Blog"))]
            //     written_blogs: Relate<Blog>,
            // }
        tokens.extend(quote!( 
            impl #crate_name::SurrealdbNode for #struct_name_ident {
                type TableNameChecker = #module_name::TableNameStaticChecker;
                type Schema = #module_name::#struct_name_ident;

                fn schema() -> Self::Schema {
                    #module_name::#struct_name_ident::new()
                }
                
                fn get_key(&self) -> ::std::option::Option<&String>{
                    self.id.as_ref()
                }
            }

            pub mod #module_name {
                use ::serde::Serialize;

                pub struct TableNameStaticChecker {
                    pub #table_name_ident: String,
                }
                
               #( #imports_referenced_node_schema) *
                

                #[derive(Debug, Serialize, Default)]
                pub struct #struct_name_ident {
                   #( #schema_struct_fields_types_kv) *
                    pub(crate) #___________graph_traversal_string: ::std::string::String,
                }

                
                impl ::std::fmt::Display for #struct_name_ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_fmt(format_args!("{}", self.#___________graph_traversal_string))
                    }
                }

                impl #struct_name_ident {
                    pub fn new() -> Self {
                        Self {
                           #( #schema_struct_fields_names_kv) *
                            #___________graph_traversal_string: "".to_string(),
                        }
                    }

                    pub fn __with_id__(mut self, id: impl ::std::fmt::Display) -> Self {
                        self.#___________graph_traversal_string.push_str(id.to_string().as_str());
                        self
                    }
                    
                    pub fn __with__(db_name: impl ::std::fmt::Display) -> Self {
                        let mut #schema_instance = Self::new();
                        #schema_instance
                            .#___________graph_traversal_string
                            .push_str(db_name.to_string().as_str());
                        #schema_instance
                    }

                    pub fn #__________connect_to_graph_traversal_string(store: &::std::string::String, clause: #crate_name::Clause) -> Self {
                        let mut #schema_instance = Self::default();
                        let connection = format!("{}{}{}", store, #struct_name_ident_as_str, #crate_name::format_clause(clause, #struct_name_ident_as_str));

                        #schema_instance.#___________graph_traversal_string.push_str(connection.as_str());
                        let #___________graph_traversal_string = &#schema_instance.#___________graph_traversal_string;
                        
                        #( #connection_with_field_appended) *
                        #schema_instance
                    }

                    #( #relate_edge_schema_method_connection) *
                    
                    #( #record_link_fields_methods) *

                    
                    pub fn __as__(&self, alias: impl ::std::fmt::Display) -> ::std::string::String {
                        format!("{} AS {}", self, alias)
                    }
                    
                    #( #relate_node_alias_method) *
                }
                
                #( #relate_edge_schema_struct_type_alias) *

                #( #relate_edge_schema_struct_type_alias_impl) *
                
                #node_edge_metadata
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
    let output = match FieldsGetterOpts::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
