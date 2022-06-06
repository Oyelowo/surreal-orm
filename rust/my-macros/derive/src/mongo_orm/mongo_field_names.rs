#![allow(dead_code)]

use convert_case::{Case, Casing};
use darling::{ast, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use strum_macros::EnumString;
use syn::{self, parse_macro_input};

/// Options: "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case",
/// "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
#[derive(Debug, Clone, Copy, EnumString, FromMeta)]
#[darling(default)]
pub enum CaseString {
    #[strum(serialize = "camelCase")]
    Camel,
    #[strum(serialize = "snake_case")]
    Snake,
    // Normal,
    #[strum(serialize = "PascalCase")]
    Pascal,

    #[strum(serialize = "lowercase")]
    Lower,

    #[strum(serialize = "UPPERCASE")]
    Upper,

    #[strum(serialize = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,

    #[strum(serialize = "kebab-case")]
    Kebab,

    #[strum(serialize = "SCREAMING-KEBAB-CASE")]
    ScreamingKebab,
}

impl Default for CaseString {
    fn default() -> Self {
        CaseString::Camel
    }
}

#[derive(Debug)]
pub struct Rename {
    serialize: String,
}

/// This enables us to handle potentially nested values i.e
///   #[serde(rename = "simple_name")]
///    or
///   #[serde(rename(serialize = "age"))]
///  #[serde(rename(serialize = "ser_name_nested", deserialize = "deser_name_nested"))]
/// However, We dont care about deserialized name from serde, so we just ignore that.
impl FromMeta for Rename {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self {
            serialize: value.into(),
        })
    }

    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
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
#[darling(attributes(key_getter, serde), forward_attrs(allow, doc, cfg))]
struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    rename: Option<Rename>,

    #[darling(default)]
    skip_serializing_if: util::Ignored,

    #[darling(default)]
    with: util::Ignored,

    #[darling(default)]
    default: util::Ignored,

    /// We declare this as an `Option` so that during tokenization we can write
    /// `field.case.unwrap_or(derive_input.case)` to facilitate field-level
    /// overrides of struct-level settings. I.O.W, if this is not provided
    /// at field level, we can fall back to the struct level settings by doing
    /// field.case.unwrap_or(struct_level.case). struct_level is from derive_input
    #[darling(default)]
    case: Option<CaseString>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(key_getter, serde), forward_attrs(allow, doc, cfg))]
pub struct KeyNamesGetterOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<util::Ignored, MyFieldReceiver>,

    #[darling(default)]
    rename_all: Option<::std::string::String>,

    #[darling(default)]
    typee: ::std::string::String,

    #[darling(default)]
    case: Option<CaseString>,
}

impl ToTokens for KeyNamesGetterOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let KeyNamesGetterOpts {
            ident: ref my_struct,
            ref data,
            ref case,
            rename_all: ref rename_all_from_serde,
            ..
        } = *self;

        let struct_level_casing = rename_all_from_serde.as_ref().map_or_else(
            || *case,
            |case_from_serde| CaseString::from_str(case_from_serde.as_str()).ok(),
        );

        let fields = get_fields(data);

        let FieldStore {
            struct_ty_fields,
            struct_values_fields,
        } = get_struct_types_and_fields(fields, struct_level_casing);

        let struct_name = syn::Ident::new(
            format!("{my_struct}KeyNames").as_str(),
            ::proc_macro2::Span::call_site(),
        );

        let struct_type = quote!(pub struct #struct_name {
           #( #struct_ty_fields), *
        });

        tokens.extend(quote! {
            #struct_type
            impl KeyNamesGetter for #my_struct {
                type KeyNames = #struct_name;
                fn get_field_names() -> Self::KeyNames {
                    #struct_name {
                        #( #struct_values_fields), *
                    }
                }


            }
        });
    }
}

#[derive(Debug, Default)]
struct FieldStore {
    struct_ty_fields: Vec<TokenStream>,
    struct_values_fields: Vec<TokenStream>,
}

fn get_struct_types_and_fields(
    fields: Vec<&MyFieldReceiver>,
    struct_level_casing: Option<CaseString>,
) -> FieldStore {
    let mut field_store = FieldStore::default();

    fields
        .into_iter()
        .enumerate()
        .for_each(|(index, field_receiver)| {
            create_fields_types_and_values(
                field_receiver,
                struct_level_casing,
                index,
                &mut field_store,
            );
        });

    field_store
}

fn create_fields_types_and_values(
    f: &MyFieldReceiver,
    struct_level_casing: Option<CaseString>,
    i: usize,
    store: &mut FieldStore,
) {
    let field_case = get_case_string(f, struct_level_casing);
    let field_ident = get_field_identifier(f, i);
    let field_identifier_string = ::std::string::ToString::to_string(&field_ident);
    let (key_as_str, key_ident) = get_key_str_and_ident(field_case, field_identifier_string, f);
    // struct type used to type the function
    store
        .struct_ty_fields
        .push(quote!(pub #key_ident: &'static str));
    // struct values themselves
    store
        .struct_values_fields
        .push(quote!(#key_ident: #key_as_str));
}

fn get_key_str_and_ident(
    field_case: CaseString,
    field_identifier_string: ::std::string::String,
    f: &MyFieldReceiver,
) -> (String, proc_macro2::Ident) {
    let key = &to_key_case_string(field_case, field_identifier_string);
    // Tries to keep the key name at camel if ure using kebab case which cannot be used
    // as an identifier
    let key_ident = match field_case {
        CaseString::Kebab | CaseString::ScreamingKebab => key.to_case(Case::Camel),
        _ => ::std::string::ToString::to_string(key),
    };
    let mut key = key.as_str();
    let mut key_ident = syn::Ident::from_string(key_ident.as_str())
        .expect("Problem converting key string to syntax identifier");

    // Prioritize serde renaming for key string
    let rename_field_from_serde = f.rename.as_ref();
    if let Some(name) = rename_field_from_serde {
        // We only care about the serialized string
        key = name.serialize.as_str();
        key_ident = syn::Ident::from_string(key)
            .expect("Problem converting key string to syntax identifier");
    }
    (::std::string::ToString::to_string(key), key_ident)
}

fn get_field_identifier(f: &MyFieldReceiver, index: usize) -> TokenStream {
    // This works with named or indexed fields, so we'll fall back to the index so we can
    // write the output as a key-value pair.
    f.ident.as_ref().map_or_else(
        || {
            let i = syn::Index::from(index);
            quote!(#i)
        },
        |v| quote!(#v),
    )
}

fn get_fields(data: &ast::Data<util::Ignored, MyFieldReceiver>) -> Vec<&MyFieldReceiver> {
    let fields = data
        .as_ref()
        .take_struct()
        .expect("Should never be enum")
        .fields;
    fields
}

fn get_case_string(f: &MyFieldReceiver, struct_level_casing: Option<CaseString>) -> CaseString {
    // Fallback to the struct metadata value if not provided for the field.
    // If not provided in both, fallback to camel.
    f.case.or(struct_level_casing).unwrap_or(CaseString::Camel)
}

// fn to_key_case_string(field_case: CaseString, field_identifier_string: ::std::string::String) -> ::std::string::String {
fn to_key_case_string(
    field_case: CaseString,
    field_identifier_string: ::std::string::String,
) -> ::std::string::String {
    let convert = |case: convert_case::Case| {
        convert_case::Converter::new()
            .to_case(case)
            .convert(&field_identifier_string)
    };
    match field_case {
        CaseString::Camel => convert(convert_case::Case::Camel),
        CaseString::Snake => convert(convert_case::Case::Snake),
        CaseString::Pascal => convert(convert_case::Case::Pascal),
        CaseString::Lower => convert(convert_case::Case::Lower),
        CaseString::Upper => convert(convert_case::Case::Upper),
        CaseString::ScreamingSnake => convert(convert_case::Case::ScreamingSnake),
        CaseString::Kebab => convert(convert_case::Case::Kebab),
        CaseString::ScreamingKebab => convert(convert_case::Case::UpperKebab),
    }
}

pub fn generate_key_names_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input);
    let output = KeyNamesGetterOpts::from_derive_input(&input).expect("Wrong options");
    quote!(#output).into()
}
