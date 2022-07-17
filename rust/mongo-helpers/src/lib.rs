pub mod operators;
mod utils;

// Used by the operators macros
pub(crate) use mongodb as mongo;
pub use operators as ops;
pub use utils::{bson::as_bson, sync_mongo_models};

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
