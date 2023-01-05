pub trait KeySerializeControl {
  /// By default a ForeignKey does not serialize its value if it is in the `Loaded`
  /// state. The value would be transformed into a key using the [IntoKey] trait
  /// methods before serializing it.
  ///
  /// There are cases where this behaviour is not what you wish to happen, calling
  /// [`ForeignKey::allow_value_serialize()`] flags the ForeignKey to ignore the
  /// default behaviour and serialize any potential value it may hold.
  fn allow_value_serialize(&self);

  /// By default a ForeignKey does not serialize its value if it is in the `Loaded`
  /// state. The value would be transformed into a key using the [IntoKey] trait
  /// methods before serializing it.
  ///
  /// There are cases where this behaviour is not what you wish to happen, calling
  /// [`ForeignKey::allow_value_serialize()`] flags the ForeignKey to ignore the
  /// default behaviour and serialize any potential value it may hold.
  ///
  /// On the other this method remove the flag to go back to the default behaviour of
  /// transforming any Value into a Key before serialization.
  fn disallow_value_serialize(&self);
}

/// Blanket implementation for anything that implements KeySerializeControl and
/// that is in a Vec.
///
/// This implementation allows calling KeySerializeControl methods directly on
/// the vector itself to mutate every single child element.
impl<T> KeySerializeControl for Vec<T>
where
  T: KeySerializeControl,
{
  fn allow_value_serialize(&self) {
    self
      .iter()
      .for_each(KeySerializeControl::allow_value_serialize);
  }

  fn disallow_value_serialize(&self) {
    self
      .iter()
      .for_each(KeySerializeControl::disallow_value_serialize);
  }
}
