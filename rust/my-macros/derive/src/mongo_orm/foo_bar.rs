// The use of fields in debug print commands does not count as "used",
// which causes the fields to trigger an unwanted dead code warning.
#![allow(dead_code)]

use std::any::Any;

use darling::{ast, util, FromDeriveInput, FromField, FromMeta, ToTokens};
use proc_macro2::TokenStream;
use quote::{quote};
use syn::{self, parse_macro_input, DeriveInput};
use syn::{parse_str, ItemFn};

/*
pub trait MyTrait {
    fn answer() -> i32;
}
*/

#[derive(Debug, Clone, Copy, Default, FromMeta)]
#[darling(default)]
pub struct Levell {
    answer: Option<i32>,
    // name: Option<String>,
}

#[derive(Debug, Clone, Copy, FromMeta)]
#[darling(default)]
enum Level {
    Low,
    Medium,
    High,
}

impl Default for Level {
    fn default() -> Self {
        Level::High
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(my_trait))]
struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,
    /// This magic field name pulls the type from the input.
    ty: syn::Type,

    
    /// We declare this as an `Option` so that during tokenization we can write
    /// `field.volume.unwrap_or(derive_input.volume)` to facilitate field-level
    /// overrides of struct-level settings.
    #[darling(default)]
    level: Option<Level>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(my_trait), supports(struct_named))] // others: struct_named, struct_any, struct_unit, struct_newtype, struct_tuple
struct MyInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<util::Ignored, MyFieldReceiver>,


    /// The Input Receiver demands a volume, so use `Volume::Normal` if the
    /// caller doesn't provide one.
    #[darling(default)]
    level: Level,
    // attrs: Vec<syn::Attribute>,
    #[darling(default)]
    answer: u32,
}


impl ToTokens for MyInputReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let MyInputReceiver {
            ident: ref my_struct,
            ref generics,
            ref data,
            answer,
            level,
        } = *self;

        let (_imp, _typ, _wher) = generics.split_for_impl();

        let _fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        // let ref mut tokens  = quote!{
        //     impl MyTrait for #my_struct {
        //         fn answer() -> u32 {
        //             42
        //         }

        //     }
        // };

        let kk = format!("My level is: {:?}", level);

        tokens.extend(quote!{
            impl MyTrait for #my_struct {
                fn answer() -> u32 {
                    #answer
                }

                fn level() -> &'static str {
                    #kk
                }

            }
        });
    }
}

pub fn generate_foo_bar(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input = parse_macro_input!(input);
    let output = MyInputReceiver::from_derive_input(&input).expect("Wrong options");
    quote!(#output).into()

    // output.into()
}
