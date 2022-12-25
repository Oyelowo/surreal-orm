/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use darling::{ast, util};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    relations::{RelateAttribute, RelationType},
    serialize_skipper::SkipSerializing,
    trait_generator::MyFieldReceiver,
};

/// A struct that contains the `struct_ty_fields` and `struct_values_fields` vectors.
#[derive(Default)]
pub(crate) struct ModelAttributesTokensDeriver {
    pub all_schema_reexported_aliases: Vec<TokenStream>,
    pub all_model_imports: Vec<TokenStream>,
    pub all_schema_names_basic: Vec<TokenStream>,
    pub all_fields: Vec<TokenStream>,
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
                let meta = Self::get_model_metadata(field_receiver, struct_level_casing, index);

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
    fn get_model_metadata(
        field_receiver: &MyFieldReceiver,
        struct_level_casing: CaseString,
        index: usize,
    ) -> ModelMedataTokenStream {
        let field_ident = Self::get_field_identifier_name(&field_receiver, index);
        let uncased_field_name = ::std::string::ToString::to_string(&field_ident);

        // pub determines whether the field will be serialized or not during creation/update
        let visibility: TokenStream = SkipSerializing(field_receiver.skip_serializing).into();

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


#[derive(Default)]
struct ModelMetadataBasic {
    model_import: TokenStream,
    schema_name_basic: TokenStream,
    // account::schema::model -> AccountSchema
    schema_reexported_alias: TokenStream,
}

impl From<super::relations::NodeObject> for ModelMetadataBasic {
    fn from(node_object: super::relations::NodeObject) -> Self {
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
