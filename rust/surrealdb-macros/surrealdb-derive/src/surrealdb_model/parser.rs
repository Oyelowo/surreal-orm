/*
Author: Oyelowo Oyedayo
Email: oyelowooyedayo@gmail.com
*/

#![allow(dead_code)]

use std::{collections::HashSet, hash::Hash};

use darling::{ast, util};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    casing::{CaseString, FieldIdentCased, FieldIdentUnCased},
    get_crate_name,
    relations::{RelateAttribute, RelationType},
    serialize_skipper::SkipSerializing,
    trait_generator::MyFieldReceiver,
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
pub(crate) struct ModelAttributesTokensDeriver {
    // We need imports to be unique, hence the hashset
    pub all_model_imports: HashSet<ModelImport>,
    pub all_schema_names_basic: Vec<TokenStream>,
    pub all_model_schema_fields: Vec<TokenStream>,
    pub all_static_assertions: Vec<TokenStream>,
    pub all_serialized_field_names_normalised: Vec<String>,
    pub edge_metadata: EdgeModelAttr,
}

#[derive(Clone, Default)]
pub(crate) struct EdgeModelAttr {
    pub in_node_type: Option<TokenStream>,
    pub out_node_type: Option<TokenStream>,
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
        struct_name_ident: &syn::Ident,
    ) -> Self {
        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        let metas = fields.into_iter().enumerate().fold(
            ModelAttributesTokensDeriver::default(),
            |mut acc, (index, field_receiver)| {
                let struct_level_casing = struct_level_casing.unwrap_or(CaseString::None);
                let meta = Self::get_model_metadata(
                    field_receiver,
                    struct_level_casing,
                    index,
                    struct_name_ident,
                );

                acc.all_model_schema_fields.push(meta.model_schema_field);

                acc.all_model_imports.insert(meta.extra.model_import.into());

                acc.all_schema_names_basic.push(meta.extra.schema_name);

                acc.all_serialized_field_names_normalised
                    .push(meta.original_field_name_normalised.clone());

                acc.all_static_assertions.push(meta.static_assertions);

                let field_type = &field_receiver.ty;
                // let field_type_from_attr = &field_receiver
                //     .link_one
                //     .as_ref()
                //     .map(|ty_name| format_ident!("{ty_name}"));

                match EdgeOrientation::from(&meta.original_field_name_normalised) {
                    EdgeOrientation::In => {
                        // acc.edge_metadata.in_node_type = Some(quote!(#field_type_from_attr));
                        acc.edge_metadata.in_node_type = Some(quote!(#field_type));
                    }
                    EdgeOrientation::Out => {
                        // acc.edge_metadata.out_node_type = Some(quote!(#field_type_from_attr));
                        acc.edge_metadata.out_node_type = Some(quote!(#field_type));
                    }
                    EdgeOrientation::None => {}
                };
                acc
            },
        );
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
        struct_name_ident: &syn::Ident,
    ) -> ModelMedataTokenStream {
        let field_ident = Self::get_field_identifier_name(&field_receiver, index);
        let field_type = &field_receiver.ty;
        let crate_name = get_crate_name(false);
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

        let field_ident_normalised =
            if original_field_name_normalised.trim_start_matches("r#") == "in".to_string() {
                quote!(in)
            } else {
                quote!(#field_ident_normalised)
            };
        let relationship = RelationType::from(field_receiver);

        match relationship {
            RelationType::Relate(relation) => {
                let relation_attributes = RelateAttribute::from(relation.clone());

                let arrow_direction = TokenStream::from(relation_attributes.edge_direction);

                let edge_action = TokenStream::from(relation_attributes.edge_action);
                let destination_node = TokenStream::from(relation_attributes.node_object.clone());
                let extra = ModelMetadataBasic::from(relation_attributes.node_object);
                let struct_name = quote!(#struct_name_ident);
                let schema_name_basic = &extra.schema_name;
                // TODO: Make edge required.
                let edge_struct_ident = format_ident!(
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
                ::static_assertions::assert_type_eq_all!(<#edge_struct_ident as #crate_name::Edge>::InNode, #crate_name::links::LinkOne<#in_node>);
                ::static_assertions::assert_type_eq_all!(<#edge_struct_ident as #crate_name::Edge>::OutNode, #crate_name::links::LinkOne<#out_node>);
                type #edge_checker_alias  = <#edge_struct_ident as Edge>::EdgeChecker;
                ::static_assertions::assert_fields!(#edge_checker_alias : #edge_action);

                // assert field type and attribute reference match
                    ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::Relate<#schema_name_basic>);
                                        );
                /*
                 *
                // This can the access the alias
                  model!(Student {
                    pub ->takes->Course as enrolled_courses, // This is what we want
                  })
                */
                // e.g: ->has->Account as field_name
                let field = quote!(#arrow_direction #edge_action #arrow_direction #schema_name_basic as #field_ident_normalised,);
                // let field = quote!(#visibility #arrow_direction #edge_action #arrow_direction #schema_name_basic as #field_ident_normalised,);
                ModelMedataTokenStream {
                    model_schema_field: quote!(#field),
                    original_field_name_normalised,
                    static_assertions: relation_assertions,
                    extra,
                }
            }
            RelationType::LinkOne(node_object) => {
                let extra = ModelMetadataBasic::from(node_object);
                let schema_name_basic = &extra.schema_name;

                ModelMedataTokenStream {
                    // friend<User>
                    model_schema_field: quote!(#visibility #field_ident_normalised<#schema_name_basic>,),
                    original_field_name_normalised,
                    static_assertions: quote!(
                     ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::LinkOne<#schema_name_basic>);
                    ),
                    extra,
                }
            }
            RelationType::LinkSelf(node_object) => {
                let extra = ModelMetadataBasic::from(node_object);
                let schema_name_basic = &extra.schema_name;

                // if schema_name_basic.to_string() != struct_name_ident.to_string() {
                //     panic!("linkself has to refer to same struct")
                // }
                // ::static_assertions::assert_type_eq_all!(LinkOne<Course>, LinkOne<Course>);
                ModelMedataTokenStream {
                    // friend<User>
                    model_schema_field: quote!(#visibility #field_ident_normalised<#schema_name_basic>,),
                    original_field_name_normalised,
                    static_assertions: quote!(
                     ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::LinkSelf<#schema_name_basic>);
                     ::static_assertions::assert_type_eq_all!(#struct_name_ident,  #schema_name_basic);
                    ),
                    extra,
                }
            }
            RelationType::LinkMany(node_object) => {
                let extra = ModelMetadataBasic::from(node_object);
                let schema_name_basic = &extra.schema_name;

                ModelMedataTokenStream {
                    // friend<Vec<User>>
                    // TODO: Confirm/Or fix this on the querybuilder side this.
                    // TODO: Semi-updated. It seems linkmany and link one both use same mechanisms
                    // for accessing linked fields or all
                    model_schema_field: quote!(#visibility #field_ident_normalised<#schema_name_basic>,),
                    original_field_name_normalised,
                    static_assertions: quote!(
                    ::static_assertions::assert_type_eq_all!(#field_type,  #crate_name::links::LinkMany<#schema_name_basic>);
                                                                 ),
                    extra,
                }
            }
            RelationType::None => {
                ModelMedataTokenStream {
                    // email,
                    model_schema_field: quote!(#visibility #field_ident_normalised,),
                    original_field_name_normalised,
                    static_assertions: quote!(),
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
    static_assertions: TokenStream,
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
