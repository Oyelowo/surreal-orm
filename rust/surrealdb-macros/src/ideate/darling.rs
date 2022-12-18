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



/* 
#[derive(Model, Debug, PartialEq)]
#[model(module = "custom_schema", model = "CustomModel", id = "id", foreign = "categories")]
struct File {
  id: i32,
  name: String,
  categories: Vec<Foreign<Category>>,
}

// Generated code:
model!(File {
  id<i32>,
  name,
  categories<Vec<Foreign<Category>>>
});

fn main() {
  use custom_schema::model as file;

  let query = format!("select {} from {file}", file.name);
  assert_eq!("select name from CustomModel", query);
}


*/


#[derive(Model, Debug, PartialEq)]
#[model(id = "id")]
struct Account {
  id: i32,
  #[relate("->manage->Project as managed_projects")]
  managed_projects: Vec<Foreign<Project>>,
  #[relate("<-manage<-Project as managed_by")]
  managed_by: Vec<Foreign<Project>>,
}

#[derive(Model, Debug, PartialEq)]
#[model(id = "id")]
struct Project {
  id: i32,
  name: String,
  #[relate("<-manage<-Account as authors")]
  authors: Vec<Foreign<Account>>,
}


fn main() {
  use account::schema::model as account;
  use project::schema::model as project;

  let query = format!("select {} from {account}", account.managed_projects);
  assert_eq!("select ->manage->Project from Account", query);

  let query = format!(
    "select {} from {account}",
    account.managed_by().name.as_alias("project_names")
  );
  assert_eq!("select <-manage<-Project.name as project_names from Account", query);
}
