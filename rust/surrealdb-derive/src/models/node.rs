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
use std::str::FromStr;

use syn::{self, parse_macro_input, LitStr, Error};

use super::{
    attributes::FieldsGetterOpts,
    casing::CaseString,
    errors,
    parser::{SchemaFieldsProperties, SchemaPropertiesArgs},
    variables::VariablesModelMacro,
};

fn generate_as(lit: &LitStr) -> Result<TokenStream, Error> {
    let str = lit.value();
    let tokens: TokenStream = str.parse().map_err(|err| syn::Error::from(err))?;
    Ok(quote! { (#tokens) })
}

impl ToTokens for FieldsGetterOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldsGetterOpts {
            ident: ref struct_name_ident,
            ref data,
            ref rename_all,
            ref table_name,
            ref relax_table_name,
            ref drop,
            ref schemafull,
            ref as_fn,
            ref permissions,
            ref permissions_fn,
            ref define_fn,
            ..
        } = *self;

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
            __________connect_to_graph_traversal_string,
            ___________graph_traversal_string,
            ___________bindings,
            ___________errors,
            schema_instance,
            ..
        } = VariablesModelMacro::new();
        let schema_props_args = SchemaPropertiesArgs {
            data,
            struct_level_casing,
            struct_name_ident,
            table_name_ident,
        };

        let SchemaFieldsProperties {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            static_assertions,
            mut imports_referenced_node_schema,
            connection_with_field_appended,
            record_link_fields_methods,
            node_edge_metadata,
            schema_struct_fields_names_kv_empty,
            serialized_field_name_no_skip,
            ..
        } = SchemaFieldsProperties::from_receiver_data(schema_props_args);
        let node_edge_metadata_tokens = node_edge_metadata.generate_token_stream();
        // let imports_referenced_node_schema = imports_referenced_node_schema.dedup_by(|a, b| a.to_string() == b.to_string());
        let imports_referenced_node_schema = imports_referenced_node_schema
            .into_iter()
            .collect::<Vec<_>>();

        let node_edge_metadata_static_assertions = node_edge_metadata.generate_static_assertions();

        // imports_referenced_node_schema.dedup_by(|a, b| a.to_string().trim() == b.to_string().trim());

        let test_function_name = format_ident!("test_{schema_mod_name}_edge_name");
        let module_name = format_ident!("{}", struct_name_ident.to_string().to_lowercase());



        // let sele = quote!(#as_fn);
        let sele = if as_fn.is_some(){ generate_as(as_fn.as_ref().unwrap()).unwrap()} else {quote!(43)};
        let asa = if as_fn.is_some(){ generate_as(as_fn.as_ref().unwrap()).unwrap()} else {quote!(43)};
        
        let mut define_table_methods = vec![];
        if let Some(drop) = drop  {
            define_table_methods.push(quote!(.drop()))
                                                
        }
        if let Some(select) = as_fn  {
            let select = generate_as(select).unwrap();
            define_table_methods.push(quote!(.as_select(#select)))
            
        }
        let get_table_def =||{
                     quote!(define_table(user_table)
                        .drop()
                        .as_select(
                            select(All)
                                .from(fake_id2)
                                .where_(country.is("INDONESIA"))
                                .order_by(order(&age).numeric().desc())
                                .limit(20)
                                .start(5),
                        )
                        .schemafull()
                        .permissions_for(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
                        .permissions_for(for_(&[Create, Delete]).where_(name.is("Oyedayo"))) //Multiple
                        .permissions_for(&[
                            for_(&[Create, Delete]).where_(name.is("Oyedayo")),
                            for_(Update).where_(age.less_than_or_equal(130)),
                        ]))
        };



        // #[derive(SurrealdbModel, TypedBuilder, Serialize, Deserialize, Debug, Clone)]
        // #[serde(rename_all = "camelCase")]
        // #[surrealdb(table_name = "student", drop, schemafull, permission_fn, define_fn="any_fnc")]
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
            impl #crate_name::SurrealdbNode for #struct_name_ident {
                type TableNameChecker = #module_name::TableNameStaticChecker;
                type Schema = #module_name::#struct_name_ident;

                fn with(clause: impl Into<#crate_name::Clause>) -> Self::Schema {
                    let clause: #crate_name::Clause = clause.into();
                    
                    #module_name::#struct_name_ident::#__________connect_to_graph_traversal_string(
                                "".into(),
                                clause,
                                // #module_name::#struct_name_ident::new().get_bindings()
                                vec![],
                                vec![],
                    )
                }
                
                fn schema() -> Self::Schema {
                    #module_name::#struct_name_ident::new()
                }
                
                
                fn get_key<T: From<#crate_name::RecordId>>(self) -> ::std::option::Option<T>{
                    let record_id = self.id.map(|id| #crate_name::RecordId::from(id).into());
                    record_id
                }
                
                fn get_table_name() -> ::surrealdb::sql::Table {
                    #table_name_str.into()
                }
                
            }
        impl  #struct_name_ident {

        fn polo ()->String{
            println!("lowo sabi {}", #sele);
            #sele.to_string()
        }
        }

            impl #crate_name::SurrealdbModel for #struct_name_ident {
                fn table_name() -> ::surrealdb::sql::Table {
                    #table_name_str.into()
                }
                
                fn get_serializable_field_names() -> Vec<&'static str> {
                    return vec![#( #serialized_field_name_no_skip), *]
                }
                
                fn define_table() -> #crate_name::statements::DefineTableStatement {
                     #crate_name::statements::define_table(Self::table_name())
                        #( # define_table_methods) *
                        .drop()
                        // .as_select(
                        //     #sele
                        // )
                        .schemafull()
                        // .permissions_for(for_(Select).where_(age.greater_than_or_equal(18))) // Single works
                        // .permissions_for(for_(&[Create, Delete]).where_(name.is("Oyedayo"))) //Multiple
                        // .permissions_for(&[
                        //     for_(&[Create, Delete]).where_(name.is("Oyedayo")),
                        //     for_(Update).where_(age.less_than_or_equal(130)),
                        // ])
                }
                
                fn define_fields() -> Vec<#crate_name::statements::DefineFieldStatement> {
                    todo!()
                }
            }
            
            pub mod #module_name {
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
                
                impl ::std::fmt::Display for #struct_name_ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_fmt(format_args!("{}", self.#___________graph_traversal_string))
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
                    
                    pub fn #__________connect_to_graph_traversal_string(
                        store: ::std::string::String,
                        clause: impl Into<#crate_name::Clause>,
                        existing_bindings: #crate_name::BindingsList,
                        existing_errors: Vec<String>,
                    ) -> Self {
                        let mut #schema_instance = Self::empty(); 
                        let clause: #crate_name::Clause = clause.into();
                        let bindings = [&existing_bindings[..], &clause.get_bindings()[..]].concat();
                        let bindings = bindings.as_slice();

                        schema_instance.#___________bindings = bindings.into();
                        
                        let clause_errors = clause.get_errors(#table_name_str.into());
                        let errors = [&existing_errors[..], &clause_errors[..]].concat();
                        let errors = errors.as_slice();

                        schema_instance.#___________errors = errors.into();
                        
                        
                        let connection = format!("{}{}", store, clause.format_with_model(#table_name_str));

                        #schema_instance.#___________graph_traversal_string.push_str(connection.as_str());
                        let #___________graph_traversal_string = &#schema_instance.#___________graph_traversal_string;
                        
                        #( #connection_with_field_appended) *
                        #schema_instance
                    }
                    
                    #( #record_link_fields_methods) *

                    pub fn __as__<'a, T>(&self, alias: T) -> ::std::string::String
                        where T: Into<::std::borrow::Cow<'a, #crate_name::Field>>
                    {
                        let alias: &#crate_name::Field = &alias.into();
                        format!("{} AS {}", self, alias.to_string())
                    }
                    
                }
                
                #node_edge_metadata_tokens
            }

                
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
    let output = match FieldsGetterOpts::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
