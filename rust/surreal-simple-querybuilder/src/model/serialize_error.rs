use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type SqlSerializeResult<T> = std::result::Result<T, SqlSerializeError>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Debug)]
pub enum SqlSerializeError {
  // One or more variants that can be created by data structures through the
  // `ser::Error` and `de::Error` traits. For example the Serialize impl for
  // Mutex<T> might return an error because the mutex is poisoned, or the
  // Deserialize impl for a struct may return an error because a required
  // field is missing.
  Message(String),
}

impl ser::Error for SqlSerializeError {
  fn custom<T: Display>(msg: T) -> Self {
    SqlSerializeError::Message(msg.to_string())
  }
}

impl de::Error for SqlSerializeError {
  fn custom<T: Display>(msg: T) -> Self {
    SqlSerializeError::Message(msg.to_string())
  }
}

impl Display for SqlSerializeError {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match self {
      SqlSerializeError::Message(msg) => formatter.write_str(msg),
    }
  }
}

impl std::error::Error for SqlSerializeError {}
