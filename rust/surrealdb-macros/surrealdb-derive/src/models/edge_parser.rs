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
    node::MyFieldReceiver,
    get_crate_name,
    relations::{RelationType, NodeName}, node::Relate,
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

pub struct MacroVariables<'a> {
    /// This joins present model to the currently built graph.
    /// e.g Account->likes->Book.name
    /// For SurrealdbNode, this is usually just concatenating dot and the model fields i.e
    /// Mode.fieldname1, Model.fieldname2
    /// For edges, it usually surrounds the SurrealdbEdge with arrows e.g ->writes-> or <-writes<-
    /// Overall, this helps us do the graph traversal
    pub __________connect_to_graph_traversal_string: &'a syn::Ident,
    pub ___________graph_traversal_string: &'a syn::Ident,
    pub schema_instance: &'a syn::Ident,
    // Mainly used in edge schema to remove the start and end arrows for field access e.g
    // when we have "->writes->", we may want writes.time_written in case we want to access
    // a field on an edge itself because at the end of the day, an edge is a model or table
    // in the database itself
    pub schema_instance_edge_arrow_trimmed: &'a syn::Ident,
}

pub struct SchemaPropertiesArgs<'a> {
    pub macro_variables: &'a MacroVariables<'a>,
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
        let SchemaPropertiesArgs { macro_variables, data, struct_level_casing, struct_name_ident } = args;
        let MacroVariables { __________connect_to_graph_traversal_string, ___________graph_traversal_string, schema_instance, schema_instance_edge_arrow_trimmed } = macro_variables;
        
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .into_iter()
            .fold(Self::default(), |mut acc,  field_receiver| {
                let field_ident = field_receiver.ident.as_ref().unwrap();
                let field_type = &field_receiver.ty;
                let crate_name = get_crate_name(false);
                let uncased_field_name = ::std::string::ToString::to_string(&field_ident);
                let field_ident_cased = FieldIdentCased::from(FieldIdentUnCased {
                    uncased_field_name,
                    casing: struct_level_casing,
                });

                // get the field's proper serialized format. Renaming should take precedence
                let original_field_name_normalised = &field_receiver.rename.as_ref().map_or_else(
                    || field_ident_cased.into(),
                    |renamed| renamed.clone().serialize,
                );
                let ref field_ident_normalised = format_ident!("{original_field_name_normalised}");
                let relationship = RelationType::from(field_receiver);

                let (ref field_ident_normalised,  field_ident_normalised_as_str) =
                    if original_field_name_normalised.trim_start_matches("r#") == "in".to_string() {
                        (format_ident!("in_") , "in".to_string())
                    } else {
                        (field_ident_normalised.to_owned(), field_ident_normalised.to_string() )
                        
                    };

                let referenced_node_meta = match relationship {
                    RelationType::LinkOne(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised, macro_variables)
                    }
                    RelationType::LinkSelf(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised, macro_variables)
                    }
                    RelationType::LinkMany(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised, macro_variables)
                    }
                    RelationType::None | RelationType::Relate(_) => ReferencedNodeMeta::default(),
                };
                
                acc.static_assertions.push(referenced_node_meta.destination_node_type_validator);

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

                acc.imports_referenced_node_schema
                    .push(referenced_node_meta.destination_node_schema_import.into());

                acc.record_link_fields_methods
                    .push(referenced_node_meta.record_link_default_alias_as_method.into());

                acc
            });
    fields
    }
}

#[derive(Default, Clone)]
struct ReferencedNodeMeta {
    destination_node_schema_import: TokenStream,
    record_link_default_alias_as_method: TokenStream,
    destination_node_type_validator: TokenStream,
}

impl ReferencedNodeMeta {
    fn from_relate(relate: Relate, destination_node: &TokenStream) -> Self {
        let crate_name = get_crate_name(false);
            Self{ 
                destination_node_schema_import:  quote!(
                        type #destination_node = <super::#destination_node as #crate_name::SurrealdbNode>::Schema;
                    ), 
                
                record_link_default_alias_as_method: quote!(), 

                destination_node_type_validator: quote!(),
            }
    }
    
    fn from_record_link(
        node_name: &NodeName,
        normalized_field_name: &::syn::Ident,
        macro_variables: &MacroVariables
    ) -> Self {
        let MacroVariables { __________connect_to_graph_traversal_string, ___________graph_traversal_string, .. } = macro_variables;
        let schema_name = format_ident!("{node_name}");
        let crate_name = get_crate_name(false);
        
        Self {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SurrealdbNode>::Schema;
            destination_node_schema_import: quote!(
                        type #schema_name = <super::#schema_name as #crate_name::SurrealdbNode>::Schema;
                    ),
                    
            destination_node_type_validator: quote!(::static_assertions::assert_impl_one!(#schema_name: #crate_name::SurrealdbNode);),
            
            record_link_default_alias_as_method: quote!(
                        pub fn #normalized_field_name(&self, clause: #crate_name::Clause) -> #schema_name {
                            #schema_name::#__________connect_to_graph_traversal_string(&self.#___________graph_traversal_string, clause) }
                    ),
        }
    }
}

fn get_ident(name: &String) -> syn::Ident {
    if vec!["in", "r#in"].contains(&name.as_str()) {
        syn::Ident::new_raw(name.trim_start_matches("r#"), Span::call_site())
    } else {
        syn::Ident::new(name.as_str(), Span::call_site())
    }
}
