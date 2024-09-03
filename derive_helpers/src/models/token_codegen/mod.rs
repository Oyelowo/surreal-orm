/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

mod aliases;
mod define_statement;
mod field_connection_build;
mod field_metadata;
mod field_type_assertions;
mod field_value_setter;
mod link_methods;
mod relate;
mod serialized_field_fmts;
mod simple;
mod struct_partial;

use convert_case::{Case, Casing};
use quote::format_ident;
use std::collections::HashSet;

use crate::models::*;

use self::{
    field_value_setter::FieldSetterImplTokens,
    link_methods::{ForeignNodeSchemaImport, LinkFieldTraversalMethodToken},
    relate::NodeEdgeMetadataLookupTable,
};

create_ident_wrapper!(ModuleNameInternalIdent);
create_ident_wrapper!(ModuleNameRexportedIdent);
create_ident_wrapper!(AliasesStructNameIdent);
create_ident_wrapper!(TestFunctionNameIdent);
create_ident_wrapper!(NonNullUpdaterStructNameIdent);
create_ident_wrapper!(SchemaDefIdent);
create_ident_wrapper!(StructWithRenamedSerializedFields);

pub(crate) struct CommonIdents {
    pub module_name_internal: ModuleNameInternalIdent,
    pub module_name_rexported: ModuleNameRexportedIdent,
    pub aliases_struct_name: AliasesStructNameIdent,
    pub test_function_name: TestFunctionNameIdent,
    pub struct_with_renamed_serialized_fields: StructWithRenamedSerializedFields,
    pub _____schema_def: SchemaDefIdent,
}

#[derive(Default, Clone)]
pub struct Codegen<'a> {
    /// list of fields names that are actually serialized and not skipped as ident
    /// used for partial udpater struct. We stick to the original field name,
    /// to stay consistent with rust struct declaration within the code but
    /// handle the renaming automatically for the user.
    /// Personally, I think this is better than using the serialized field name in this specific
    /// context.
    /// NOTE: Currently, I am skipping id field since it's immutable and should not be updated.
    /// And considering same for in and out fields of edge tables if surrealdb 2.0 does not support
    /// that.
    /// e.g pub time_written: Time,
    ///
    /// However, when trying to get database name for doing query-building, I typically
    /// use he serialized field name since this is how it would appear in the database and I want
    /// to be as close at possible to the actual database field name to avoid confusion in this
    /// other scenario.
    pub serialized_ident_struct_partial_init_fields: Vec<SerializedIdentStructPartialInitFields>,
    /// list of fields names that are actually serialized and not skipped as string.
    /// Usually, this is every field except the many-to-many relation fields which
    /// are not serialized because they are not directly part of the struct.
    /// They are readonly fields that are accessed through the relation field.
    pub serialized_fmt_db_field_names_instance: Vec<SerializableField>,
    /// The name of the all fields that are linked i.e line_one, line_many, or line_self.
    pub linked_fields: Vec<LinkedField>,
    /// The names of link_one fields
    pub link_one_fields: Vec<LinkOneField>,
    /// The names of link_self fields
    pub link_self_fields: Vec<LinkSelfField>,
    /// The names of link_one and link_self fields
    pub link_one_and_self_fields: Vec<LinkOneAndSelfField>,
    /// The names of link_many fields
    pub link_many_fields: Vec<LinkManyField>,
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
    #[allow(unused)]
    pub serialized_alias_name_no_skip: Vec<String>,

    /// Field names after taking into consideration
    /// serde serialized renaming or casings
    /// i.e time_written => timeWritten if serde camelizes
    pub serialized_field_names_normalised: Vec<DbFieldNamesToken>,

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
    pub field_definitions: Vec<DefineFieldStatementToken>,
    pub field_metadata: Vec<FieldMetadataToken>,
    pub node_edge_metadata: NodeEdgeMetadataLookupTable<'a>,
    pub fields_relations_aliased: Vec<FieldsRelationsAliased>,
    /// e.g pub time: surreal_orm::Maybe<<Time as surreal_orm::PartialUpdater>::StructPartial>,
    pub struct_partial_fields: Vec<StructPartialFields>,
    //     pub fn #field_name(mut self, value: #original_field_type) -> Self {
    //     self.0.#field_name = value;
    //             self
    //     }
    pub struct_partial_associated_functions: Vec<StructPartialMethods>,
    pub renamed_serialized_fields_kv: Vec<RenamedSerializedFields>,
    pub table_id_type: TableIdType,

    struct_attributes_data: StructAttributesData<'a>,
}

#[derive(Clone, Default)]
struct StructAttributesData<'a> {
    field_receiver: Option<&'a MyFieldReceiver>,
    struct_basic_model_attributes: Option<&'a ModelAttributes<'a>>,
}

impl<'a> Codegen<'a> {
    fn new(model_attributes: &'a ModelAttributes) -> Self {
        let struct_attributes_data = StructAttributesData {
            struct_basic_model_attributes: Some(model_attributes),
            field_receiver: None,
        };

        Self {
            struct_attributes_data,
            ..Default::default()
        }
    }

    pub(crate) fn common_idents(&self) -> CommonIdents {
        let struct_name_ident = self.table_derive_attributes().ident().to_string();
        let struct_name_ident_snake = struct_name_ident.to_case(Case::Snake);
        let module_name_internal =
            format_ident!("________internal_{struct_name_ident_snake}_schema");

        CommonIdents {
            test_function_name: format_ident!(
                "_________test_{module_name_internal}_static_funcs_name__________"
            )
            .into(),
            module_name_internal: module_name_internal.into(),
            module_name_rexported: format_ident!("{struct_name_ident_snake}").into(),
            aliases_struct_name: format_ident!("{struct_name_ident}Aliases").into(),
            struct_with_renamed_serialized_fields: format_ident!(
                "{struct_name_ident}RenamedCreator"
            )
            .into(),
            _____schema_def: format_ident!("_____schema_def").into(),
        }
    }

    fn set_field_receiver(&mut self, field_receiver: &'a MyFieldReceiver) {
        self.struct_attributes_data.field_receiver = Some(field_receiver);
    }

    pub(crate) fn field_receiver(&self) -> &'a MyFieldReceiver {
        self.struct_attributes_data.field_receiver
            .expect("Field receiver has not been set. Make sure it has been set by calling set_field_receiver")
    }

    pub(crate) fn table_derive_attributes(&self) -> &'a ModelAttributes<'a> {
        self.struct_attributes_data
            .struct_basic_model_attributes
            .expect("Model attributes has not been set. Make sure it has been set by calling set_field_receiver")
    }

    pub(crate) fn data_type(&self) -> DataType {
        self.table_derive_attributes().to_data_type()
    }

    /// Derive the schema properties for a struct
    pub(crate) fn parse_fields(model_attributes: &'a ModelAttributes<'a>) -> ExtractorResult<Self> {
        let mut tokens_generator = Self::new(model_attributes);

        for field_receiver in model_attributes.fields()? {
            tokens_generator.set_field_receiver(field_receiver);

            tokens_generator.create_table_id_type_token()?;
            tokens_generator.create_field_definitions()?;
            tokens_generator.create_db_field_names_token()?;
            tokens_generator.create_field_type_static_assertion_token()?;
            tokens_generator.create_field_setter_impl()?;
            tokens_generator.create_field_metadata_token()?;
            tokens_generator.create_field_connection_builder_token()?;
            tokens_generator.create_relation_connection_tokenstream()?;
            tokens_generator.create_db_fields_for_links_and_loaders()?;
            tokens_generator.create_link_methods()?;
            tokens_generator.create_relation_aliases_struct_fields_types_kv()?;
            tokens_generator.create_struct_partial_metadata()?;
        }
        Ok(tokens_generator)
    }
}
