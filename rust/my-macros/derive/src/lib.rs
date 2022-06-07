extern crate proc_macro;

use proc_macro::TokenStream;

mod fields_getter;
mod examples;

#[proc_macro_derive(HelloMacro)]
pub fn hello_mracro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    examples::generate_hello_macro(input)
}

#[proc_macro_derive(MyTrait, attributes(my_trait))]
pub fn foo_bar_derive(input: TokenStream) -> TokenStream {
    examples::generate_foo_bar(input)
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
#[proc_macro_derive(FieldsGetter, attributes(field_getter))]
pub fn key_names_getter_trait_derive(input: TokenStream) -> TokenStream {
    // let p = mongo_orm::mongo_field_names::MyTraitOpts::from(input);
    fields_getter::generate_fields_getter_trait(input)
}
