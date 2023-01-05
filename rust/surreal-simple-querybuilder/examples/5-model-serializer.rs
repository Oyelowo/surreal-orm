#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use surreal_simple_querybuilder::prelude::*;

model!(User {
  // 👇 note how id is not `pub`
  id,

  // 👇 while these two fields are
  pub age,
  pub name,
});

fn main() -> Result<(), SqlSerializeError> {
  use schema::model as user;

  let query = QueryBuilder::new()
    .create(user)
    // 👇 all `pub` fields will be serialized while the others won't.
    .set_model(&user)?
    .build();

  // CREATE User SET age = $age , name = $name
  println!("query: {query}");

  Ok(())
}
