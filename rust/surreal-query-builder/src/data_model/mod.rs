/// Contains all the casting functions
mod casting;

/// Contains the future function
mod futures;

/// Mathematic constants supported by surrealdb
pub mod math_constants;

pub use math_constants as MATH;

/// Contains all the casting functions and future
pub mod cast {
    pub use super::casting::*;
    pub use super::futures::*;
}
