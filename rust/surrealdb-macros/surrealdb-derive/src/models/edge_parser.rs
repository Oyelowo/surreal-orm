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
    relations::{RelationType, NodeName}, attributes::{MyFieldReceiver, Relate, ReferencedNodeMeta, NormalisedField}, variables::VariablesModelMacro, node_parser::ModelProps 
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
    pub(crate) model_props: ModelProps,
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
            .fold(Self::default(), |mut store,  field_receiver| {
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
                
                store.model_props.static_assertions.push(referenced_node_meta.destination_node_type_validator);

                store.model_props.imports_referenced_node_schema
                    .push(referenced_node_meta.destination_node_schema_import.into());

                store.model_props.record_link_fields_methods
                    .push(referenced_node_meta.record_link_default_alias_as_method.into());
                store.model_props.schema_struct_fields_types_kv
                    .push(quote!(pub #field_ident_normalised: #crate_name::DbField, ));

                store.model_props.schema_struct_fields_names_kv
                    .push(quote!(#field_ident_normalised: #field_ident_normalised_as_str.into(), ));

                store.model_props.serialized_field_names_normalised
                    .push(field_ident_normalised_as_str.to_owned());

                store.model_props.connection_with_field_appended
                    .push(quote!(
                               #schema_instance.#field_ident_normalised
                                     .push_str(format!("{}.{}", #___________graph_traversal_string, #field_ident_normalised_as_str).as_str());
                    ).into());


                store
            });
    fields
    }
}

