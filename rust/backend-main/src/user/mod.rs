mod query_root;
mod query_user;

mod mutation_root;
mod type_gql;

pub mod model;

pub mod query {
    pub use super::{query_root::UserQueryRoot, query_user::User};
}

pub mod mutation {
    pub use super::{mutation_root::UserMutationRoot};
}


pub mod inputs {
    pub use super::type_gql::UserInput;
}