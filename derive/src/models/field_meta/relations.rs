/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use quote::ToTokens;
use syn::{spanned::Spanned, Type};

use crate::{errors::ExtractorResult, models::MyFieldReceiver};

use super::*;

#[derive(Debug, Clone)]
pub(crate) enum RelationType {
    // ->studies->Course
    Relate(Relate),
    LinkOne(NodeType),
    LinkSelf(NodeType),
    LinkMany(NodeType),
    NestObject(NodeType),
    NestArray(NodeType),
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
            } => RelationType::LinkOne(link_one.into()),
            MyFieldReceiver {
                link_self: Some(link_self),
                ..
            } => RelationType::LinkSelf(link_self.into()),
            MyFieldReceiver {
                link_many: Some(link_many),
                ..
            } => RelationType::LinkMany(link_many.into()),
            MyFieldReceiver {
                nest_object: Some(nest_object),
                ..
            } => RelationType::NestObject(nest_object.into()),
            MyFieldReceiver {
                nest_array: Some(nest_array),
                ..
            } => RelationType::NestArray(nest_array.into()),
            _ => RelationType::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum EdgeDirection {
    OutArrowRight,
    InArrowLeft,
}

impl ToTokens for EdgeDirection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let arrow = match self {
            EdgeDirection::OutArrowRight => "->",
            EdgeDirection::InArrowLeft => "<-",
        };
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
        match direction {
            EdgeDirection::OutArrowRight => "->",
            EdgeDirection::InArrowLeft => "<-",
        }
    }
}

impl From<&EdgeDirection> for String {
    fn from(direction: &EdgeDirection) -> Self {
        match direction {
            EdgeDirection::OutArrowRight => "->".into(),
            EdgeDirection::InArrowLeft => "<-".into(),
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

impl ::std::fmt::Display for EdgeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow = match self {
            EdgeDirection::OutArrowRight => "->",
            EdgeDirection::InArrowLeft => "<-",
        };
        f.write_fmt(format_args!("{}", arrow))
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
    };
}

#[derive(Debug, Clone)]
pub(crate) struct EdgeTableName(String);
wrapper_struct_to_ident!(EdgeTableName);

#[derive(Debug, Clone)]
pub(crate) struct NodeTableName(String);
wrapper_struct_to_ident!(NodeTableName);

// TODO: Remove. Just use LinkRustType from which type name could be extracted
// #[derive(Debug, Clone)]
// pub(crate) struct NodeTypeName(String);
// wrapper_struct_to_ident!(NodeTypeName);

//
#[derive(Debug, Clone)]
pub(crate) struct EdgeType(Type);

impl From<&Type> for EdgeType {
    fn from(ty: &Type) -> Self {
        Self(ty.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NodeType(Type);

impl From<Type> for NodeType {
    fn from(ty: Type) -> Self {
        Self(ty)
    }
}

impl ToTokens for NodeType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = &self.0;
        tokens.extend(ty.to_token_stream());
    }
}

impl NodeType {
    pub fn into_inner(self) -> Type {
        self.0
    }

    pub fn ident(&self) -> ExtractorResult<syn::Ident> {
        match &self.0 {
            Type::Path(type_path) => {
                let ident = type_path
                    .path
                    .segments
                    .last()
                    .expect("type path must have at least one segment")
                    .ident
                    .clone();
                Ok(ident)
            }
            _ => Err(syn::Error::new(
                self.0.to_token_stream().span(),
                "Only path type is supported",
            )
            .into()),
        }
    }
}

impl From<&Type> for NodeType {
    fn from(ty: &Type) -> Self {
        Self(ty.clone())
    }
}

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

impl From<&Relate> for RelateAttribute {
    fn from(relation: &Relate) -> Self {
        let right_arrow_count = relation.connection_model.matches("->").count();
        let left_arrow_count = relation.connection_model.matches("<-").count();
        let edge_direction = match (left_arrow_count, right_arrow_count) {
            (2, 0) => EdgeDirection::InArrowLeft,
            (0, 2) => EdgeDirection::OutArrowRight,
            _ => panic!("Arrow incorrectly used"),
        };

        let edge_direction_str: String = edge_direction.into();
        let mut substrings = relation
            .connection_model
            .split(edge_direction_str.as_str())
            .filter(|x| !x.is_empty());

        let (edge_action, node_object) =
            match (substrings.next(), substrings.next(), substrings.next()) {
                (Some(action), Some(node_obj), None) => {
                    (EdgeTableName(action.into()), NodeTableName(node_obj.into()))
                }
                _ => panic!(
                    "too many actions or object, {}",
                    get_relation_error(relation)
                ),
            };

        Self {
            node_table_name: node_object,
            edge_table_name: edge_action,
            edge_direction,
        }
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
