pub mod authentication;
pub mod configurations;
pub mod error_handling;
mod macros;
pub mod middleware;
pub mod utils;
pub mod oauth;

pub use macros::{calculator, helpers};

#[macro_use]
// extern crate ;
#[cfg(test)]
mod tests {
    #[test]
    fn test_adder() {
        assert_eq!(super::sum!(55, 5), 60);
    }
}
