use std::str::FromStr;

use darling;
use syn::{self, parse_quote, GenericArgument, Path, PathArguments, Type};

use super::StructIdent;

use crate::models::{edge::EdgeToken, node::NodeToken, object::ObjectToken, *};

pub trait ModelAttributes
where
    Self: Sized,
{
    fn rename_all(&self) -> Option<Rename>;
    fn ident(&self) -> StructIdent;
    fn generics(&self) -> &StructGenerics;

    fn casing(&self) -> ExtractorResult<StructLevelCasing> {
        let struct_level_casing = self
            .rename_all()
            .as_ref()
            .map(|case| CaseString::from_str(case.serialize.as_str()));

        let casing = match struct_level_casing {
            Some(Ok(case)) => case,
            Some(Err(e)) => return Err(darling::Error::custom(e.to_string()).into()),
            None => CaseString::None,
        };
        Ok(casing.into())
    }

    fn struct_as_path_no_bounds(&self) -> Path {
        // let replacement_path: Path = parse_quote!(#struct_name #ty_generics);
        self.construct_struct_type_without_bounds()
            .replace_self_with_current_struct_concrete_type(self)
            .to_path()
    }

    fn construct_struct_type_without_bounds(&self) -> CustomType {
        let mut path = Path::from(self.ident());
        let generics = self.generics().to_basic_generics();

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

            path.segments.last_mut().unwrap().arguments =
                PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: generics.lt_token.unwrap(),
                    args,
                    gt_token: generics.gt_token.unwrap(),
                });
        }

        Type::Path(syn::TypePath { qself: None, path }).into()
    }
}

// enum ModelAtt {
//     Node(NodeToken),
//     Edge(EdgeToken),
//     Object(ObjectToken),
// }
//
// impl ModelAtt {
//     fn rename_all(&self) -> Option<Rename> {
//         match self {
//             ModelAtt::Node(node) => node.rename_all(),
//             ModelAtt::Edge(edge) => edge.rename_all(),
//             ModelAtt::Object(object) => object.rename_all(),
//         }
//     }
//
//     fn ident(&self) -> StructIdent {
//         match self {
//             ModelAtt::Node(node) => node.ident(),
//             ModelAtt::Edge(edge) => edge.ident(),
//             ModelAtt::Object(object) => object.ident(),
//         }
//     }
//
//     fn generics(&self) -> &StructGenerics {
//         match self {
//             ModelAtt::Node(node) | ModelAtt::Edge(node) | ModelAtt::Object(node) => node.generics(),
//             // ModelAtt::Edge(edge) => edge.generics(),
//             // ModelAtt::Object(object) => object.generics(),
//         }
//     }
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
