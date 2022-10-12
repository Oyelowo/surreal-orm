extern crate proc_macro;

use proc_macro::TokenStream;

mod examples;
mod fields_getter;

// #[proc_macro_derive(<ChangeMeToYourTrait>, attributes(<attribute1>, <attribute2>, ..))]
/// #[derive(KeyNamesGetter)]
/// #[attribute1(rename_all="snake")]
/// pub struct ConsumingType {
///     #[attribute2(rename = "username")] // if attribute2 is implemented in FromField trait.
///     name_of_me: String,
///     #[attribute2(rename = "ageCount")]
///     age: u8,
///     #[attribute2(rename(serialized="homeAddress"))]
///     address: u8,
/// }
///  
#[proc_macro_derive(FieldsGetter, attributes(field_getter))]
pub fn fields_getter_trait_derive(input: TokenStream) -> TokenStream {
    fields_getter::generate_fields_getter_trait(input)
}

// Examples
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
