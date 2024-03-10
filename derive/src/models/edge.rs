/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;

use syn::{self, parse_macro_input};

use crate::models::*;

// #[derive(Debug, FromDeriveInput)]
// #[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
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

#[derive(Clone, Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct EdgeToken(pub TableDeriveAttributes);

impl Deref for EdgeToken {
    type Target = TableDeriveAttributes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for EdgeToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_name = get_crate_name(false);
        let table_derive_attributes = self.deref();
        let struct_name_ident = &table_derive_attributes.ident();
        let (impl_generics, ty_generics, where_clause) =
            table_derive_attributes.generics.split_for_impl();
        let struct_marker = table_derive_attributes.generics().phantom_marker_type();
        let table_name_ident = match table_derive_attributes.table_name() {
            Ok(table_name) => table_name,
            Err(err) => return tokens.extend(err.write_errors()),
        };
        let table_name_str = table_name_ident.as_string();
        let VariablesModelMacro {
            __________connect_edge_to_graph_traversal_string,
            ___________graph_traversal_string,
            ___________model,
            _____field_names,
            schema_instance,
            ___________in_marker,
            ___________out_marker,
            ___________bindings,
            ____________update_many_bindings,
            ___________errors,
            _____struct_marker_ident,
            ..
        } = VariablesModelMacro::new();
        let table_definitions = match self.get_table_definition_token() {
            Ok(table_definitions) => table_definitions,
            Err(err) => return tokens.extend(err.write_errors()),
        };

        let table_attrs = ModelAttributes::Edge(self.clone());
        let code_gen = match Codegen::parse_fields(&table_attrs) {
            Ok(props) => props,
            Err(err) => return tokens.extend(err.write_errors()),
        };
        let Codegen {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            schema_struct_fields_names_kv_prefixed,
            field_wrapper_type_custom_implementations,
            serialized_field_names_normalised,
            static_assertions,
            imports_referenced_node_schema,
            connection_with_field_appended,
            record_link_fields_methods,
            schema_struct_fields_names_kv_empty,
            field_definitions,
            serialized_fmt_db_field_names_instance: serializable_fields,
            linked_fields,
            link_one_fields,
            link_self_fields,
            link_one_and_self_fields,
            link_many_fields,
            struct_partial_fields,
            struct_partial_associated_functions,
            renamed_serialized_fields_kv,
            table_id_type,
            field_metadata,
            ..
        } = &code_gen;

        let has_in_and_out_fields = || {
            !serialized_field_names_normalised
                .iter()
                .any(|this| this.to_string() == "in")
                || !serialized_field_names_normalised
                    .iter()
                    .any(|this| this.to_string() == "out")
        };

        if has_in_and_out_fields() {
            tokens.extend(
                ExtractorError::Darling(darling::Error::custom(
                    "Edge struct must include 'in' and 'out'",
                ))
                .write_errors(),
            );
        }
        let imports_referenced_node_schema = Vec::from_iter(imports_referenced_node_schema);
        let CommonIdents {
            module_name_internal,
            module_name_rexported,
            test_function_name,
            struct_with_renamed_serialized_fields,
            _____schema_def,
            ..
        } = code_gen.common_idents();

        let serializable_fields_count = serializable_fields.len();
        let struct_partial_ident = struct_name_ident.partial_ident();
        let struct_partial_builder_ident = struct_name_ident.partial_builder_ident();

        tokens.extend(quote!(
                use #crate_name::{ToRaw as _};

                impl #impl_generics #struct_name_ident #ty_generics #where_clause {
                      // pub const ALLOWED_FIELDS: [&'static str; 2] = ["name", "strength"];
                    pub const fn __get_serializable_field_names() -> [&'static str; #serializable_fields_count] {
                        // this is used for static checking, so, we dont need the fields
                        unimplemented!()
                    }
                }

                impl #impl_generics #crate_name::SchemaGetter for #struct_name_ident #ty_generics #where_clause  {
                    type Schema = #module_name_internal::#struct_name_ident #ty_generics;

                    fn schema() -> #module_name_rexported::Schema {
                        #module_name_rexported::Schema:: #ty_generics ::new()
                    }

                fn schema_prefixed(prefix: impl ::std::convert::Into<#crate_name::ValueLike>) -> #module_name_rexported::Schema {
                        #module_name_rexported::Schema:: #ty_generics ::new_prefixed(prefix)
                    }
                }
            
                impl #impl_generics #crate_name::Updater for #struct_name_ident #ty_generics #where_clause {
                    type PartialBuilder = #struct_partial_builder_ident #ty_generics;

                    fn partial_builder() -> Self::PartialBuilder {
                        #struct_partial_builder_ident::default()
                    }
                }

                #[allow(non_snake_case)]
                impl #impl_generics #crate_name::Edge for #struct_name_ident #ty_generics #where_clause   {
                    type In = In;
                    type Out = Out;
                    type TableNameChecker = #module_name_internal::TableNameStaticChecker;
                    // type Schema = #module_name::#struct_name_ident;

                    // fn schema() -> Self::Schema {
                    //     #module_name::#struct_name_ident::new()
                    // }

                    #[allow(non_snake_case)]
                    fn get_table_name() -> #crate_name::Table {
                        #table_name_str.into()
                    }
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
                #[derive(#crate_name::serde::Serialize,  ebug, Clone)]
                pub struct #struct_with_renamed_serialized_fields {
                   #(
                        #renamed_serialized_fields_kv
                    ) *
                }

                #[allow(non_snake_case)]
                impl #impl_generics #crate_name::Model for #struct_name_ident #ty_generics #where_clause {
                    type Id = #table_id_type;
                    type StructRenamedCreator = #struct_with_renamed_serialized_fields;

                    fn table_name() -> #crate_name::Table {
                        #table_name_str.into()
                    }

                    fn get_id(self) -> Self::Id {
                        self.id
                    }

                    fn get_id_as_thing(&self) -> #crate_name::sql::Thing {
                        #crate_name::sql::thing(self.id.to_raw().as_str()).unwrap()
                    }

                    fn get_serializable_fields() -> ::std::vec::Vec<#crate_name::Field> {
                        return vec![#( #serializable_fields), *]
                    }

                    fn get_linked_fields() -> ::std::vec::Vec<#crate_name::Field> {
                        return vec![#( #linked_fields), *]
                    }

                    fn get_link_one_fields() -> ::std::vec::Vec<#crate_name::Field> {
                        return vec![#( #link_one_fields), *]
                    }

                    fn get_link_self_fields() -> ::std::vec::Vec<#crate_name::Field> {
                        return vec![#( #link_self_fields), *]
                    }

                    fn get_link_one_and_self_fields() -> ::std::vec::Vec<#crate_name::Field> {
                        return vec![#( #link_one_and_self_fields), *]
                    }

                    fn get_link_many_fields() -> ::std::vec::Vec<#crate_name::Field> {
                        return vec![#( #link_many_fields), *]
                    }

                    fn define_table() -> #crate_name::Raw{
                        #table_definitions
                    }

                    fn define_fields() -> ::std::vec::Vec<#crate_name::Raw> {
                        vec![
                           #( #field_definitions), *
                        ]
                    }

                    fn get_field_meta() -> ::std::vec::Vec<#crate_name::FieldMetadata> {
                        return vec![#( #field_metadata), *]
                    }
                }

                #[allow(non_snake_case)]
                pub mod #module_name_rexported {
                    pub use super::#module_name_internal::#_____schema_def::Schema;
                }


                #[allow(non_snake_case)]
                mod #module_name_internal {
                    use #crate_name::Node;
                    use #crate_name::Parametric as _;
                    use #crate_name::Buildable as _;
                    use #crate_name::Erroneous as _;

                    pub struct TableNameStaticChecker {
                        pub #table_name_ident: ::std::string::String,
                    }


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

                    impl #impl_generics #crate_name::Buildable for #struct_name_ident #ty_generics #where_clause {
                        fn build(&self) -> ::std::string::String {
                            self.#___________graph_traversal_string.to_string()
                        }
                    }

                    impl #impl_generics #crate_name::Parametric for #struct_name_ident #ty_generics #where_clause {
                        fn get_bindings(&self) -> #crate_name::BindingsList {
                            self.#___________bindings.to_vec()
                        }
                    }

                    impl #impl_generics #crate_name::Erroneous for #struct_name_ident #ty_generics #where_clause {
                        fn get_errors(&self) -> Vec<::std::string::String> {
                            self.#___________errors.to_vec()
                        }
                    }
        
                    impl #impl_generics #crate_name::Aliasable for #struct_name_ident #ty_generics #where_clause {}

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

                        pub fn #__________connect_edge_to_graph_traversal_string(
                            connection: impl #crate_name::Buildable + #crate_name::Parametric + #crate_name::Erroneous,
                            clause: impl ::std::convert::Into<#crate_name::EdgeClause>,
                        ) -> Self {
                            let mut schema_instance = Self::empty();
                            let clause: #crate_name::EdgeClause = clause.into();
                            let bindings = [connection.get_bindings().as_slice(), clause.get_bindings().as_slice()].concat();
                            let bindings = bindings.as_slice();
                            schema_instance.#___________bindings = bindings.into();

                            let errors = [connection.get_errors().as_slice(), clause.get_errors().as_slice()].concat();
                            let errors = errors.as_slice();
                            schema_instance.#___________errors = errors.into();

                            let schema_edge_str_with_arrow = format!(
                                "{}{}",
                                connection.build(),
                                clause.build(),
                            );

                            #schema_instance.#___________graph_traversal_string.push_str(schema_edge_str_with_arrow.as_str());

                            let #___________graph_traversal_string = &#schema_instance
                                .#___________graph_traversal_string;

                            // let #___________graph_traversal_string = &#schema_instance
                            //     .#___________graph_traversal_string
                            //     .replace(arrow_direction, "");

                            #( #connection_with_field_appended) *

                            #schema_instance
                        }

                        #( #record_link_fields_methods) *
                    }
                }

            #[allow(non_snake_case)]
            fn #test_function_name() {
                #( #static_assertions) *
            }
        ));
    }
}

pub fn generate_fields_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = match EdgeToken::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
