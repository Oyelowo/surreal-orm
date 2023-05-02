/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};

use convert_case::{Case, Casing};
use darling::{ast, util, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    attributes::{MyFieldReceiver, NormalisedField, ReferencedNodeMeta, Relate},
    casing::CaseString,
    get_crate_name,
    relations::{EdgeDirection, RelateAttribute, RelationType, NodeTypeName},
    variables::VariablesModelMacro 
};

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct NodeEdgeMetadata {
    /// Example value: writes
    edge_table_name: syn::Ident,
    /// The current struct name ident.
    /// e.g given: struct Student {  }, value = Student
    origin_struct_ident: syn::Ident,
    /// The database table name of the edge. Used for generating other tokens
    /// e.g "writes"
    direction: EdgeDirection,
    /// Example of value: `StudentWritesBook`
    ///
    /// For each edge table e.g writes, we usually can have many aliases reusing thesame edge
    /// e.g for Writes<In, Out>, we could have  StudentWritesBook, StudentWritesBlog, for each direction(e.g ->),
    /// we want to select one of these to use its schema which is aliased as the Cased table name
    /// in the calling location e.g
    /// for a model field annotation e.g relate(edge="StudentWritesBook", link="->writes->book")
    /// So we can do
    /// type Writes = <StudentWritesBook as SurrealdbEdge>::Schema;
    edge_relation_model_selected_ident: syn::Ident,
    /// Example Generated:
    /// ```
    ///   type BookModel = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
    ///   type Book = <BookModel as surrealdb_macros::SurrealdbNode>::Schema;
    ///
    ///   type BlogModel = <StudentWritesBlog as surrealdb_macros::SurrealdbEdge>::Out;
    ///   type Blog = <BlogModel as surrealdb_macros::SurrealdbNode>::Schema;
    /// ```
    ///
    /// Example Value:
    /// ```
    /// vec![
    ///    quote!(
    ///       type BookModel = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
    ///       type Book = <BookModel as surrealdb_macros::SurrealdbNode>::Schema;
    ///     ),
    ///     quote!(
    ///       type BlogModel = <StudentWritesBlog as surrealdb_macros::SurrealdbEdge>::Out;
    ///       type Blog = <BlogModel as surrealdb_macros::SurrealdbNode>::Schema;
    ///     ),
    /// ],
    /// ```
    destination_node_schema: Vec<TokenStream>,
    destination_node_name: String,
    /// Example Generated:
    ///
    /// ```
    /// impl Writes__ {
    ///     fn book(&self, filter: Filter) -> Book {
    ///         Book::__________connect_to_graph_traversal_string(
    ///             &self.___________graph_traversal_string,
    ///             filter,
    ///         )
    ///     }
    ///
    ///     fn blog(&self, filter: Filter) -> Blog {
    ///         Blog::__________connect_to_graph_traversal_string(
    ///             &self.___________graph_traversal_string,
    ///             filter,
    ///         )
    ///     }
    /// }
    /// ```
    ///
    /// Example Value:
    /// ```
    /// vec![
    ///     quote!(
    ///        fn book(&self, filter: Filter) -> Book {
    ///            Book::__________connect_to_graph_traversal_string(
    ///                &self.___________graph_traversal_string,
    ///                filter,
    ///            )
    ///        }
    ///     ),
    ///     quote!(
    ///        fn blog(&self, filter: Filter) -> Blog {
    ///            Blog::__________connect_to_graph_traversal_string(
    ///                &self.___________graph_traversal_string,
    ///                filter,
    ///            )
    ///        }
    ///     ),
    ///    ]
    /// ```
    foreign_node_connection_method: Vec<TokenStream>,
    static_assertions: Vec<TokenStream>,
    imports: Vec<TokenStream>,
    edge_name_as_method_ident: syn::Ident,
}

#[derive(Default, Clone)]
pub struct SchemaFieldsProperties {
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
    /// to build up struct for generating fields of a Schema of the SurrealdbEdge
    /// The full thing can look like:
    /// ```
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

    /// Generated Field wrapper type implementations for each fiekd around `Field` type
    /// Example value:
    /// ```
    /// struct Email(pub(super) Field);
    /// 
    /// impl std::ops::Deref for Email {
    ///     type Target = #crate_name::Field;
    ///
    ///     fn deref(&self) -> &Self::Target {
    ///         &self.0
    ///     }
    /// }
    /// impl #crate_name::SetterAssignable<sql::Duration> for Email {}
    /// ```
     pub field_wrapper_type_custom_implementations: Vec<TokenStream>,

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
    
    /// Used to build up empty string values for all schema fields
    /// Example value: pub timeWritten: "".into(),
    /// Used to build up e.g:
    /// Self {
    ///     id: "id".into(),
    ///     r#in: "in".into(),
    ///     out: "out".into(),
    ///     timeWritten: "timeWritten".into(),
    /// }
    /// ```
    pub schema_struct_fields_names_kv_empty: Vec<TokenStream>,

    /// Generated example: pub writtenBooks: AliasName,
    /// This is used when you have a relate attribute signaling a graph with e.g node->edge->node
    /// The full thing can look like:
    /// ```
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
    /// ```
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
    /// ```
    /// // For relate field
    /// type StudentWritesBlogTableName = <StudentWritesBlog as SurrealdbEdge>::TableNameChecker;
    /// ::static_assertions::assert_fields!(StudentWritesBlogTableName: Writes);
    ///
    /// type StudentWritesBlogInNode = <StudentWritesBlog as SurrealdbEdge>::In;
    /// ::static_assertions::assert_type_eq_all!(StudentWritesBlogInNode, Student);
    ///
    /// type StudentWritesBlogOutNode = <StudentWritesBlog as SurrealdbEdge>::Out;
    /// ::static_assertions::assert_type_eq_all!(StudentWritesBlogOutNode, Blog);
    ///
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
    pub imports_referenced_node_schema: HashSet<TokenStreamHashable>,

    /// This generates a function that is usually called by other Nodes/Structs
    /// self_instance.drunk_water
    /// .push_str(format!("{}.drunk_water", xx.___________graph_traversal_string).as_str());
    ///
    /// so that we can do e.g
    /// ```
    /// Student.field_name
    /// ```
    pub connection_with_field_appended: Vec<TokenStream>,

    /// When a field references another model as Link, we want to generate a method for that
    /// to be able to access the foreign fields
    /// Generated Example for e.g field with best_student: <Student>
    /// ```
    /// pub fn best_student(&self, filter: Filter) -> Student {
    ///     Student::__________connect_to_graph_traversal_string(&self.___________graph_traversal_string, filter)
    /// }
    /// ```
    pub record_link_fields_methods: Vec<TokenStream>,
    pub field_definitions: Vec<TokenStream>,
    pub node_edge_metadata: NodeEdgeMetadataStore,
    pub fields_relations_aliased: Vec<TokenStream>,
    pub non_null_updater_fields: Vec<TokenStream>,
}

#[derive(Clone)]
pub struct TokenStreamHashable(TokenStream);

impl ToTokens for TokenStreamHashable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.clone());
    }
}

impl Deref for TokenStreamHashable {
    type Target = TokenStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TokenStream> for TokenStreamHashable {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}

impl Eq for TokenStreamHashable {}

impl PartialEq for TokenStreamHashable {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl std::hash::Hash for TokenStreamHashable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

pub struct SchemaPropertiesArgs<'a> {
    pub data: &'a ast::Data<util::Ignored, MyFieldReceiver>,
    pub struct_level_casing: Option<CaseString>,
    pub struct_name_ident: &'a syn::Ident,
    // pub table_name_ident: &'a syn::Ident,
}
impl SchemaFieldsProperties {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub(crate) fn from_receiver_data(args: SchemaPropertiesArgs) -> Self {
        let SchemaPropertiesArgs {
            data,
            struct_level_casing,
            struct_name_ident,
            ..
        } = args;

        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .into_iter()
            .fold(Self::default(), |mut store, field_receiver| {
                let crate_name = get_crate_name(false);
                let field_type = &field_receiver.ty;
                let field_name_original = field_receiver.ident.as_ref().unwrap();
                let relationship = RelationType::from(field_receiver);
                let NormalisedField { 
                         ref field_ident_normalised,
                         ref field_ident_normalised_as_str,
                } = NormalisedField::from_receiever(field_receiver, struct_level_casing);
                
                let VariablesModelMacro { 
                    ___________graph_traversal_string, 
                    ____________update_many_bindings,
                    _____field_names,
                    schema_instance, 
                    bindings,
                    .. 
                } = VariablesModelMacro::new();
                
        
                let get_link_meta_with_defs = |node_object: &NodeTypeName| {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised, struct_name_ident) 
                            .with_field_definition(field_receiver, &struct_name_ident.to_string(), field_ident_normalised_as_str)                                        
                };
                
                let get_nested_meta_with_defs = |node_object: &NodeTypeName| {
                        ReferencedNodeMeta::from_nested(&node_object, field_ident_normalised, struct_name_ident) 
                            .with_field_definition(field_receiver, &struct_name_ident.to_string(), field_ident_normalised_as_str)                                        
                };

                let update_ser_field_type = |serializable_field_type: & mut Vec<TokenStream>| {
                    if !field_receiver.skip_serializing && !field_receiver.skip {
                        serializable_field_type.push(quote!(#crate_name::Field::new(#field_ident_normalised_as_str)));
                    }
                };
                
                let mut update_aliases_struct_fields_types_kv = || {
                    store.aliases_struct_fields_types_kv
                        .push(quote!(pub #field_ident_normalised: #crate_name::AliasName, ));
                    
                    store.aliases_struct_fields_names_kv
                        .push(quote!(#field_ident_normalised: #field_ident_normalised_as_str.into(),));
                };
            
                
                let mut update_field_names_fields_types_kv = || {
                    // TODO:
                    let field_name_as_camel = format_ident!("{}", field_name_original.to_string().to_case(Case::Camel));
                    store.field_wrapper_type_custom_implementations
                        .push(quote!(
                            pub struct #field_name_as_camel(pub #crate_name::Field);

                            impl From<&str> for #field_name_as_camel {
                                fn from(field_name: &str) -> Self {
                                    Self(#crate_name::Field::new(field_name))
                                }
                            }

                            impl ::std::ops::Deref for #field_name_as_camel {
                                type Target = #crate_name::Field;

                                fn deref(&self) -> &Self::Target {
                                    &self.0
                                }
                            }

                        impl #crate_name::SetterAssignable<#field_type> for #field_name_as_camel  {}

                    ));
                    store.schema_struct_fields_types_kv
                        .push(quote!(pub #field_ident_normalised: #_____field_names::#field_name_as_camel, ));
                    
                    store.schema_struct_fields_names_kv
                        .push(quote!(#field_ident_normalised: #field_ident_normalised_as_str.into(),));
                    
                    store.schema_struct_fields_names_kv_empty
                        .push(quote!(#field_ident_normalised: "".into(),));
                    
                    store.connection_with_field_appended
                        .push(quote!(
                                    #schema_instance.#field_ident_normalised = #schema_instance.#field_ident_normalised
                                      .set_graph_string(format!("{}.{}", #___________graph_traversal_string, #field_ident_normalised_as_str))
                                            .#____________update_many_bindings(#bindings);
                                ));

                };
                
                let mut insert_non_null_updater_token = |updater_field_token:  TokenStream| {
                    let is_invalid = &["id", "in", "out"].contains(&field_ident_normalised_as_str.as_str());
                    if !is_invalid {
                        store.non_null_updater_fields.push(updater_field_token);
                    }
                };

                update_ser_field_type(&mut store.serializable_fields);
                
                let referenced_node_meta = match relationship.clone() {
                    RelationType::Relate(relation) => {
                            store.node_edge_metadata.update(&relation, struct_name_ident, field_type);
                        update_aliases_struct_fields_types_kv();
                        let connection = relation.connection_model; 
                        store.fields_relations_aliased.push(quote!(#crate_name::Field::new(#connection).__as__(#crate_name::AliasName::new(#field_ident_normalised_as_str))));
                            ReferencedNodeMeta::default()
                                
                    },
                        
                    RelationType::LinkOne(node_object) => {
                        let foreign_node = format_ident!("{node_object}");
                        update_ser_field_type(&mut store.link_one_fields);
                        update_ser_field_type(&mut store.link_one_and_self_fields);
                        update_ser_field_type(&mut store.linked_fields);
                        update_field_names_fields_types_kv();
                        
                        insert_non_null_updater_token(quote!(pub #field_ident_normalised: ::std::option::Option<#field_type>, ));
                        
                        store.static_assertions.push(quote!(::static_assertions::assert_type_eq_all!(#field_type, #crate_name::LinkOne<#foreign_node>);));
                        get_link_meta_with_defs(&node_object)
                    }
                    
                    RelationType::LinkSelf(node_object) => {
                        let foreign_node = format_ident!("{node_object}");
                        if node_object.to_string() != struct_name_ident.to_string() {
                            panic!("The field - `{field_name_original}` - has a linkself \
                                   attribute or type that is not pointing to the current struct. \
                                   Make sure the field attribute is link_self=\"{struct_name_ident}\" \
                                   and the type is LinkSelf<{struct_name_ident}>. ");
                        }
                        
                        update_ser_field_type(&mut store.link_self_fields);
                        update_ser_field_type(&mut store.link_one_and_self_fields);
                        update_ser_field_type(&mut store.linked_fields);
                        update_field_names_fields_types_kv();
                        
                        store.non_null_updater_fields.push(quote!(pub #field_ident_normalised: ::std::option::Option<#field_type>, ));
                        
                        store.static_assertions.push(quote!(::static_assertions::assert_type_eq_all!(#field_type, #crate_name::LinkSelf<#foreign_node>);));
                        
                        get_link_meta_with_defs(&node_object)
                    }
                    
                    RelationType::LinkMany(node_object) => {
                        let foreign_node = format_ident!("{node_object}");
                        update_ser_field_type(&mut store.link_many_fields);
                        update_ser_field_type(&mut store.linked_fields);
                        update_field_names_fields_types_kv();
                        
                        insert_non_null_updater_token(quote!(pub #field_ident_normalised: ::std::option::Option<#field_type>, ));
                        
                        store.static_assertions.push(quote!(::static_assertions::assert_type_eq_all!(#field_type, #crate_name::LinkMany<#foreign_node>);));
                        get_link_meta_with_defs(&node_object)
                    }                    
                    
                    RelationType::NestObject(node_object) => {
                        let foreign_node = format_ident!("{node_object}");
                        store.static_assertions.push(quote!(::static_assertions::assert_type_eq_all!(#field_type, #foreign_node);));
                        update_field_names_fields_types_kv();
                        
                        insert_non_null_updater_token(quote!(pub #field_ident_normalised: ::std::option::Option<<#field_type as #crate_name::SurrealdbObject>::NonNullUpdater>, ));
                        
                        get_nested_meta_with_defs(&node_object)
                    },
                        
                    RelationType::NestArray(node_object) => {
                        let foreign_node = format_ident!("{node_object}");
                        
                        insert_non_null_updater_token(quote!(pub #field_ident_normalised: ::std::option::Option<#field_type>, ));

                        store.static_assertions.push(quote!(::static_assertions::assert_type_eq_all!(#field_type, ::std::vec::Vec<#foreign_node>);));
                        
                        update_field_names_fields_types_kv();
                        get_nested_meta_with_defs(&node_object)
                    },
                    RelationType::None => {
                        update_field_names_fields_types_kv();
                        insert_non_null_updater_token(quote!(pub #field_ident_normalised: ::std::option::Option<#field_type>, ));
                        
                        ReferencedNodeMeta::default()
                            .with_field_definition(field_receiver, &struct_name_ident.to_string(), field_ident_normalised_as_str)                                        
                    }
                };
                
                if !referenced_node_meta.field_definition.is_empty() {
                    store.field_definitions.push(referenced_node_meta.field_definition);
                }
                
                store.static_assertions.push(referenced_node_meta.foreign_node_type_validator);
                store.static_assertions.extend(referenced_node_meta.field_type_validation_asserts);
                
                store.imports_referenced_node_schema
                    .insert(referenced_node_meta.foreign_node_schema_import.into());
                
                store.record_link_fields_methods
                    .push(referenced_node_meta.record_link_default_alias_as_method.into());
  
  

                store.serialized_field_names_normalised
                    .push(field_ident_normalised_as_str.to_owned());
                

                store 
            });
        fields
    }
}

type EdgeNameWithDirectionIndicator = String;

#[derive(Default, Clone)]
pub struct NodeEdgeMetadataStore(HashMap<EdgeNameWithDirectionIndicator, NodeEdgeMetadata>);

impl NodeEdgeMetadataStore {
    /// e.g for ->writes->book, gives writes__. <-writes<-book, gives __writes
    fn add_direction_indication_to_ident(
        &self,
        ident: &impl Display,
        edge_direction: &EdgeDirection,
    ) -> String {
        match edge_direction {
            EdgeDirection::OutArrowRight => format!("{ident}__"),
            EdgeDirection::InArrowLeft => format!("__{ident}"),
        }
    }

    fn create_static_assertions(
        relation: &Relate,
        origin_struct_ident: &syn::Ident,
        field_type: &syn::Type,
    ) -> TokenStream {
        let crate_name = get_crate_name(false);
        let ref relation_model = format_ident!("{}", relation.model.as_ref().unwrap());
        let relation_attributes = RelateAttribute::from(relation);
        let ref edge_table_name = TokenStream::from(&relation_attributes.edge_table_name);
        let ref foreign_node_table_name = TokenStream::from(&relation_attributes.node_table_name);

        let edge_table_name_checker_ident = format_ident!("{}EdgeTableNameChecker", relation_model);
        let home_node_ident = format_ident!("{}HomeNode", relation_model);
        let home_node_table_name_checker_ident =
            format_ident!("{}HomeNodeTableNameChecker", relation_model);
        let foreign_node_ident = format_ident!("{}ForeignNode", relation_model);
        let foreign_node_table_name_checker_ident =
            format_ident!("{}ForeignNodeTableNameChecker", relation_model);

        let (home_node_associated_type_ident, foreign_node_associated_type_ident) =
            match &relation_attributes.edge_direction {
                EdgeDirection::OutArrowRight => (format_ident!("In"), format_ident!("Out")),
                EdgeDirection::InArrowLeft => (format_ident!("Out"), format_ident!("In")),
            };

        // e.g for struct Student {
        //                   #[surrealdb(relate(mode="StudentWritesBook", connection="->writes->book"))]
        //                   fav_books: Relate<Book>
        //              }
        let static_assertions = &[
            // type HomeIdent = <StudentWritesBook  as surrealdb_macros::SurrealdbEdge>::In;
            // type HomeNodeTableChecker = <HomeIdent as
            // surrealdb_macros::SurrealdbNode>::TableNameChecker;
            // ::static_assertions::assert_type_eq_all!(HomeIdent, Student);
            // ::static_assertions::assert_impl_one!(HomeIdent, surrealdb_macros::SurrealdbNode);
            quote!(
             type #home_node_ident = <#relation_model as #crate_name::SurrealdbEdge>::#home_node_associated_type_ident;
             type #home_node_table_name_checker_ident = <#home_node_ident as #crate_name::SurrealdbNode>::TableNameChecker;
             ::static_assertions::assert_type_eq_all!(#home_node_ident, #origin_struct_ident);
             ::static_assertions::assert_impl_one!(#home_node_ident: #crate_name::SurrealdbNode);
            ),
            quote!(
             type #foreign_node_ident = <#relation_model as #crate_name::SurrealdbEdge>::#foreign_node_associated_type_ident;
             type #foreign_node_table_name_checker_ident = <#foreign_node_ident as #crate_name::SurrealdbNode>::TableNameChecker;
             ::static_assertions::assert_fields!(#foreign_node_table_name_checker_ident: #foreign_node_table_name);
             ::static_assertions::assert_impl_one!(#foreign_node_ident: #crate_name::SurrealdbNode);
            ),
            quote!(
             type #edge_table_name_checker_ident = <#relation_model as #crate_name::SurrealdbEdge>::TableNameChecker;
             ::static_assertions::assert_fields!(#edge_table_name_checker_ident: #edge_table_name);
            ),
            // assert field type and attribute reference match
            // e.g Relate<Book> should match from attribute link = "->Writes->Book"
            quote!(
             ::static_assertions::assert_impl_one!(#relation_model: #crate_name::SurrealdbEdge);
             ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::Relate<#foreign_node_ident>);
            ),
        ];
        quote!(
                #( #static_assertions) *
        )
    }

    fn update(
        &mut self,
        relation: &Relate,
        origin_struct_ident: &syn::Ident,
        field_type: &syn::Type,
    ) -> &Self {
        let crate_name = get_crate_name(false);
        let ref relation_model = format_ident!("{}", relation.model.as_ref().unwrap());
        let relation_attributes = RelateAttribute::from(relation);
        let ref edge_table_name = TokenStream::from(&relation_attributes.edge_table_name);
        
        let ref destination_node_table_name =
            TokenStream::from(&relation_attributes.node_table_name);
        let ref destination_node_table_name_str =  destination_node_table_name.to_string();

        let ref edge_direction = relation_attributes.edge_direction;
        let arrow = format!("{}", &edge_direction);

        let ref edge_name_as_method_ident =
            || self.add_direction_indication_to_ident(edge_table_name, edge_direction);

        // represents the schema but aliased as the pascal case of the destination table name
        let destination_node_schema_ident = format_ident!(
            "{}",
            destination_node_table_name
                .to_string()
                .to_case(Case::Pascal)
        );
        // Meant to represent the variable of struct model(node) itself.
        let destination_node_model_ident =
            format_ident!("______________{destination_node_schema_ident}Model");

        let VariablesModelMacro {
            __________connect_node_to_graph_traversal_string,
            ___________graph_traversal_string,
            ..
        } = VariablesModelMacro::new();
        // Within edge generics, there is usually In and Out associated types, this is used to access
        // those
        let foreign_node_in_or_out = match edge_direction {
            EdgeDirection::OutArrowRight => format_ident!("Out"),
            EdgeDirection::InArrowLeft => format_ident!("In"),
        };
        // We use super twice because we're trying to access the relation model struct name from
        // the outer outer module because all edge related functionalities are nested
        let destination_node_schema_one = || {
            quote!(
            type #destination_node_model_ident = <super::super::#relation_model as #crate_name::SurrealdbEdge>::#foreign_node_in_or_out;
            type #destination_node_schema_ident = <#destination_node_model_ident as #crate_name::SurrealdbNode>::Schema;
            )
        };

        // i.e Edge to destination Node
        let foreign_node_connection_method = || {
            quote!(
                pub fn #destination_node_table_name(self, clause: impl Into<#crate_name::NodeClause>) -> #destination_node_schema_ident {
                    let clause: #crate_name::NodeClause = clause.into();
                    let clause = clause.with_arrow(#arrow).with_table(#destination_node_table_name_str);

                    #destination_node_schema_ident::#__________connect_node_to_graph_traversal_string(
                                self,
                                clause,
                    )
                }
            )
        };
        let static_assertions =
            || Self::create_static_assertions(relation, origin_struct_ident, field_type);

        // let imports =|| quote!(use super::StudentWritesBook;);
        let imports = || quote!(use super::#relation_model;);

        let node_edge_meta = NodeEdgeMetadata {
            edge_table_name: format_ident!(
                "{}",
                &relation_attributes.edge_table_name.clone().to_string()
            ),
            direction: edge_direction.clone(),
            destination_node_schema: vec![destination_node_schema_one()],
            foreign_node_connection_method: vec![foreign_node_connection_method()],
            origin_struct_ident: origin_struct_ident.to_owned(),
            static_assertions: vec![static_assertions()],
            edge_name_as_method_ident: format_ident!("{}", edge_name_as_method_ident()),
            imports: vec![imports()],
            edge_relation_model_selected_ident: relation_model.to_owned(),
            destination_node_name:  destination_node_table_name.to_string()
        };

        match self.0.entry(edge_name_as_method_ident()) {
            Entry::Occupied(o) => {
                let node_edge_meta = o.into_mut();
                node_edge_meta
                    .destination_node_schema
                    .push(destination_node_schema_one());
                node_edge_meta
                    .foreign_node_connection_method
                    .push(foreign_node_connection_method());
                node_edge_meta.static_assertions.push(static_assertions());
                node_edge_meta.imports.push(imports());
            }
            Entry::Vacant(v) => {
                v.insert(node_edge_meta);
            }
        };
        self
    }

    pub(crate) fn generate_static_assertions(&self) -> TokenStream {
        let static_assertions = self
            .0
            .values()
            .map(|value| {
                let static_assertions = &value.static_assertions;

                quote!(
                    #( #static_assertions) *
                )
            })
            .collect::<Vec<_>>();

        quote!(#( #static_assertions) *)
    }

    pub(crate) fn generate_token_stream(&self) -> TokenStream {
        let node_edge_token_streams = self.0.values().map(|value| {
            let NodeEdgeMetadata {
                    origin_struct_ident,
                    direction,
                    edge_relation_model_selected_ident,
                    destination_node_schema,
                    foreign_node_connection_method,
                    imports,
                    edge_name_as_method_ident,
                    edge_table_name,
                    ..
            }: &NodeEdgeMetadata = value;
            
            let crate_name = get_crate_name(false);
            let arrow = format!("{}", direction);
            let edge_table_name_str = format!("{}", &edge_table_name);
            let  edge_name_as_struct_original_ident = format_ident!("{}", &edge_table_name.to_string().to_case(Case::Pascal));
            let  edge_name_as_struct_with_direction_ident = format_ident!("{}",
                                                                          self.add_direction_indication_to_ident(
                                                                                  &edge_table_name
                                                                                  .to_string()
                                                                                  .to_case(Case::Pascal),
                                                                              direction,
                                                                              )
                                                                          );
            let edge_inner_module_name = format_ident!("{}_schema________________", edge_name_as_struct_with_direction_ident.to_string().to_lowercase());
            
            let VariablesModelMacro { 
                __________connect_edge_to_graph_traversal_string, 
                ___________graph_traversal_string, 
                .. 
            } = VariablesModelMacro::new();
            
            
             quote!(
                #( #imports) *
                 
                // Edge to Node
                impl #origin_struct_ident {
                    pub fn #edge_name_as_method_ident(
                        &self,
                        clause: impl Into<#crate_name::EdgeClause>,
                    ) -> #edge_inner_module_name::#edge_name_as_struct_with_direction_ident {
                        let clause: #crate_name::EdgeClause = clause.into();
                        let clause = clause.with_arrow(#arrow).with_table(#edge_table_name_str);
                        
                        // i.e Edge to Node
                        #edge_inner_module_name::#edge_name_as_struct_original_ident::#__________connect_edge_to_graph_traversal_string(
                            self,
                            clause,
                        ).into()
                    }
                }
                
                mod #edge_inner_module_name {
                    #( #imports) *
                    use #crate_name::Parametric as _;
                    use #crate_name::Buildable as _;
                    use #crate_name::Erroneous as _;
                    
                    #( #destination_node_schema) *
                    
                    pub type #edge_name_as_struct_original_ident = <super::super::#edge_relation_model_selected_ident as #crate_name::SurrealdbEdge>::Schema; 

                    pub struct #edge_name_as_struct_with_direction_ident(#edge_name_as_struct_original_ident);
                    
                    
                    impl From<#edge_name_as_struct_original_ident> for #edge_name_as_struct_with_direction_ident {
                        fn from(value: #edge_name_as_struct_original_ident) -> Self {
                            Self(value)
                        }
                    }
                    
                    impl #crate_name::Buildable for #edge_name_as_struct_with_direction_ident {
                        fn build(&self) -> ::std::string::String {
                            self.0.build()
                        }
                    }
            
                    impl #crate_name::Parametric for #edge_name_as_struct_with_direction_ident {
                        fn get_bindings(&self) -> #crate_name::BindingsList {
                            self.0.get_bindings()
                        }
                    }
                    
                    impl #crate_name::Erroneous for #edge_name_as_struct_with_direction_ident {
                        fn get_errors(&self) -> Vec<::std::string::String> {
                            self.0.get_errors()
                        }
                    }
             
                    impl #crate_name::Buildable for &#edge_name_as_struct_with_direction_ident {
                        fn build(&self) -> ::std::string::String {
                            self.0.build()
                        }
                    }
            
                    impl #crate_name::Parametric for &#edge_name_as_struct_with_direction_ident {
                        fn get_bindings(&self) -> #crate_name::BindingsList {
                            self.0.get_bindings()
                        }
                    }
                    
                    impl #crate_name::Erroneous for &#edge_name_as_struct_with_direction_ident {
                        fn get_errors(&self) -> Vec<::std::string::String> {
                            self.0.get_errors()
                        }
                    }
             
                    impl ::std::ops::Deref for #edge_name_as_struct_with_direction_ident {
                        type Target = #edge_name_as_struct_original_ident;

                        fn deref(&self) -> &Self::Target {
                            &self.0
                        }
                    }

                    impl #edge_name_as_struct_with_direction_ident {
                        #( #foreign_node_connection_method) *
                 
                         // This is for recurive edge traversal which is supported by surrealdb: e.g ->knows(..)->knows(..)->knows(..)
                        // -- Select all 1st, 2nd, and 3rd level people who this specific person record knows, or likes, as separate outputs
                        // SELECT ->knows->(? AS f1)->knows->(? AS f2)->(knows, likes AS e3 WHERE influencer = true)->(? AS f3) FROM person:tobie;
                        pub fn #edge_name_as_method_ident(
                            &self,
                            clause: impl Into<#crate_name::EdgeClause>,
                        ) -> #edge_name_as_struct_with_direction_ident {
                            let clause: #crate_name::EdgeClause = clause.into();
                            let clause = clause.with_arrow(#arrow).with_table(#edge_table_name_str);
                            
                            // i.e Edge to Edge
                            #edge_name_as_struct_original_ident::#__________connect_edge_to_graph_traversal_string(
                                self,
                                clause,
                            ).into()
                        }
                    }
                }
                
            )
        }).collect::<Vec<_>>();

        quote!(#( #node_edge_token_streams) *)
    }
}
