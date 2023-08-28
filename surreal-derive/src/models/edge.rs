#![allow(dead_code)]

/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use convert_case::{Case, Casing};
use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{ops::Deref, str::FromStr};

use syn::{self, parse_macro_input};

use super::{
    attributes::TableDeriveAttributes,
    casing::CaseString,
    errors,
    parser::{DataType, SchemaFieldsProperties, SchemaPropertiesArgs},
    variables::VariablesModelMacro,
};

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

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
struct EdgeToken(TableDeriveAttributes);

impl Deref for EdgeToken {
    type Target = TableDeriveAttributes;

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

        let table_name_ident = &format_ident!(
            "{}",
            table_name
                .as_ref()
                .expect("table_name attribute must be provided")
        );
        let table_name_str =
            errors::validate_table_name(struct_name_ident, table_name, relax_table_name).as_str();

        let struct_level_casing = rename_all.as_ref().map(|case| {
            CaseString::from_str(case.serialize.as_str()).expect("Invalid casing, The options are")
        });

        let schema_mod_name = format_ident!("{}", struct_name_ident.to_string().to_lowercase());
        let crate_name = super::get_crate_name(false);

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
            ..
        } = VariablesModelMacro::new();
        let schema_props_args = SchemaPropertiesArgs {
            data,
            struct_level_casing,
            struct_name_ident,
            table_name: table_name_str.to_string(), // table_name_ident,
        };

        let SchemaFieldsProperties {
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
            serializable_fields,
            linked_fields,
            link_one_fields,
            link_self_fields,
            link_one_and_self_fields,
            link_many_fields,
            non_null_updater_fields,
            renamed_serialized_fields,
            table_id_type,
            ..
        } = SchemaFieldsProperties::from_receiver_data(schema_props_args, DataType::Edge);
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
        let module_name = format_ident!(
            "________internal_{}_schema",
            struct_name_ident.to_string().to_case(Case::Snake)
        );
        let module_name_rexported =
            format_ident!("{}", struct_name_ident.to_string().to_case(Case::Snake));
        let non_null_updater_struct_name = format_ident!("{}NonNullUpdater", struct_name_ident);
        let struct_with_renamed_serialized_fields =
            format_ident!("{struct_name_ident}RenamedCreator");
        let serializable_fields_count = serializable_fields.len();
        let serializable_fields_as_str = serializable_fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        tokens.extend(quote!(
                use #crate_name::{ToRaw as _};

                impl<In: #crate_name::Node, Out: #crate_name::Node> #struct_name_ident<In, Out> {
                      // pub const ALLOWED_FIELDS: [&'static str; 2] = ["name", "strength"];
                    pub const fn __get_serializable_field_names() -> [&'static str; #serializable_fields_count] {
                        [#( #serializable_fields_as_str), *]
                    }
                }

                impl<In: #crate_name::Node, Out: #crate_name::Node> #crate_name::SchemaGetter for #struct_name_ident<In, Out> {
                    type Schema = #module_name::#struct_name_ident;

                    fn schema() -> Self::Schema {
                        #module_name::#struct_name_ident::new()
                    }

                fn schema_prefixed(prefix: impl ::std::convert::Into<#crate_name::Valuex>) -> Self::Schema {
                        #module_name::#struct_name_ident::new_prefixed(prefix)
                    }
                }

                #[allow(non_snake_case)]
                impl<In: #crate_name::Node, Out: #crate_name::Node> #crate_name::Edge for #struct_name_ident<In, Out> {
                    type In = In;
                    type Out = Out;
                    type TableNameChecker = #module_name::TableNameStaticChecker;
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
                #[derive(#crate_name::serde::Serialize, #crate_name::serde::Deserialize, Debug, Clone, Default)]
                pub struct #non_null_updater_struct_name {
                   #(
                        #[serde(skip_serializing_if = "Option::is_none")]
                        #non_null_updater_fields
                    ) *
                }

                #[allow(non_snake_case)]
                #[derive(#crate_name::serde::Serialize, #crate_name::serde::Deserialize, Debug, Clone)]
                pub struct #struct_with_renamed_serialized_fields {
                   #(
                        #renamed_serialized_fields
                    ) *
                }

                #[allow(non_snake_case)]
                impl<In: #crate_name::Node, Out: #crate_name::Node> #crate_name::Model for #struct_name_ident<In, Out> {
                    type Id = #table_id_type;
                    type NonNullUpdater = #non_null_updater_struct_name;
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
                }

                #[allow(non_snake_case)]
                pub mod #module_name_rexported {
                    use super::#module_name::#_____field_names;

                    #[derive(Debug, Clone)]
                    pub struct Schema {
                       #( #schema_struct_fields_types_kv) *
                        pub(super) #___________graph_traversal_string: ::std::string::String,
                        pub(super) #___________bindings: #crate_name::BindingsList,
                        pub(super) #___________errors: ::std::vec::Vec<::std::string::String>,
                    }
                }


                #[allow(non_snake_case)]
                mod #module_name {
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

                    pub type #struct_name_ident = super::#module_name_rexported::Schema;

                    impl #crate_name::Buildable for #struct_name_ident {
                        fn build(&self) -> ::std::string::String {
                            self.#___________graph_traversal_string.to_string()
                        }
                    }

                    impl #crate_name::Parametric for #struct_name_ident {
                        fn get_bindings(&self) -> #crate_name::BindingsList {
                            self.#___________bindings.to_vec()
                        }
                    }

                    impl #crate_name::Erroneous for #struct_name_ident {
                        fn get_errors(&self) -> Vec<::std::string::String> {
                            self.#___________errors.to_vec()
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

                        pub fn new_prefixed(prefix: impl ::std::convert::Into<#crate_name::Valuex>) -> Self {
                            let prefix: #crate_name::Valuex = prefix.into();
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

                        pub fn #__________connect_edge_to_graph_traversal_string(
                            connection: impl #crate_name::Buildable + #crate_name::Parametric + #crate_name::Erroneous,
                            clause: impl Into<#crate_name::EdgeClause>,
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
