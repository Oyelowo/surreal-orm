/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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


/// ```rust
/// #[derive(Pickable, Debug, Serialize)]
/// struct Person<'a, T: 'a, U: 'a> {
///     name: String,
///     age: u8,
///     some: &'a T,
///     another: &'a U,
/// }
///
/// pick!(NewPersonWithUnusedTypeGenericsSkipped, Person<'a,_,_> as PersonPickable, [name, age]);
/// pick!(NewPerson, Person<'a,T,U> as PersonPickable, [name, age]);
///
/// pick!{
///     #[derive(Serialize)] 
///     NewPersonWithAttributes, Person<'a,_,_> as PersonPickable, 
///     [
///         #[serde(rename = "name2")]
///         name, 
///         age,
///     ] 
/// }
///
/// fn main() {
///     let person = Person {
///         name: "Oyelowo".into(),
///         age: 25,
///         some: &43,
///         another: &"kaka",
///     };
///     println!("{:?}", person);
///
///     let new2 = NewPersonWithUnusedTypeGenericsSkipped {
///         name: "Oye".to_string(),
///         age: 154,
///     };
///
///     println!("{}", new2.name);
///     println!("{}", new2.age);
///
///     let new1 = NewPerson::<'_, u32, &str> {
///         name: "Oye".to_string(),
///         age: 154,
///     };
///     println!("{}", new1.name);
///     println!("{}", new1.age);
///
///     let new3 = NewPersonWithAttributes {
///         name: "Oye".to_string(),
///         age: 154,
///     };
///     println!("{}", new3.name);
/// }
/// ```
#[proc_macro]
pub fn pick(input: TokenStream) -> TokenStream {
    let output = match syn::parse2::<PickedMeta>(input.into()) {
        Ok(out) => out,
        Err(err) => return err.to_compile_error().into()
    };

    quote!(#output).into()
}
