/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
* Licensed under the MIT license
 */

use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote};
use std::{ops::Deref, str::FromStr};

use convert_case::{Case, Casing};
use syn::{self, parse_macro_input};

use super::{
    casing::CaseString,
    derive_attributes::{ModelAttributes, TableDeriveAttributes},
    errors,
    token_codegen::{Codegen, CommonIdents},
    variables::VariablesModelMacro,
    DataType, FieldsMeta,
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
struct NodeToken(TableDeriveAttributes);

impl ModelAttributes for NodeToken {
    fn rename_all(&self) -> Option<super::Rename> {
        self.0.rename_all.clone()
    }

    fn ident(&self) -> super::derive_attributes::StructIdent {
        self.0.ident.clone()
    }

    fn generics(&self) -> &super::StructGenerics {
        &self.0.generics
    }
}

impl Deref for NodeToken {
    type Target = TableDeriveAttributes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for NodeToken {
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
            __________connect_node_to_graph_traversal_string,
            ___________graph_traversal_string,
            ___________bindings,
            ___________errors,
            _____field_names,
            schema_instance,
            ..
        } = VariablesModelMacro::new();
        let code_gen = match Codegen::parse_fields(self, DataType::Node) {
            Ok(props) => props,
            Err(err) => return tokens.extend(err.write_errors()),
        };

        let Codegen {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            schema_struct_fields_names_kv_prefixed,
            aliases_struct_fields_types_kv,
            aliases_struct_fields_names_kv,
            field_wrapper_type_custom_implementations,
            static_assertions,
            imports_referenced_node_schema,
            connection_with_field_appended,
            record_link_fields_methods,
            node_edge_metadata,
            schema_struct_fields_names_kv_empty,
            serialized_fmt_db_field_names_instance: serializable_fields,
            linked_fields,
            link_one_fields,
            link_self_fields,
            link_one_and_self_fields,
            link_many_fields,
            field_definitions,
            fields_relations_aliased,
            non_null_updater_fields,
            renamed_serialized_fields,
            table_id_type,
            field_metadata,
            ..
        } = code_gen;

        let imports_referenced_node_schema = imports_referenced_node_schema
            .into_iter()
            .collect::<Vec<_>>();

        let CommonIdents {
            module_name_internal,
            module_name_rexported,
            aliases_struct_name,
            test_function_name,
            non_null_updater_struct_name,
            struct_with_renamed_serialized_fields,
            _____schema_def,
        } = code_gen.common_idents();

        let serializable_fields_count = serializable_fields.len();
        let serializable_fields_as_str = serializable_fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        let table_definitions = match self.get_table_definition_token() {
            Ok(table_definitions) => table_definitions,
            Err(err) => return tokens.extend(err.write_errors()),
        };

        // #[derive(#crate::Model, #crate_name::serde::Serialize, #crate_name::serde::Deserialize, Debug, Clone)]
        // #[serde(rename_all = "camelCase")]
        // #[surreal_orm(table_name = "student", drop, schemafull, permission, define="any_fnc")]
        // pub struct Student {
        //     #[serde(skip_serializing_if = "Option::is_none")]
        //     #[builder(default, setter(strip_option))]
        //     id: Option<String>,
        //     first_name: String,
        //
        //     #[surreal_orm(link_one = "Book", skip_serializing)]
        //     course: LinkOne<Book>,
        //
        //     #[surreal_orm(link_many = "Book", skip_serializing)]
        //     #[serde(rename = "lowo")]
        //     all_semester_courses: LinkMany<Book>,
        //
        //     #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->Blog"))]
        //     written_blogs: Relate<Blog>,
        // }
        tokens.extend(quote!(
            use #crate_name::{ToRaw as _};
            use #crate_name::Aliasable as _;

            impl #struct_impl_generics #crate_name::SchemaGetter for #struct_name_ident #struct_ty_generics #struct_where_clause {
                type Schema = #module_name_rexported::Schema;

                fn schema() -> #module_name_rexported::Schema {
                    #module_name_rexported::Schema::new()
                }

                fn schema_prefixed(prefix: impl ::std::convert::Into<#crate_name::ValueLike>) -> #module_name_rexported::Schema {
                    #module_name_rexported::Schema::new_prefixed(prefix)
                }
            }

            impl #struct_impl_generics #crate_name::Node for #struct_name_ident #struct_ty_generics #struct_where_clause {
                type TableNameChecker = #module_name_internal::TableNameStaticChecker;
                // type Schema = #module_name::#struct_name_ident;
                type Aliases = #module_name_internal::#aliases_struct_name;
                type NonNullUpdater = #non_null_updater_struct_name #struct_ty_generics;

                fn with(clause: impl ::std::convert::Into<#crate_name::NodeClause>) -> <Self as #crate_name::SchemaGetter>::Schema {
                    let clause: #crate_name::NodeClause = clause.into();

                    #module_name_internal::#struct_name_ident::#__________connect_node_to_graph_traversal_string(
                                #module_name_internal::#struct_name_ident::empty(),
                                clause.with_table(#table_name_str),
                    )
                }
                //
                // fn schema() -> Self::Schema {
                //     #module_name::#struct_name_ident::new()
                // }
                //
                // fn schema_prefixed(prefix: String) -> Self::Schema {
                //     #module_name::#struct_name_ident::new_prefixed(prefix)
                // }

                fn aliases() -> Self::Aliases {
                    #module_name_internal::#aliases_struct_name::new()
                }


                fn get_table_name() -> #crate_name::Table {
                    #table_name_str.into()
                }

                fn get_fields_relations_aliased() -> Vec<#crate_name::Alias> {
                    vec![
                       #( #fields_relations_aliased), *
                    ]
                }

            }

            #[allow(non_snake_case)]
            #[derive(#crate_name::serde::Serialize, #crate_name::serde::Deserialize, Debug, Clone, Default)]
            pub struct  #non_null_updater_struct_name #struct_impl_generics #struct_where_clause {
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

            impl #struct_impl_generics #struct_name_ident #struct_ty_generics #struct_where_clause {
                  // pub const ALLOWED_FIELDS: [&'static str; 2] = ["name", "strength"];

                pub const fn __get_serializable_field_names() -> [&'static str; #serializable_fields_count] {
                    [#( #serializable_fields_as_str), *]
                }
            }

            impl #struct_impl_generics #crate_name::Model for #struct_name_ident #struct_ty_generics #struct_where_clause {
                type Id = #table_id_type;
                type NonNullUpdater = #non_null_updater_struct_name #struct_ty_generics;
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
                    return ::std::vec![#( #serializable_fields), *]
                }

                fn get_linked_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    return ::std::vec![#( #linked_fields), *]
                }

                fn get_link_one_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    return ::std::vec![#( #link_one_fields), *]
                }

                fn get_link_self_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    return ::std::vec![#( #link_self_fields), *]
                }

                fn get_link_one_and_self_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    return ::std::vec![#( #link_one_and_self_fields), *]
                }

                fn get_link_many_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    return ::std::vec![#( #link_many_fields), *]
                }

                fn get_field_meta() -> ::std::vec::Vec<#crate_name::FieldMetadata> {
                    return ::std::vec![#( #field_metadata), *]
                }

                fn define_table() -> #crate_name::Raw {
                    #table_definitions
                }

                fn define_fields() -> ::std::vec::Vec<#crate_name::Raw> {
                    ::std::vec![
                       #( #field_definitions), *
                    ]
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
                    pub struct Schema {
                       #( #schema_struct_fields_types_kv) *
                        pub(super) #___________graph_traversal_string: ::std::string::String,
                        pub(super) #___________bindings: #crate_name::BindingsList,
                        pub(super) #___________errors: ::std::vec::Vec<::std::string::String>,
                    }
                }
                pub type #struct_name_ident = #_____schema_def::Schema;


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

                impl From<#struct_name_ident> for #crate_name::ValueLike {
                    fn from(node: #struct_name_ident) -> Self {
                       Self::new(node)
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
                    fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
                        self.#___________errors.to_vec()
                    }
                }

                impl ::std::fmt::Display for #struct_name_ident {
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

                    pub fn #__________connect_node_to_graph_traversal_string(
                        connection: impl #crate_name::Buildable + #crate_name::Parametric + #crate_name::Erroneous,
                        clause: impl Into<#crate_name::NodeClause>,
                    ) -> Self {
                        let mut #schema_instance = Self::new();
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

                #node_edge_metadata
            }


            // #[test] // Comment out to make compiler tests fail in doctests. 25th August, 2023.
            #[allow(non_snake_case)]
            fn #test_function_name #struct_impl_generics() {
                #( #static_assertions) *
                #node_edge_metadata_static_assertions

            }
));
    }
}

pub fn generate_fields_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = match NodeToken::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
