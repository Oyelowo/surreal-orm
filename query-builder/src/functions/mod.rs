/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

//! SurrealDB comes with a large number of in-built functions for checking, manipulating, and
//! working with many different types of data. These functions are grouped into a number of
//! different packages, which can be seen below.
//!
//! Inspect each module to dive deeper into the examples and documentation for each function.
//!
//! [See the SurrealDB documentation for more information](https://surrealdb.com/docs/functions/)
//! about the different types of functions that are available.

/// This module contains the different types of functions for working with arrays.
pub mod array;

// includes the count function
mod count;
pub use count::*;

/// This module contains the different types of functions for cryptographical encryption and
/// decryption.
pub mod crypto;

/// This module contains the different types of functions for working with geo data.
pub mod geo;

/// This module contains the different types of functions for working with HTTP requests.
pub mod http;

/// This module contains the different types of functions for working with mathematical
/// operations.
pub mod math;

/// This module contains the different types of functions for parsing data.
pub mod parse;

/// This module contains the different types of functions for generating random data.
pub mod rand;
pub use self::rand::rand;

/// This module contains the scrypting functions.
mod script;
pub use script::*;

/// This module contains the different types of functions for working with session data.
pub mod session;

/// This module contains the different types of functions for working with sleep operations.
pub mod sleep;

/// This module contains the different types of functions for working with string data.
pub mod string;

/// This module contains the different types of functions for working with time data.
pub mod time;

/// This module contains the different types of functions for type conversion.
pub mod type_;

/// This module contains the different types of functions for deriving metadata from
/// surrealdb record id.
pub mod meta;

/// This module contains the different types of functions for working with vectors.
pub mod vector;

/// This module contains the different types of functions for working with search.
pub mod search;
