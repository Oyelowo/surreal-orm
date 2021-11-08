#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod utils;

// #[macro_use]
mod my_macros;

// pub use crate::utils::nested::*;

// pub use crate::my_macros::pub_struct;
pub use my_macros::pub_struct;
pub use utils::{local_function};
pub use utils::nested::{get_test_function};
// pub use utils::nested;
// pub use utils::local_function;
// pub use utils::*;
// pub use crate::utils::*;
// pub use crate::my_macros::*;

fn get_shared_function() ->  &'static str {
    utils::nested::get_test_function();
    utils::local_indirect_access();
    utils::local_function();
    get_shared_function();

    return "Oyelowo"
}