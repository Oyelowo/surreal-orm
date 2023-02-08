/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use std::{hash::Hash,
collections::HashMap};

use convert_case::{Casing, Case};
use darling::{ast, util};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Token};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    get_crate_name,
     relations::{EdgeDirection, NodeName,RelationType, RelateAttribute}, attributes::{MyFieldReceiver, Relate, ReferencedNodeMeta, NormalisedField}, variables::VariablesModelMacro,
};


// "Writes__": {
//   type: "StudentWritesBook",
//   action: "writes",
//   direction: "right",
//   action_type_alias: quote!( type Writes__ = <StudentWritesBook as #crate_name::SurrealdbEdge>::Schema; ),
//   foreign_node_schema: vec![
//       quote!(
//         type BookModel = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
//         type Book = <BookModel as surrealdb_macros::SurrealdbNode>::Schema;
//       ),
//       quote!(
//         type BlogModel = <StudentWritesBlog as surrealdb_macros::SurrealdbEdge>::Out;
//         type Blog = <BlogModel as surrealdb_macros::SurrealdbNode>::Schema;
//       ),
//   ],
//   edge_to_nodes_trait_methods: vec![
//       quote!(
//          fn book(&self, clause: Clause) -> Book;
//       ),
//       quote!(
//          fn blog(&self, clause: Clause) -> Blog;
//       ),
//   ],
//   edge_to_nodes_trait_methods_impl: vec![
//       quote!(
//          fn book(&self, clause: Clause) -> Book {
//              Book::__________connect_to_graph_traversal_string(
//                  &self.___________graph_traversal_string,
//                  clause,
//              )
//          }
//       ),
//       quote!(
//          fn blog(&self, clause: Clause) -> Blog {
//              Blog::__________connect_to_graph_traversal_string(
//                  &self.___________graph_traversal_string,
//                  clause,
//              )
//          }
//       ),
//
//   ],
// },
#[derive(Clone, Debug)]
struct NodeEdgeMetadata {
  /// e.g if edge table name is writes, this could be Writes__ or __Writes depending on the arrow
  /// direction 
  edge_name_with_direction_indicator: String,
  /// In addition to action, this is used for building an edge from 
  /// a model field annotation e.g relate(edge="StudentWritesBook", link="->writes->book") 
  /// Example of value: StudentWritesBook
  /// Generates: 
  /// ```
  /// type Writes__ = <StudentWritesBook as SurrealdbEdge>::Schema;
  /// ```
  connection_model: syn::Ident, 
  /// The database table name of the edge. Used for generating other tokens
  /// e.g "writes"
  action: String,
  direction: EdgeDirection,
  /// Example Generated:
  /// type Writes__ = <StudentWritesBook as #crate_name::SurrealdbEdge>::Schema;
  /// 
  /// Example Value:
  /// if arrow right outgoing:
  /// quote!( type Writes__ = <StudentWritesBook as #crate_name::SurrealdbEdge>::Schema; )
  /// 
  /// if arrow left incoming:
  /// quote!( type __Writes = <StudentWritesBook as #crate_name::SurrealdbEdge>::Schema; ) 
  action_type_alias: TokenStream,
  /// Example Generated:
  ///   type BookModel = <StudentWritesBook as surrealdb_macros::SurrealdbEdge>::Out;
  ///   type Book = <BookModel as surrealdb_macros::SurrealdbNode>::Schema;
  ///
  ///   type BlogModel = <StudentWritesBlog as surrealdb_macros::SurrealdbEdge>::Out;
  ///   type Blog = <BlogModel as surrealdb_macros::SurrealdbNode>::Schema;
  ///
  /// Example Value:
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
  foreign_node_schema:  Vec<TokenStream>,
  /// Example to be Generated:
  /// trait WriteArrowRightTrait {
  ///     fn book(&self, clause: #crate::Clause) -> Book;
  ///     fn blog(&self, clause: #crate::Clause) -> Blog;
  /// }
  /// 
  /// Example Value:
  /// vec![
  ///     quote!(trait WriteArrowRightTrait {), // trait opening braces
  ///     quote!(
  ///        fn book(&self, clause: Clause) -> Book;
  ///     ),
  ///     quote!(
  ///        fn blog(&self, clause: Clause) -> Blog;
  ///     ),
  ///     quote!(}),  // Closing braces
  /// ] 
  edge_to_nodes_trait_method: Vec<TokenStream>,
  /// Example Generated:
  ///
  /// impl WriteArrowRightTrait for Writes__ {
  ///     fn book(&self, clause: Clause) -> Book {
  ///         Book::__________connect_to_graph_traversal_string(
  ///             &self.___________graph_traversal_string,
  ///             clause,
  ///         )
  ///     }
  ///
  ///     fn blog(&self, clause: Clause) -> Blog {
  ///         Blog::__________connect_to_graph_traversal_string(
  ///             &self.___________graph_traversal_string,
  ///             clause,
  ///         )
  ///     }
  /// }
  ///
  /// Example Value:
  /// vec![
  ///     quote!(impl WriteArrowRightTrait for Writes__ {), // implementation opening braces
  ///     quote!(
  ///        fn book(&self, clause: Clause) -> Book {
  ///            Book::__________connect_to_graph_traversal_string(
  ///                &self.___________graph_traversal_string,
  ///                clause,
  ///            )
  ///        }
  ///     ),
  ///     quote!(
  ///        fn blog(&self, clause: Clause) -> Blog {
  ///            Blog::__________connect_to_graph_traversal_string(
  ///                &self.___________graph_traversal_string,
  ///                clause,
  ///            )
  ///        }
  ///     ),
  ///     quote!(}),  // Closing braces
  ///    ]
  edge_to_nodes_trait_methods_impl:Vec<TokenStream>, 
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
    /// Used when you use a SurrealdbNode in field e.g: favourite_book: LinkOne<Book>,
    /// e.g: type Book = <super::Book as SurrealdbNode>::Schema;
    pub imports_referenced_node_schema: Vec<TokenStream>,
    
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
    /// pub fn best_student(&self, clause: Clause) -> Student {
    ///     Student::__________connect_to_graph_traversal_string(&self.___________graph_traversal_string, clause)
    /// }
    /// ```
    pub record_link_fields_methods: Vec<TokenStream>,
    /// Generated example: 
    /// ```
    /// type Writes = super::writes_schema::Writes<Student>;
    /// ```
    /// The above is generated if a Student struct field uses "->Writes->Book". 
    /// Must be unique to prevent collision because it's possible for an edge to be
    /// reused.
    // NOTE: Replaced with relate_edge_struct_type_alias. Remove
    // pub referenced_edge_schema_struct_alias: Vec<TokenStream>,
    
    
    /// Used for importing and aliasing edge schema used in present SurrealdbNode. 
    /// The generic represents the current SurrealdbNode e.g Student
    /// Note: Still considering either of the two options 
    /// type Writes = super::writes::Writes<Student>;
    /// type Writes = super::WritesSchema<#struct_name_ident>;
    /// ```
    /// type Writes = super::writes::Writes<Student>;
    /// ```
    pub relate_edge_schema_struct_type_alias : Vec<TokenStream>,
    /// Generated example:
    /// ```
    ///impl Writes {
    ///     pub fn book(&self, clause: #crate_name::Clause) -> Book {
    ///         Book::__________update_edge(&self.__________update_connection, clause)
    ///     }
    /// }
    /// ```
    /// This helps to connect present origin node struct to destination node
    /// and it the edge itself is a struct here. This allows us to give more
    /// specific autocompletion when user accesses available destination node 
    /// from a specific edge from an origin struct.
    /// e.g Student::schema().writes__().book();
    /// This allows us to do `.book()` as shown above
    pub relate_edge_schema_struct_type_alias_impl: Vec<TokenStream>,
    
    /// Genearated example:
    /// ```
    /// pub fn writes__(&self, clause: Clause) -> Writes {
    ///     Writes::__________connect_to_graph_traversal_string(
    ///         &self.___________graph_traversal_string,
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
    
    pub node_edge_metadata: NodeEdgeMetadataStore
}

type AllNodeEdgeMetadata = HashMap<&'static str, NodeEdgeMetadata>;


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
        let SchemaPropertiesArgs {  data, struct_level_casing, struct_name_ident }= args;
        
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .into_iter()
            .fold(Self::default(), |mut store, field_receiver| {
                let field_type = &field_receiver.ty;
                let crate_name = get_crate_name(false);
                let relationship = RelationType::from(field_receiver);
                let NormalisedField { 
                         ref field_ident_normalised,
                         ref field_ident_normalised_as_str
                } = NormalisedField::from_receiever(field_receiver, struct_level_casing);
                
                // println!("ddLAKALAK----Struct Name{}{:?}", struct_name_ident, field_receiver.ident.clone());
                let VariablesModelMacro { 
                    __________connect_to_graph_traversal_string, 
                    ___________graph_traversal_string, 
                    schema_instance, .. 
                } = VariablesModelMacro::new();
                
                let referenced_node_meta = match relationship {
                    RelationType::Relate(relation) => {
                        let relation_attributes = RelateAttribute::from(&relation);
                        let arrow_direction = String::from(&relation_attributes.edge_direction);
                        let edge_name = TokenStream::from(&relation_attributes.edge_name);
                        let ref destination_node = TokenStream::from(relation_attributes.node_name.clone());
                        // let extra = ReferencedNodeMeta::from_ref_node_meta(relation_attributes.node_name, field_ident_normalised);
                        //
                        // let destination_node = relation_attributes.node_name ;
                        let ref struct_name = quote!(#struct_name_ident);
                        // let schema_name_basic = &extra.schema_name;
                        let field_name_as_alias = format_ident!("__as_{field_ident_normalised_as_str}__");
                        
                        // TODO: Check ===> this should probably be only: AS <the deserialized name format>
                        store.relate_node_alias_method.push(quote!(
                                    pub fn #field_name_as_alias(&self) -> ::std::string::String {
                                        format!("{} AS {}", self, #field_ident_normalised_as_str)
                                    })
                            );

                        // e.g type Writes = super::WritesSchema<#struct_name_ident>;
                        let edge_schema_alias_name = VariablesModelMacro::get_schema_alias(&format_ident!("{edge_name}"));
                        
                        store.relate_edge_schema_struct_type_alias.push(quote!(
                            type #edge_name = super::#edge_schema_alias_name<#struct_name_ident>;
                        ));
                        
                        store.relate_edge_schema_struct_type_alias_impl.push(quote!(
                                    impl #edge_name {
                                        // Could potantially make the method name all small letters
                                        // or just use exactly as the table name is written
                                        pub fn #destination_node(&self, clause: #crate_name::Clause) -> #destination_node {
                                           #destination_node::#__________connect_to_graph_traversal_string(&self.#___________graph_traversal_string, clause)
                                        }
                                    })
                                );
                        

                        let edge_method_name_with_direction = match relation_attributes.clone()
                            .edge_direction {
                            EdgeDirection::OutArrowRight => format_ident!("{edge_name}__"),
                            EdgeDirection::InArrowLeft => format_ident!("__{edge_name}"),
                        };
                        
                        // let edge_type_alias = format_ident!("{}", relation_attributes.edge_name.to_string());
                        store.relate_edge_schema_method_connection.push(quote!(
                                    pub fn #edge_method_name_with_direction(&self, clause: #crate_name::Clause) -> #edge_method_name_with_direction {
                                        #edge_name::#__________connect_to_graph_traversal_string(
                                            &self.#___________graph_traversal_string,
                                            clause,
                                            #arrow_direction,
                                        )
                                    }
                                )
                            );
                        
                        // e.g from Writes<In, Out> (Writes<Student, Book>) generics, we can create StudentWritesBook
                        let edge_alias_specific = format_ident!("{}", relation.model.as_ref().expect("Edge must be specified for relations"));
                        // type StudentWritesBlogInNode = <StudentWritesBlog as SurrealdbEdge>::In;
                        let (in_node, out_node) = match relation_attributes.clone().edge_direction {
                            // If OutArrowRight, the current struct should be InNode, and
                            // OutNode in "->edge_action->OutNode", should be OutNode
                            EdgeDirection::OutArrowRight => {
                                (struct_name, destination_node)
                            }
                            EdgeDirection::InArrowLeft => (destination_node, struct_name),
                        };
                        
                        let relation_alias_struct_renamed = format_ident!("{}TableName", edge_alias_specific);
                        let relation_alias_struct_in_node = format_ident!("{}InNode", edge_alias_specific);
                        let relation_alias_struct_out_node = format_ident!("{}OutNode", edge_alias_specific);
                        
                        store.static_assertions.push(quote!(
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

                            store.node_edge_metadata.update(&relation);
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
                
                store.static_assertions.push(referenced_node_meta.destination_node_type_validator);
                store.imports_referenced_node_schema
                    .push(referenced_node_meta.destination_node_schema_import.into());

                store.record_link_fields_methods
                    .push(referenced_node_meta.record_link_default_alias_as_method.into());
  
                store.schema_struct_fields_types_kv
                    .push(quote!(pub #field_ident_normalised: #crate_name::DbField, ));
  
                store.schema_struct_fields_names_kv
                    .push(quote!(#field_ident_normalised: #field_ident_normalised_as_str.into(),));

                store.serialized_field_names_normalised
                    .push(field_ident_normalised_as_str.to_owned());

                store.connection_with_field_appended
                    .push(quote!(
                               #schema_instance.#field_ident_normalised
                                     .push_str(format!("{}.{}", #___________graph_traversal_string, #field_ident_normalised_as_str).as_str());));
  

                store 
            });
    fields
    }
} 

type EdgeNameWithDirectionIndicator = String;

#[derive(Default, Clone)]
pub struct NodeEdgeMetadataStore(HashMap<EdgeNameWithDirectionIndicator, NodeEdgeMetadata>);

impl NodeEdgeMetadataStore {
      fn update(&mut self,  relation: &Relate) ->&Self{
            let connection_model = format_ident!("{}", relation.model.as_ref().unwrap());
            let relation_attributes = RelateAttribute::from(relation);
            // let arrow_direction = String::from(relation_attributes.edge_direction);
            let edge_name = TokenStream::from(&relation_attributes.edge_name);
            let ref destination_node = TokenStream::from(&relation_attributes.node_name);
            
            // The dunder "__" suffix/prefix determines the arrow direction. Suffixed dunder indicates
            // Outgoing arrow right while prefixed implies incoming arrow left
            let crate_name = get_crate_name(false);
            let edge_name_with_direction_title_case = edge_name.to_string().to_case(Case::Title);
            
            let ref edge_direction = relation_attributes.edge_direction;
            let ref edge_name_with_direction_indicator =|| match edge_direction {
                EdgeDirection::OutArrowRight => format!("{edge_name}__"),
                EdgeDirection::InArrowLeft => format!("__{edge_name}"),
            };
            let destination_node_title_case = destination_node.to_string().to_case(Case::Title);
            let destination_node_model = format_ident!("{destination_node}Model");
            
            let VariablesModelMacro { 
                __________connect_to_graph_traversal_string, 
                ___________graph_traversal_string, 
                schema_instance, .. 
            } = VariablesModelMacro::new();
            // Within edge generics, there is usually In and Out associated types, this is used to access
            // those
            let in_or_out = match edge_direction {
                EdgeDirection::OutArrowRight => format_ident!("Out"),
                EdgeDirection::InArrowLeft => format_ident!("In"),
            };
            let foreign_node_schema_one = || quote!(
                                type #destination_node_model = <#connection_model as surrealdb_macros::SurrealdbEdge>::#in_or_out;
                                type #destination_node = <#destination_node_model as surrealdb_macros::SurrealdbNode>::Schema;
                                );
            
            let edge_to_nodes_trait_method_one = ||  quote!(fn #destination_node(&self, clause: #crate_name::Clause) -> #destination_node_title_case;);
            
            let edge_to_nodes_trait_methods_impl_one = || quote!(
                                    fn #destination_node(&self, clause: #crate_name::Clause) -> #destination_node_title_case {
                                        #destination_node_title_case::#__________connect_to_graph_traversal_string(
                                                    &self.#___________graph_traversal_string,
                                                    clause,
                                        )
                                    }
                                );
            
            let nn = NodeEdgeMetadata {
                      edge_name_with_direction_indicator: edge_name_with_direction_indicator() ,  // Closing braces
                      connection_model: format_ident!("{}",relation.connection), 
                      action: relation_attributes.edge_name.clone().to_string(), 
                      direction: edge_direction.clone(), 
                      // We only need to take one connection_model for each edge e.g `writes` ,since
                      // they should share similar Schema besides `in` and `out` which are just
                      // defaulted to ids for edges because, one can always access in and out
                      // models indirectly from the origin and destination nodes rather than the
                      // edges themselves. This allows us to minmise confusion
                      action_type_alias: quote!( type #edge_name_with_direction_title_case = <#connection_model as #crate_name::SurrealdbEdge>::Schema; ), 
                      foreign_node_schema: vec![foreign_node_schema_one()], 
                      edge_to_nodes_trait_method: vec![
                                edge_to_nodes_trait_method_one()
                      ], 
                      edge_to_nodes_trait_methods_impl: vec![
                                edge_to_nodes_trait_methods_impl_one() 
                        ],
            };
        //     let values = match map.entry(key) {
        //     Entry::Occupied(o) => o.into_mut(),
        //     Entry::Vacant(v) => v.insert(default),
        // };
             self.0.entry(edge_name_with_direction_indicator()).and_modify(|x| {
                // x.direction = EdgeDirection::OutArrowRight;
                x.foreign_node_schema.push(foreign_node_schema_one());
                x.edge_to_nodes_trait_method.push(edge_to_nodes_trait_method_one());
                x.edge_to_nodes_trait_methods_impl.push(edge_to_nodes_trait_methods_impl_one());
            }).or_insert(nn);
                // xx
                self
      }     

  
    pub(crate) fn generate_token_stream(&self) -> TokenStream{
        let node_edge_token_streams = self.0.values().map(|value| {
            let edge_name_with_direction_indicator = format!("{}", value.edge_name_with_direction_indicator );
            let edge_trait_name = format_ident!("{edge_name_with_direction_indicator}Trait");
            let NodeEdgeMetadata {
                    edge_to_nodes_trait_methods_impl,
                    edge_name_with_direction_indicator,
                    edge_to_nodes_trait_method,
                    action_type_alias,
                    foreign_node_schema,
                    ..
            } = value;
             quote!(
                    #action_type_alias
                
                    #( #foreign_node_schema) *

                    trait #edge_trait_name {
                        #( #edge_to_nodes_trait_method) *
                    }

                    impl #edge_trait_name for #edge_name_with_direction_indicator{
                        #( #edge_to_nodes_trait_methods_impl) *
                    }
                
                )
        }).collect::<Vec<_>>();
        
        quote!(#( #node_edge_token_streams) *)
    }
}
