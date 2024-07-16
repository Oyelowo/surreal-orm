/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Generics, Ident, Path, Result, Token,
};

use crate::models::create_tokenstream_wrapper;

pub struct PickedMeta {
    attributes: Vec<syn::Attribute>,
    new_struct: Ident,
    old_struct: Ident,
    old_struct_trait: Path,
    generics_without_bounds: Generics,
    field_names: Vec<FieldPicked>,
}

create_tokenstream_wrapper!( => GenericsWithOmiitedAsPhantomData);
create_tokenstream_wrapper!( => FilteredEmptyGenerics);

impl PickedMeta {
    fn map_empty_generics_to_phantom_placeholder(&self) -> GenericsWithOmiitedAsPhantomData {
        let generics = &self.generics_without_bounds;
        let new_generics_params = generics.params.iter().enumerate().map(|(i, param)| {
            let is_last = i == generics.params.len() - 1;
            let separator = if is_last {
                quote! {}
            } else {
                quote! {,}
            };
            match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = &type_param.ident;
                    if ident.to_string() == "_" {
                        quote! { ::std::marker::PhantomData<dyn ::std::any::Any> #separator }
                    } else {
                        quote! { #ident #separator }
                    }
                }
                syn::GenericParam::Lifetime(lifetime) => {
                    let lifetime = &lifetime.lifetime;
                    quote! { #lifetime #separator }
                }
                syn::GenericParam::Const(const_param) => {
                    let ident = &const_param.ident;
                    quote! { #ident #separator }
                }
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

struct FieldPicked {
    name: Ident,
    attrs: Vec<syn::Attribute>,
}

impl Parse for FieldPicked {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name = input.parse()?;
        Ok(FieldPicked { name, attrs })
    }
}

impl Parse for PickedMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let new_struct = input.parse()?;
        input.parse::<Token![,]>()?;

        let old_struct = input.parse()?;
        let generics = input.parse::<Generics>()?;

        let _as = input.parse::<Token![as]>()?;

        let pickee_struct_trait = input.parse::<Path>()?;

        input.parse::<Token![,]>()?;

        let content;
        let _brace_token = syn::bracketed!(content in input);

        let fields_names = Punctuated::<FieldPicked, Token![,]>::parse_terminated(&content)?;

        Ok(PickedMeta {
            attributes,
            new_struct,
            old_struct,
            old_struct_trait: pickee_struct_trait,
            generics_without_bounds: generics,
            field_names: fields_names.into_iter().collect(),
        })
    }
}

impl ToTokens for PickedMeta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            new_struct,
            old_struct,
            old_struct_trait,
            field_names,
            attributes,
            ..
        } = &self;

        let new_generics_type_args = self.map_empty_generics_to_phantom_placeholder().0;
        let filtered_generics = self.filter_empty_generic_params();

        let fields = field_names.iter().map(|field| {
            let FieldPicked { name, attrs } = field;
            quote! {
                #(#attrs)*
                #name: <#old_struct #new_generics_type_args as #old_struct_trait> ::#name,
            }
        });

        tokens.extend(quote! {
            #(#attributes)*
            // #[derive(::serde::Deserialize)]
            pub struct #new_struct #filtered_generics {
                #(
                    #fields
                )*
                // #(
                //     #field_names: <#old_struct #new_generics_type_args as #old_struct_trait> ::#field_names,
                // )*
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;

    #[test]
    fn test_parse_single_without_generics() {
        let input = quote! {
            PickedPerson, Person as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 0);

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson {
                name: <Person as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_without_generics_and_trait_paths() {
        let input = quote! {
            PickedPerson, Person as crate :: person::Pickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 0);

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson {
                name: <Person as crate::person::Pickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string(),);
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime() {
        let input = quote! {
            PickedPerson, Person<'a> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 1);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a> {
                name: <Person<'a> as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime_with_phantom_data() {
        let input = quote! {
            PickedPerson, Person<'a, _> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime_with_phantom_data_and_another_lifetime() {
        let input = quote! {
            PickedPerson, Person<'a, _, 'b> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "'b",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, 'b> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, 'b > as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime_with_phantom_data_and_another_lifetime_and_const() {
        let input = quote! {
            PickedPerson, Person<'a, _, 'b, _> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 4);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "'b", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, 'b> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, 'b, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_multiple_lifetimes_only() {
        let input = quote! {
            PickedPerson, Person<'a, 'b> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec!["name".to_string(), "age".to_string()]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "'b",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, 'b> {
                name: <Person<'a, 'b> as PersonPickable>::name,
                age: <Person<'a, 'b> as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_multiple_lifetimes_and_skipped_at_beginning() {
        let input = quote! {
            PickedPerson, Person<_, 'a, 'b> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["_", "'a", "'b",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, 'b> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, 'a, 'b> as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, 'a, 'b> as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_multiple_lifetimes_and_skipped_at_end() {
        let input = quote! {
            PickedPerson, Person<'a, 'b, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "'b", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime_and_type_param() {
        let input = quote! {
            PickedPerson, Person<'a, T> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
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
        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, T> {
                name: <Person<'a, T> as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse() {
        let input = quote! {
            PickedPerson, Person<'a, _, U, _> as PersonPickable, [name, age, some, another]
        };

        // let picked_meta = PickedMeta::parse(input.into()).unwrap();
        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .len(),
            4
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![
                String::from("name"),
                String::from("age"),
                String::from("some"),
                String::from("another"),
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
        let expected = quote! {
            pub struct PickedPerson<'a, U> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
                some: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::some,
                another: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::another,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_single_type_params_generics() {
        let input = quote! {
            PickedPerson, Person<T> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 1);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["T",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_single_type_params_generics_with_phantom_data() {
        let input = quote! {
            PickedPerson, Person<T, _> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["T", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T> {
                name: <Person<T, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics() {
        let input = quote! {
            PickedPerson, Person<T, U> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["T", "U",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<T, U> as PersonPickable>::name,
                age: <Person<T, U> as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_end() {
        let input = quote! {
            PickedPerson, Person<T, U, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["T", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<T, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<T, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning() {
        let input = quote! {
            PickedPerson, Person<_, T, U> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["_", "T", "U",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, U > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, U > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning_and_end() {
        let input = quote! {
            PickedPerson, Person<_, T, U, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 4);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["_", "T", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning_and_end_and_middle() {
        let input = quote! {
            PickedPerson, Person<_, T, _, U, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 5);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["_", "T", "_", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };

        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning_and_end_and_middle_and_lifetime(
    ) {
        let input = quote! {
            PickedPerson, Person<'a, _, T, _, U, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 6);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "T", "_", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, T, U> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning_and_end_and_middle_and_lifetime_and_const(
    ) {
        let input = quote! {
            PickedPerson, Person<'a, _, T, _, U, _, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 7);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "T", "_", "U", "_", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, T, U> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_skips_at_beginning_and_end() {
        let input = quote! {
            PickedPerson, Person<_, _, _, U, _, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_skips_at_beginning_and_end_and_middle() {
        let input = quote! {
            PickedPerson, Person<_, _, T, _, U, _, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_skips_at_beginning_all() {
        let input = quote! {
            PickedPerson, Person<_, _, _, _, _, _, T> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input.into()).expect("failed to parse");
        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T> {
                name: <Person
        <::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        T >
        as PersonPickable>::name,

                age: <Person
        <::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        T >
        as PersonPickable>::age,
            }
        };

        assert_eq!(tokenstream, expected.to_string());

        insta::assert_snapshot!(tokenstream);
    }
}
