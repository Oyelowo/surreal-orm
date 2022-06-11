pub mod authentication;
pub mod configurations;
pub mod error_handling;
mod macros;
pub mod middleware;
pub mod my_time;
pub mod utils;

pub use macros::{calculator, helpers, sync_mongo_models};
pub use util_module_alternative::greeter_alt::alt_good_morning;
pub use utils::{good_morning, local_function, maths};

#[macro_use]
// extern crate ;
#[cfg(test)]
mod tests {
    #[test]
    fn test_adder() {
        assert_eq!(super::sum!(55, 5), 60);
    }
}
