/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/


use std::{collections::{HashMap, hash_map::Entry, HashSet}, fmt::Display, ops::Deref};

use convert_case::{Casing, Case};
use darling::{ast, util, ToTokens};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    get_crate_name,
    casing::CaseString,
    attributes::{ MyFieldReceiver, Relate, ReferencedNodeMeta, NormalisedField },
    variables::VariablesModelMacro,
    relations::{EdgeDirection, RelationType, RelateAttribute}
};


#[derive(Clone, Debug)]
struct NodeEdgeMetadata {
  /// Example value: writes 
  edge_table_name:  syn::Ident,
  /// The current struct name ident.
  /// e.g given: struct Student {  }, value = Student
  origin_struct_ident:  syn::Ident,
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
  destination_node_schema:  Vec<TokenStream>,
  /// Example Generated:
  ///
  /// ```
  /// impl Writes__ {
  ///     fn book(&self, filter: DbFilter) -> Book {
  ///         Book::__________connect_to_graph_traversal_string(
  ///             &self.___________graph_traversal_string,
  ///             filter,
  ///         )
  ///     }
  ///
  ///     fn blog(&self, filter: DbFilter) -> Blog {
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
  ///        fn book(&self, filter: DbFilter) -> Book {
  ///            Book::__________connect_to_graph_traversal_string(
  ///                &self.___________graph_traversal_string,
  ///                filter,
  ///            )
  ///        }
  ///     ),
  ///     quote!(
  ///        fn blog(&self, filter: DbFilter) -> Blog {
  ///            Blog::__________connect_to_graph_traversal_string(
  ///                &self.___________graph_traversal_string,
  ///                filter,
  ///            )
  ///        }
  ///     ),
  ///    ]
  /// ```
  foreign_node_connection_method:Vec<TokenStream>, 
  static_assertions: Vec<TokenStream>,
  imports: Vec<TokenStream>,
  edge_name_as_method_ident: syn::Ident 
}

#[derive(Default, Clone)]
pub struct SchemaFieldsProperties {
    /// list of fields names that are actually serialized and not skipped.
    pub serialized_field_name_no_skip: Vec<String>,
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
    /// pub fn best_student(&self, filter: DbFilter) -> Student {
    ///     Student::__________connect_to_graph_traversal_string(&self.___________graph_traversal_string, filter)
    /// }
    /// ```
    pub record_link_fields_methods: Vec<TokenStream>,
    pub node_edge_metadata: NodeEdgeMetadataStore
}


#[derive(Clone)]
pub struct TokenStreamHashable(TokenStream);

impl ToTokens for TokenStreamHashable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.clone());
    }
}

impl Deref for TokenStreamHashable {
    type Target=TokenStream;

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
    pub table_name_ident: &'a syn::Ident,
}
impl SchemaFieldsProperties {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub(crate) fn from_receiver_data(
        args: SchemaPropertiesArgs
    ) -> Self {
        let SchemaPropertiesArgs {  data, struct_level_casing, struct_name_ident, ..  } = args;
        
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
                    __________connect_to_graph_traversal_string, 
                    ___________graph_traversal_string, 
                    ____________update_many_bindings,
                    schema_instance, 
                    bindings,
                    .. 
                } = VariablesModelMacro::new();
                
                let referenced_node_meta = match relationship {
                    RelationType::Relate(relation) => {
                            store.node_edge_metadata.update(&relation, struct_name_ident, field_type);
                            ReferencedNodeMeta::default()
                                
                    },
                    RelationType::LinkOne(node_object) => {
                        let foreign_node = format_ident!("{node_object}");
                        store.static_assertions.push(quote!(::static_assertions::assert_type_eq_all!(#field_type, #crate_name::links::LinkOne<#foreign_node>);));
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised, struct_name_ident) 
                    }
                    RelationType::LinkSelf(node_object) => {
                        let foreign_node = format_ident!("{node_object}");
                        if node_object.to_string() != struct_name_ident.to_string() {
                            panic!("The field - `{field_name_original}` - has a linkself \
                                   attribute or type that is not pointing to the current struct. \
                                   Make sure the field attribute is link_self=\"{struct_name_ident}\" \
                                   and the type is LinkSelf<{struct_name_ident}>. ");
                        }
                        
                        store.static_assertions.push(quote!(::static_assertions::assert_type_eq_all!(#field_type, #crate_name::links::LinkSelf<#foreign_node>);));
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised, struct_name_ident) 
                    }
                    RelationType::LinkMany(node_object) => {
                        let foreign_node = format_ident!("{node_object}");
                        store.static_assertions.push(quote!(::static_assertions::assert_type_eq_all!(#field_type, #crate_name::links::LinkMany<#foreign_node>);));
                        ReferencedNodeMeta::from_record_link(&node_object, field_ident_normalised, struct_name_ident) 
                    }
                    RelationType::None => ReferencedNodeMeta::default(),
                };
                
                store.static_assertions.push(referenced_node_meta.foreign_node_type_validator);
                
                store.imports_referenced_node_schema
                    .insert(referenced_node_meta.foreign_node_schema_import.into());
                
                store.record_link_fields_methods
                    .push(referenced_node_meta.record_link_default_alias_as_method.into());
  
                store.schema_struct_fields_types_kv
                    .push(quote!(pub #field_ident_normalised: #crate_name::DbField, ));
  
                store.schema_struct_fields_names_kv
                    .push(quote!(#field_ident_normalised: #field_ident_normalised_as_str.into(),));
                
                store.schema_struct_fields_names_kv_empty
                    .push(quote!(#field_ident_normalised: "".into(),));

                store.serialized_field_names_normalised
                    .push(field_ident_normalised_as_str.to_owned());
                
                if !field_receiver.skip_serializing {
                    store.serialized_field_name_no_skip
                        .push(field_ident_normalised_as_str.to_owned());
                }

                store.connection_with_field_appended
                    .push(quote!(
                                #schema_instance.#field_ident_normalised =
                                    #crate_name::DbField::new(format!("{}.{}", #___________graph_traversal_string, #field_ident_normalised_as_str).as_str())
                                        .#____________update_many_bindings(#bindings);
                                ));

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
    fn add_direction_indication_to_ident (&self,  ident: &impl Display, edge_direction: &EdgeDirection) -> String { 
        match edge_direction {
            EdgeDirection::OutArrowRight => format!("{ident}__"),
            EdgeDirection::InArrowLeft => format!("__{ident}"),
        }
    }

    fn create_static_assertions(relation: &Relate, origin_struct_ident: &syn::Ident, field_type: &syn::Type) -> TokenStream {
        let crate_name = get_crate_name(false);
        let ref relation_model = format_ident!("{}", relation.model.as_ref().unwrap());
        let relation_attributes = RelateAttribute::from(relation);
        let ref edge_table_name = TokenStream::from(&relation_attributes.edge_table_name);
        let ref foreign_node_table_name = TokenStream::from(&relation_attributes.node_table_name);
        
        let edge_table_name_checker_ident = format_ident!("{}EdgeTableNameChecker", relation_model);
        let home_node_ident = format_ident!("{}HomeNode", relation_model);
        let home_node_table_name_checker_ident = format_ident!("{}HomeNodeTableNameChecker", relation_model);
        let foreign_node_ident = format_ident!("{}ForeignNode", relation_model);
        let foreign_node_table_name_checker_ident = format_ident!("{}ForeignNodeTableNameChecker", relation_model);
        
        let (home_node_associated_type_ident, foreign_node_associated_type_ident) = match &relation_attributes.edge_direction {
            EdgeDirection::OutArrowRight => {
               (format_ident!("In"), format_ident!("Out"))
            }
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
                        ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::Relate<#foreign_node_ident>);
                       ),
            ]; 
        quote!(
                #( #static_assertions) *
        )
    }
    
    fn update(&mut self,  relation: &Relate, origin_struct_ident: &syn::Ident, field_type: &syn::Type) ->&Self{
        let crate_name = get_crate_name(false);
        let ref relation_model = format_ident!("{}", relation.model.as_ref().unwrap());
        let relation_attributes = RelateAttribute::from(relation);
        let ref edge_table_name = TokenStream::from(&relation_attributes.edge_table_name);
        let ref destination_node_table_name = TokenStream::from(&relation_attributes.node_table_name);
        let ref edge_direction = relation_attributes.edge_direction;
        
        
        let ref edge_name_as_method_ident =||self.add_direction_indication_to_ident(edge_table_name, edge_direction); 
        
        
        
        // represents the schema but aliased as the pascal case of the destination table name
        let destination_node_schema_ident = format_ident!("{}", destination_node_table_name.to_string().to_case(Case::Pascal));
        // Meant to represent the variable of struct model(node) itself.
        let destination_node_model_ident = format_ident!("______________{destination_node_schema_ident}Model");
        
        let VariablesModelMacro { 
            __________connect_to_graph_traversal_string, 
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
        let destination_node_schema_one = || quote!(
                            type #destination_node_model_ident = <super::super::#relation_model as #crate_name::SurrealdbEdge>::#foreign_node_in_or_out;
                            type #destination_node_schema_ident = <#destination_node_model_ident as #crate_name::SurrealdbNode>::Schema;
                            );
        
        
        let foreign_node_connection_method = || quote!(
                                pub fn #destination_node_table_name(&self, clause: impl Into<#crate_name::Clause>) -> #destination_node_schema_ident {
                                    let clause: #crate_name::Clause = clause.into();
                                    
                                    #destination_node_schema_ident::#__________connect_to_graph_traversal_string(
                                                &self.#___________graph_traversal_string,
                                                clause,
                                                self.get_bindings(),
                                    )
                                }
                            );
        let static_assertions =||  Self::create_static_assertions(relation, origin_struct_ident, field_type); 
        
        // let imports =|| quote!(use super::StudentWritesBook;);
        let imports =|| quote!(use super::#relation_model;);
        
        let node_edge_meta = NodeEdgeMetadata {
                    edge_table_name: format_ident!("{}", &relation_attributes.edge_table_name.clone().to_string()), 
                    direction: edge_direction.clone(), 
                    destination_node_schema: vec![destination_node_schema_one()], 
                    foreign_node_connection_method: vec![ foreign_node_connection_method()],
                    origin_struct_ident: origin_struct_ident.to_owned(),
                    static_assertions: vec![static_assertions()],
                    edge_name_as_method_ident: format_ident!("{}", edge_name_as_method_ident()),
                    imports: vec![imports()],
                    edge_relation_model_selected_ident: relation_model.to_owned(),
        };
        
         match self.0.entry(edge_name_as_method_ident()) {
                Entry::Occupied(o) => {
                    let node_edge_meta = o.into_mut();
                    node_edge_meta.destination_node_schema.push(destination_node_schema_one());
                    node_edge_meta.foreign_node_connection_method.push(foreign_node_connection_method());
                    node_edge_meta.static_assertions.push(static_assertions());
                    node_edge_meta.imports.push(imports());
                },
                Entry::Vacant(v) => {v.insert(node_edge_meta);}
            };
            self
    }     

  
    pub(crate) fn generate_static_assertions(&self) -> TokenStream{

        let static_assertions = self.0.values().map(|value| {
            let static_assertions = &value.static_assertions;
            
            quote!(
                #( #static_assertions) *
            )
        }).collect::<Vec<_>>();

        quote!(#( #static_assertions) *)
    }
    
    pub(crate) fn generate_token_stream(&self) -> TokenStream{
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
                __________connect_to_graph_traversal_string, 
                ___________graph_traversal_string, 
                .. 
            } = VariablesModelMacro::new();
            
            
             quote!(
                #( #imports) *
                 
                impl #origin_struct_ident {
                    pub fn #edge_name_as_method_ident(
                        &self,
                        clause: impl Into<#crate_name::Clause>,
                    ) -> #edge_inner_module_name::#edge_name_as_struct_with_direction_ident {
                        let clause: #crate_name::DbFilter = clause.into();
                        
                        #edge_inner_module_name::#edge_name_as_struct_original_ident::#__________connect_to_graph_traversal_string(
                            &self.#___________graph_traversal_string,
                            clause,
                            #arrow,
                            self.get_bindings()
                        ).into()
                    }
                }
                
                mod #edge_inner_module_name {
                    #( #imports) *
                    use #crate_name::Parametric as _;
                    
                    #( #destination_node_schema) *
                    
                    pub type #edge_name_as_struct_original_ident = <super::super::#edge_relation_model_selected_ident as #crate_name::SurrealdbEdge>::Schema; 

                    pub struct #edge_name_as_struct_with_direction_ident(#edge_name_as_struct_original_ident);
                    
                    
                    impl From<#edge_name_as_struct_original_ident> for #edge_name_as_struct_with_direction_ident {
                        fn from(value: #edge_name_as_struct_original_ident) -> Self {
                            Self(value)
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
                    }
                }
                
            )
        }).collect::<Vec<_>>();
        
        quote!(#( #node_edge_token_streams) *)
    }
}
