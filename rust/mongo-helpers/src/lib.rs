mod model;
pub mod operators;
pub mod converters;

pub use model::Model;
pub use mongodb as mongo;
pub use operators as ops;

#[cfg(test)]
mod tests {
    // use crate::operators::*;

    #[test]
    fn it_works() {
        // assert_eq!(result, 4);
        let result = 2 + 2;
    }
}
