extern crate proc_macro;

use proc_macro::TokenStream;
mod mongo_orm;


#[proc_macro_derive(HelloMacro)]
pub fn hello_mracro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    mongo_orm::gene(input)
}

