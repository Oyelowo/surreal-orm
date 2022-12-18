extern crate darling;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Fields};

#[derive(FromMeta)]
struct FieldAttrs {
  foreign: Option<String>,
  relation: Option<String>,
}

#[proc_macro_derive(Model, attributes(field))]
pub fn model_derive(input: TokenStream) -> TokenStream {
  // Parse the input token stream into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Extract the name and fields of the struct
  let name = &input.ident;
  let fields = match input.data {
    syn::Data::Struct(ref s) => &s.fields,
    _ => panic!("Model can only be derived for structs"),
  };

  // Parse the field attributes
  let field_attrs: Vec<FieldAttrs> = fields.iter().map(|f| {
    let mut attrs = FieldAttrs {
      foreign: None,
      relation: None,
    };
    for attr in &f.attrs {
      if attr.path.is_ident("field") {
        attrs.merge(FieldAttrs::from_meta(&attr.parse_meta().unwrap()));
      }
    }
    attrs
  }).collect();

  // Generate the model! macro invocation
  let mut field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
  let mut model_fields = Vec::new();
  for (f, attrs) in field_names.iter().zip(field_attrs) {
    let mut field = quote!(#f);
    if let Some(foreign) = &attrs.foreign {
      field = quote!(#f<Foreign<#foreign>>);
    }
    if let Some(relation) = &attrs.relation {
      field_names.push(format!("{}_{}", f, relation));
      field = quote!(#f<#relation>);
    }
    model_fields.push(field);
  }
  let expanded = quote! {
    model!(#name {
      #(#model_fields),*
    });
  };

  // Return the expanded code as a token stream
  TokenStream::from(expanded)
}





fn main() {
  use account::schema::model as account;

  let query = format!("select {} from {account}", account.manage_projects);
  assert_eq!("select ->manage->Project from Account", query);

  let query = format!("select {} from {account}", account.manage_projects().name.as_alias("project_names"));
  assert_eq!("select ->manage->Project.name as project_names from Account", query);
}

/* 
Note that the relation is automatically appended to the field name and the foreign type is used to specify the type of the field.

If you want to use multiple foreign fields, you can simply add more #[field] attributes to your struct. For example:
*/

#[derive(Model)]
struct Account {
  id: Option<i32>,
  handle: String,
  password: String,

  #[field(foreign = "Project", relation = "manage")]
  managed_projects: Foreign<Vec<Project>>,

  #[field(foreign = "Task", relation = "assign")]
  assigned_tasks: Foreign<Vec<Task>>,
}


/* 
This would generate a Account model with two foreign fields, managed_projects and assigned_tasks, both of type Foreign<Vec<T>> where T is the specified foreign type.

You can also use the relation attribute to specify a relation between two models. For example:
*/

#[derive(Model)]
struct Project {
  id: Option<i32>,
  name: String,

  #[field(relation = "manage")]
  managers: Foreign<Vec<Account>>,
}

#[derive(Model)]
struct Account {
  id: Option<i32>,
  handle: String,
  password: String,

  #[field(relation = "manage")]
  managed_projects: Foreign<Vec<Project>>,
}


#[derive(Model)]
struct Project {
  id: Option<i32>,
  name: String,

  #[field(relation = "manage", direction = "from_foreign")]
  managers: Foreign<Vec<Account>>,
}

#[derive(Model)]
struct Account {
  id: Option<i32>,
  handle: String,
  password: String,

  #[field(relation = "manage", direction = "to_foreign")]
  managed_projects: Foreign<Vec<Project>>,
}


/* 

In this example, the Account model has a foreign field managed_projects with a relation manage and a direction to_foreign, which translates to the -> arrow syntax in the Surreal Query Language. Similarly, the Project model has a foreign field managers with a relation manage and a direction from_foreign, which translates to the <- arrow syntax in the Surreal Query Language.

Using the direction attribute allows you to specify the direction of the relation at the field level, rather than having to use the arrow syntax directly in the field name.*/


extern crate darling;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use darling::{FromMeta, FromMetaItem};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromMeta)]
struct FieldAttrs {
  relation: Option<String>,
}

#[derive(Debug, FromMeta)]
struct ModelAttrs {
  id: Option<String>,
  foreign: Vec<String>,
}

#[derive(Debug, FromMeta)]
struct Relation {
  relation: String,
  schema: String,
}

#[derive(Debug, FromMeta)]
struct ForeignAttr {
  field_type: String,
  relation: Option<Relation>,
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
    let mut foreign_attr = None;
    if let Some(ref id) = attrs.id {
      if *f == id {
        field = quote!(#f<Option<i32>>);
      }
    }
    if attrs.foreign.contains(f.to_string().as_str()) {
      let field_type = f.to_string();
      for attr in &f.attrs {
        if attr.path.is_ident("field") {
          let field_attrs: FieldAttrs = attr
            .parse_meta()
            .unwrap()
            .into_meta()
            .unwrap()
            .try_into()
            .unwrap();
          if let Some(relation) = field_attrs.relation {
            let relation: Relation = relation.parse().unwrap();
            foreign_attr = Some(ForeignAttr {
              field_type,
              relation: Some(relation),
            });
          } else {
            foreign_attr = Some(ForeignAttr {
              field_type,
              relation: None,
            });
