use darling::FromDeriveInput;
use surreal_derive_helpers::utilities::TableDeriveAttributesPickable;
use syn::parse_macro_input;
use quote::quote;

pub fn generate_table_resources_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = match TableDeriveAttributesPickable::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
