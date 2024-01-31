/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
    ops::Deref, option,
};

use convert_case::{Case, Casing};
use darling::{ast, util, ToTokens};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::models::{
    attributes::FieldGenericsMeta, relations::NodeType, replace_lifetimes_with_underscore,
    replace_self_in_type_str, DestinationNodeTypeOriginal, FieldGenericsMeta, NormalisedFieldMeta,
    ReferencedNodeMeta,
};

use super::{
    attributes::{MyFieldReceiver, NormalisedField, ReferencedNodeMeta, Relate},
    casing::CaseString,
    count_vec_nesting,
    derive_attributes::TableDeriveAttributes,
    errors::ExtractorResult,
    generate_nested_vec_type,
    get_crate_name,
    relations::{EdgeDirection, NodeTypeName, RelateAttribute, RelationType},
    // replace_self_in_id,
    variables::VariablesModelMacro,
    DataType,
    GenericTypeExtractor,
    TokenStreamHashable,
    TypeStripper, FieldSetterImplTokens, DefineFieldStatementToken, LinkFieldTraversalMethodToken, ForeignNodeSchemaImport, StaticAssertionToken, NodeEdgeMetadataStore,
};

#[derive(Default, Clone)]
pub struct FieldsMeta {
    /// list of fields names that are actually serialized and not skipped.
    pub serializable_fields: Vec<TokenStream>,
    /// The name of the all fields that are linked i.e line_one, line_many, or line_self.
    pub linked_fields: Vec<TokenStream>,
    /// The names of link_one fields
    pub link_one_fields: Vec<TokenStream>,
    /// The names of link_self fields
    pub link_self_fields: Vec<TokenStream>,
    /// The names of link_one and link_self fields
    pub link_one_and_self_fields: Vec<TokenStream>,
    /// The names of link_many fields
    pub link_many_fields: Vec<TokenStream>,
    /// Generated example: pub timeWritten: Field,
    /// key(normalized_field_name)-value(Field) e.g pub out: Field, of field name and Field type
    /// to build up struct for generating fields of a Schema of the Edge
    /// The full thing can look like:
    /// ```rust,ignore
    /// mod _______field_module {
    ///     pub struct Id(pub(super) Field);
    ///     pub struct In(pub(super) Field);
    ///     pub struct Out(pub(super) Field);
    ///     pub struct TimeWritten(pub(super) Field);
    /// }
    ///
    /// #[derive(Debug, Default)]
    /// pub struct Writes<Model: ::serde::Serialize + Default> {
    ///     pub id: #_____field_module::Id,
    ///     pub r#in: #_____field_module::In,
    ///     pub out: #_____field_module::Out,
    ///     pub timeWritten: #_____field_module::TimeWritten,
    /// }
    /// ```
    pub schema_struct_fields_types_kv: Vec<TokenStream>,

    pub field_wrapper_type_custom_implementations: Vec<FieldSetterImplTokens>,

    /// Generated example: pub timeWritten: "timeWritten".into(),
    /// This is used to build the actual instance of the model during intialization e,g out:
    /// "out".into()
    /// The full thing can look like and the fields should be in normalized form:
    /// i.e time_written => timeWritten if serde camelizes
    /// ```rust,ignore
    /// Self {
    ///     id: "id".into(),
    ///     r#in: "in".into(),
    ///     out: "out".into(),
    ///     timeWritten: "timeWritten".into(),
    /// }
    /// ```
    pub schema_struct_fields_names_kv: Vec<TokenStream>,
    pub schema_struct_fields_names_kv_prefixed: Vec<TokenStream>,

    /// Used to build up empty string values for all schema fields
    /// Example value: pub timeWritten: "".into(),
    /// Used to build up e.g:
    /// ```rust,ignore
    /// Self {
    ///     id: "".into(),
    ///     r#in: "".into(),
    ///     out: "".into(),
    ///     timeWritten: "".into(),
    /// }
    /// ```
    pub schema_struct_fields_names_kv_empty: Vec<TokenStream>,

    /// Generated example: pub writtenBooks: AliasName,
    /// This is used when you have a relate attribute signaling a graph with e.g node->edge->node
    /// The full thing can look like:
    /// ```rust,ignore
    ///     #[derive(Debug, Default)]
    ///     pub struct Writes<Model: ::serde::Serialize + Default> {
    ///                pub writtenBooks: AliasName,
    ///          }
    /// ```
    pub aliases_struct_fields_types_kv: Vec<TokenStream>,

    /// Generated example: pub writtenBooks: "writtenBooks".into(),
    /// This is used to build the actual instance of the struct with aliases
    /// The full thing can look like and the fields should be in normalized form:
    /// i.e writtenBooks => writtenBooks if serde camelizes
    /// ```rust, ignore
    /// Self {
    ///                pub writtenBooks: AliasName,
    /// }
    /// ```
    pub aliases_struct_fields_names_kv: Vec<TokenStream>,

    /// list of fields names that are actually serialized and not skipped.
    pub serialized_alias_name_no_skip: Vec<String>,

    /// Field names after taking into consideration
    /// serde serialized renaming or casings
    /// i.e time_written => timeWritten if serde camelizes
    pub serialized_field_names_normalised: Vec<String>,

    /// Generated example:
    /// ```rust,ignore
    /// // For relate field
    /// type StudentWritesBlogTableName = <StudentWritesBlog as Edge>::TableNameChecker;
    /// #crate_name::validators::assert_fields!(StudentWritesBlogTableName: Writes);
    ///
    /// type StudentWritesBlogInNode = <StudentWritesBlog as Edge>::In;
    /// #crate_name::validators::assert_type_eq_all!(StudentWritesBlogInNode, Student);
    ///
    /// type StudentWritesBlogOutNode = <StudentWritesBlog as Edge>::Out;
    /// #crate_name::validators::assert_type_eq_all!(StudentWritesBlogOutNode, Blog);
    ///
    ///
    /// #crate_name::validators::assert_impl_one!(StudentWritesBlog: Edge);
    /// #crate_name::validators::assert_impl_one!(Student: Node);
    /// #crate_name::validators::assert_impl_one!(Blog: Node);
    /// #crate_name::validators::assert_type_eq_all!(LinkOne<Book>, LinkOne<Book>);
    /// ```
    /// Perform all necessary static checks
    pub static_assertions: Vec<StaticAssertionToken>,

    /// Generated example:
    /// ```rust,ignore
    /// type Book = <super::Book as Node>::Schema;
    /// ```
    /// We need imports to be unique, hence the hashset
    /// Used when you use a Node in field e.g: favourite_book: LinkOne<Book>,
    /// e.g: type Book = <super::Book as Node>::Schema;
    pub imports_referenced_node_schema: HashSet<ForeignNodeSchemaImport>,

    /// This generates a function that is usually called by other Nodes/Structs
    /// self_instance.drunk_water
    /// .push_str(format!("{}.drunk_water", xx.___________graph_traversal_string).as_str());
    ///
    /// so that we can do e.g
    /// ```rust,ignore
    /// Student.field_name
    /// ```
    pub connection_with_field_appended: Vec<TokenStream>,

    /// When a field references another model as Link, we want to generate a method for that
    /// to be able to access the foreign fields
    /// Generated Example for e.g field with best_student: <Student>
    /// ```rust, ignore
    /// pub fn best_student(&self, filter: Filter) -> Student {
    ///     Student::__________connect_to_graph_traversal_string(&self.___________graph_traversal_string, filter)
    /// }
    /// ```
    pub record_link_fields_methods: Vec<LinkFieldTraversalMethodToken>,
    pub field_definitions: Vec<Vec<DefineFieldStatementToken>>,
    pub field_metadata: Vec<TokenStream>,
    pub node_edge_metadata: NodeEdgeMetadataStore,
    pub fields_relations_aliased: Vec<TokenStream>,
    pub non_null_updater_fields: Vec<TokenStream>,
    pub renamed_serialized_fields: Vec<TokenStream>,
    pub table_id_type: TokenStream,

    pub table_derive_attributes: Option< TableDeriveAttributes >,
    pub data_type: Option< DataType >,
}

impl FieldsMeta {
    fn new(table_derive_attributes: TableDeriveAttributes, data_type: DataType) -> Self {
        let mut store = Self {
            table_derive_attributes: Some(table_derive_attributes),
            data_type: Some(data_type),
            ..Default::default()
        };
       store 
    }
        
    pub(crate) fn table_derive_attributes(&self) -> &TableDeriveAttributes {
        self.table_derive_attributes.as_ref().expect("Table derive attribute has not been set. Make sure it has been set")
    }
    
    pub(crate) fn data_type(&self) -> &DataType {
        self.data_type.as_ref().expect("Table derive attribute has not been set. Make sure it has been set")
    }
    
    /// Derive the schema properties for a struct
    pub(crate) fn parse_fields(
        table_derive_attributes: &TableDeriveAttributes,
        data_type: DataType,
    ) -> ExtractorResult<Self> {
        let struct_level_casing = table_derive_attributes.struct_level_casing()?;
        let struct_generics = &table_derive_attributes.generics;
        let mut store = Self::new(table_derive_attributes, data_type);

        for field_receiver in table_derive_attributes.data
            .as_ref()
            .take_struct()
            .ok_or_else(|| darling::Error::custom("Expected a struct"))?
            .fields
        {
            field_receiver.create_field_definitions(&mut store, table_derive_attrs);
            field_receiver.create_field_setter_impl(&mut store, table_derive_attributes);
                

                
            let crate_name = get_crate_name(false);
            let field_name_original = field_receiver
                .ident
                .as_ref()
                .expect("field identifier does not exist");
            let old_field_name_ts = match field_receiver.old_name.as_ref() {
                Some(old_name) => {
                    let old_name = old_name.to_string();
                    quote!(::std::option::Option::Some(#old_name.into()))
                }
                _ => quote!(::std::option::Option::None),
            };
            let relationship = RelationType::from(field_receiver);
            let NormalisedFieldMeta {
                field_ident_raw_to_underscore_suffix,
                field_ident_serialized_fmt,
            } = &field_receiver.normalize_ident(struct_level_casing);
            let (_, struct_ty_generics, _) = struct_generics.split_for_impl();
            let field_type = &field_receiver
                .rust_field_type()
                .to_type_no_self(table_derive_attributes);
            let FieldGenericsMeta {
                field_impl_generics,
                field_ty_generics,
                field_where_clause,
                ..
            } = field_receiver.get_field_generics_meta(table_derive_attributes);

            let VariablesModelMacro {
                ___________graph_traversal_string,
                ____________update_many_bindings,
                _____field_names,
                schema_instance,
                bindings,
                ..
            } = VariablesModelMacro::new();


            
                
            let get_link_meta_with_defs =
                |node_object: &DestinationNodeTypeOriginal, is_list: bool| {
                    ReferencedNodeMeta::from_record_link(
                        node_object,
                        field_ident_raw_to_underscore_suffix,
                        struct_name_ident,
                        is_list,
                    )
                    .with_field_definition(
                        field_receiver,
                        struct_name_ident,
                        field_ident_serialized_fmt,
                        &data_type,
                        &table_name,
                    )
                };

            let get_nested_meta_with_defs =
                |node_object: &DestinationNodeTypeOriginal, is_list: bool| {
                    ReferencedNodeMeta::from_nested(
                        node_object,
                        field_ident_raw_to_underscore_suffix,
                        struct_name_ident,
                        is_list,
                    )?
                    .with_field_definition(
                        field_receiver,
                        struct_name_ident,
                        field_ident_serialized_fmt,
                        &data_type,
                        &table_name,
                    )
                };

            let update_ser_field_type = |serializable_field_type: &mut Vec<TokenStream>| {
                if !field_receiver.skip_serializing && !field_receiver.skip {
                    serializable_field_type
                        .push(quote!(#crate_name::Field::new(#field_ident_serialized_fmt)));
                }
            };

            let mut update_aliases_struct_fields_types_kv = || {
                store.aliases_struct_fields_types_kv.push(
                    quote!(pub #field_ident_raw_to_underscore_suffix: #crate_name::AliasName, ),
                );

                store
                    .aliases_struct_fields_names_kv
                    .push(quote!(#field_ident_raw_to_underscore_suffix: #field_ident_serialized_fmt.into(),));
            };


                store.schema_struct_fields_types_kv.push(
                    quote!(pub #field_ident_raw_to_underscore_suffix: #_____field_names::#field_name_as_camel, ),
                );

                store
                    .schema_struct_fields_names_kv
                    .push(quote!(#field_ident_raw_to_underscore_suffix: #field_ident_serialized_fmt.into(),));

                store.schema_struct_fields_names_kv_prefixed
                            .push(quote!(#field_ident_raw_to_underscore_suffix:
                                                #crate_name::Field::new(format!("{}.{}", prefix.build(), #field_ident_serialized_fmt))
                                                .with_bindings(prefix.get_bindings()).into(),));

                store
                    .schema_struct_fields_names_kv_empty
                    .push(quote!(#field_ident_raw_to_underscore_suffix: "".into(),));

                store.connection_with_field_appended
                        .push(quote!(
                                    #schema_instance.#field_ident_raw_to_underscore_suffix = #schema_instance.#field_ident_raw_to_underscore_suffix
                                      .set_graph_string(format!("{}.{}", #___________graph_traversal_string, #field_ident_serialized_fmt))
                                            .#____________update_many_bindings(#bindings).into();
                                ));
            };

            let mut insert_non_null_updater_token = |updater_field_token: TokenStream| {
                // let is_invalid =
                //     &["id", "in", "out"].contains(&field_ident_normalised_as_str.as_str());
                // if !is_invalid {
                //     store
                //         .non_null_updater_fields
                //         .push(updater_field_token.clone());
                // }
                store
                    .non_null_updater_fields
                    .push(updater_field_token.clone());
                // We dont care about the field type. We just use this struct to check for
                // renamed serialed field names at compile time by asserting that the a field
                // exist.
                store
                    .renamed_serialized_fields
                    .push(quote!(pub #field_ident_raw_to_underscore_suffix: &'static str, ));
            };

            update_ser_field_type(&mut store.serializable_fields);

            let referenced_node_meta = match relationship.clone() {
                RelationType::Relate(relation) => {
                    store
                        .node_edge_metadata
                        .update(&relation, struct_name_ident, field_type);
                    update_aliases_struct_fields_types_kv();
                    let connection = relation.connection;
                    store.fields_relations_aliased.push(quote!(#crate_name::Field::new(#connection).__as__(#crate_name::AliasName::new(#field_ident_serialized_fmt))));
                    ReferencedNodeMeta::default()
                }

                RelationType::LinkOne(node_object) => {
                    // let foreign_node = format_ident!("{node_object}");
                    let foreign_node = node_object.into_inner();
                    update_ser_field_type(&mut store.link_one_fields);
                    update_ser_field_type(&mut store.link_one_and_self_fields);
                    update_ser_field_type(&mut store.linked_fields);
                    update_field_names_fields_types_kv(None);

                    insert_non_null_updater_token(
                        quote!(pub #field_ident_raw_to_underscore_suffix: ::std::option::Option<#field_type>, ),
                    );

                    // let delifed_type = replace_lifetimes_with_underscore(&mut field_type.clone());
                    store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkOne<#foreign_node>);));
                    get_link_meta_with_defs(&node_object, false)
                        .map_err(|e| syn::Error::new_spanned(field_name_original, e.to_string()))?
                }

                RelationType::LinkSelf(node_object) => {
                    let foreign_node = format_ident!("{node_object}");
                    if *struct_name_ident != node_object.to_string() {
                        return Err(syn::Error::new_spanned(
                            field_name_original,
                            "The field - `{field_name_original}` - has a linkself \
                                   attribute or type that is not pointing to the current struct. \
                                   Make sure the field attribute is link_self=\"{struct_name_ident}\" \
                                   and the type is LinkSelf<{struct_name_ident}>. ",
                        )
                        .into());
                    }

                    // insert_non_null_updater_token(
                    //     quote!(pub #field_ident_normalised: ::std::option::Option<#field_type>, ),
                    // );
                    update_ser_field_type(&mut store.link_self_fields);
                    update_ser_field_type(&mut store.link_one_and_self_fields);
                    update_ser_field_type(&mut store.linked_fields);
                    update_field_names_fields_types_kv(None);

                    store.non_null_updater_fields.push(
                        quote!(pub #field_ident_raw_to_underscore_suffix: ::std::option::Option<#field_type>, ),
                    );

                    store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkSelf<#foreign_node>);));

                    get_link_meta_with_defs(&node_object, false)
                        .map_err(|e| syn::Error::new_spanned(field_name_original, e.to_string()))?
                }

                RelationType::LinkMany(foreign_node) => {
                    update_ser_field_type(&mut store.link_many_fields);
                    update_ser_field_type(&mut store.linked_fields);
                    update_field_names_fields_types_kv(Some(
                        quote!(<#foreign_node as #crate_name::Model>::Id),
                    ));

                    insert_non_null_updater_token(
                        quote!(pub #field_ident_raw_to_underscore_suffix: ::std::option::Option<#field_type>, ),
                    );
                

                    store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::LinkMany<#foreign_node>);));
                    get_link_meta_with_defs(&node_object, true)
                        .map_err(|e| syn::Error::new_spanned(field_name_original, e.to_string()))?
                }

                RelationType::NestObject(node_object) => {
                    let foreign_node = format_ident!("{node_object}");
                    store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #foreign_node);));
                    update_field_names_fields_types_kv(None);

                    insert_non_null_updater_token(
                        quote!(pub #field_ident_raw_to_underscore_suffix: ::std::option::Option<<#field_type as #crate_name::Object>::NonNullUpdater>, ),
                    );

                    get_nested_meta_with_defs(&node_object, false)
                        .map_err(|e| syn::Error::new_spanned(field_name_original, e.to_string()))?
                }

                RelationType::NestArray(foreign_object_item) => {
                    insert_non_null_updater_token(
                        quote!(pub #field_ident_raw_to_underscore_suffix: ::std::option::Option<#field_type>, ),
                    );

                    let nesting_level = count_vec_nesting(field_type);
                    let nested_vec_type = generate_nested_vec_type(&foreign_node, nesting_level);

                    store.static_assertions.push(quote! {
                        #crate_name::validators::assert_type_eq_all!(#field_type, #nested_vec_type);
                    });

                    update_field_names_fields_types_kv(Some(quote!(#foreign_object_item)));
                    get_nested_meta_with_defs(&node_object, true)
                        .map_err(|e| syn::Error::new_spanned(field_name_original, e.to_string()))?
                }
                RelationType::None => {
                    update_field_names_fields_types_kv(None);
                    insert_non_null_updater_token(
                        quote!(pub #field_ident_raw_to_underscore_suffix: ::std::option::Option<#field_type>, ),
                    );

                    let ref_node_meta = if field_receiver.rust_field_type().is_list() {
                        ReferencedNodeMeta::from_simple_array(field_ident_raw_to_underscore_suffix)
                    } else {
                        ReferencedNodeMeta::default()
                    };
                    ref_node_meta
                        .with_field_definition(
                            field_receiver,
                            struct_name_ident,
                            field_ident_serialized_fmt,
                            &data_type,
                            &table_name,
                        )
                        .map_err(|e| syn::Error::new_spanned(field_name_original, e.to_string()))?
                }
                RelationType::List(_) => todo!(),
            };

            if field_ident_serialized_fmt == "id" {
                store.table_id_type = quote!(#field_type);
                // store.static_assertions.push(quote!(#crate_name::validators::assert_type_eq_all!(#field_type, #crate_name::SurrealId<#struct_name_ident>);));
            }

            if !referenced_node_meta.field_definition.is_empty() {
                store
                    .field_definitions
                    .push(referenced_node_meta.field_definition.clone());

                let field_definition = referenced_node_meta.field_definition.clone();
                store
                    .field_metadata
                    .push(quote!(#crate_name::FieldMetadata {
                        name: #field_ident_serialized_fmt.into(),
                        old_name: #old_field_name_ts,
                        definition: ::std::vec![ #field_definition ]
                    }));
            }

            store
                .static_assertions
                .push(referenced_node_meta.foreign_node_type_validator);
            store
                .static_assertions
                .extend(referenced_node_meta.field_type_validation_asserts);

            store
                .imports_referenced_node_schema
                .insert(referenced_node_meta.foreign_node_schema_import.into());

            store
                .record_link_fields_methods
                .push(referenced_node_meta.record_link_default_alias_as_method);

            store
                .serialized_field_names_normalised
                .push(field_ident_serialized_fmt.to_owned());
        }

        Ok(store)
    }
}
