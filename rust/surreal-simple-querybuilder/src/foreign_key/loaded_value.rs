use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoadedValue<V, K> {
  Loaded(V),
  Key(K),

  Unloaded,
}

impl<V, K> Default for LoadedValue<V, K> {
  fn default() -> Self {
    Self::Unloaded
  }
}

impl<V, K> LoadedValue<V, K> {
  /// Access the inner value by checking if it is loaded or not, thus returning
  /// an `Option<&T>` that is `Some` if it is loaded and `None` if it isn't.
  pub fn value(&self) -> Option<&V> {
    match self {
      Self::Loaded(v) => Some(v),
      _ => None,
    }
  }

  /// Access the inner key by checking if the foreign key is currently
  /// holding the key, thus returning a `Some<&I>` if it is one and `None`
  /// if it isn't.
  pub fn key(&self) -> Option<&K> {
    match self {
      Self::Key(i) => Some(i),
      _ => None,
    }
  }

  /// Return whether the current ForeignKey is unloaded. Returns `false` if `self`
  /// is either a key or a loaded value.
  pub fn is_unloaded(&self) -> bool {
    match &self {
      Self::Unloaded => true,
      _ => false,
    }
  }

  /// Drop any data `self` may currently hold and set it to the `Loaded` variant
  /// with the given value.
  pub fn set_value(&mut self, value: V) {
    *self = Self::Loaded(value);
  }

  /// Drop any data `self` may currently hold and set it to the `Key` variant
  /// with the given identifier.
  pub fn set_key(&mut self, identifier: K) {
    *self = Self::Key(identifier);
  }

  /// Drop the currently held value and set `self` to the `Unloaded` variant.
  pub fn unload(&mut self) {
    *self = Self::Unloaded;
  }
}

impl<V, K> Serialize for LoadedValue<V, K>
where
  K: Serialize,
  V: Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match &self {
      Self::Loaded(v) => v.serialize(serializer),
      Self::Key(i) => i.serialize(serializer),
      Self::Unloaded => Option::<K>::None.serialize(serializer),
    }
  }
}

impl<V, K> Debug for LoadedValue<V, K>
where
  V: Debug,
  K: Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Loaded(arg0) => f.debug_tuple("Loaded").field(arg0).finish(),
      Self::Key(arg0) => f.debug_tuple("Key").field(arg0).finish(),
      Self::Unloaded => write!(f, "Unloaded"),
    }
  }
}
