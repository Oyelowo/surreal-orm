pub(crate) mod attributes;
pub(crate) mod casing;
pub(crate) mod edge;
mod edge_parser;
pub(crate) mod node;
mod node_parser;
pub(crate) mod relations;

use proc_macro2::{Span, TokenStream, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

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
