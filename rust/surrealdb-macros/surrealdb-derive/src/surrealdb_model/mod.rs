mod parser;
mod trait_generator;
pub(crate) mod relations;
pub(crate) mod serialize_skipper;
pub(crate) mod casing;

use proc_macro2::{Span, TokenStream, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
pub use trait_generator::generate_fields_getter_trait;

use syn::{self, Ident};

pub fn get_crate_name(internal: bool) -> TokenStream {
    if internal {
        quote! { crate }
    } else {
        let name = match crate_name("surrealdb-macros") {
            Ok(FoundCrate::Name(name)) => name,
            Ok(FoundCrate::Itself) | Err(_) => "surrealdb_macros".to_string(),
        };
        TokenTree::from(Ident::new(&name, Span::call_site())).into()
    }
}
