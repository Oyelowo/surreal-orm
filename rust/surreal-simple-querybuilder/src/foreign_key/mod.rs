mod foreign_key;
mod into_key;
mod key_ser_control;
mod loaded_value;

use loaded_value::*;

pub use foreign_key::*;
pub use into_key::*;
pub use key_ser_control::*;

/// A `ForeignKey` whose `Key` type is set to a `String` by default.
pub type Foreign<T> = ForeignKey<T, String>;

/// A `ForeignKey` whose `Key` type is set to a `Vec<String>` by default, and whose
/// `Value` type is set to be a `Vec<T>`
pub type ForeignVec<T> = ForeignKey<Vec<T>, Vec<String>>;
