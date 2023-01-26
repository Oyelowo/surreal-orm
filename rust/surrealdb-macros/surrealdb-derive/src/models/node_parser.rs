/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use std::{hash::Hash};

use darling::{ast, util};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    node::{MyFieldReceiver, Relate},
    get_crate_name,
    node_relations::{RelationType, RelateAttribute},
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
    /// type StudentWritesBlogTableName = <StudentWritesBlog as SurrealdbEdge>::TableNameChecker;
    /// static_assertions::assert_fields!(StudentWritesBlogTableName: Writes);
    /// ```
    /// Perform all necessary static checks
    pub static_assertions: Vec<TokenStream>,

    /// Generated example: 
    /// ```
    /// type Book = <super::Book as SurrealdbNode>::Schema;
    /// ```
    /// We need imports to be unique, hence the hashset
    /// Used when you use a SurrealdbNode in field e.g: best_student: LinkOne<Student>,
    /// e.g: type Book = <super::Book as SurrealdbNode>::Schema;
    pub referenced_node_schema_imports: Vec<TokenStream>,

    /// Generated example: 
    /// ```
    /// type Writes = super::writes_schema::Writes<Student>;
    /// ```
    /// The above is generated if a Student struct field uses "->Writes->Book". 
    /// Must be unique to prevent collision because it's possible for an edge to be
    /// reused.
    pub referenced_edge_schema_struct_alias: Vec<TokenStream>,

    /// Generated example:
    /// ```
    ///impl Writes {
    ///     pub fn book(&self, clause: #crate_name::Clause) -> Book {
    ///         Book::__________update_connection(&self.__________store, clause)
    ///     }
    /// }
    /// ```
    /// This helps to connect present origin node struct to destination node
    /// and it the edge itself is a struct here. This allows us to give more
    /// specific autocompletion when user accesses available destination node 
    /// from a specific edge from an origin struct.
    /// e.g Student::get_schema().writes__().book();
    /// This allows us to do `.book()` as shown above
    pub relate_edge_schema_struct_alias_impl: Vec<TokenStream>,
    
    /// Genearated example:
    /// ```
    /// pub fn writes__(&self, clause: Clause) -> Writes {
    ///     Writes::__________update_edge(
    ///         &self.___________store,
    ///         clause,
    ///         #crate_name::EdgeDirection::OutArrowRight,
    ///     )
    /// }
    /// ```
    ///  This is used within the current origin node struct e.g Student implementation
    /// e.g Student::get_schema().writes__(); 
    /// it can be writes__ or __writes depending on the arrow direction
    pub relate_edge_schema_method_connection: Vec<TokenStream>,

    /// This is used to alias a relation and uses the field name as default
    /// alias with which a relation can deserialized into
    /// Generated example:
    /// ```
    /// pub fn __as_book_written__(&self) -> String {
    ///     format!("{self} AS book_written")
    /// }
    /// ```
    /// The above can be used for e.g ->Writes->Book as book_written
    pub relate_node_alias_method: Vec<TokenStream>,
    
    /// When a field references another model as Link, we want to generate a method for that
    /// to be able to access the foreign fields
    /// Generated Example for e.g field with best_student: line!()<Student>
    /// ```
    /// pub fn best_student(&self, clause: Clause) -> Student {
    ///     Student::__________update_connection(&self.__________store, clause)
    /// }
    /// ```
    pub record_link_fields_methods: Vec<TokenStream>,
    
    
    /// so that we can do e.g
    /// ```
    /// ->writes[WHERE id = "writes:1"].field_name
    /// self_instance.normalized_field_name.push_str(format!("{}.normalized_field_name", store_without_end_arrow).as_str());
    /// ```
    /// This generates a function that is usually called by other Nodes/Structs
    pub connection_with_field_appended: Vec<TokenStream>,
}

impl SchemaFieldsProperties {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn from_receiver_data(
        data: &ast::Data<util::Ignored, MyFieldReceiver>,
        struct_level_casing: Option<CaseString>,
        struct_name_ident: &syn::Ident,
    ) -> Self {
        let fields = data
            // .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .into_iter()
            .fold(Self::default(), |acc, field_receiver| {
                let field_ident = field_receiver.ident.unwrap();
                let field_type = &field_receiver.ty;
                let crate_name = get_crate_name(false);
                let uncased_field_name = ::std::string::ToString::to_string(&field_ident);
                let field_ident_cased = FieldIdentCased::from(FieldIdentUnCased {
                    uncased_field_name,
                    casing: struct_level_casing,
                });

                // get the field's proper serialized format. Renaming should take precedence
                let original_field_name_normalised = field_receiver.rename.as_ref().map_or_else(
                    || field_ident_cased.into(),
                    |renamed| renamed.clone().serialize,
                );
                let field_ident_normalised = format_ident!("{original_field_name_normalised}");
                let relationship = RelationType::from(&field_receiver);

                let field_ident_normalised_as_str =
                    if original_field_name_normalised.trim_start_matches("r#") == "in".to_string() {
                        "in".into()
                    } else {
                        field_ident_normalised.to_string()
                    };

                let referenced_node_meta = match relationship {
                    RelationType::Relate(relation) => {
                        let relation_attributes = RelateAttribute::from(relation.clone());
                        let arrow_direction = TokenStream::from(relation_attributes.edge_direction);
                        let edge_name = TokenStream::from(relation_attributes.edge_name);
                        let destination_node = TokenStream::from(relation_attributes.node_name.clone());
                        // let extra = ReferencedNodeMeta::from_ref_node_meta(relation_attributes.node_name, field_ident_normalised);
                        //
                        acc.relate_node_alias_method;
                        let struct_name = quote!(#struct_name_ident);
                        let schema_name_basic = &extra.schema_name;
                        acc.relate_node_alias_method.push(todo!());
                        acc.relate_edge_schema_struct_alias_impl.push(todo!());
                        acc.relate_edge_schema_method_connection.push(todo!());
                        // e.g from Writes<In, Out> (Writes<Student, Book>) generics, we can create StudentWritesBook
                        let edge_alias_specific = format_ident!(
                            "{}",
                            relation.edge.expect("Edge must be specified for relations")
                        );
                        // let node_assertion = quote!(<AccountManageProject as Edge>::InNode, Account);
                        let (in_node, out_node) = match relation_attributes.edge_direction {
                            // If OutArrowRight, the current struct should be InNode, and
                            // OutNode in "->edge_action->OutNode", should be OutNode
                            super::relations::EdgeDirection::OutArrowRight => {
                                (struct_name, destination_node)
                            }
                            super::relations::EdgeDirection::InArrowLeft => (destination_node, struct_name),
                        };
                        let edge_checker_alias =
                            format_ident!("EdgeChecker{edge_struct_ident}{edge_action}");
                        let relation_assertions = quote!(
                        // ::static_assertions::assert_type_eq_all!(<AccountManageProject as Edge>::InNode, Account);
                        // ::static_assertions::assert_type_eq_all!(<AccountManageProject as Edge>::OutNode, Project);
                        // type EdgeCheckerAlias = <AccountManageProject as Edge>::EdgeChecker;
                        ::static_assertions::assert_type_eq_all!(<#edge_alias_specific as #crate_name::Edge>::InNode, #crate_name::links::LinkOne<#in_node>);
                        ::static_assertions::assert_type_eq_all!(<#edge_alias_specific as #crate_name::Edge>::OutNode, #crate_name::links::LinkOne<#out_node>);
                        type #edge_checker_alias  = <#edge_alias_specific as Edge>::EdgeChecker;
                        ::static_assertions::assert_fields!(#edge_checker_alias : #edge_name);

                        // assert field type and attribute reference match
                            ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::Relate<#schema_name_basic>);
                                                );
                        relate
                        /*
                         *
                        // This can the access the alias
                          model!(Student {
                            pub ->takes->Course as enrolled_courses, // This is what we want
                          })
                        */
                        // e.g: ->has->Account as field_name
                        let field = quote!(#arrow_direction #edge_name #arrow_direction #schema_name_basic as #field_ident_normalised,);
                        // let field = quote!(#visibility #arrow_direction #edge_action #arrow_direction #schema_name_basic as #field_ident_normalised,);
                        ModelMedataTokenStream {
                            model_schema_field: quote!(#field),
                            original_field_name_normalised,
                            static_assertions: relation_assertions,
                            extra,
                        }
                    },
                    RelationType::LinkOne(node_object) => {
                        ReferencedNodeMeta::from_record_link(node_object, field_ident_normalised)
                    }
                    RelationType::LinkSelf(node_object) => {
                        ReferencedNodeMeta::from_record_link(node_object, field_ident_normalised)
                    }
                    RelationType::LinkMany(node_object) => {
                        ReferencedNodeMeta::from_record_link(node_object, field_ident_normalised)
                    }
                    RelationType::None => ReferencedNodeMeta::default(),
                };
                
                acc.static_assertions.push(referenced_node_meta.destination_node_type_validator);

                acc.schema_struct_fields_types_kv
                    .push(quote!(pub #field_ident_normalised: #crate_name::DbField).into());

                acc.schema_struct_fields_names_kv
                    .push(quote!(#field_ident_normalised: #field_ident_normalised_as_str.into()).into());

                acc.serialized_field_names_normalised
                    .push(field_ident_normalised_as_str);

                acc.connection_with_field_appended
                    .push(quote!(
                               schema_instance.#field_ident_normalised
                                     .push_str(format!("{}.{}", store_without_end_arrow, #field_ident_normalised_as_str).as_str());
                    ).into());

                acc.referenced_node_schema_imports
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
    record_link_default_alias_as_method: Option<TokenStream>,
    destination_node_type_validator: TokenStream,
}

impl ReferencedNodeMeta {
    fn from_relate(relate: Relate) {
        todo!()
    }
    
    fn from_record_link(
        node_name: super::node_relations::NodeName,
        normalized_field_name: ::syn::Ident,
    ) -> ReferencedNodeMeta {
        let schema_name = format_ident!("{node_name}");
        let crate_name = get_crate_name(false);
        
        Self {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SurrealdbNode>::Schema;
            destination_node_schema_import: quote!(
                        type #schema_name = <super::#schema_name as #crate_name::SurrealdbNode>::Schema;
                    ),
                    
            destination_node_type_validator: quote!(::static_assertions::assert_impl_one!(#schema_name: #crate_name::SurrealdbNode)),
            
            record_link_default_alias_as_method: Some(quote!(
                        pub fn #normalized_field_name(&self, clause: #crate_name::Clause) -> #schema_name {
                            #schema_name:__________update_connection(&self.__________store, clause)
                        }
                    )),
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
