#![allow(dead_code)]

use std::any::Any;
use std::fmt::format;

use convert_case::{Case, Casing};
use darling::{ast, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};
use syn::{parse_str, ItemFn};

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct Lorem {
    #[darling(rename = "sit")]
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(Debug, Clone, Copy, FromMeta)]
#[darling(default)]
pub enum CaseString {
    Camel,
    Snake,
    Normal,
}

impl Default for CaseString {
    fn default() -> Self {
        CaseString::Camel
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(mongoye, serde), forward_attrs(allow, doc, cfg))]
struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    rename: Option<String>,

    /// We declare this as an `Option` so that during tokenization we can write
    /// `field.case.unwrap_or(derive_input.case)` to facilitate field-level
    /// overrides of struct-level settings. I.O.W, if this is not provided
    /// at field level, we can fall back to the struct level settings by doing
    /// field.case.unwrap_or(struct_level.case). struct_level is from derive_input
    #[darling(default)]
    case: Option<CaseString>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(mongoye), forward_attrs(allow, doc, cfg, serde))]
pub struct SpaceTraitOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<util::Ignored, MyFieldReceiver>,
    // lorem: Lorem,
    // #[darling(default)]
    typee: String,

    #[darling(default)]
    case: CaseString,
}

impl ToTokens for SpaceTraitOpts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SpaceTraitOpts {
            ident: ref my_struct,
            ref attrs,
            ref generics,
            ref typee,
            ref data,
            ref case,
            // ref lorem,
            // ref data,
            // answer,
            // level,
            ..
        } = *self;

        let (imp, _typ, _wher) = generics.split_for_impl();

        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        let mut struct_ty_fields = vec![];
        let mut struct_values_fields = vec![];
        for (i, f) in fields.into_iter().enumerate() {
            // Fallback to the struct metadata value if not provided for the field
    
            let field_case = f.case.unwrap_or_else(|| *case);
            // This works with named or indexed fields, so we'll fall back to the index so we can
            // write the output as a key-value pair.
            let field_ident = f.ident.as_ref().map_or_else(
                || {
                    let i = syn::Index::from(i);
                    quote!(#i)
                },
                |v| quote!(#v),
            );

            // let field_ident = f.rename.as_ref().map_or_else(||field_ident, |renamed| quote!(#renamed));

            let field_identifier_string = ::std::string::ToString::to_string(&field_ident);
            let convert = |case: convert_case::Case| {
                convert_case::Converter::new()
                    .to_case(case)
                    .convert(&field_identifier_string)
            };

            let key = match field_case {
                CaseString::Normal => field_identifier_string,
                CaseString::Camel => convert(convert_case::Case::Camel),
                CaseString::Snake => convert(convert_case::Case::Snake),
            };

            let key_as_str = key.as_str();
            let key_as_ident = syn::Ident::from_string(key_as_str)
                .expect("Problem converting key string to syntax identifier");

            match f.rename {
                Some(ref name) => {
                    let key_as_str = name.as_str();
                    let key_as_ident = syn::Ident::from_string(key_as_str)
                        .expect("Problem converting key string to syntax identifier");

                    // struct type used to type the function
                    struct_ty_fields.push(quote!(#key_as_ident: &'static str));

                    // struct values themselves
                    struct_values_fields.push(quote!(#key_as_ident: #key_as_str));
                }
                None => {
                    // struct type used to type the function
                    struct_ty_fields.push(quote!(#key_as_ident: &'static str));

                    // struct values themselves
                    struct_values_fields.push(quote!(#key_as_ident: #key_as_str));
                }
            }
        }

        let struct_name = syn::Ident::new(
            format!("{my_struct}KeyNames").as_str(),
            ::proc_macro2::Span::call_site(),
        );
        // .expect("problem creating ident from struct name string");

        let struct_type = quote!(struct #struct_name {
           #( #struct_ty_fields), *
        });

        let mm = quote! {
            pub #struct_type
            impl SpaceTrait for #my_struct {
                type Naam = #struct_name;
                fn get_field_names() -> Self::Naam {
                    #struct_name {
                        #( #struct_values_fields), *
                    }
                }

                // fn level() -> &'static str {
                //     #kk
                // }

            }
        };
        tokens.extend(mm);
        // println!("rewrter \n{}", mm.to_string())
    }
}

pub fn generate_space_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input);
    let output = SpaceTraitOpts::from_derive_input(&input).expect("Wrong options");
    quote!(#output).into()

    // let {,..} = SpaceTraitOpts::from_derive_input(input).expect("Failed to parse");
    // Build the trait implementation

    // let name = output.ident;
    // let kk = name.to_string().to_case(Case::UpperSnake);
    // let gen = quote! {
    //     // mod format!("")

    //     impl #name {
    //         pub fn hello_macro() {
    //             println!("Hello, Macro! My name is {}!", stringify!(#kk));
    //         }
    //     }
    // };
    // gen.into()

    // output.into()
}
