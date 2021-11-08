#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_eq!(super::utils::maths::add_one(3), 4);
    }
}




mod utils;

pub use utils::{maths, nested};

// pub use crate::utils::maths;

// pub use crate::utils::nested;
// pub use crate::utils::maths;
// pub use crate::utils::{nested, maths};
// pub use crate::utils::maths;

// pub fn eat_at_restaurant() {
//     // maths::add_one(5);
//     utils::maths::add_one(56);
//     // nested::get_test_function();
// }