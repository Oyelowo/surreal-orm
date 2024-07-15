/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
#![allow(
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::type_complexity,
    clippy::needless_doctest_main
)]
#![warn(
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::mut_mut,
    clippy::non_ascii_literal,
    clippy::similar_names,
    clippy::unicode_not_nfc,
    clippy::enum_glob_use,
    clippy::if_not_else,
    clippy::items_after_statements,
    clippy::used_underscore_binding,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![cfg_attr(test, allow(clippy::unwrap_used))]
extern crate proc_macro;

mod migrations;
mod models;
mod utilities;


use darling::FromDeriveInput;
use proc_macro::TokenStream;
use surreal_derive_helpers::utilities::{PickedMeta, TableDeriveAttributesPickable};
use quote::quote;
use syn::parse_macro_input;


#[proc_macro_derive(Node, attributes(surreal_orm))]
pub fn surreal_node_trait_derive(input: TokenStream) -> TokenStream {
    models::node::generate_fields_getter_trait(input)
}

#[proc_macro_derive(Edge, attributes(surreal_orm))]
pub fn surreal_edge_trait_derive(input: TokenStream) -> TokenStream {
    models::edge::generate_fields_getter_trait(input)
}

#[proc_macro_derive(Object, attributes(surreal_orm))]
pub fn surreal_object_trait_derive(input: TokenStream) -> TokenStream {
    models::object::generate_fields_getter_trait(input)
}

#[proc_macro_derive(TableResources, attributes(surreal_orm))]
pub fn surreal_table_resources_derive(input: TokenStream) -> TokenStream {
    migrations::table::generate_table_resources_trait(input)
}

#[proc_macro_derive(Pickable)]
pub fn surreal_pickable_resources_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let output = match TableDeriveAttributesPickable::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
    
}


/// pick!(PickedPerson, Person, [name]);
/// ```rust
/// use std::any::Any;
///
/// struct Person<'a, T: 'a, U: 'a> {
///     name: String,
///     age: u8,
///     some: &'a T,
///     another: &'a U,
/// }
///
/// trait PersonPickable {
///     type name;
///     type age;
///     type some;
///     type another;
/// }
///
/// // impl<'a, T> PersonPicker for Person<'a, T> {
/// impl<'a, T: 'a, U: 'a> PersonPickable for Person<'a, T, U> {
///     type name = String;
///     type age = u8;
///     type some = &'a T;
///     type another = &'a U;
/// }
///
/// // struct PickedPerson<'a, T> {
/// //     name: <Person<'a, T> as PersonPicker>::name,
/// // }
/// struct PickedPerson<'a> {
///     name: <Person<'a, std::marker::PhantomData<dyn Any>, std::marker::PhantomData<dyn Any>> as PersonPickable>::name,
///     // __phantom_data: std::marker::PhantomData<&'a T>,
///     // kaka: T
/// }
///
/// struct PickedPersonAll<'a, U> {
///     // name: <Person<'a, std::marker::PhantomData<dyn Any>> as PersonPickable>::name,
///     name: <Person<'a, std::marker::PhantomData<dyn Any>, U> as PersonPickable>::name,
///     // kaka: &'a std::marker::PhantomData<dyn Any>, U
///     // some: <Person<'a, std::marker::PhantomData<dyn Any>, U> as PersonPickable>::some,
///     another: <Person<'a, std::marker::PhantomData<dyn Any>, U> as PersonPickable>::another,
/// }
///
/// fn main() {
///     // let person = Person<'a, T> {
///     let person = Person {
///         name: "Oyelowo".into(),
///         age: 25,
///         some: &43,
///         another: &"kaka",
///     };
///
///     // let something = PickedPerson::<'_, u32>{
///     let something = PickedPerson {
///         name: "Oyelowo".into(),
///         // name: person.name,
///         // kaka: 43,
///         // __phantom_data: std::marker::PhantomData,
///     };
/// // std::marker::PhantomData<dyn Any>
///     let p2 = PickedPersonAll {
///         name: "Oyelowo".into(),
///         // some: &43,
///         another: &"kaka",
///     };
/// }
///
/// // pick!(PickedPerson, Person, [name]);
/// //
/// // #[pick(Person, [name])]
/// // #[pick(AnotherPerson, [age])]
/// // struct PickedPerson {
/// //     more_fields: u8,
/// // }
/// //
/// //
/// // #[derive(Serialize, Deserialize)]
/// // struct NewPerson {
/// //     #[serde(flatten)]
/// //     picked_person: PickedPerson,
/// //     more_fields: u8,
/// // }
#[proc_macro]
pub fn pick(input: TokenStream) -> TokenStream {
    let output = match syn::parse2::<PickedMeta>(input.into()) {
        Ok(out) => out,
        Err(err) => return err.to_compile_error().into()
    };

    quote!(#output).into()
}
