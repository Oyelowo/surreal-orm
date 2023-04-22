/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

//! This is it

pub mod array;
mod count;
pub use count::*;
pub mod crypto;
pub mod geo;
pub mod http;
pub mod math;
pub mod parse;
pub mod rand;
pub use self::rand::rand;
mod script;
pub use script::*;
pub mod session;
pub mod sleep;
pub mod string;
pub mod time;
pub mod type_;
mod validation;
pub use validation::is;
