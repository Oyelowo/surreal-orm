// Don't export all utils module directly, let pub use do the selective exporting, so u expose only what's necessary by default
// pub mod utils;
mod macros;
mod util_module_alternative;
mod utils;

pub use macros::{calculator, helpers};
pub use util_module_alternative::greeter_alt::alt_good_morning;
pub use utils::{greet::good_morning, local_function, maths};

#[macro_use]
// extern crate ;
#[cfg(test)]
mod tests {
    #[test]
    fn test_adder() {
        assert_eq!(super::sum!(55, 5), 60);
    }
}
