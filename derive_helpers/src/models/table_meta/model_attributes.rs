/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::str::FromStr;

use syn::{self, parse_quote, GenericArgument, Path, PathArguments, Type};

use super::{table::TableNameIdent, StructIdent};

use crate::models::{edge::EdgeToken, node::NodeToken, object::ObjectToken, *};

// pub trait ModelAttributes
// where
//     Self: Sized,
// {
//     fn rename_all(&self) -> Option<Rename>;
//     fn ident(&self) -> StructIdent;
//     fn generics(&self) -> &StructGenerics;
//
//     fn casing(&self) -> ExtractorResult<StructLevelCasing> {
//         let struct_level_casing = self
//             .rename_all()
//             .as_ref()
//             .map(|case| CaseString::from_str(case.serialize.as_str()));
//
//         let casing = match struct_level_casing {
//             Some(Ok(case)) => case,
//             Some(Err(e)) => return Err(darling::Error::custom(e.to_string()).into()),
//             None => CaseString::None,
//         };
//         Ok(casing.into())
//     }
//
//     fn struct_as_path_no_bounds(&self) -> Path {
//         // let replacement_path: Path = parse_quote!(#struct_name #ty_generics);
//         self.construct_struct_type_without_bounds()
//             .replace_self_with_current_struct_concrete_type(self)
//             .to_path()
//     }
//
//     fn construct_struct_type_without_bounds(&self) -> CustomType {
//         let mut path = Path::from(self.ident());
//         let generics = self.generics().to_basic_generics();
//
//         // Process generics, excluding bounds
//         if !generics.params.is_empty() {
//             let args = generics
//                 .params
//                 .iter()
//                 .map(|param| match param {
//                     syn::GenericParam::Type(type_param) => {
//                         GenericArgument::Type(parse_quote!(#type_param))
//                     }
//                     syn::GenericParam::Lifetime(lifetime_def) => {
//                         GenericArgument::Lifetime(lifetime_def.lifetime.clone())
//                     }
//                     syn::GenericParam::Const(const_param) => {
//                         // TODO: Test this in struct
//                         GenericArgument::Const(
//                             const_param
//                                 .default
//                                 .clone()
//                                 .expect("absent const expression"),
//                         )
//                     }
//                 })
//                 .collect();
//
//             path.segments.last_mut().unwrap().arguments =
//                 PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
//                     colon2_token: None,
//                     lt_token: generics.lt_token.unwrap(),
//                     args,
//                     gt_token: generics.gt_token.unwrap(),
//                 });
//         }
//
//         Type::Path(syn::TypePath { qself: None, path }).into()
//     }
// }

create_tokenstream_wrapper!(=>ExplicitFullyQualifiedGenericsPath);

#[derive(Clone, Debug)]
pub enum ModelAttributes<'a> {
    Node(&'a NodeToken),
    Edge(&'a EdgeToken),
    Object(&'a ObjectToken),
}

impl<'a> ModelAttributes<'a> {
    pub fn from_node(node: &'a NodeToken) -> Self {
        ModelAttributes::Node(node)
    }

    pub fn from_edge(edge: &'a EdgeToken) -> Self {
        ModelAttributes::Edge(edge)
    }

    pub fn from_object(object: &'a ObjectToken) -> Self {
        ModelAttributes::Object(object)
    }

    pub fn fields(&self) -> ExtractorResult<Vec<&MyFieldReceiver>> {
        let fields = match self {
            ModelAttributes::Node(node) => &node.0.data,
            ModelAttributes::Edge(edge) => &edge.0.data,
            ModelAttributes::Object(object) => &object.data,
        };
        Ok(fields
            .as_ref()
            .take_struct()
            .ok_or(darling::Error::custom("Expected a struct"))?
            .fields)
    }

    pub fn rename_all(&self) -> Option<&Rename> {
        match self {
            ModelAttributes::Node(node) => node.0.rename_all.as_ref(),
            ModelAttributes::Edge(edge) => edge.0.rename_all.as_ref(),
            ModelAttributes::Object(object) => object.rename_all.as_ref(),
        }
    }

    pub fn ident(&self) -> StructIdent {
        match self {
            ModelAttributes::Node(node) => node.ident(),
            ModelAttributes::Edge(edge) => edge.ident(),
            ModelAttributes::Object(object) => object.ident(),
        }
    }

    pub fn generics(&self) -> &StructGenerics {
        use ModelAttributes::{Edge, Node, Object};
        match self {
            Node(node) => node.generics(),
            Edge(edge) => edge.generics(),
            Object(object) => object.generics(),
        }
    }

    pub fn table(&self) -> ExtractorResult<Option<&TableNameIdent>> {
        let table_name: Option<&table_meta::table::TableNameIdent> = match self {
            ModelAttributes::Node(node) => Some(node.table()?),
            ModelAttributes::Edge(edge) => Some(edge.table()?),
            // Objects don't have a table
            ModelAttributes::Object(_object) => None,
        };
        Ok(table_name)
    }

    pub fn explicit_fully_qualified_generics_path(&self) -> ExplicitFullyQualifiedGenericsPath {
        let (_struct_impl_generics, struct_ty_generics, _struct_where_clause) =
            &self.generics().split_for_impl();
        let explicit_generics = if struct_ty_generics.into_token_stream().is_empty() {
            quote!()
        } else {
            quote!(::#struct_ty_generics)
        };
        explicit_generics.into()
    }

    pub fn to_data_type(&self) -> DataType {
        match self {
            ModelAttributes::Node(_) => DataType::Node,
            ModelAttributes::Edge(_) => DataType::Edge,
            ModelAttributes::Object(_) => DataType::Object,
        }
    }

    pub fn casing(&self) -> ExtractorResult<StructLevelCasing> {
        let struct_level_casing = self
            .rename_all()
            .as_ref()
            .map(|case| case.serialize.clone())
            .flatten()
            .map(|case| CaseString::from_str(case.as_str()));

        let casing = match struct_level_casing {
            Some(Ok(case)) => case,
            Some(Err(e)) => return Err(darling::Error::custom(e.to_string()).into()),
            None => CaseString::None,
        };
        Ok(casing.into())
    }

    pub fn struct_no_bounds(&self) -> ExtractorResult<CustomTypeNoSelf> {
        // let replacement_path: Path = parse_quote!(#struct_name #ty_generics);
        self.construct_struct_type_without_bounds()
            .replace_self_with_current_struct_concrete_type(self)
    }

    fn construct_struct_type_without_bounds(&self) -> CustomType {
        let mut path = Path::from(self.ident());
        let generics = self.generics().to_basic_generics_ref();

        // Process generics, excluding bounds
        if !generics.params.is_empty() {
            let args = generics
                .params
                .iter()
                .map(|param| match param {
                    syn::GenericParam::Type(type_param) => {
                        GenericArgument::Type(parse_quote!(#type_param))
                    }
                    syn::GenericParam::Lifetime(lifetime_def) => {
                        GenericArgument::Lifetime(lifetime_def.lifetime.clone())
                    }
                    syn::GenericParam::Const(const_param) => {
                        // TODO: Test this in struct
                        GenericArgument::Const(
                            const_param
                                .default
                                .clone()
                                .expect("absent const expression"),
                        )
                    }
                })
                .collect();

            path.segments
                .last_mut()
                .expect("Problem getting last segment of path. Path potentially empty.")
                .arguments = PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: generics.lt_token.expect("Missing lt token"),
                args,
                gt_token: generics.gt_token.expect("Missing gt token"),
            });
        }

        Type::Path(syn::TypePath { qself: None, path }).into()
    }
}
