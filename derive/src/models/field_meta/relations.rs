/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro::Span;
use quote::ToTokens;
use syn::{spanned::Spanned, Type};

use crate::{
    errors::{ExtractorError, ExtractorResult},
    models::MyFieldReceiver,
};

use super::*;
// Tuple, Array (T, T), [T; N]

// FieldTypeMetadata ->
// 1. RawFieldType -> Ident, LifeTime, Generics.
// 1. RawFieldType -> Ident, LifeTime, Generics.

// #[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "company")]
// pub struct Company<'a, 'b: 'a, T, U: Clone + Default> {
//     pub id: SurrealSimpleId<Self>,
//     pub name: &'b T, // &'b T RawField
//     pub moniker: &'a Somestuff<T, U>, // &'a Somestuff<T, U> RawFieldType
//     #[surreal_orm(link_self = Self<...>)]
//     pub branch: LinkSelf<Self<...>>, // user<'a, t>> rawfieldtype, linkrustfieldtype
//     #[surreal_orm(link_many = user<'a, t>)]
//     pub users: LinkMany<user<'a, t>>, // user<'a, t>> rawfieldtype, linkrustfieldtype
//     // impl Company<'a, 'b, T, U> {
//     //    pub fn users(&self) -> User<'a, T> { }
//
//     #[surreal_orm(relate(model = "CompanyLikeUser<'a, 'b, T, U>", connection = "->like->user"))]
//     pub devs: RelatedUser<'a, T, U>>, // RawFieldType, EdgeFieldType, DestinationFieldType
//     // pub devs: Relate<User<T>>,
// }

// - Company<'a, 'b, T, U>
// - Like<'a, 'b, T, U>  // Bridge or edge or link between origin and destination nodes
// - User<'a, T, U >
// type RelatedUser<'a, T> = Relate<User<'a, T>>;

#[derive(Debug, Clone)]
pub(crate) enum RelationType {
    // ->studies->Course
    Relate(Relate),
    LinkOne(LinkOneAttrType),
    LinkSelf(LinkSelfAttrType),
    LinkMany(LinkManyAttrType),
    NestObject(NestObjectAttrType),
    NestArray(NestArrayAttrType),
    List(ListSimple),
    None,
}

impl RelationType {
    pub fn is_none_relational(&self) -> bool {
        matches!(self, RelationType::None)
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, RelationType::None)
    }
}

impl From<&MyFieldReceiver> for RelationType {
    fn from(field_receiver: &MyFieldReceiver) -> Self {
        match field_receiver {
            MyFieldReceiver {
                relate: Some(relation),
                ..
            } => RelationType::Relate(relation.to_owned()),
            MyFieldReceiver {
                link_one: Some(link_one),
                ..
            } => RelationType::LinkOne(link_one.to_owned()),
            MyFieldReceiver {
                link_self: Some(link_self),
                ..
            } => RelationType::LinkSelf(link_self.to_owned()),
            MyFieldReceiver {
                link_many: Some(link_many),
                ..
            } => RelationType::LinkMany(link_many.to_owned()),
            MyFieldReceiver {
                nest_object: Some(nest_object),
                ..
            } => RelationType::NestObject(nest_object.to_owned()),
            MyFieldReceiver {
                nest_array: Some(nest_array),
                ..
            } => RelationType::NestArray(nest_array.to_owned()),
            _ if field_receiver.is_list() => RelationType::List(ListSimple),
            _ => RelationType::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum EdgeDirection {
    OutArrowRight,
    InArrowLeft,
}

impl EdgeDirection {
    pub fn as_arrow_symbol(&self) -> &'static str {
        let arrow = match self {
            EdgeDirection::OutArrowRight => "->",
            EdgeDirection::InArrowLeft => "<-",
        };
        arrow
    }
}

impl ToTokens for EdgeDirection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let arrow = self.as_arrow_symbol();
        tokens.extend(arrow.parse::<proc_macro2::TokenStream>().unwrap());
    }
}

impl From<EdgeDirection> for ::proc_macro2::TokenStream {
    fn from(direction: EdgeDirection) -> Self {
        match direction {
            EdgeDirection::OutArrowRight => quote::quote!(->),
            EdgeDirection::InArrowLeft => quote::quote!(<-),
        }
    }
}

impl From<EdgeDirection> for &str {
    fn from(direction: EdgeDirection) -> Self {
        direction.as_arrow_symbol()
    }
}

impl From<&EdgeDirection> for String {
    fn from(direction: &EdgeDirection) -> Self {
        direction.as_arrow_symbol().into()
    }
}

impl From<EdgeDirection> for String {
    fn from(direction: EdgeDirection) -> Self {
        direction.as_arrow_symbol().into()
    }
}

impl ::std::fmt::Display for EdgeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow = direction.as_arrow_symbol().into();
        write!(f, "{arrow}")
        // f.write_fmt(format_args!("{arrow}"))
    }
}

macro_rules! wrapper_struct_to_ident {
    ($simple_wrapper_struct:ty) => {
        impl From<&$simple_wrapper_struct> for ::proc_macro2::TokenStream {
            fn from(simple_wrapper_struct: &$simple_wrapper_struct) -> Self {
                let ident = ::quote::format_ident!("{}", simple_wrapper_struct.0);
                ::quote::quote!(#ident)
            }
        }

        impl From<$simple_wrapper_struct> for ::proc_macro2::TokenStream {
            fn from(simple_wrapper_struct: $simple_wrapper_struct) -> Self {
                let ident = ::quote::format_ident!("{}", simple_wrapper_struct.0);
                ::quote::quote!(#ident)
            }
        }

        impl ::std::fmt::Display for $simple_wrapper_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{}", self.0))
            }
        }

        impl From<&String> for $simple_wrapper_struct {
            fn from(value: &String) -> Self {
                Self(value.into())
            }
        }

        impl From<$simple_wrapper_struct> for String {
            fn from(value: $simple_wrapper_struct) -> Self {
                value.0
            }
        }


        impl ToTokens for $simple_wrapper_struct {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                let ident = ::quote::format_ident!("{self}");
                tokens.extend(ident.into_token_stream());
            }
        }
    };
}

#[derive(Debug, Clone)]
pub(crate) struct EdgeTableName(String);
wrapper_struct_to_ident!(EdgeTableName);

#[derive(Debug, Clone)]
pub(crate) struct NodeTableName(String);
wrapper_struct_to_ident!(NodeTableName);

#[derive(Debug, Clone)]
pub(crate) struct RelateAttribute {
    pub(crate) edge_direction: EdgeDirection,
    pub(crate) edge_table_name: EdgeTableName,
    pub(crate) node_table_name: NodeTableName,
}

// TODO: Remove
// impl From<RelateAttribute> for ::proc_macro2::TokenStream {
//     fn from(relate_attrs: RelateAttribute) -> Self {
//         let edge_direction = ::proc_macro2::TokenStream::from(relate_attrs.edge_direction);
//         let edge_name = ::proc_macro2::TokenStream::from(relate_attrs.edge_table_name);
//         let node_name = ::proc_macro2::TokenStream::from(relate_attrs.node_table_name);
//         // ->action->NodeObject
//         // <-action<-NodeObject
//         // e.g ->manages->Project
//         ::quote::quote!(#edge_direction #edge_name #edge_direction #node_name)
//     }
// }

impl TryFrom<&Relate> for RelateAttribute {
    type Error = ExtractorError;

    fn try_from(value: &Relate) -> Result<Self, Self::Error> {
        let right_arrow_count = relation.connection.matches("->").count();
        let left_arrow_count = relation.connection.matches("<-").count();
        let edge_direction = match (left_arrow_count, right_arrow_count) {
            (2, 0) => EdgeDirection::InArrowLeft,
            (0, 2) => EdgeDirection::OutArrowRight,
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Invalid arrow direction usage. Should be either only -> or <-",
                )
                .into())
            }
        };

        let edge_direction_str: String = edge_direction.into();
        let mut substrings = relation
            .connection
            .split(edge_direction_str.as_str())
            .filter(|x| !x.is_empty());

        let (edge_action, node_object) =
            match (substrings.next(), substrings.next(), substrings.next()) {
                (Some(action), Some(node_obj), None) => {
                    (EdgeTableName(action.into()), NodeTableName(node_obj.into()))
                }
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        format!("too many edges or nodes, {}", get_relation_error(relation)),
                    ))
                }
            };

        Ok(Self {
            node_table_name: node_object,
            edge_table_name: edge_action,
            edge_direction,
        })
    }
}

fn get_relation_error<'a>(_relation: &Relate) -> ::std::fmt::Arguments<'a> {
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
