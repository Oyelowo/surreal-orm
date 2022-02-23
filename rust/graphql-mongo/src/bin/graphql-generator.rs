use graphql_mongo::configs::GraphQlApp;

fn main() {
    let path = format!("{}/generated/schema.graphql", env!("CARGO_PKG_NAME"));
    GraphQlApp::generate_schema(path);
}
