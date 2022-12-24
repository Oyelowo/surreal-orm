#![allow(dead_code)]

use std::fmt::Arguments;

use darling::{ast, util};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use syn;

use super::{trait_generator::MyFieldReceiver, types::CaseString};

/// A struct that contains the serialized and identifier versions of a field.
pub(crate) struct FieldIdentifier {
    /// The serialized version of the field name.
    serialized: ::std::string::String,
    /// The identifier version of the field name.
    ident: syn::Ident,
    surrealdb_field_ident: TokenStream,
    surrealdb_imported_schema_dependency: TokenStream,
    // surrealdb_field_ident: ::std::string::String,
}

/// A struct that contains the `struct_ty_fields` and `struct_values_fields` vectors.
#[derive(Debug, Default)]
pub(crate) struct FieldsNames {
    /// A vector of token streams representing the struct type fields.
    pub struct_ty_fields: Vec<TokenStream>,
    /// A vector of token streams representing the struct value fields.
    pub struct_values_fields: Vec<TokenStream>,

    pub models_serialized_values: Vec<TokenStream>,
    pub surrealdb_imported_schema_dependencies: Vec<TokenStream>,
}

impl FieldsNames {
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

        fields.into_iter().enumerate().fold(
            Self::default(),
            |mut field_names_accumulator, (index, field_receiver)| {
                let field_case = struct_level_casing.unwrap_or(CaseString::None);
                let field_ident = Self::get_field_identifier_name(field_receiver, index);
                let field_identifier_string = ::std::string::ToString::to_string(&field_ident);

                let FieldIdentifier {
                    serialized,
                    ident,
                    surrealdb_field_ident,
                    surrealdb_imported_schema_dependency,
                } = FieldCaseMapper::new(field_case, field_identifier_string)
                    .get_field_ident(&field_receiver);

                // struct type used to type the function
                field_names_accumulator
                    .struct_ty_fields
                    .push(quote!(pub #ident: String));

                // struct values themselves
                field_names_accumulator
                    .struct_values_fields
                    .push(quote!(#ident: #serialized));

                field_names_accumulator
                    .models_serialized_values
                    .push(quote!(#surrealdb_field_ident));

                field_names_accumulator
                    .surrealdb_imported_schema_dependencies
                    .push(surrealdb_imported_schema_dependency);
                field_names_accumulator
            },
        )
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
}

#[derive(Debug, Clone)]
struct FieldCaseMapper {
    field_case: CaseString,
    field_identifier_string: ::std::string::String,
}

impl FieldCaseMapper {
    fn new(field_case: CaseString, field_identifier_string: ::std::string::String) -> Self {
        Self {
            field_case,
            field_identifier_string,
        }
    }

    /// Converts the field identifier string to the specified case.
    /// Also, if rename_all attribute is not specified to change the casing,
    /// it defaults to exactly how the fields are written out.
    /// However, Field rename attribute overrides this.
    pub(crate) fn to_case_string(&self) -> ::std::string::String {
        let convert_field_identifier = |case: convert_case::Case| {
            convert_case::Converter::new()
                .to_case(case)
                .convert(&self.field_identifier_string)
        };

        match self.field_case {
            CaseString::None => self.field_identifier_string.to_string(),
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
    pub(crate) fn get_field_ident(self, field_receiver: &MyFieldReceiver) -> FieldIdentifier {
        let field = self.to_case_string();
        let field = field.as_str();
        let field_ident_exact = syn::Ident::new(field, ::proc_macro2::Span::call_site());

        let surreal_schema_serializer = if field_receiver.skip_serializing {
            ::quote::quote!()
        } else {
            ::quote::quote!(pub)
        };

        let field_ident = match &self.field_case {
            // Tries to keep the field name ident as written in the struct
            //  if ure using kebab case which cannot be used as an identifier.
            // However, Field rename attribute overrides this
            CaseString::Kebab | CaseString::ScreamingKebab => &self.field_identifier_string,
            _ => field,
        };

        let field_ident = syn::Ident::new(field_ident, ::proc_macro2::Span::call_site());

        let x = RelateAttribute::from(field_receiver.relate.unwrap().into());
        let arrow_direction = TokenStream::from(x.edge_direction);
        let edge_action = TokenStream::from(x.edge_action);
        let schema_name_str = x.node_object.into();
        let schema_name = format_ident!("{schema_name_str}");
        let schema_name_alias = format_ident!("{schema_name_str}Schema");
        //  use super::AccountSchema as Account;
        let model_import_stream = quote!(use super::#schema_name_alias as #schema_name;);

        /*
        // This can the access the alias
          model!(Account{
            ->has->Account,
          })
        */
        let model_field_stream =
            quote!(#arrow_direction #edge_action #arrow_direction #schema_name,);

        // Prioritize serde/field_getter field_attribute renaming for field string
        if let ::std::option::Option::Some(name) = field_receiver.rename.as_ref() {
            let field_renamed_from_attribute = name.serialize.to_string();
            // let field_renamed_from_attribute = syn::Ident::new(name.serialize.to_string(), ::proc_macro2::Span::call_site());

            let (surreal_model_field, surrealdb_imported_schema_dependency) =
                match field_receiver.relate.clone() {
                    Some(relation) => {
                        // ->loves->Project as fav_project
                        // let dependent_type =
                        let xx = format_ident!("{relation} as {field_renamed_from_attribute}");
                        // let ident =
                        // ::quote::quote!(#relation as #field_renamed_from_attribute)
                        (
                            ::quote::quote!(->loves->Project as fav_proj),
                            ::quote::quote!(
                                use super::ProjectSchema as Project;
                            ),
                        )
                    }
                    None => {
                        let field_renamed_from_attribute_ident =
                            format_ident!("{}", &field_renamed_from_attribute);

                        (
                            ::quote::quote!(#field_renamed_from_attribute_ident),
                            quote!(),
                        )
                    }
                };
            // let surreal_model_field = match field_receiver.relate.clone() {
            //     Some(relation) => ::quote::quote!(#relation as #field_renamed_from_attribute),
            //     None => ::quote::quote!(#field_renamed_from_attribute),
            // };

            return FieldIdentifier {
                ident: ::quote::format_ident!("{}", &field_renamed_from_attribute),
                serialized: field_renamed_from_attribute,
                // surrealdb_field_ident: syn::Ident::new(&field_renamed_from_attribute, ::proc_macro2::Span::call_site()),
                // surrealdb_field_ident: ::quote::quote!(#surreal_schema_serializer #surreal_model_field),
                surrealdb_field_ident: surreal_model_field,
                surrealdb_imported_schema_dependency,
            };
        }

        // TODO: Dededup with the above
        /*      let (surreal_model_field, surrealdb_imported_schema_dependency) = match field_receiver
                   .relate
                   .clone()
               {
                   Some(relation) => {
                       let right_arrow_count = relation.matches("->").count();
                       let left_arrow_count = relation.matches("->").count();
                       let substrings = relation
                           .split("->")
                           .flat_map(|s| s.split("<-"))
                           .filter(|x| !x.is_empty())
                           .collect::<Vec<&str>>();
                       let span = syn::spanned::Spanned::span(&relation);
                       let start = span.start();
                       let end = span.end();

                       let message = format_args!(
                               "Invalid expression at  Check that your arrows are properly faced. e.g ->has->Heart or <-owned_by<-Human",
                               // &start.line,
                               // &start.column,
                               // &end.column
                           );

                       if right_arrow_count > 2 || left_arrow_count > 2 || substrings.len() > 2 {
                           panic!("{}", &message);
                           // let error = syn::Error::new_spanned(2, "Input cannot be empty");
                           // return Err(error);
                       }

                       let string = "hello world";
                       let literal = Literal::string(string);
                       // let tokens: TokenStream = literal.into_token_stream();

                       let direction = if right_arrow_count == 2 {
                           quote::quote!(->)
                       } else {
                           quote::quote!(<-)
                       };
                       // let xxxx = match substrings.as_slice() {
                       //     [edge_action, node_object] => {
                       //         let edge_action = format_ident!("{edge_action}");
                       //         let node_object = format_ident!("{node_object}");
                       //         quote!(->#edge_action->#node_object)
                       //     }
                       //     _ => panic!("{}", &message),
                       // };
                       let (edge_action, node_object) = substrings.get(0).zip(substrings.get(1)).unwrap();
                       let edge_action = format_ident!("{edge_action}");
                       let node_object = format_ident!("{node_object}");

                       // if right_arrow_count == 2 {
                       //     ::quote::quote!(->loves->Project as fav_proj)
                       // }
                       let x = "".split("pat").collect::<Vec<_>>();
                       // let strp = Literal::from("->loves->Project as fav_proj");
                       let strp = map_string_to_tokenstream("->loves->Project as fav_proj");
                       (
                           // ::quote::quote!(#relation as #field_ident_exact),
                           // ::quote::quote!(->loves->Project as fav_proj),
                           ::quote::quote!(#direction #edge_action #direction #node_object as #field_ident_exact),
                           // ::quote::quote!(#strp),
                           ::quote::quote!(
                               use super::ProjectSchema as Project;
                               // use super::AccountSchema as Account;
                           ),
                       )
                   }
                   None => (::quote::quote!(#field_ident_exact), quote!()),
               };
        */

        let x = RelateAttribute::from(field_receiver.relate.unwrap().into());
        // let skip_serializing = SkipSerializing::from(field_receiver.skip_serializing);
        FieldIdentifier {
            ident: field_ident.clone(),
            serialized: ::std::string::ToString::to_string(field),
            // surrealdb_field_ident: ::quote::quote!(#surreal_schema_serializer #surreal_model_field),
            // surrealdb_field_ident: ::std::string::ToString::to_string(field),
            surrealdb_field_ident: ::quote::quote!(#surreal_model_field),
            surrealdb_imported_schema_dependency: surrealdb_imported_schema_dependency,
        }
    }
}

fn map_string_to_tokenstream(string: &str) -> TokenStream {
    let literal = Literal::string(string);
    quote::quote_spanned! {literal.span()=> #literal}
}

#[derive(Debug, Clone)]
enum RelationType {
    RelationGraph(Relation),
    ReferenceOne(NodeObject),
    ReferenceMany(NodeObject),
    None,
}

#[derive(Debug, Clone, Copy)]
enum SkipSerializing {
    Yes,
    No,
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
            SkipSerializing::Yes => quote!(pub),
            SkipSerializing::No => quote!(),
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
    let span = syn::spanned::Spanned::span(relation.0.as_str()).clone();

    let start = span.clone().start().clone();
    let end = span.clone().end().clone();
    let start_line = start.line;
    let start_column = start.column;
    let end_column = end.column;
    let c = format_args!(
        " Check that your arrows are properly faced. e.g ->has->Heart or <-owned_by<-Human",
        // start_line,
        // start_column,
        // end_column
    );
    c
}
