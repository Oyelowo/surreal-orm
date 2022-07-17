pub mod operations;
mod utils;

pub use mongodm::operator;
pub use utils::{bson::as_bson, sync_mongo_models};

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
