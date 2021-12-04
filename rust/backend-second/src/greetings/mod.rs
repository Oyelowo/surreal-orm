mod hello;


//pub use hello::{getGreeterServer};
pub mod grr {
    pub use super::hello::*;
}

