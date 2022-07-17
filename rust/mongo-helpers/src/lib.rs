pub mod operators;
pub mod utils;

// Used by the operators macros
pub(crate) use mongodb as mongo;
pub use operators as ops;
pub use utils::{sync_mongo_models, bson::as_bson};
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
