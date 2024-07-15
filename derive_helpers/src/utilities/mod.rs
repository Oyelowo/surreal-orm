pub mod ident;

use darling::{ast::Data, util, FromDeriveInput, FromField, FromMeta};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, format_ident, ToTokens};
use std::str::FromStr;
use strum_macros::EnumString;
use syn::{Ident, Type};

use crate::models::{CaseString, ExtractorResult, StructGenerics};

use self::ident::FieldIdentNormalizedDeserialized;

#[derive(Debug, Clone)]
pub struct RenameDeserialize {
    pub(crate) deserialize: String,
}

/// This enables us to handle potentially nested values i.e
///   #[serde(rename = "simple_name")]
///    or
///   #[serde(rename(deserialize = "age"))]
///  #[serde(rename(serialize = "ser_name_nested", deserialize = "deser_name_nested"))]
/// However, We dont care about deserialized name from serde, so we just ignore that.
impl FromMeta for RenameDeserialize {
    fn from_string(value: &str) -> ::darling::Result<Self> {
        Ok(Self {
            deserialize: value.into(),
        })
    }

    fn from_list(items: &[darling::ast::NestedMeta]) -> ::darling::Result<Self> {
        #[derive(FromMeta)]
        struct FullRename {
            deserialize: String,

            #[darling(default)]
            #[allow(dead_code)]
            serialize: util::Ignored, // Ignore deserialize since we only care about the serialized string
        }

        impl From<FullRename> for RenameDeserialize {
            fn from(v: FullRename) -> Self {
                let FullRename { deserialize, .. } = v;
                Self { deserialize }
            }
        }
        FullRename::from_list(items).map(RenameDeserialize::from)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StructLevelCasingDeserialize(CaseString);

impl From<CaseString> for StructLevelCasingDeserialize {
    fn from(value: CaseString) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for StructLevelCasingDeserialize {
    type Target = CaseString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct TableDeriveAttributesPickable {
    pub(crate) ident: Ident,
    // pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: StructGenerics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    pub data: Data<util::Ignored, FieldAttribute>,

    #[darling(default)]
    pub(crate) rename_all: Option<RenameDeserialize>,
}

#[derive(Debug, Clone, Default)]
struct PickableMetadata<'a> {
    pub(crate) field_name_normalized_deserialized: Vec<FieldIdentNormalizedDeserialized>,
    pub(crate) field_type: Vec<&'a Type>,
}

impl TableDeriveAttributesPickable {
    pub fn casing_deserialize(&self) -> ExtractorResult<StructLevelCasingDeserialize> {
        let struct_level_casing = self
            .rename_all
            .as_ref()
            .map(|case| CaseString::from_str(case.deserialize.as_str()));

        let casing = match struct_level_casing {
            Some(Ok(case)) => case,
            Some(Err(e)) => return Err(darling::Error::custom(e.to_string()).into()),
            None => CaseString::None,
        };
        Ok(casing.into())
    }

    pub(crate) fn get_meta(&self) -> ExtractorResult<PickableMetadata> {
        let fields =
            self.data.as_ref().take_struct().ok_or(
                darling::Error::custom("Only structs are supported").with_span(&self.ident),
            )?;
        let struct_casing_de = self.casing_deserialize()?;

        let mut meta = PickableMetadata::default();

        for field_attr in fields {
            let f = field_attr.field_ident_normalized_deserialized_rawable(&struct_casing_de)?;
            meta.field_name_normalized_deserialized.push(f);
            meta.field_type.push(&field_attr.ty);
        }

        Ok(meta)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, FromField)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct FieldAttribute {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    pub ident: Option<Ident>,
    /// This magic field name pulls the type from the input.
    pub ty: Type,
    pub attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub(crate) rename: Option<RenameDeserialize>,
}

struct FieldNameDeserialzed(Ident);

// #[derive(Debug, Copy, Clone)]
// pub struct StructLevelCasing(CaseString);
/// Options: "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case",
/// "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"

impl FieldAttribute {
    // pub fn field_names_deserialized_fmt(&self) -> ExtractorResult<FieldNameDeserialzed> {
    //     let ident = self.ident.as_ref().unwrap();
    //     format_ident!("__{}_deserialized_fmt", ident)
    // }
}

impl ToTokens for TableDeriveAttributesPickable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_name = get_crate_name(false);
        let table_derive_attributes = self;
        let struct_name_ident = &table_derive_attributes.ident;
        let (struct_impl_generics, struct_ty_generics, struct_where_clause) =
            &table_derive_attributes.generics.split_for_impl();
        let struct_marker = table_derive_attributes.generics.phantom_marker_type();
        let meta = match table_derive_attributes.get_meta() {
            Ok(meta) => meta,
            Err(err) => return tokens.extend(err.write_errors()),
        };
        let PickableMetadata {
            field_name_normalized_deserialized,
            field_type,
        } = meta;

        // use std::any::Any;
        //
        // struct Person<'a, T: 'a, U: 'a> {
        //     name: String,
        //     age: u8,
        //     some: &'a T,
        //     another: &'a U,
        // }
        //
        // trait PersonPickable {
        //     type name;
        //     type age;
        //     type some;
        //     type another;
        // }
        //
        // impl<'a, T: 'a, U: 'a> PersonPickable for Person<'a, T, U> {
        //     type name = String;
        //     type age = u8;
        //     type some = &'a T;
        //     type another = &'a U;
        // }
        let pickable_name = format_ident!("{struct_name_ident}Pickable");
        tokens.extend(quote!(
            pub trait #pickable_name {
                #( type #field_name_normalized_deserialized ;) *
            }

            impl #struct_impl_generics pickable_name for #struct_name_ident #struct_ty_generics #struct_where_clause {
                #( type #field_name_normalized_deserialized = #field_type ;) *
            }

        ));
    }
}
