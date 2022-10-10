use graphql_surrealdb::utils::graphql;

fn main() {
    let path = format!("{}/generated/schema.graphql", env!("CARGO_PKG_NAME"));
    std::fs::remove_file(&path).expect("Problem removing file");
    graphql::generate_schema(path);
}
