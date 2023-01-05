/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use darling::{ast, util};
use proc_macro2::{Span, TokenStream};
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
    pub all_model_imports: Vec<TokenStream>,
    pub all_schema_names_basic: Vec<TokenStream>,
    pub all_model_schema_fields: Vec<TokenStream>,
    pub all_original_field_names_normalised: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
enum EdgeOrientation {
    In,
    Out,
    None,
}

impl From<&String> for EdgeOrientation {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "in" | "r#in" => Self::In,
            "out" => Self::Out,
            _ => Self::None,
        }
    }
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
        relation_name: Option<String>,
    ) -> Self {
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        let metas = fields.into_iter().enumerate().fold(
            Self::default(),
            |mut acc, (index, field_receiver)| {
                let struct_level_casing = struct_level_casing.unwrap_or(CaseString::None);
                let meta = Self::get_model_metadata(field_receiver, struct_level_casing, index);

                acc.all_model_schema_fields.push(meta.model_schema_field);

                acc.all_model_imports.push(meta.extra.model_import);

                acc.all_schema_names_basic.push(meta.extra.schema_name);
                acc.all_original_field_names_normalised
                    .push(meta.original_field_name_normalised);

                acc
            },
        );
        let has_orig_dest_nodes = metas
            .all_original_field_names_normalised
            .iter()
            .map(EdgeOrientation::from)
            .filter(|f| matches!(f, EdgeOrientation::In | EdgeOrientation::Out))
            .count()
            == 2;
        let is_invalid_edge_model = relation_name.is_some() && !has_orig_dest_nodes;
        if is_invalid_edge_model {
            panic!("in and out fields have to be specified with origin and destination nodes");
        }
        metas
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
        let original_field_name_normalised = field_receiver.rename.as_ref().map_or_else(
            || field_ident_cased.into(),
            |renamed| renamed.clone().serialize,
        );
        let field_ident_normalised = format_ident!("{original_field_name_normalised}");

        let xx = if original_field_name_normalised.trim_start_matches("r#") == "in".to_string() {
            // let xo = format_ident!("in");
            let xo = syn::Ident::new_raw("in", Span::call_site());
            let r#in = syn::Ident::new_raw("in", Span::call_site());
            quote!(in)
        } else {
            quote!(#field_ident_normalised)
        };
        let relationship = RelationType::from(field_receiver);

        match relationship {
            RelationType::RelationGraph(relation) => {
                let relation_attributes = RelateAttribute::from(relation);
                let arrow_direction = TokenStream::from(relation_attributes.edge_direction);

                let edge_action = TokenStream::from(relation_attributes.edge_action);
                let extra = ModelMetadataBasic::from(relation_attributes.node_object);
                let schema_name_basic = &extra.schema_name;
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
                    model_schema_field: quote!(#field),
                    original_field_name_normalised,
                    extra,
                }
            }
            RelationType::ReferenceOne(node_object) => {
                let extra = ModelMetadataBasic::from(node_object);
                let schema_name_basic = &extra.schema_name;

                ModelMedataTokenStream {
                    // friend<User>
                    model_schema_field: quote!(#visibility #field_ident_normalised<#schema_name_basic>,),
                    original_field_name_normalised,
                    extra,
                }
            }
            RelationType::ReferenceMany(node_object) => {
                let extra = ModelMetadataBasic::from(node_object);
                let schema_name_basic = &extra.schema_name;

                ModelMedataTokenStream {
                    // friend<Vec<User>>
                    // TODO: Confirm/Or fix this on the querybuilder side this.
                    model_schema_field: quote!(#visibility #field_ident_normalised<Vec<#schema_name_basic>>,),
                    original_field_name_normalised,
                    extra,
                }
            }
            RelationType::None => {
                ModelMedataTokenStream {
                    // email,
                    model_schema_field: quote!(#visibility #xx,),
                    original_field_name_normalised,
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
    model_schema_field: TokenStream,
    original_field_name_normalised: String,
    extra: ModelMetadataBasic,
}

#[derive(Default)]
struct ModelMetadataBasic {
    model_import: TokenStream,
    schema_name: TokenStream,
}

impl From<super::relations::NodeObject> for ModelMetadataBasic {
    fn from(node_object: super::relations::NodeObject) -> Self {
        let schema_name = format_ident!("{node_object}");

        // imports for specific model schema from the trait Generic Associated types e.g
        // type Account<const T: usize> = <super::Account as super::Account>::Schema<T>;
        let model_import = quote!(type #schema_name<const T:usize> =  <super::#schema_name as super::SurrealdbModel>::Schema<T>;);

        Self {
            model_import,
            schema_name: quote!(#schema_name),
        }
    }
}
// test hunk save
