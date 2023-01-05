#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use surreal_simple_querybuilder::prelude::*;

model!(User {
  pub age,
  pub name,
});

fn main() {
  use schema::model as user;

  let filter_name: Option<&str> = Some("John");
  let filter_age: Option<&str> = None;

  let query = QueryBuilder::new()
    .select("*")
    .from(user)
    .filter("true")
    .if_then(filter_name.is_some(), |q| {
      q.and(user.name.equals(&filter_name.unwrap().quoted()))
    })
    .if_then(filter_age.is_some(), |q| {
      q.and(user.age.equals(filter_age.unwrap()))
    })
    .build();

  // SELECT * FROM User WHERE true AND name = "John"
  println!("query: {query}");
}
