/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::{ast, util, FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use syn::{self, parse_macro_input};

use crate::models::*;

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct ObjectToken {
    pub(crate) ident: Ident,
    #[allow(dead_code)]
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: StructGenerics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: ast::Data<util::Ignored, MyFieldReceiver>,

    #[darling(default)]
    pub(crate) rename_all: ::std::option::Option<Rename>,
}

impl ObjectToken {
    pub fn ident(&self) -> StructIdent {
        self.ident.clone().into()
    }

    pub fn generics(&self) -> &StructGenerics {
        &self.generics
    }
}

impl ToTokens for ObjectToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_name = get_crate_name(false);
        let struct_name_ident = &self.ident();
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();
        let struct_marker = self.generics().phantom_marker_type();
        let VariablesModelMacro {
            __________connect_object_to_graph_traversal_string,
            ___________graph_traversal_string,
            ___________bindings,
            ___________errors,
            _____field_names,
            schema_instance,
            _____struct_marker_ident,
            ..
        } = VariablesModelMacro::new();
        let table_attrs = ModelAttributes::Object(self.clone());
        let code_gen = match Codegen::parse_fields(&table_attrs) {
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
            struct_partial_fields,
            struct_partial_associated_functions,
            ..
        } = &code_gen;

        let imports_referenced_node_schema =
            imports_referenced_node_schema.iter().collect::<Vec<_>>();
        let CommonIdents {
            module_name_internal,
            module_name_rexported,
            test_function_name,
            _____schema_def,
            ..
        } = code_gen.common_idents();
        let struct_partial_ident = struct_name_ident.partial_ident();
        let struct_partial_builder_ident = struct_name_ident.partial_builder_ident();

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
                type Schema = #module_name_internal::#struct_name_ident #ty_generics;

                fn schema() -> #module_name_rexported::Schema #ty_generics #where_clause {
                    #module_name_rexported::Schema:: #ty_generics ::new()
                }

                fn schema_prefixed(prefix: impl ::std::convert::Into<#crate_name::ValueLike>) -> #module_name_rexported::Schema #ty_generics #where_clause {
                    #module_name_rexported::Schema:: #ty_generics ::new_prefixed(prefix)
                }
            }
        
            impl #impl_generics #crate_name::PartialUpdater for #struct_name_ident #ty_generics #where_clause {
                type PartialBuilder = #struct_partial_builder_ident #ty_generics;

                fn partial_builder() -> Self::PartialBuilder {
                    #struct_partial_builder_ident::default()
                }
            }

            impl #impl_generics #crate_name::Object for #struct_name_ident #ty_generics #where_clause {
                // type Schema = #module_name::#struct_name_ident;

                // fn schema() -> Self::Schema {
                //     #module_name::#struct_name_ident::new()
                // }
            }

            #[allow(non_snake_case)]
            #[derive(#crate_name::serde::Serialize, Debug, Clone, Default)]
            pub struct  #struct_partial_ident #impl_generics #where_clause {
               #(
                    #struct_partial_fields
                ) *
            }

            #[derive(#crate_name::serde::Serialize, Debug, Clone, Default)]
            pub struct #struct_partial_builder_ident #impl_generics (#struct_partial_ident #ty_generics) #where_clause;

            impl #impl_generics #struct_partial_builder_ident #ty_generics #where_clause {
                #( #struct_partial_associated_functions) *

                pub fn build(self) -> #struct_partial_ident #ty_generics {
                    self.0
                }
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
                use super::*;

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
                    pub struct Schema #ty_generics #where_clause {
                       #( #schema_struct_fields_types_kv) *
                        pub(super) #___________graph_traversal_string: ::std::string::String,
                        pub(super) #___________bindings: #crate_name::BindingsList,
                        pub(super) #___________errors: ::std::vec::Vec<::std::string::String>,
                        pub(super) #_____struct_marker_ident: #struct_marker,
                    }
                }
                pub type #struct_name_ident #ty_generics = #_____schema_def::Schema #ty_generics;

                impl #impl_generics #crate_name::Parametric for #struct_name_ident #ty_generics #where_clause{
                    fn get_bindings(&self) -> #crate_name::BindingsList {
                        self.#___________bindings.to_vec()
                    }
                }

                impl #impl_generics #crate_name::Buildable for #struct_name_ident #ty_generics #where_clause {
                    fn build(&self) -> ::std::string::String {
                        self.#___________graph_traversal_string.to_string()
                    }
                }

                impl #impl_generics #crate_name::Erroneous for #struct_name_ident #ty_generics #where_clause {
                    fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
                        self.#___________errors.to_vec()
                    }
                }

                impl #impl_generics ::std::fmt::Display for #struct_name_ident #ty_generics #where_clause {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_fmt(format_args!("{}", self.#___________graph_traversal_string))
                    }
                }

                impl #impl_generics #crate_name::Aliasable for &#struct_name_ident #ty_generics #where_clause {}

                impl #impl_generics #crate_name::Parametric for &#struct_name_ident #ty_generics #where_clause {
                    fn get_bindings(&self) -> #crate_name::BindingsList {
                        self.#___________bindings.to_vec()
                    }
                }

                impl #impl_generics #crate_name::Buildable for &#struct_name_ident #ty_generics #where_clause {
                    fn build(&self) -> ::std::string::String {
                        self.#___________graph_traversal_string.to_string()
                    }
                }

                impl #impl_generics #crate_name::Erroneous for &#struct_name_ident #ty_generics #where_clause {
                    fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
                        self.#___________errors.to_vec()
                    }
                }


                impl #impl_generics #struct_name_ident #ty_generics #where_clause {
                    pub fn new() -> Self {
                        Self {
                           #( #schema_struct_fields_names_kv) *
                            #___________graph_traversal_string: "".into(),
                            #___________bindings: vec![],
                            #___________errors: vec![],
                            #_____struct_marker_ident: ::std::marker::PhantomData,
                        }
                    }

                    pub fn new_prefixed(prefix: impl ::std::convert::Into<#crate_name::ValueLike>) -> Self {
                        let prefix: #crate_name::ValueLike = prefix.into();
                        Self {
                           #( #schema_struct_fields_names_kv_prefixed) *
                            #___________graph_traversal_string: prefix.build(),
                            #___________bindings: prefix.get_bindings(),
                            #___________errors: vec![],
                            #_____struct_marker_ident: ::std::marker::PhantomData,
                        }
                    }

                    pub fn empty() -> Self {
                        Self {
                           #( #schema_struct_fields_names_kv_empty) *
                            #___________graph_traversal_string: "".into(),
                            #___________bindings: vec![],
                            #___________errors: vec![],
                            #_____struct_marker_ident: ::std::marker::PhantomData,
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
