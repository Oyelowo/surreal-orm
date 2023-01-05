#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use surreal_simple_querybuilder::prelude::*;

model!(User {
  pub age,
  pub name,

  best_friend<User>,

  // 👇 edges can be defined using a custom syntax similar to the SQL one
  ->likes->User as friends
});

fn main() {
  use schema::model as user;

  let query = QueryBuilder::new()
    // 👇 edges can be referenced using an alias
    .select(user.friends.as_alias("friends"))
    .from(user)
    // 👇 but also in queries
    .filter(user.friends.filter(&user.age.greater_than("10")))
    .build();

  // SELECT ->likes->User AS friends FROM User WHERE ->likes->(User WHERE age > 10)
  println!("query: {query}");
}
