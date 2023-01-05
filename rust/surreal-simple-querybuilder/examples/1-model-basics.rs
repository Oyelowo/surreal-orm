use surreal_simple_querybuilder::prelude::*;

// Using the `model` macro you can quickly define the schemas from of your
// database using a rust-like syntax.
model!(User {
  pub age,
  pub name
});

fn main() {
  // 👇 a "schema" module is created with a static "model" variable you can use
  // 👇 anywhere in your code.
  use schema::model as user;

  let query = QueryBuilder::new()
    .select("*")
    // you can pass the entire model directly to reference its name
    .from(user)
    // or you can access its fields and use the various traits imported from
    // the querybuilding crate to form complex queries
    .filter(user.age.greater_than("10"))
    .and(user.name.equals("'John'"))
    .build();

  // SELECT * FROM User WHERE age > 10 AND name = 'John'
  println!("query: {query}");
}
