/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use std::{collections::HashSet, hash::Hash};

use darling::{ast, util};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    edge::MyFieldReceiver,
    get_crate_name,
    relations::{RelateAttribute, RelationType},
    serialize_skipper::SkipSerializing,
};

#[derive(Default, Clone)]
pub(crate) struct ModelImport(TokenStream);

impl From<TokenStream> for ModelImport {
    fn from(value: TokenStream) -> Self {
        Self(value)
    }
}

impl From<ModelImport> for TokenStream {
    fn from(value: ModelImport) -> Self {
        value.0
    }
}
impl PartialEq for ModelImport {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}
impl Eq for ModelImport {}

impl Hash for ModelImport {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

/// A struct that contains the `struct_ty_fields` and `struct_values_fields` vectors.
#[derive(Default, Clone)]
pub(crate) struct SchemaFieldsProperties {
    // Generated example: pub timeWritten: DbField,
    // key(normalized_field_name)-value(DbField) e.g pub out: DbField, of field name and DbField type
    // to build up struct for generating fields of a Schema of the SurrealdbEdge
    // The full thing can look like:
    //     #[derive(Debug, Default)]
    //     pub struct Writes<Model: ::serde::Serialize + Default> {
    //                pub id: Dbfield,
    //                pub r#in: Dbfield,
    //                pub out: Dbfield,
    //                pub timeWritten: Dbfield,
    //          }
    pub schema_struct_fields_types_kv: TokenStream,

    // Generated example: pub timeWritten: "timeWritten".into(),
    // This is used to build the actual instance of the model during intialization e,g out:
    // "out".into()
    // The full thing can look like and the fields should be in normalized form:
    // i.e time_written => timeWritten if serde camelizes
    //
    // Self {
    //     id: "id".into(),
    //     r#in: "in".into(),
    //     out: "out".into(),
    //     timeWritten: "timeWritten".into(),
    // }
    pub schema_struct_fields_names_kv: TokenStream,

    // Field names after taking into consideration
    // serde serialized renaming or casings
    // i.e time_written => timeWritten if serde camelizes
    pub serialized_field_names_normalised: String,

    // Generated example:
    // type StudentWritesBlogTableName = <StudentWritesBlog as SurrealdbEdge>::TableNameChecker;
    // static_assertions::assert_fields!(StudentWritesBlogTableName: Writes);
    // Perform all necessary static checks
    pub static_assertions: TokenStream,

    // Generated example: type Book = <super::Book as SurrealdbNode>::Schema;
    // We need imports to be unique, hence the hashset
    // Used when you use a SurrealdbNode in field e.g: best_student: LinkOne<Student>,
    // e.g: type Book = <super::Book as SurrealdbNode>::Schema;
    pub referenced_foreign_nodes_imports: HashSet<ModelImport>,

    // so that we can do e.g ->writes[WHERE id = "writes:1"].field_name
    // self_instance.normalized_field_name.push_str(format!("{}.normalized_field_name", store_without_end_arrow).as_str());
    pub fields_connection_to_struct: TokenStream,

    // Generated Example for e.g field with best_student: line!()<Student>
    // pub fn best_student(&self, clause: Clause) -> Student {
    //     Student::__________update_connection(&self.__________store, clause)
    // }
    pub fields_with_record_links_method: TokenStream,
}

struct EdgeModelMetadata {
    field_receiver: MyFieldReceiver,
    struct_level_casing: CaseString,
    struct_name_ident: syn::Ident,
}

impl From<EdgeModelMetadata> for SchemaFieldsProperties {
    /// Constructs a `FieldsNames` struct from the given `data` and `struct_level_casing`.
    ///
    /// # Arguments
    ///
    /// * `data` - An `ast::Data` struct containing field receivers.
    /// * `struct_level_casing` - An optional `CaseString` representing the casing to be applied to the fields.
    /// Ident format is the name used in the code
    /// e.g
    /// ```
    /// struct User {
    ///     //user_name is ident and the serialized format by serde is "user_name"
    ///     user_name: String  
    /// }
    /// ```
    /// This is what we use as the field name and is mostly same as the serialized format
    /// except in the case of kebab-case serialized format in which case we fallback
    /// to the original ident format as written exactly in the code except when a user
    /// uses rename attribute on a specific field, in which case that takes precedence.
    fn from(value: EdgeModelMetadata) -> Self {
        let field_ident = value.field_receiver.ident.unwrap();
        let field_type = &value.field_receiver.ty;
        let crate_name = get_crate_name(false);
        let uncased_field_name = ::std::string::ToString::to_string(&field_ident);

        // pub determines whether the field will be serialized or not during creation/update
        let visibility: TokenStream = SkipSerializing(value.field_receiver.skip_serializing).into();

        let field_ident_cased = FieldIdentCased::from(FieldIdentUnCased {
            uncased_field_name,
            casing: value.struct_level_casing,
        });

        // get the field's proper serialized format. Renaming should take precedence
        let original_field_name_normalised = value.field_receiver.rename.as_ref().map_or_else(
            || field_ident_cased.into(),
            |renamed| renamed.clone().serialize,
        );
        let field_ident_normalised = format_ident!("{original_field_name_normalised}");

        let relationship = RelationType::from(&value.field_receiver);
        // pub time_written: DbField,
        let schema_struct_fields_types_kv =
            quote!(pub #field_ident_normalised: #crate_name::DbField);

        let field_ident_normalised_as_str =
            if original_field_name_normalised.trim_start_matches("r#") == "in".to_string() {
                "in".into()
            } else {
                field_ident_normalised.to_string()
            };
        // timeWritten: "timeWritten".into(),
        let schema_struct_fields_names_kv =
            quote!(#field_ident_normalised: #field_ident_normalised_as_str.into());
        // TODO: Abstract variable name-store_without_end_arrow- within quote token stream into a variable and reference
        // it
        let connection_with_field_appended = quote!(
        xx.time_written
            .push_str(format!("{}.time_written", store_without_end_arrow).as_str());
            );

        match relationship {
            RelationType::LinkOne(node_object) => {
                let extra = ReferencedNodeMeta::from(node_object);
                let schema_name_basic = &extra.schema_struct_name;

                Self {
                    schema_struct_fields_types_kv,
                    schema_struct_fields_names_kv,
                    serialized_field_names_normalised: field_ident_normalised_as_str,
                    static_assertions: quote!(),
                    referenced_foreign_nodes_imports: todo!(),
                    fields_connection_to_struct: todo!(),
                    fields_with_record_links_method: todo!(),
                }
            }
            RelationType::LinkSelf(node_object) => {
                let extra = ReferencedNodeMeta::from(node_object);
                let schema_name_basic = &extra.schema_struct_name;

                // if schema_name_basic.to_string() != struct_name_ident.to_string() {
                //     panic!("linkself has to refer to same struct")
                // }
                // ::static_assertions::assert_type_eq_all!(LinkOne<Course>, LinkOne<Course>);
                Self {
                    // friend<User>
                    schema_field: quote!(#visibility #field_ident_normalised<#schema_name_basic>,),
                    original_field_name_normalised,
                    static_assertions: quote!(
                     ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::LinkSelf<#schema_name_basic>);
                     ::static_assertions::assert_type_eq_all!(#struct_name_ident,  #schema_name_basic);
                    ),
                    referenced_node_meta: extra,
                }
            }
            RelationType::LinkMany(node_object) => {
                let extra = ReferencedNodeMeta::from(node_object);
                let schema_name_basic = &extra.schema_struct_name;

                Self {
                    // friend<Vec<User>>
                    // TODO: Confirm/Or fix this on the querybuilder side this.
                    // TODO: Semi-updated. It seems linkmany and link one both use same mechanisms
                    // for accessing linked fields or all
                    schema_field: quote!(#visibility #field_ident_normalised<#schema_name_basic>,),
                    original_field_name_normalised,
                    static_assertions: quote!(
                    ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::LinkMany<#schema_name_basic>);
                                                                 ),
                    referenced_node_meta: extra,
                }
            }
            RelationType::None => {
                Self {
                    // email,
                    schema_field: quote!(#visibility #field_ident_normalised,),
                    original_field_name_normalised,
                    static_assertions: quote!(),
                    referenced_node_meta: ReferencedNodeMeta::default(),
                }
            }
        }
    }
}

#[derive(Default)]
struct ReferencedNodeMeta {
    // pub referenced_foreign_nodes_imports: HashSet<ModelImport>,
    schema_import: TokenStream,
    // When a field references another model as Link, we want to generate a method for that
    // to be able to access the foreign fields
    generated_method: TokenStream,
}

impl From<super::relations::NodeName> for ReferencedNodeMeta {
    fn from(node_name: super::relations::NodeName) -> Self {
        let schema_name = format_ident!("{node_name}");

        let crate_name = get_crate_name(false);
        // imports for specific schema from the trait Generic Associated types e.g
        // TODO: Remove OLD comment: type Account<const T: usize = <super::Account as super::Account>::Schema<T>;
        // type Book = <super::Book as SurrealdbNode>::Schema;
        let schema_import = quote!(
            type #schema_name = <super::#schema_name as #crate_name::SurrealdbNode>::Schema;
        );

        Self {
            schema_import,
            generated_method: todo!(),
        }
    }
}

fn get_ident(name: &String) -> syn::Ident {
    // let xx = match FieldName::from(name) {
    //   FieldName::Keyword(x) => syn::Ident::new_raw(x.into(), Span::call_site()),
    //   FieldName::Others(o) => syn::Ident::new(o.as_str(), Span::call_site()),
    // };
    // if name == &"in".to_string() {

    if vec!["in", "r#in"].contains(&name.as_str()) {
        syn::Ident::new_raw(name.trim_start_matches("r#"), Span::call_site())
    // } else if name == "r#in" {
    // syn::Ident::new("in", Span::call_site())
    } else {
        syn::Ident::new(name.as_str(), Span::call_site())
    }
}
