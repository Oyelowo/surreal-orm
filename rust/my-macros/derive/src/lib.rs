extern crate proc_macro;

use proc_macro::TokenStream;
// mod mongo_orm;
mod mongo_orm;

#[proc_macro_derive(HelloMacro)]
pub fn hello_mracro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    mongo_orm::generate_hello_macro(input)
}

#[proc_macro_derive(MyTrait, attributes(my_trait))]
pub fn foo_bar_derive(input: TokenStream) -> TokenStream {
    mongo_orm::generate_foo_bar(input)
}

// #[proc_macro_derive(<ChangeMeToYourTrait>, attributes(<attribute1>, <attribute2>, ..))]
/// #[derive(KeyNamesGetter)]
/// #[attribute1(typee = "Hello", case="snake")]
/// pub struct ConsumingType {
///     #[attribute2(case = "camel")] // if attribute2 is implemented in FromField trait.
///     name_of_me: String,
///     #[attribute2(case = "camel")]
///     age: u8,
/// }
///  
#[proc_macro_derive(KeyNamesGetter, attributes(key_getter))]
pub fn key_names_getter_trait_derive(input: TokenStream) -> TokenStream {
    // let p = mongo_orm::mongo_field_names::MyTraitOpts::from(input);
    mongo_orm::generate_key_names_getter_trait(input)
}
