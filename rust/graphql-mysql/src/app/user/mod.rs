pub mod model;
pub(in crate::app) mod mutation_root;
pub(in crate::app) mod query_root;

pub use self::{model as user, mutation_root::UserMutationRoot, query_root::UserQueryRoot};
