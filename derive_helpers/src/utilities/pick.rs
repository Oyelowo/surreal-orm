/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Generics, Ident, Result, Token,
};

use crate::models::create_tokenstream_wrapper;

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

create_tokenstream_wrapper!( => GenericsWithOmiitedAsPhantomData);
create_tokenstream_wrapper!( => FilteredEmptyGenerics);

impl PickedMeta {
    fn map_empty_generics_to_phantom_placeholder(&self) -> GenericsWithOmiitedAsPhantomData {
        let generics = &self.generics_without_bounds;
        // replace empty generics with phantom data type
        let new_generics_params = generics.params.iter().map(|param| match param {
            syn::GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                if ident.to_string() == "_" {
                    quote! { ::std::marker::PhantomData<dyn Any>, }
                } else {
                    quote! { #ident, }
                }
            }
            syn::GenericParam::Lifetime(lifetime) => {
                let lifetime = &lifetime.lifetime;
                quote! { #lifetime, }
            }
            syn::GenericParam::Const(const_param) => {
                let ident = &const_param.ident;
                quote! { #ident, }
            }
        });
        let no_generics = generics.into_token_stream().is_empty();
        let new_generics_type_args = if no_generics {
            quote! {}
        } else {
            quote!( <#(#new_generics_params)* >)
        };
        new_generics_type_args.into()
    }

    fn filter_empty_generic_params(&self) -> FilteredEmptyGenerics {
        // let generics = &self.generics_without_bounds;
        // let filtered_generics = generics.params.iter().map(|param| match param {
        //     syn::GenericParam::Type(type_param) => {
        //         let ident = &type_param.ident;
        //         if ident.to_string() == "_" {
        //             quote! {}
        //         } else {
        //             quote! { #ident, }
        //         }
        //     }
        //     syn::GenericParam::Lifetime(lifetime) => {
        //         let lifetime = &lifetime.lifetime;
        //         quote! { #lifetime, }
        //     }
        //     syn::GenericParam::Const(const_param) => {
        //         let ident = &const_param.ident;
        //         quote! { #ident, }
        //     }
        // });
        //
        // let no_generics = generics.into_token_stream().is_empty();
        // let filtered_generics = if no_generics {
        //     quote! {}
        // } else {
        //     quote!( <#(#filtered_generics)* >)
        // };
        // filtered_generics.into()
        //
        //
        //

        let generics = &self.generics_without_bounds;

        let filtered_generics = generics.params.iter().filter(|param| match param {
            syn::GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                ident.to_string() != "_"
            }
            _ => true,
        });

        let mut generics = Generics::default();
        for param in filtered_generics {
            generics.params.push(param.clone());
        }

        quote! {#generics}.into()
    }
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

        let new_generics_type_args = self.map_empty_generics_to_phantom_placeholder().0;
        let filtered_generics = self.filter_empty_generic_params();

        let old_struct_trait_name = format_ident!("{old_struct}Pickable");

        tokens.extend(quote! {
            struct #new_struct #filtered_generics {
                #(
                    #field_names: <#old_struct #new_generics_type_args as #old_struct_trait_name>::#field_names,
                )*
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;
    #[test]
    fn test_parse_single_without_generics() {
        let input = quote! {
            PickedPerson, Person, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(picked_meta.field_names.len(), 1);
        assert_eq!(picked_meta.field_names, vec![format_ident!("name"),]);
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 0);

        let tokenstream = picked_meta.to_token_stream().to_string();
        assert_eq!(
            tokenstream,
            "struct PickedPerson { name : < Person as PersonPickable > :: name , }"
        );
    }

    #[test]
    fn test_parse_single() {
        let input = quote! {
            PickedPerson, Person<'a, T>, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(picked_meta.field_names.len(), 1);
        assert_eq!(picked_meta.field_names, vec![format_ident!("name"),]);
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "T",]
        );
        assert_eq!(
            picked_meta.to_token_stream().to_string(),
            "struct PickedPerson < 'a , T > { name : < Person < 'a , T , > as PersonPickable > :: name , }"
        );
    }

    #[test]
    fn test_parse() {
        let input = quote! {
            PickedPerson, Person<'a, _, U, _>, [name, age, some, another]
        };

        // let picked_meta = PickedMeta::parse(input.into()).unwrap();
        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(picked_meta.field_names.len(), 4);
        assert_eq!(
            picked_meta.field_names,
            vec![
                format_ident!("name"),
                format_ident!("age"),
                format_ident!("some"),
                format_ident!("another"),
            ]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 4);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        insta::assert_snapshot!(tokenstream.replace(" ", ""));
        insta::assert_snapshot!(tokenstream);
        // assert_eq!(tokenstream, "struct PickedPerson < 'a , _ , U , _ > { name : < Person < 'a , _ , U , _ > as PersonPickable > :: name , age : < Person < 'a , _ , U , _ > as PersonPickable > :: age , some : < Person < 'a , _ , U , _ > as PersonPickable > :: some , another : < Person < 'a , _ , U , _ > as PersonPickable > :: another , }");
    }

    // #[test]
    // fn test_to_tokens() {
    //     let input = quote! {
    //         PickedPerson, Person<'a, _, U>, [name]
    //     };
    //
    //     let picked_meta = PickedMeta::parse(input).unwrap();
    //
    //     let tokens = quote! { #picked_meta };
    //
    //     let expected = quote! {
    //         struct PickedPerson<'a, U> {
    //             name: <Person<'a, std::marker::PhantomData<dyn Any>, U> as PersonPickable>::name,
    //         }
    //     };
    //
    //     assert_eq!(tokens.to_string(), expected.to_string());
    // }
    //
}
