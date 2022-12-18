use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

pub fn generate_hello_macro(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).expect("Failed to parse");
    // Build the trait implementation

    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
