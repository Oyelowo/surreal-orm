/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use std::hash::Hash;

use darling::{ast, util};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    get_crate_name,
    relations::{RelationType, NodeName}, attributes::{MyFieldReceiver, Relate, ReferencedNodeMeta, NormalisedField}, variables::VariablesModelMacro 
};

#[derive(Default, Clone)]
pub(crate) struct FieldTokenStream(TokenStream);

impl From<TokenStream> for FieldTokenStream {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}

impl From<FieldTokenStream> for TokenStream {
    fn from(value: FieldTokenStream) -> Self {
        value.0
    }
}
impl PartialEq for FieldTokenStream {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}
impl Eq for FieldTokenStream {}

impl Hash for FieldTokenStream {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

#[derive(Default, Clone)]
pub struct SchemaFieldsProperties {
    /// Generated example: pub timeWritten: DbField,
    /// key(normalized_field_name)-value(DbField) e.g pub out: DbField, of field name and DbField type
    /// to build up struct for generating fields of a Schema of the SurrealdbEdge
    /// The full thing can look like:
    /// ```
    ///     #[derive(Debug, Default)]
    ///     pub struct Writes<Model: ::serde::Serialize + Default> {
    ///                pub id: Dbfield,
    ///                pub r#in: Dbfield,
    ///                pub out: Dbfield,
    ///                pub timeWritten: Dbfield,
    ///          }
    /// ```
    pub schema_struct_fields_types_kv: Vec<TokenStream>,

    /// Generated example: pub timeWritten: "timeWritten".into(),
    /// This is used to build the actual instance of the model during intialization e,g out:
    /// "out".into()
    /// The full thing can look like and the fields should be in normalized form:
    /// i.e time_written => timeWritten if serde camelizes
    /// ```
    /// Self {
    ///     id: "id".into(),
    ///     r#in: "in".into(),
    ///     out: "out".into(),
    ///     timeWritten: "timeWritten".into(),
    /// }
    /// ```
    pub schema_struct_fields_names_kv: Vec<TokenStream>,

    /// Field names after taking into consideration
    /// serde serialized renaming or casings
    /// i.e time_written => timeWritten if serde camelizes
    pub serialized_field_names_normalised: Vec<String>,

    /// Generated example:
    /// ```
    /// // For relate field
    /// type StudentWritesBlogTableName = <StudentWritesBlog as SurrealdbEdge>::TableNameChecker;
    /// ::static_assertions::assert_fields!(StudentWritesBlogTableName: Writes);
    ///
    /// ::static_assertions::assert_impl_one!(StudentWritesBlog: SurrealdbEdge);
    /// ::static_assertions::assert_impl_one!(Student: SurrealdbNode);
    /// ::static_assertions::assert_impl_one!(Blog: SurrealdbNode);
    /// ::static_assertions::assert_type_eq_all!(LinkOne<Book>, LinkOne<Book>);
    /// ```
    /// Perform all necessary static checks
    pub static_assertions: Vec<TokenStream>,

    /// Generated example: 
    /// ```
    /// type Book = <super::Book as SurrealdbNode>::Schema;
    /// ```
    /// We need imports to be unique, hence the hashset
    /// Used when you use a SurrealdbNode in field e.g: favourite_book: LinkOne<Book>,
    /// e.g: type Book = <super::Book as SurrealdbNode>::Schema;
    pub imports_referenced_node_schema: Vec<TokenStream>,

    
    /// When a field references another model as Link, we want to generate a method for that
    /// to be able to access the foreign fields
    /// Generated Example for e.g field with best_student: <Student>
    /// ```
    /// pub fn best_student(&self, clause: Clause) -> Student {
    ///     Student::__________connect_to_graph_traversal_string(&self.___________graph_traversal_string, clause)
    /// }
    /// ```
    pub record_link_fields_methods: Vec<TokenStream>,
    
    
    /// This generates a function that is usually called by other Nodes/Structs
    /// self_instance.drunk_water
    /// .push_str(format!("{}.drunk_water", xx.___________graph_traversal_string).as_str());
    /// 
    /// so that we can do e.g
    /// ```
    /// Student.field_name
    /// ```
    pub connection_with_field_appended: Vec<TokenStream>,
}


pub struct SchemaPropertiesArgs<'a> {
    pub data: &'a ast::Data<util::Ignored, MyFieldReceiver>,
    pub struct_level_casing: Option<CaseString>,
    pub struct_name_ident: &'a syn::Ident,
}

impl SchemaFieldsProperties {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub(crate) fn from_receiver_data(
        args : SchemaPropertiesArgs
    ) -> Self {
        let SchemaPropertiesArgs { data, struct_level_casing, struct_name_ident } = args;
        let  VariablesModelMacro {
            __________connect_to_graph_traversal_string,
            ___________graph_traversal_string,
            schema_instance,
            schema_instance_edge_arrow_trimmed, ..
        } = VariablesModelMacro::new();
        
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .into_iter()
            .fold(Self::default(), |mut acc,  field_receiver| {
                let crate_name = get_crate_name(false);
                let relationship = RelationType::from(field_receiver);
                let NormalisedField { 
                         ref field_ident_normalised,
                         ref field_ident_normalised_as_str
                } = NormalisedField::from_receiever(field_receiver, struct_level_casing);


                let referenced_node_meta = match relationship {
                    RelationType::LinkOne(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised)
                    }
                    RelationType::LinkSelf(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised)
                    }
                    RelationType::LinkMany(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised)
                    }
                    RelationType::None | RelationType::Relate(_) => ReferencedNodeMeta::default(),
                };
                
                acc.static_assertions.push(referenced_node_meta.destination_node_type_validator);

                acc.imports_referenced_node_schema
                    .push(referenced_node_meta.destination_node_schema_import.into());

                acc.record_link_fields_methods
                    .push(referenced_node_meta.record_link_default_alias_as_method.into());
                acc.schema_struct_fields_types_kv
                    .push(quote!(pub #field_ident_normalised: #crate_name::DbField, ));

                acc.schema_struct_fields_names_kv
                    .push(quote!(#field_ident_normalised: #field_ident_normalised_as_str.into(), ));

                acc.serialized_field_names_normalised
                    .push(field_ident_normalised_as_str.to_owned());

                acc.connection_with_field_appended
                    .push(quote!(
                               #schema_instance.#field_ident_normalised
                                     .push_str(format!("{}.{}", #schema_instance_edge_arrow_trimmed, #field_ident_normalised_as_str).as_str());
                    ).into());


                acc
            });
    fields
    }
}

