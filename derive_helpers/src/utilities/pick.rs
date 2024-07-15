use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens };
use syn::{parse::{Parse, ParseStream}, Result, punctuated::Punctuated, Field, Generics, Ident, Lifetime,  Token, Type};

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
// // impl<'a, T> PersonPicker for Person<'a, T> {
// impl<'a, T: 'a, U: 'a> PersonPickable for Person<'a, T, U> {
//     type name = String;
//     type age = u8;
//     type some = &'a T;
//     type another = &'a U;
// }
//
// // struct PickedPerson<'a, T> {
// //     name: <Person<'a, T> as PersonPicker>::name,
// // }
// struct PickedPerson<'a> {
//     name: <Person<'a, std::marker::PhantomData<dyn Any>, std::marker::PhantomData<dyn Any>> as PersonPickable>::name,
//     // __phantom_data: std::marker::PhantomData<&'a T>,
//     // kaka: T
// }
//

pub struct PickedMeta {
    new_struct: Ident,
    old_struct: Ident,
    // enerics_without_bounds: Vec<CustomGenericsPattern>,
    generics_without_bounds: Generics,
    field_names: Vec<Ident>,
}

enum CustomGenericsPattern {
    Lifetime,
    Type,
    SkippedPhantomData,
}

impl Parse for PickedMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        let new_struct = input.parse()?;
        input.parse::<Token![,]>()?;
        let old_struct = input.parse()?;
        let generics = input.parse::<Generics>()?;

        input.parse::<Token![,]>()?;

        let content;
        let _brace_token = syn::bracketed!(content in input);

        let fields_names = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
        

        Ok(PickedMeta {
            new_struct,
            old_struct,
            generics_without_bounds: generics,
            field_names: fields_names.into_iter().collect(),
        })
    }
}

// pick!(PickedPerson, Person<'a, _, U>, [name]);
// pick!(PickedPerson; Person<'a, _, U> ; [name]);


impl ToTokens for PickedMeta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_struct = &self.new_struct;
        let old_struct = &self.old_struct;
        let generics = &self.generics_without_bounds;
        let field_names = &self.field_names;

        // replace empty generics with phantom data type
        let new_generics = generics.params.iter().map(|param| {
            match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = &type_param.ident;
                    if ident.to_string() == "_" {
                        quote! { #ident: ::std::marker::PhantomData<dyn Any> }
                    } else {
                        quote! { #ident }
                    }
                }
                syn::GenericParam::Lifetime(lifetime) => {
                    let lifetime = &lifetime.lifetime;
                    quote! { #lifetime }
                }
                syn::GenericParam::Const(const_param) => {
                    let ident = &const_param.ident;
                    quote! { #ident }
                }
            }
        });

        let old_struct_trait_name = format_ident!("{old_struct}Pickable");

        tokens.extend(quote! {
            struct #new_struct #generics {
                #(
                    #field_names: <#old_struct #new_generics as #old_struct_trait_name>::#field_names,
                )*
            }
        });
    }
}

