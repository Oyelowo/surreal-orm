use std::ops::Deref;

/// Any type used inside a [ForeignKey] must implement this trait. It allows you
/// to transform the `I` type into an ID when `I` is serialized.
pub trait IntoKey<I> {
  fn into_key<E>(&self) -> Result<I, E>
  where
    E: serde::ser::Error;
}

impl<V, K> IntoKey<Vec<K>> for Vec<V>
where
  V: IntoKey<K>,
  K: std::iter::FromIterator<K>,
{
  fn into_key<E>(&self) -> Result<Vec<K>, E>
  where
    E: serde::ser::Error,
  {
    self.iter().map(|c| c.into_key()).collect()
  }
}

impl<V: IntoKey<K>, K> IntoKey<K> for Box<V> {
  fn into_key<E>(&self) -> Result<K, E>
  where
    E: serde::ser::Error,
  {
    self.deref().into_key()
  }
}
