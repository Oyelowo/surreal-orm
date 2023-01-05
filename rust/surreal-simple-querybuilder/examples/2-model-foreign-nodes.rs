#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use surreal_simple_querybuilder::prelude::*;

model!(User {
  pub age,
  pub name,

  // 👇 you can define foreign nodes that may represented either by their IDs
  // 👇 or the complete type with all of its fields.
  best_friend<User>
});

fn main() {
  use schema::model as user;

  let query = QueryBuilder::new()
    // 👇 pass the field to reference the User's `best_friend` field
    .select(user.best_friend)
    .from(user)
    // 👇 call the function with the same name to access the foreign type's fields
    .filter(user.best_friend().name.equals("'John'"))
    .build();

  // SELECT best_friend FROM User WHERE best_friend.name = 'John'
  println!("query: {query}");
}
