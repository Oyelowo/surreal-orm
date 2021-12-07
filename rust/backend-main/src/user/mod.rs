mod query_root;
mod query_user;

mod mutation_root;
mod type_gql;

mod model;

// pub use self::{query_root::UserQueryRoot, query_user::User};
// pub mod query {
//     pub use super::{query_root::UserQueryRoot, query_user::User};
// }

// pub use self::{mutation_root::UserMutationRoot};
// pub mod mutation {
//     pub use super::{mutation_root::UserMutationRoot};
// }

// pub mod usr {
//     pub use super::{type_gql::UserInput, mutation_root::UserMutationRoot, query_root::UserQueryRoot, query_user::User};
// }
// pub use self::type_gql::UserInput;

pub use self::{type_gql::*, mutation_root::*, query_root::*, query_user::*, model::*};

// pub mod inputs {
//     pub use super::type_gql::UserInput;
// }