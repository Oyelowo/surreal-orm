/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 */

use proc_macro2::Span;
use quote::format_ident;
use surreal_query_builder::EdgeDirection;

use crate::models::*;

use super::*;
// Tuple, Array (T, T), [T; N]

// FieldTypeMetadata ->
// 1. RawFieldType -> Ident, LifeTime, Generics.
// 1. RawFieldType -> Ident, LifeTime, Generics.

// #[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// #[orm(table = "company")]
// pub struct Company<'a, 'b: 'a, T, U: Clone + Default> {
//     pub id: SurrealSimpleId<Self>,
//     pub name: &'b T, // &'b T RawField
//     pub moniker: &'a Somestuff<T, U>, // &'a Somestuff<T, U> RawFieldType
//     #[orm(link_self = Self<...>)]
//     pub branch: LinkSelf<Self<...>>, // user<'a, t>> rawfieldtype, linkrustfieldtype
//     #[orm(link_many = user<'a, t>)]
//     pub users: LinkMany<user<'a, t>>, // user<'a, t>> rawfieldtype, linkrustfieldtype
//     // impl Company<'a, 'b, T, U> {
//     //    pub fn users(&self) -> User<'a, T> { }
//
//     #[orm(relate(model = "CompanyLikeUser<'a, 'b, T, U>", connection = "->like->user"))]
//     pub devs: RelatedUser<'a, T, U>>, // RawFieldType, EdgeFieldType, DestinationFieldType
//     // pub devs: Relate<User<T>>,
// }

// - Company<'a, 'b, T, U>
// - Like<'a, 'b, T, U>  // Bridge or edge or link between origin and destination nodes
// - User<'a, T, U >
// type RelatedUser<'a, T> = Relate<User<'a, T>>;

#[derive(Debug, Clone)]
pub enum RelationType {
    // ->studies->Course
    Relate(Relate),
    LinkOne(LinkOneAttrType),
    LinkSelf(LinkSelfAttrType),
    LinkMany(LinkManyAttrType),
    // These are in and out nodes in edge tables and
    // we usually dont want to do much code gen for it beyond compile-time check codegen but
    // have it for deserialization and serialization, to enjoy
    // the benefits that `LinkMany` helper wrapper struct provides.
    LinkManyInAndOutEdgeNodesInert(LinkManyAttrType),
    NestObject(NestObjectAttrType),
    NestArray(NestArrayAttrType),
    List(ListSimple),
    None,
}

impl RelationType {
    pub fn is_relate_graph(&self) -> bool {
        matches!(self, RelationType::Relate(_))
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, RelationType::None)
    }

    pub fn is_link(&self) -> bool {
        matches!(
            self,
            RelationType::LinkOne(_)
                | RelationType::LinkSelf(_)
                | RelationType::LinkMany(_)
                | RelationType::LinkManyInAndOutEdgeNodesInert(_)
        )
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
            _ if field_receiver.is_array() || field_receiver.is_set() => {
                RelationType::List(ListSimple)
            }
            _ => RelationType::None,
        }
    }
}

// macro_rules! wrapper_struct_to_ident {
//     ($simple_wrapper_struct:ty) => {
//         impl ::std::convert::From<&$simple_wrapper_struct> for ::proc_macro2::TokenStream {
//             fn from(simple_wrapper_struct: &$simple_wrapper_struct) -> Self {
//                 let ident = ::quote::format_ident!("{}", simple_wrapper_struct.0);
//                 ::quote::quote!(#ident)
//             }
//         }
//
//         impl ::std::convert::From<$simple_wrapper_struct> for ::proc_macro2::TokenStream {
//             fn from(simple_wrapper_struct: $simple_wrapper_struct) -> Self {
//                 let ident = ::quote::format_ident!("{}", simple_wrapper_struct.0);
//                 ::quote::quote!(#ident)
//             }
//         }
//
//         impl ::std::fmt::Display for $simple_wrapper_struct {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 f.write_fmt(format_args!("{}", self.0))
//             }
//         }
//
//         impl ::std::convert::From<&String> for $simple_wrapper_struct {
//             fn from(value: &String) -> Self {
//                 Self(value.into())
//             }
//         }
//
//         impl ::std::convert::From<$simple_wrapper_struct> for String {
//             fn from(value: $simple_wrapper_struct) -> Self {
//                 value.0
//             }
//         }
//
//
//         impl ::quote::ToTokens for $simple_wrapper_struct {
//             fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//                 let ident = ::quote::format_ident!("{self}");
//                 tokens.extend(ident.into_token_stream());
//             }
//         }
//     };
// }

// #[derive(Debug, Clone)]
// pub struct EdgeTableName(String);
// wrapper_struct_to_ident!(EdgeTableName);
//
// #[derive(Debug, Clone)]
// pub struct NodeTableName(String);
// wrapper_struct_to_ident!(NodeTableName);

create_ident_wrapper!(EdgeTableName);
create_ident_wrapper!(NodeTableName);

#[derive(Debug, Clone)]
pub struct RelateAttribute {
    pub(crate) edge_direction: EdgeDirection,
    pub(crate) edge_table: EdgeTableName,
    /// user->writes->book // here, user is current struct, book is the foreign node
    /// book<-writes<-user // here, book is current struct, user is the foreign node
    pub(crate) foreign_node_table: NodeTableName,
}

// TODO: Remove
// impl From<RelateAttribute> for ::proc_macro2::TokenStream {
//     fn from(relate_attrs: RelateAttribute) -> Self {
//         let edge_direction = ::proc_macro2::TokenStream::from(relate_attrs.edge_direction);
//         let edge_name = ::proc_macro2::TokenStream::from(relate_attrs.edge_table);
//         let node_name = ::proc_macro2::TokenStream::from(relate_attrs.node_table);
//         // ->action->NodeObject
//         // <-action<-NodeObject
//         // e.g ->manages->Project
//         ::quote::quote!(#edge_direction #edge_name #edge_direction #node_name)
//     }
// }

impl TryFrom<&Relate> for RelateAttribute {
    type Error = ExtractorError;

    fn try_from(relation: &Relate) -> Result<Self, Self::Error> {
        let right_arrow_count = relation.connection.matches("->").count();
        let left_arrow_count = relation.connection.matches("<-").count();
        let edge_direction = match (left_arrow_count, right_arrow_count) {
            (2, 0) => EdgeDirection::In,
            (0, 2) => EdgeDirection::Out,
            _ => {
                // return Err(syn::Error::new(
                //     Span::call_site(),
                //     "Invalid arrow direction usage. Should be either only -> or <-",
                // )
                // .into())
                return Err(darling::Error::custom(
                    "Invalid arrow direction usage. Should be either only -> or <-",
                )
                .into());
            }
        };

        let edge_direction_str = edge_direction.as_arrow_symbol().to_string();
        let mut substrings = relation
            .connection
            .split(edge_direction_str.as_str())
            .filter(|this| !this.is_empty());

        let (edge_action, node_object) =
            match (substrings.next(), substrings.next(), substrings.next()) {
                (Some(action), Some(node_obj), None) => (
                    EdgeTableName(format_ident!("{action}")),
                    NodeTableName(format_ident!("{node_obj}")),
                ),
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        format!("too many edges or nodes, {}", get_relation_error(relation)),
                    )
                    .into())
                }
            };

        Ok(Self {
            foreign_node_table: node_object,
            edge_table: edge_action,
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
