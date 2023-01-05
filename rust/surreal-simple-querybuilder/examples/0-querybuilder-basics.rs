use surreal_simple_querybuilder::querybuilder::QueryBuilder;

fn main() {
  let query = QueryBuilder::new()
    .select("*")
    .from("User")
    .filter("age > 10")
    .and("name = 'John'")
    .build();

  // SELECT * FROM User WHERE age > 10 AND name = 'John'
  println!("query: {query}");
}
