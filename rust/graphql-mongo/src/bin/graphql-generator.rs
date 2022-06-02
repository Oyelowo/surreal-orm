use graphql_mongo::utils;

fn main() {
    let path = format!("{}/generated/schema.graphql", env!("CARGO_PKG_NAME"));
    utils::graphql::generate_schema(path);
}
