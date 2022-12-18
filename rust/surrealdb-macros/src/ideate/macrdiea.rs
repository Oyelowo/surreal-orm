use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

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

  // Check for attributes on the struct and fields
  let mut id_field = None;
  let mut foreign_fields = Vec::new();
  for attr in input.attrs {
    if attr.path.is_ident("model") {
      let meta = attr.parse_meta().unwrap();
      if let syn::Meta::List(ref list) = meta {
        for p in list.nested.iter() {
          if let syn::NestedMeta::Meta(ref meta) = *p {
            if meta.path().is_ident("id") {
              id_field = Some(meta.literal().unwrap().to_string().replace("\"", ""));
            } else if meta.path().is_ident("foreign") {
              if let syn::Meta::NameValue(ref nv) = *meta {
                let value = nv.lit.to_string().replace("\"", "");
                foreign_fields.push(value);
              }
            }
          }
        }
      }
    }
  }

  // Create the model! macro invocation
  let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
  let model_fields = field_names.iter().map(|f| {
    let mut field = quote!(#f);
    if let Some(ref id) = id_field {
      if *f == id {
        field = quote!(#f<Option<i32>>);
      }
    }
    if foreign_fields.contains(f.to_string().as_str()) {
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


model for the Post struct with the id field marked as the primary key, and the author field marked as a foreign key.

To use the derive macro, you will need to add it to your crate and import the Model derive. You will also need to add the syn and quote crates as dependencies.

Here's an example of how you might use the derive macro in your code:


#[macro_use]
extern crate surreal_simple_querybuilder;

use surreal_simple_querybuilder::prelude::*;

#[derive(Model, Default, Debug)]
#[model(id = "id")]
struct User {
  id: Option<i32>,
  name: String,
}

#[derive(Model, Default, Debug)]
#[model(id = "id", foreign = "author")]
struct Post {
  id: Option<i32>,
  title: String,
  body: String,
  author: Foreign<User>,
}

fn main() {
  // Use the schema module created by the model macro
  use schema::model as user;
  use schema::model as post;

  // Create a new user
  let mut new_user = User {
    id: None,
    name: "John".to_string(),
  };

  // Serialize the user into an INSERT statement
  let insert_query = new_user.to_insert_query();
  // => "INSERT INTO User (name) VALUES ($1)"

  // Insert the new user into the database and get the ID
  let inserted_user = query_one(insert_query, &[&new_user.name]);
  new_user.id = inserted_user.id;

  // Create a new post with the


inserted user as the author
let mut new_post = Post {
id: None,
title: "My First Post".to_string(),
body: "This is the body of my first post.".to_string(),
author: Foreign::new(new_user),
};

// Serialize the post into an INSERT statement
let insert_query = new_post.to_insert_query();
// => "INSERT INTO Post (title, body, author) VALUES ($1, $2, $3)"

// Insert the new post into the database and get the ID
let inserted_post = query_one(insert_query, &[&new_post.title, &new_post.body, &new_post.author.key()]);
new_post.id = inserted_post.id;

// Now suppose we want to serialize the post with the full user model included
new_post.author.allow_value_serialize();
let update_query = new_post.to_update_query();
// => "UPDATE Post SET title = $1, body = $2, author = $3 WHERE id = $4"

// Update the post in the database
query(update_query, &[&new_post.title, &new_post.body, &new_post.author.value(), &new_post.id]);
}

Copy code

This example shows how to use the `Model` derive to automatically generate a model for the `User` and `Post` structs, and how to use the `to_insert_query` and `to_update_query` methods to serialize the models into INSERT and UPDATE statements. It also shows how to use the `allow_value_serialize` method to serialize the `Foreign` field as the related model, rather than just the ID.


