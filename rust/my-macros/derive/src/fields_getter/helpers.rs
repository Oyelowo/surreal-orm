#![allow(dead_code)]

use darling::{ast, util};
use proc_macro2::{Span, TokenStream, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

use syn::{self, Ident};

use super::{trait_generator::MyFieldReceiver, types::CaseString};

#[derive(Debug, Default)]
pub(crate) struct FieldStore {
    pub struct_ty_fields: Vec<TokenStream>,
    pub struct_values_fields: Vec<TokenStream>,
}

pub(crate) fn get_struct_types_and_fields(
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

pub(crate) fn create_fields_types_and_values(
    f: &MyFieldReceiver,
    struct_level_casing: Option<CaseString>,
    i: usize,
    store: &mut FieldStore,
) {
    let field_case = struct_level_casing.unwrap_or(CaseString::Untouched);
    let field_ident = get_field_identifier(f, i);
    let field_identifier_string = ::std::string::ToString::to_string(&field_ident);

    let FieldFormat { serialized, ident } =
        get_field_str_and_ident(&field_case, &field_identifier_string, f);

    // struct type used to type the function
    store
        .struct_ty_fields
        .push(quote!(pub #ident: &'static str));

    // struct values themselves
    store.struct_values_fields.push(quote!(#ident: #serialized));
}

pub(crate) struct FieldFormat {
    serialized: ::std::string::String,
    ident: syn::Ident,
}
pub(crate) fn get_field_str_and_ident(
    field_case: &CaseString,
    field_identifier_string: &::std::string::String,
    f: &MyFieldReceiver,
) -> FieldFormat {
    let field = to_case_string(field_case, field_identifier_string);
    let mut field = field.as_str();

    use CaseString::*;
    let field_ident = match field_case {
        // Tries to keep the field name ident as written in the struct
        //  if ure using kebab case which cannot be used as an identifier.
        // However, Field rename attribute overrides this
        Kebab | ScreamingKebab => field_identifier_string,
        _ => field,
    };

    let mut field_ident = syn::Ident::new(field_ident, ::proc_macro2::Span::call_site());

    // Prioritize serde renaming for field string
    let rename_field_from_serde = f.rename.as_ref();
    if let ::std::option::Option::Some(name) = rename_field_from_serde {
        // We only care about the serialized string
        field = name.serialize.as_str();
        field_ident = syn::Ident::new(field, ::proc_macro2::Span::call_site());
    }
    FieldFormat {
        /*
        Ident format is the name used in the code
        e.g struct User{
             user_name: String    // Here: user_name is ident and the serialized format by serde is "user_name"
        }
        This is what we use as the field name and is mostly same as the serialized format
        except in the case of kebab-case serialized format in whcih case we fallback
        to the original ident format as written exactly in the code except when a use
        uses rename attribute on a specific field, in which case that takes precedence.
        */
        ident: field_ident,
        serialized: ::std::string::ToString::to_string(field),
    }
}

pub(crate) fn get_field_identifier(f: &MyFieldReceiver, index: usize) -> TokenStream {
    // This works with named or indexed fields, so we'll fall back to the index so we can
    // write the output as a key-value pair.
    // the index is really not necessary since our models will nevel be tuple struct
    // but leaving it as is anyways
    f.ident.as_ref().map_or_else(
        || {
            let i = syn::Index::from(index);
            quote!(#i)
        },
        |v| quote!(#v),
    )
}

pub(crate) fn get_fields(
    data: &ast::Data<util::Ignored, MyFieldReceiver>,
) -> Vec<&MyFieldReceiver> {
    let fields = data
        .as_ref()
        .take_struct()
        .expect("Should never be enum")
        .fields;
    fields
}

pub(crate) fn to_case_string(
    field_case: &CaseString,
    field_identifier_string: &::std::string::String,
) -> ::std::string::String {
    let convert = |case: convert_case::Case| {
        convert_case::Converter::new()
            .to_case(case)
            .convert(field_identifier_string)
    };
    match field_case {
        // Also, if rename_all attribute is not specified to change the casing,
        // it defaults to exactly how the fields are written out.
        // However, Field rename attribute overrides this
        CaseString::Untouched => field_identifier_string.to_string(),
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

pub fn get_crate_name(internal: bool) -> TokenStream {
    if internal {
        quote! { crate }
    } else {
        let name = match crate_name("my-macros") {
            Ok(FoundCrate::Name(name)) => name,
            Ok(FoundCrate::Itself) | Err(_) => "my_macros".to_string(),
        };
        TokenTree::from(Ident::new(&name, Span::call_site())).into()
    }
}
