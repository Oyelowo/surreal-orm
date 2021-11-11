pub mod model;
pub mod type_gql;

mod query_droid;
mod query_human;
mod query_root;

// Merge all the queries here, so, it's easier for other modules and the binary to consume it.
pub mod query {
    pub use super::{query_droid::Droid, query_human::Human, query_root::StarWarQueryRoot};
}
