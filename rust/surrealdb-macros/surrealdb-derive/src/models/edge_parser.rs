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
pub(crate) struct AllSchemaFieldsProperties {
    /// Generated example: pub timeWritten: DbField,
    /// key(normalized_field_name)-value(DbField) e.g pub out: DbField, of field name and DbField type
    /// to build up struct for generating fields of a Schema of the SurrealdbEdge
    /// The full thing can look like:
    ///     #[derive(Debug, Default)]
    ///     pub struct Writes<Model: ::serde::Serialize + Default> {
    ///                pub id: Dbfield,
    ///                pub r#in: Dbfield,
    ///                pub out: Dbfield,
    ///                pub timeWritten: Dbfield,
    ///          }
    pub schema_struct_fields_types_kv: HashSet<FieldTokenStream>,

    /// Generated example: pub timeWritten: "timeWritten".into(),
    /// This is used to build the actual instance of the model during intialization e,g out:
    /// "out".into()
    /// The full thing can look like and the fields should be in normalized form:
    /// i.e time_written => timeWritten if serde camelizes
    //
    /// Self {
    ///     id: "id".into(),
    ///     r#in: "in".into(),
    ///     out: "out".into(),
    ///     timeWritten: "timeWritten".into(),
    /// }
    pub schema_struct_fields_names_kv: HashSet<FieldTokenStream>,

    /// Field names after taking into consideration
    /// serde serialized renaming or casings
    /// i.e time_written => timeWritten if serde camelizes
    pub serialized_field_names_normalised: HashSet<String>,

    /// Generated example:
    /// type StudentWritesBlogTableName = <StudentWritesBlog as SurrealdbEdge>::TableNameChecker;
    /// static_assertions::assert_fields!(StudentWritesBlogTableName: Writes);
    /// Perform all necessary static checks
    pub static_assertions: HashSet<FieldTokenStream>,

    /// Generated example: type Book = <super::Book as SurrealdbNode>::Schema;
    /// We need imports to be unique, hence the hashset
    /// Used when you use a SurrealdbNode in field e.g: best_student: LinkOne<Student>,
    /// e.g: type Book = <super::Book as SurrealdbNode>::Schema;
    pub referenced_node_schema_import: HashSet<FieldTokenStream>,
    /// When a field references another model as Link, we want to generate a method for that
    /// to be able to access the foreign fields
    /// Generated Example for e.g field with best_student: line!()<Student>
    /// pub fn best_student(&self, clause: Clause) -> Student {
    ///     Student::__________update_connection(&self.__________store, clause)
    /// }
    pub referenced_field_record_link_method: HashSet<FieldTokenStream>,

    /// so that we can do e.g ->writes[WHERE id = "writes:1"].field_name
    /// self_instance.normalized_field_name.push_str(format!("{}.normalized_field_name", store_without_end_arrow).as_str());
    pub connection_with_field_appended: HashSet<FieldTokenStream>,
}

impl AllSchemaFieldsProperties {
    fn from_field_receiver(
        data: ast::Data<util::Ignored, MyFieldReceiver>,
        struct_level_casing: Option<CaseString>,
        struct_name_ident: syn::Ident,
    ) {
        let fields = data
            // .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .into_iter()
            .fold(Self::default(), |acc, val| {
                let props = SchemaFieldsProperties::from(EdgeModelMetadata {
                    field_receiver: val,
                    struct_level_casing,
                    struct_name_ident,
                });

                acc.static_assertions.insert(props.static_assertions.into());

                acc.schema_struct_fields_types_kv
                    .insert(props.schema_struct_fields_types_kv.into());

                acc.schema_struct_fields_names_kv
                    .insert(props.schema_struct_fields_names_kv.into());

                acc.serialized_field_names_normalised
                    .insert(props.serialized_field_names_normalised.into());

                acc.connection_with_field_appended
                    .insert(props.connection_with_field_appended.into());

                acc.referenced_node_schema_import
                    .insert(props.referenced_node_meta.schema_import.into());

                acc.referenced_field_record_link_method
                    .insert(props.referenced_node_meta.field_record_link_method.into());

                acc
            });
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

    // Metadata about a field that is a record link
    pub referenced_node_meta: ReferencedNodeMeta,

    // so that we can do e.g ->writes[WHERE id = "writes:1"].field_name
    // self_instance.normalized_field_name.push_str(format!("{}.normalized_field_name", store_without_end_arrow).as_str());
    pub connection_with_field_appended: TokenStream,
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
        // let visibility: TokenStream = SkipSerializing(value.field_receiver.skip_serializing).into();

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
        // let field_ident_normalised_as_str_ident = format_ident!("{field_ident_normalised_as_str}");
        let connection_with_field_appended = quote!(
        schema_instance.#field_ident_normalised
            .push_str(format!("{}.{}", store_without_end_arrow, #field_ident_normalised_as_str).as_str());
            );

        let referenced_node_meta = match relationship {
            RelationType::LinkOne(node_object) => {
                ReferencedNodeMeta::from_ref_node_meta(node_object, field_ident_normalised)
            }
            RelationType::LinkSelf(node_object) => {
                ReferencedNodeMeta::from_ref_node_meta(node_object, field_ident_normalised)
            }
            RelationType::LinkMany(node_object) => {
                ReferencedNodeMeta::from_ref_node_meta(node_object, field_ident_normalised)
            }
            RelationType::None => ReferencedNodeMeta::default(),
        };
        Self {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            serialized_field_names_normalised: field_ident_normalised_as_str,
            static_assertions: quote!(),
            referenced_node_meta,
            connection_with_field_appended,
        }
    }
}

#[derive(Default, Clone)]
struct ReferencedNodeMeta {
    // Generated example: type Book = <super::Book as SurrealdbNode>::Schema;
    // We need imports to be unique, hence the hashset
    // Used when you use a SurrealdbNode in field e.g: best_student: LinkOne<Student>,
    // e.g: type Book = <super::Book as SurrealdbNode>::Schema;
    schema_import: TokenStream,
    // When a field references another model as Link, we want to generate a method for that
    // to be able to access the foreign fields
    // Generated Example for e.g field with best_student: line!()<Student>
    // pub fn best_student(&self, clause: Clause) -> Student {
    //     Student::__________update_connection(&self.__________store, clause)
    // }
    field_record_link_method: TokenStream,
}

impl ReferencedNodeMeta {
    fn from_ref_node_meta(
        node_name: super::relations::NodeName,
        normalized_field_name: ::syn::Ident,
    ) -> ReferencedNodeMeta {
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
            // pub fn fav_student(&self, clause: Clause) -> Student {
            //     Student::__________update_connection(&self.__________store, clause)
            // }
            field_record_link_method: quote!(
                pub fn #normalized_field_name(&self, clause: #crate_name::Clause) -> #schema_name {
                    #schema_name:__________update_connection(&self.__________store, clause)
                }
            ),
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
