


/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]


use convert_case::{Casing, Case};
use darling::{ast, util, FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::str::FromStr;

use syn::{self, parse_macro_input};

use super::{parser::{SchemaFieldsProperties,  SchemaPropertiesArgs},  casing::CaseString,  attributes::{Rename, MyFieldReceiver}, variables::VariablesModelMacro, errors};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surrealdb, serde), forward_attrs(allow, doc, cfg))]
pub struct FieldsGetterOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<util::Ignored, MyFieldReceiver>,

    #[darling(default)]
    rename_all: ::std::option::Option<Rename>,

    #[darling(default)]
    pub(crate) table_name: ::std::option::Option<String>,

    #[darling(default)]
    pub(crate) relax_table_name: ::std::option::Option<bool>,
}

impl ToTokens for FieldsGetterOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldsGetterOpts {
            ident: ref struct_name_ident,
            ref table_name,
            ref data,
            ref rename_all,
            ref relax_table_name,
            ..
        } = *self;

        let expected_table_name = struct_name_ident.to_string().to_case(Case::Snake);
        let ref table_name_ident = format_ident!("{}", table_name.as_ref().unwrap());
        let table_name_str = errors::validate_table_name(struct_name_ident, table_name, relax_table_name).as_str();
        
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
            ___________model,
            schema_instance_edge_arrow_trimmed,
            schema_instance,
            ___________in_marker,
            ___________out_marker,
            ___________bindings,
            ____________update_many_bindings,
            bindings,  
        } = VariablesModelMacro::new();
        let schema_props_args = SchemaPropertiesArgs {  data, struct_level_casing, struct_name_ident, table_name_ident };

        let SchemaFieldsProperties {
                schema_struct_fields_types_kv,
                schema_struct_fields_names_kv,
                serialized_field_names_normalised,
                static_assertions,
                mut imports_referenced_node_schema,
                connection_with_field_appended,
                record_link_fields_methods,
                schema_struct_fields_names_kv_empty,
                ..
        } = SchemaFieldsProperties::from_receiver_data(
            schema_props_args,
        );
        // if serialized_field_names_normalised.conta("")
            if !serialized_field_names_normalised.contains(&"in".into()) || !serialized_field_names_normalised.contains(&"out".into()) {
               panic!("Vector does not contain both 'in' and 'out'");
            }
        let imports_referenced_node_schema = Vec::from_iter(imports_referenced_node_schema);
        // imports_referenced_node_schema.dedup_by(|a,
        //                                         b| a.to_string() == b.to_string());
        // schema_struct_fields_names_kv.dedup_by(same_bucket)

        let test_name = format_ident!("test_{schema_mod_name}_edge_name");

        // let field_names_ident = format_ident!("{struct_name_ident}DbFields");
        let module_name = format_ident!("{}_schema", struct_name_ident.to_string().to_lowercase());
        
        
        tokens.extend(quote!( 
                        
                impl<In: #crate_name::SurrealdbNode, Out: #crate_name::SurrealdbNode> #crate_name::SurrealdbEdge for #struct_name_ident<In, Out> {
                    type In = In;
                    type Out = Out;
                    type TableNameChecker = #module_name::TableNameStaticChecker;
                    type Schema = #module_name::#struct_name_ident;

                    fn schema() -> Self::Schema {
                        #module_name::#struct_name_ident::new()
                    }
                    
                    fn get_table_name() -> ::surrealdb::sql::Table {
                        #table_name_str.into()
                    }
                
                    
                    fn get_key<T: From<#crate_name::RecordId>>(self) -> ::std::option::Option<T>{
                        let record_id = self.id.map(|id| #crate_name::RecordId::from(id).into());
                        record_id
                    }
                }
                
                pub mod #module_name {
                    use #crate_name::SurrealdbNode;
                    use #crate_name::Parametric;
                    
                    pub struct TableNameStaticChecker {
                        pub #table_name_ident: String,
                    }

                
                    #( #imports_referenced_node_schema) *

                    #[derive(Debug)]
                    pub struct #struct_name_ident {
                       #( #schema_struct_fields_types_kv) *
                        pub #___________graph_traversal_string: ::std::string::String,
                        #___________bindings: #crate_name::BindingsList,
                    }

                    impl #struct_name_ident {
                        pub fn new() -> Self {
                            Self {
                               #( #schema_struct_fields_names_kv) *
                                #___________graph_traversal_string: "".into(),
                                #___________bindings: vec![],
                            }
                        }

                        pub fn empty() -> Self {
                            Self {
                               #( #schema_struct_fields_names_kv_empty) *
                                #___________graph_traversal_string: "".into(),
                                #___________bindings: vec![],
                            }
                        }
                        
                        pub fn #__________connect_to_graph_traversal_string(
                            store: &::std::string::String,
                            filter: impl Into<#crate_name::DbFilter>,
                            arrow_direction: &str,
                            existing_bindings: #crate_name::BindingsList,
                        ) -> Self {
                            let mut schema_instance = Self::empty();
                            let filter: #crate_name::DbFilter = filter.into();
                            let bindings = [&existing_bindings[..], &filter.get_bindings()[..]].concat();
                            let bindings = bindings.as_slice();
                            schema_instance.#___________bindings = bindings.into();
                            
                            let schema_edge_str_with_arrow = format!(
                                "{}{}{}{}{}",
                                store.as_str(),
                                arrow_direction,
                                #table_name_str,
                                #crate_name::format_filter(filter),
                                arrow_direction,
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
    let output = match FieldsGetterOpts::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
