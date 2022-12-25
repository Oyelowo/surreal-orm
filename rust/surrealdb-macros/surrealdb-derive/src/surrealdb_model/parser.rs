/* 
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use std::fmt::Arguments;

use darling::{ast, util};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use syn;

use super::{trait_generator::MyFieldReceiver, types::CaseString};

/// A struct that contains the serialized and identifier versions of a field.
// pub(crate) struct FieldIdentifier {
//     /// The serialized version of the field name.
//     serialized: ::std::string::String,
//     /// The identifier version of the field name.
//     ident: syn::Ident,
//     surrealdb_field_ident: TokenStream,
//     surrealdb_imported_schema_dependency: TokenStream,
//     // surrealdb_field_ident: ::std::string::String,
// }

/// A struct that contains the `struct_ty_fields` and `struct_values_fields` vectors.
#[derive(Default)]
pub(crate) struct ModelAttributesTokensDeriver {
    pub all_schema_reexported_aliases: Vec<TokenStream>,
    pub all_model_imports: Vec<TokenStream>,
    pub all_schema_names_basic: Vec<TokenStream>,
    pub all_fields: Vec<TokenStream>,
    // metadata: Vec<ModelMedataTokenStream>,
}

impl ModelAttributesTokensDeriver {
    /// Constructs a `FieldsNames` struct from the given `data` and `struct_level_casing`.
    ///
    /// # Arguments
    ///
    /// * `data` - An `ast::Data` struct containing field receivers.
    /// * `struct_level_casing` - An optional `CaseString` representing the casing to be applied to the fields.
    pub(crate) fn from_receiver_data(
        data: &ast::Data<util::Ignored, MyFieldReceiver>,
        struct_level_casing: Option<CaseString>,
    ) -> Self {
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        fields
            .into_iter()
            .enumerate()
            .fold(Self::default(), |mut acc, (index, field_receiver)| {
                let struct_level_casing = struct_level_casing.unwrap_or(CaseString::None);
                let meta = Self::get_field_ident(field_receiver, struct_level_casing, index);

                acc.all_fields.push(meta.field);

                acc.all_model_imports.push(meta.extra.model_import);

                acc.all_schema_names_basic
                    .push(meta.extra.schema_name_basic);

                acc.all_schema_reexported_aliases
                    .push(meta.extra.schema_reexported_alias);

                acc
            })
    }

    /// Returns a `TokenStream` representing the field identifier for the given `field_receiver` and `index`.
    ///
    /// If the `field_receiver` has a named field, it returns a `TokenStream` representing that name.
    /// Otherwise, it returns a `TokenStream` representing the `index`.
    ///
    /// This function works with both named and indexed fields.
    ///
    /// # Arguments
    ///
    /// * `field_receiver` - A field receiver containing field information.
    /// * `index` - The index of the field.
    fn get_field_identifier_name(field_receiver: &MyFieldReceiver, index: usize) -> TokenStream {
        // This works with named or indexed fields, so we'll fall back to the index so we can
        // write the output as a key-value pair.
        // The index is rarely necessary since our models are usually not tuple struct
        // but leaving it as is anyways.
        field_receiver.ident.as_ref().map_or_else(
            || {
                let index_ident = ::syn::Index::from(index);
                quote!(#index_ident)
            },
            |name_ident| quote!(#name_ident),
        )
    }

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
    fn get_field_ident(
        field_receiver: &MyFieldReceiver,
        struct_level_casing: CaseString,
        index: usize,
    ) -> ModelMedataTokenStream {
        // let struct_level_casing = struct_level_casing.unwrap_or(CaseString::None);
        let field_ident = Self::get_field_identifier_name(&field_receiver.clone(), index);
        let uncased_field_name = ::std::string::ToString::to_string(&field_ident);

        // pub determines whether the field will be serialized or not during creation/update
        let visibility: TokenStream =
            SkipSerializing::from(field_receiver.clone().skip_serializing).into();

        let field_ident_cased = FieldIdentCased::from(FieldIdentUnCased {
            uncased_field_name,
            casing: struct_level_casing,
        });

        // get the field's proper serialized format. Renaming should take precedence
        let field_ident_normalised = field_receiver.rename.as_ref().map_or_else(
            || field_ident_cased.into(),
            |renamed| renamed.clone().serialize,
        );

        let field_ident_normalised = format_ident!("{field_ident_normalised}");

        let relationship = RelationType::from(field_receiver);

        match relationship {
            RelationType::RelationGraph(relation) => {
                let x = RelateAttribute::from(relation);
                let arrow_direction = TokenStream::from(x.edge_direction);
                let edge_action = TokenStream::from(x.edge_action);
                let extra = ModelMetadataBasic::from(x.node_object);
                let schema_name_basic = &extra.schema_name_basic;
                //
                /*
                // This can the access the alias
                  model!(Student {
                    pub ->takes->Course as enrolled_courses, // This is what we want
                  })
                */
                // e.g: ->has->Account
                let field = quote!(#visibility #arrow_direction #edge_action #arrow_direction #schema_name_basic as #field_ident_normalised,);
                ModelMedataTokenStream {
                    field: quote!(#field),
                    extra,
                }
            }
            RelationType::ReferenceOne(node_object) => {
                let extra = ModelMetadataBasic::from(node_object);
                let schema_name_basic = &extra.schema_name_basic;

                ModelMedataTokenStream {
                    // friend<User>
                    field: quote!(#visibility #field_ident_normalised<#schema_name_basic>,),
                    extra,
                }
            }
            RelationType::ReferenceMany(node_object) => {
                let extra = ModelMetadataBasic::from(node_object);
                let schema_name_basic = &extra.schema_name_basic;

                ModelMedataTokenStream {
                    // friend<Vec<User>>
                    // TODO: Confirm/Or fix this on the querybuilder side this.
                    field: quote!(#visibility #field_ident_normalised<Vec<#schema_name_basic>>,),
                    extra,
                }
            }
            RelationType::None => {
                ModelMedataTokenStream {
                    // email,
                    field: quote!(#visibility #field_ident_normalised,),
                    extra: ModelMetadataBasic::default(),
                }
            }
        }
    }
}

/*
mod account {
    // Project is schema name, ProjectSchema is schema alias
    use super::ProjectSchema as Project;   //  import
    use super::SchemaSchema as Schema;      // import
    model!(Account {
        field1,  // field
        field2   // field
        field_with_reference<Project>,  // field
        ->manages->School as managed_school  //field
    })
}
*/
struct ModelMedataTokenStream {
    field: TokenStream,
    extra: ModelMetadataBasic,
}

/*
          let schema_name_str = String::from(x.node_object);
                let schema_name_basic = format_ident!("{schema_name_str}");
                let schema_name_basic_lower_case =
                    format_ident!("{}", schema_name_str.to_lowercase());
                let schema_name_aliased = format_ident!("{schema_name_str}Schema");
                //  import Schema from outside. To prevent model name collision with their struct names,
                //  all schemas are suffixed-aliased to i.e<schema_name>Schema e.g Account => AccountSchema
                //  use super::AccountSchema as Account;
                let model_import = quote!(use super::#schema_name_aliased as #schema_name_basic;);
*/

#[derive(Default)]
struct ModelMetadataBasic {
    model_import: TokenStream,
    schema_name_basic: TokenStream,
    // account::schema::model -> AccountSchema
    schema_reexported_alias: TokenStream,
}

impl From<NodeObject> for ModelMetadataBasic {
    fn from(node_object: NodeObject) -> Self {
        let schema_name_str = String::from(node_object);
        let schema_name_basic = format_ident!("{schema_name_str}");
        let schema_name_basic_lower_case = format_ident!("{}", schema_name_str.to_lowercase());
        let schema_name_aliased = format_ident!("{schema_name_str}Schema");
        //  import Schema from outside. To prevent model name collision with their struct names,
        //  all schemas are suffixed-aliased to i.e<schema_name>Schema e.g Account => AccountSchema
        //  use super::AccountSchema as Account;
        let model_import = quote!(use super::#schema_name_aliased as #schema_name_basic;);
        let schema_reexported_alias = quote!(use #schema_name_basic_lower_case::schema::#schema_name_basic as #schema_name_aliased;);

        Self {
            schema_reexported_alias,
            model_import,
            schema_name_basic: quote!(#schema_name_basic),
        }
    }
}

#[derive(Debug, Clone)]
struct FieldIdentUnCased {
    uncased_field_name: String,
    casing: CaseString,
}

#[derive(Debug, Clone)]
struct FieldIdentCased(String);

impl From<String> for FieldIdentCased {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<FieldIdentCased> for String {
    fn from(value: FieldIdentCased) -> Self {
        value.0
    }
}

impl From<FieldIdentUnCased> for FieldIdentCased {
    /// Converts the field identifier string to the specified case.
    /// Also, if rename_all attribute is not specified to change the casing,
    /// it defaults to exactly how the fields are written out.
    /// However, Field rename attribute overrides this.
    fn from(field_uncased: FieldIdentUnCased) -> Self {
        let convert_field_identifier = |case: convert_case::Case| {
            convert_case::Converter::new()
                .to_case(case)
                .convert(&field_uncased.uncased_field_name)
        };

        match field_uncased.casing {
            CaseString::None => field_uncased.uncased_field_name,
            CaseString::Camel => convert_field_identifier(convert_case::Case::Camel),
            CaseString::Snake => convert_field_identifier(convert_case::Case::Snake),
            CaseString::Pascal => convert_field_identifier(convert_case::Case::Pascal),
            CaseString::Lower => convert_field_identifier(convert_case::Case::Lower),
            CaseString::Upper => convert_field_identifier(convert_case::Case::Upper),
            CaseString::ScreamingSnake => {
                convert_field_identifier(convert_case::Case::ScreamingSnake)
            }
            CaseString::Kebab => convert_field_identifier(convert_case::Case::Kebab),
            CaseString::ScreamingKebab => convert_field_identifier(convert_case::Case::UpperKebab),
        }
        .into()
    }
}

#[derive(Debug, Clone)]
enum RelationType {
    RelationGraph(Relation),
    ReferenceOne(NodeObject),
    ReferenceMany(NodeObject),
    None,
}

impl From<&MyFieldReceiver> for RelationType {
    fn from(field_receiver: &MyFieldReceiver) -> Self {
        use RelationType::*;
        match field_receiver {
            MyFieldReceiver {
                relate: Some(relation),
                ..
            } => RelationGraph(Relation(relation.to_owned())),
            MyFieldReceiver {
                reference_one: Some(ref_one),
                ..
            } => ReferenceOne(NodeObject(ref_one.to_owned())),
            MyFieldReceiver {
                reference_many: Some(ref_many),
                ..
            } => ReferenceMany(NodeObject(ref_many.to_owned())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SkipSerializing {
    Yes,
    No,
}

impl From<bool> for SkipSerializing {
    fn from(value: bool) -> Self {
        match value {
            true => SkipSerializing::Yes,
            false => SkipSerializing::No,
        }
    }
}

impl From<SkipSerializing> for bool {
    fn from(value: SkipSerializing) -> Self {
        match value {
            SkipSerializing::Yes => true,
            SkipSerializing::No => false,
        }
    }
}

impl From<SkipSerializing> for TokenStream {
    fn from(value: SkipSerializing) -> Self {
        match value {
            SkipSerializing::Yes => quote!(),
            SkipSerializing::No => quote!(pub),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EdgeDirection {
    OutArrowRight,
    InArrowLeft,
}

impl From<EdgeDirection> for TokenStream {
    fn from(direction: EdgeDirection) -> Self {
        match direction {
            EdgeDirection::OutArrowRight => quote::quote!(->),
            EdgeDirection::InArrowLeft => quote::quote!(<-),
        }
    }
}

impl From<EdgeDirection> for String {
    fn from(direction: EdgeDirection) -> Self {
        match direction {
            EdgeDirection::OutArrowRight => "->".into(),
            EdgeDirection::InArrowLeft => "<-".into(),
        }
    }
}

macro_rules! wrapper_struct_to_ident {
    ($simple_wrapper_struct:ty) => {
        impl From<$simple_wrapper_struct> for TokenStream {
            fn from(simple_wrapper_struct: $simple_wrapper_struct) -> Self {
                let ident = format_ident!("{}", simple_wrapper_struct.0);
                ::quote::quote!(#ident)
            }
        }
    };
}

/*
impl From<EdgeAction> for TokenStream {
    fn from(edge_action: EdgeAction) -> Self {
        let action = format_ident!("{}", edge_action.0);
        quote!(#action)
    }
}
*/

#[derive(Debug, Clone)]
struct EdgeAction(String);
wrapper_struct_to_ident!(EdgeAction);

#[derive(Debug, Clone)]
struct NodeObject(String);

impl From<NodeObject> for String {
    fn from(value: NodeObject) -> Self {
        value.0
    }
}
wrapper_struct_to_ident!(NodeObject);

#[derive(Debug, Clone)]
struct RelateAttribute {
    edge_direction: EdgeDirection,
    edge_action: EdgeAction,
    node_object: NodeObject,
}

impl From<RelateAttribute> for TokenStream {
    fn from(relate_attrs: RelateAttribute) -> Self {
        let edge_direction = TokenStream::from(relate_attrs.edge_direction);
        let edge_action = TokenStream::from(relate_attrs.edge_action);
        let node_object = TokenStream::from(relate_attrs.node_object);
        // ->action->NodeObject
        // <-action<-NodeObject
        // e.g ->manages->Project
        quote!(#edge_direction #edge_action #node_object)
    }
}

#[derive(Debug, Clone)]
struct Relation(String);

impl From<Relation> for String {
    fn from(relation: Relation) -> Self {
        relation.0
    }
}
impl From<String> for Relation {
    fn from(str: String) -> Self {
        Relation(str)
    }
}

impl From<Relation> for RelateAttribute {
    fn from(relation: Relation) -> Self {
        let right_arrow_count = relation.0.matches("->").count();
        let left_arrow_count = relation.0.matches("<-").count();
        let edge_direction = match (left_arrow_count, right_arrow_count) {
            (2, 0) => EdgeDirection::InArrowLeft,
            (0, 2) => EdgeDirection::OutArrowRight,
            _ => panic!("Arrow incorrectly used"),
        };

        let edge_direction_str: String = edge_direction.into();
        let mut substrings = relation
            .0
            .split(edge_direction_str.as_str())
            .filter(|x| !x.is_empty());

        let (edge_action, node_object) =
            match (substrings.next(), substrings.next(), substrings.next()) {
                (Some(action), Some(node_obj), None) => {
                    (EdgeAction(action.into()), NodeObject(node_obj.into()))
                }
                _ => panic!(
                    "too many actions or object, {}",
                    get_relation_error(&relation)
                ),
            };

        Self {
            node_object,
            edge_action,
            edge_direction,
        }
    }
}

fn get_relation_error<'a>(relation: &Relation) -> Arguments<'a> {
    // let span = syn::spanned::Spanned::span(relation.0.clone()).clone();
    // let span = syn::spanned::Spanned::span(relation.0.as_str()).clone();

    // let start = span.clone().start().clone();
    // let end = span.clone().end().clone();
    // let start_line = start.line;
    // let start_column = start.column;
    // let end_column = end.column;
    let c = format_args!(
        " Check that your arrows are properly faced. e.g ->has->Heart or <-owned_by<-Human",
        // start_line,
        // start_column,
        // end_column
    );
    c
}
