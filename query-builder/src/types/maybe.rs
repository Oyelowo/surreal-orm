use serde::Serialize;

/// Represents a value that may or may not be present
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Maybe<T> {
    /// When the value is present
    Some(T),
    /// When the value is absent
    None,
}

impl<T> Maybe<T> {
    /// Checks if the value is present
    pub fn is_some(&self) -> bool {
        matches!(self, Maybe::Some(_))
    }

    /// Checks if the value is absent
    pub fn is_none(&self) -> bool {
        matches!(self, Maybe::None)
    }
}
