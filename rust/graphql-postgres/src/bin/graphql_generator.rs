use graphql_postgres::utils::graphql;

fn main() {
    let path = format!("{}/generated/schema.graphql", env!("CARGO_PKG_NAME"));
    graphql::generate_schema(path);
}
