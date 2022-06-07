#![allow(dead_code)]

use super::{
    helpers::{
        get_field_identifier, get_field_str_and_ident, get_fields, get_struct_types_and_fields,
        FieldStore,
    },
    types::CaseString,
};
use darling::{ast, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use strum_macros::EnumString;
use syn::{self, parse_macro_input};

#[derive(Debug, Clone)]
pub struct Rename {
    pub(crate) serialize: String,
}

/// This enables us to handle potentially nested values i.e
///   #[serde(rename = "simple_name")]
///    or
///   #[serde(rename(serialize = "age"))]
///  #[serde(rename(serialize = "ser_name_nested", deserialize = "deser_name_nested"))]
/// However, We dont care about deserialized name from serde, so we just ignore that.
impl FromMeta for Rename {
    fn from_string(value: &str) -> ::darling::Result<Self> {
        Ok(Self {
            serialize: value.into(),
        })
    }

    fn from_list(items: &[syn::NestedMeta]) -> ::darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRename {
            serialize: String,

            #[darling(default)]
            deserialize: util::Ignored, // Ignore deserialize since we only care about the serialized string
        }

        impl From<FullRename> for Rename {
            fn from(v: FullRename) -> Self {
                let FullRename { serialize, .. } = v;
                Self { serialize }
            }
        }
        FullRename::from_list(items).map(Rename::from)
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(field_getter, serde), forward_attrs(allow, doc, cfg))]
pub(crate) struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub(crate) ident: ::std::option::Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub(crate) rename: ::std::option::Option<Rename>,

    #[darling(default)]
    skip_serializing_if: ::darling::util::Ignored,

    #[darling(default)]
    with: ::darling::util::Ignored,

    #[darling(default)]
    default: ::darling::util::Ignored,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(field_getter, serde), forward_attrs(allow, doc, cfg))]
pub struct FieldsGetterOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<util::Ignored, self::MyFieldReceiver>,

    #[darling(default)]
    rename_all: ::std::option::Option<Rename>,
}

impl ToTokens for FieldsGetterOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldsGetterOpts {
            ident: ref my_struct,
            ref data,
            ref rename_all,
            ..
        } = *self;

        let struct_level_casing = rename_all.as_ref().map(|case| {
            CaseString::from_str(case.serialize.as_str()).expect("Invalid casing, The options are")
        });

        let fields = get_fields(data);

        let FieldStore {
            struct_ty_fields,
            struct_values_fields,
        } = get_struct_types_and_fields(fields, struct_level_casing);

        let struct_name = syn::Ident::new(
            format!("{my_struct}Fields").as_str(),
            ::proc_macro2::Span::call_site(),
        );

        let struct_type = quote!(pub struct #struct_name {
           #( #struct_ty_fields), *
        });

        tokens.extend(quote! {
            #struct_type
            impl FieldsGetter for #my_struct {
                type Fields = #struct_name;
                fn get_fields_serialized() -> Self::Fields {
                    #struct_name {
                        #( #struct_values_fields), *
                    }
                }


            }
        });
    }
}

pub fn generate_fields_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input);
    // let output = FieldsGetterOpts::from_derive_input(&input).expect("Wrong options");
    let output = match FieldsGetterOpts::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
