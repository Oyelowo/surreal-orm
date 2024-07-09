/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
    ops::Deref,
    option,
};

use convert_case::{Case, Casing};
use darling::{ast, util, ToTokens};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::models::{
    attributes::FieldGenericsMeta, relations::NodeType, replace_lifetimes_with_underscore,
    replace_self_in_type_str, DestinationNodeTypeOriginal, FieldGenericsMeta, NormalisedFieldMeta,
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
    AliasesStructFieldsNamesKv,
    AliasesStructFieldsTypesKv,
    ConnectionWithFieldAppended,
    DataType,
    DefineFieldStatementToken,
    FieldMetadataToken,
    FieldSetterImplTokens,
    FieldsRelationsAliased,
    ForeignNodeSchemaImport,
    GenericTypeExtractor,
    LinkFieldTraversalMethodToken,
    LinkManyFields,
    LinkOneAndSelfFields,
    LinkOneFields,
    LinkSelfFields,
    LinkedFields,
    NodeEdgeMetadataLookupTable,
    NodeEdgeMetadataStore,
    NonNullUpdaterFields,
    RenamedSerializedFields,
    SchemaStructFieldsNamesKv,
    SchemaStructFieldsNamesKvEmpty,
    SchemaStructFieldsNamesKvPrefixed,
    SchemaStructFieldsTypesKv,
    SerializableFields,
    SerializedFieldNamesNormalised,
    StaticAssertionToken,
    TableIdType,
    TokenStreamHashable,
    TypeStripper,
};

#[derive(Default, Clone)]
pub struct FieldsMeta {
    /// list of fields names that are actually serialized and not skipped.
    pub serializable_fields: Vec<SerializableFields>,
    /// The name of the all fields that are linked i.e line_one, line_many, or line_self.
    pub linked_fields: Vec<LinkedFields>,
    /// The names of link_one fields
    pub link_one_fields: Vec<LinkOneFields>,
    /// The names of link_self fields
    pub link_self_fields: Vec<LinkSelfFields>,
    /// The names of link_one and link_self fields
    pub link_one_and_self_fields: Vec<LinkOneAndSelfFields>,
    /// The names of link_many fields
    pub link_many_fields: Vec<LinkManyFields>,
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
    pub schema_struct_fields_types_kv: Vec<SchemaStructFieldsTypesKv>,

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
    pub schema_struct_fields_names_kv: Vec<SchemaStructFieldsNamesKv>,
    pub schema_struct_fields_names_kv_prefixed: Vec<SchemaStructFieldsNamesKvPrefixed>,

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
    pub schema_struct_fields_names_kv_empty: Vec<SchemaStructFieldsNamesKvEmpty>,

    /// Generated example: pub writtenBooks: AliasName,
    /// This is used when you have a relate attribute signaling a graph with e.g node->edge->node
    /// The full thing can look like:
    /// ```rust,ignore
    ///     #[derive(Debug, Default)]
    ///     pub struct Writes<Model: ::serde::Serialize + Default> {
    ///                pub writtenBooks: AliasName,
    ///          }
    /// ```
    pub aliases_struct_fields_types_kv: Vec<AliasesStructFieldsTypesKv>,

    /// Generated example: pub writtenBooks: "writtenBooks".into(),
    /// This is used to build the actual instance of the struct with aliases
    /// The full thing can look like and the fields should be in normalized form:
    /// i.e writtenBooks => writtenBooks if serde camelizes
    /// ```rust, ignore
    /// Self {
    ///                pub writtenBooks: AliasName,
    /// }
    /// ```
    pub aliases_struct_fields_names_kv: Vec<AliasesStructFieldsNamesKv>,

    /// list of fields names that are actually serialized and not skipped.
    pub serialized_alias_name_no_skip: Vec<String>,

    /// Field names after taking into consideration
    /// serde serialized renaming or casings
    /// i.e time_written => timeWritten if serde camelizes
    pub serialized_field_names_normalised: Vec<SerializedFieldNamesNormalised>,

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
    pub connection_with_field_appended: Vec<ConnectionWithFieldAppended>,

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
    pub field_metadata: Vec<FieldMetadataToken>,
    pub node_edge_metadata: NodeEdgeMetadataLookupTable,
    pub fields_relations_aliased: Vec<FieldsRelationsAliased>,
    pub non_null_updater_fields: Vec<NonNullUpdaterFields>,
    pub renamed_serialized_fields: Vec<RenamedSerializedFields>,
    pub table_id_type: TableIdType,

    pub table_derive_attributes: Option<TableDeriveAttributes>,
    pub data_type: Option<DataType>,
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
        self.table_derive_attributes
            .as_ref()
            .expect("Table derive attribute has not been set. Make sure it has been set")
    }

    pub(crate) fn data_type(&self) -> &DataType {
        self.data_type
            .as_ref()
            .expect("Table derive attribute has not been set. Make sure it has been set")
    }

    /// Derive the schema properties for a struct
    pub(crate) fn parse_fields(
        table_derive_attributes: &TableDeriveAttributes,
        data_type: DataType,
    ) -> ExtractorResult<Self> {
        let struct_level_casing = table_derive_attributes.struct_level_casing()?;
        let struct_generics = &table_derive_attributes.generics;
        let mut store = Self::new(table_derive_attributes, data_type);

        for field_receiver in table_derive_attributes
            .data
            .as_ref()
            .take_struct()
            .ok_or_else(|| darling::Error::custom("Expected a struct"))?
            .fields
        {
            let crate_name = get_crate_name(false);
            field_receiver.create_field_definitions(&mut store, table_derive_attrs);
            field_receiver.create_field_setter_impl(&mut store, table_derive_attributes);
            field_receiver
                .create_relation_connection_tokenstream(&mut store, table_derive_attributes);
            field_receiver.create_serialized_fields(&mut store);
            field_receiver.create_relation_aliases_struct_fields_types_kv(&mut store);
            field_receiver
                .create_non_null_updater_struct_fields(&mut store, table_derive_attributes);
            field_receiver.create_field_metada_token(&mut store, table_derive_attrs);
            field_receiver.create_field_connection_builder_token(&mut store, table_derive_attrs);
            field_receiver.create_simple_tokens(&mut store, table_derive_attrs);
            field_receiver.create_field_type_static_assertion_token(&mut store, table_derive_attrs);

            Ok(store)
        }
    }
}
