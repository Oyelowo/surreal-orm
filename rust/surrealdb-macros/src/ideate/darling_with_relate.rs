#[macro_use]
extern crate darling;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromMeta)]
struct ModelAttrs {
  id: Option<String>,
  foreign: Vec<String>,
  relate: Vec<String>,
}

#[proc_macro_derive(Model, attributes(model))]
pub fn model_derive(input: TokenStream) -> TokenStream {
  // Parse the input token stream into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Extract the name and fields of the struct
  let name = &input.ident;
  let fields = match input.data {
    syn::Data::Struct(ref s) => &s.fields,
    _ => panic!("Model can only be derived for structs"),
  };

  // Parse the model attributes
  let attrs = input
    .attrs
    .iter()
    .filter(|a| a.path.is_ident("model"))
    .map(|a| ModelAttrs::from_meta(&a.parse_meta().unwrap()))
    .next()
    .unwrap();

  // Create the model! macro invocation
  let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
  let model_fields = field_names.iter().map(|f| {
    let mut field = quote!(#f);
    if let Some(ref id) = attrs.id {
      if *f == id {
        field = quote!(#f<Option<i32>>);
      }
    }
    if attrs.foreign.contains(f.to_string().as_str()) {
      field = quote!(#f<Foreign<T>>);
    }
    if let Some(relate) = attrs.relate.iter().find(|r| r.starts_with(f.to_string().as_str())) {
      field = quote!(#[relate(#relate)] #field);
    }
    field
  });
  let expanded = quote! {
    model!(#name {
      #(#model_fields),*
    });
  };

  // Return the expanded code as a token stream
  TokenStream::from(expanded)
}

// Define the models using the derive macro and `#[relate]` attributes
#[model(id = "id", foreign = "friends", relate = "->friends->Account as friends")]
struct Account {
  id: Option<i32>,
  handle: String,
  friends: Vec<Foreign<Account>>,
}

#[model(id = "id", foreign = "authors", relate = "<-manage<-Account as managed_by")]
struct Project {
  id: Option<i32>,
