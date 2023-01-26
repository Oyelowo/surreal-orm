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
    
    
    /// edge schema struct aliased type import
    /// ```
    /// type Writes = super::writes::Writes<Student>;
    /// ```
    pub relate_edge_struct_type_alias : Vec<TokenStream>,
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
    pub(crate) fn from_receiver_data(
        data: &ast::Data<util::Ignored, MyFieldReceiver>,
        struct_level_casing: Option<CaseString>,
        struct_name_ident: &syn::Ident,
    ) -> Self {
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .into_iter()
            .fold(Self::default(), |mut acc, field_receiver| {
                let field_ident = field_receiver.ident.as_ref().unwrap();
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
                let ref field_ident_normalised = format_ident!("{original_field_name_normalised}");
                let relationship = RelationType::from(field_receiver);

                let ref field_ident_normalised_as_str =
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
                        let ref destination_node = TokenStream::from(relation_attributes.node_name.clone());
                        // let extra = ReferencedNodeMeta::from_ref_node_meta(relation_attributes.node_name, field_ident_normalised);
                        //
                        // let destination_node = relation_attributes.node_name ;
                        let ref struct_name = quote!(#struct_name_ident);
                        // let schema_name_basic = &extra.schema_name;
                        let field_name_as_alias = format_ident!("__as_{field_ident_normalised_as_str}__");
                        
                        acc.relate_node_alias_method.push(quote!(
                                    pub fn #field_name_as_alias(&self) -> String {
                                        format!("{self} AS {}", #field_ident_normalised_as_str)
                                    })
                            );

                        // e.g type Writes = super::WritesSchema<#struct_name_ident>;
                        // TODO: remove or reuse if makes sense: let edge_schema = format_ident!("{edge_name}Schema");
                        let edge_module_name = format_ident!("{}", edge_name.to_string().to_lowercase());
                        acc.relate_edge_struct_type_alias.push(quote!(
                            type #edge_name = super::#edge_module_name::#edge_name<#struct_name_ident>;
                        ));
                        
                        acc.relate_edge_schema_struct_alias_impl.push(quote!(
                                    impl #edge_name {
                                        // Could potantially make the method name all small letters
                                        // or just use exactly as the table name is written
                                        pub fn #destination_node(&self, clause: #crate_name::Clause) -> #destination_node {
                                           #destination_node::__________update_connection(&self.__________store, clause)
                                        }
                                    })
                                );
                        

                        let edge_method_name_with_direction = match relation_attributes.edge_direction {
                            super::node_relations::EdgeDirection::OutArrowRight => format_ident!("{edge_name}__"),
                            super::node_relations::EdgeDirection::InArrowLeft => format_ident!("__{edge_name}"),
                        };
                        
                        acc.relate_edge_schema_method_connection.push(quote!(
                                    pub fn #edge_method_name_with_direction(&self, clause: #crate_name::Clause) -> #edge_name {
                                        #edge_name::__________update_edge(
                                            &self.___________store,
                                            clause,
                                            #crate_name::EdgeDirection::OutArrowRight,
                                        )
                                    }
                                )
                            );
                        
                        // e.g from Writes<In, Out> (Writes<Student, Book>) generics, we can create StudentWritesBook
                        let edge_alias_specific = format_ident!("{}", relation.edge.as_ref().expect("Edge must be specified for relations"));
                        // type StudentWritesBlogInNode = <StudentWritesBlog as SurrealdbEdge>::In;
                        let (in_node, out_node) = match relation_attributes.edge_direction {
                            // If OutArrowRight, the current struct should be InNode, and
                            // OutNode in "->edge_action->OutNode", should be OutNode
                            super::node_relations::EdgeDirection::OutArrowRight => {
                                (struct_name, destination_node)
                            }
                            super::node_relations::EdgeDirection::InArrowLeft => (destination_node, struct_name),
                        };
                        
                        let relation_alias_struct_renamed = format_ident!("{}TableName", edge_alias_specific);
                        let relation_alias_struct_in_node = format_ident!("{}InNode", edge_alias_specific);
                        let relation_alias_struct_out_node = format_ident!("{}OutNode", edge_alias_specific);
                        
                        acc.static_assertions.push(quote!(
                                type #relation_alias_struct_renamed = <#edge_alias_specific as #crate_name::SurrealdbEdge>::TableNameChecker;
                                ::static_assertions::assert_fields!(#relation_alias_struct_renamed: #edge_name);

                                // ::static_assertions::assert_type_eq_all!(<StudentWritesBook as SurrealdbEdge>::In, Student);
                                // ::static_assertions::assert_type_eq_all!(<StudentWritesBook as SurrealdbEdge>::Out, Book);
                                // type EdgeCheckerAlias = <AccountManageProject as Edge>::EdgeChecker;
                                type #relation_alias_struct_in_node = <#edge_alias_specific as #crate_name::SurrealdbEdge>::In;
                                ::static_assertions::assert_type_eq_all!(#relation_alias_struct_in_node, #in_node);

                                type #relation_alias_struct_out_node = <#edge_alias_specific as #crate_name::SurrealdbEdge>::Out;
                                ::static_assertions::assert_type_eq_all!(#relation_alias_struct_out_node, #out_node);

                                ::static_assertions::assert_impl_one!(#edge_alias_specific: #crate_name::SurrealdbEdge);
                                ::static_assertions::assert_impl_one!(#in_node: #crate_name::SurrealdbNode);
                                ::static_assertions::assert_impl_one!(#out_node: #crate_name::SurrealdbNode);
                                
                                // assert field type and attribute reference match
                                // e.g Relate<Book> should match from attribute link = "->Writes->Book"
                                ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::Relate<#destination_node>);
                            )
                        );

                            ReferencedNodeMeta::from_relate(relation, destination_node)
                                
                    },
                    RelationType::LinkOne(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised)
                    }
                    RelationType::LinkSelf(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised)
                    }
                    RelationType::LinkMany(node_object) => {
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised)
                    }
                    RelationType::None => ReferencedNodeMeta::default(),
                };
                
                acc.static_assertions.push(referenced_node_meta.destination_node_type_validator);

                acc.schema_struct_fields_types_kv
                    .push(quote!(pub #field_ident_normalised: #crate_name::DbField).into());

                acc.schema_struct_fields_names_kv
                    .push(quote!(#field_ident_normalised: #field_ident_normalised_as_str.into()).into());

                acc.serialized_field_names_normalised
                    .push(field_ident_normalised_as_str.to_owned());

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
    record_link_default_alias_as_method: TokenStream,
    destination_node_type_validator: TokenStream,
}

impl ReferencedNodeMeta {
    fn from_relate(relate: Relate, destination_node: &TokenStream)->Self {
        let crate_name = get_crate_name(false);
        // let destination_node_schema_import = quote!();
        // let schema_name = relate
            Self{ 
                destination_node_schema_import:  quote!(
                        type #destination_node = <super::#destination_node as #crate_name::SurrealdbNode>::Schema;
                    ), 
                
                record_link_default_alias_as_method: quote!(), 

                destination_node_type_validator: quote!(),
            }
    }
    
    fn from_record_link(
        node_name: &super::node_relations::NodeName,
        normalized_field_name: &::syn::Ident,
    ) -> Self {
        let schema_name = format_ident!("{node_name}");
        let crate_name = get_crate_name(false);
        
        Self {
            // imports for specific schema from the trait Generic Associated types e.g
            // type Book = <super::Book as SurrealdbNode>::Schema;
            destination_node_schema_import: quote!(
                        type #schema_name = <super::#schema_name as #crate_name::SurrealdbNode>::Schema;
                    ),
                    
            destination_node_type_validator: quote!(::static_assertions::assert_impl_one!(#schema_name: #crate_name::SurrealdbNode)),
            
            record_link_default_alias_as_method: quote!(
                        pub fn #normalized_field_name(&self, clause: #crate_name::Clause) -> #schema_name {
                            #schema_name:__________update_connection(&self.__________store, clause)
                        }
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
