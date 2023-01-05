use crate::model::SqlSerializeError;
use crate::model::SqlSerializeResult;
use serde::{ser, Serialize};

/// Serialize a struct into a series of `field = $field` pairs for every field
/// the struct has, using serde attributes.
pub fn to_parameters<T>(value: &T) -> SqlSerializeResult<String>
where
  T: Serialize,
{
  let mut serializer = SqlFieldSerializer {
    output: String::new(),
  };
  value.serialize(&mut serializer)?;
  Ok(serializer.output)
}

/// A special kind of serializer, its goal is to serialize structs and perhaps
/// maps to a series of `field = $field` pairs for every key in the supplied object.
///
/// Anything else will not be serialized, and values are ignored completely.
pub struct SqlFieldSerializer {
  output: String,
}

impl<'a> ser::Serializer for &'a mut SqlFieldSerializer {
  type Ok = ();

  type Error = SqlSerializeError;

  type SerializeSeq = Self;
  type SerializeTuple = Self;
  type SerializeTupleStruct = Self;
  type SerializeTupleVariant = Self;
  type SerializeMap = Self;
  type SerializeStruct = Self;
  type SerializeStructVariant = Self;

  fn serialize_bool(self, v: bool) -> SqlSerializeResult<()> {
    self.output += if v { "true" } else { "false" };
    Ok(())
  }

  fn serialize_i8(self, v: i8) -> SqlSerializeResult<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i16(self, v: i16) -> SqlSerializeResult<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i32(self, v: i32) -> SqlSerializeResult<()> {
    self.serialize_i64(i64::from(v))
  }

  fn serialize_i64(self, v: i64) -> SqlSerializeResult<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_u8(self, v: u8) -> SqlSerializeResult<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u16(self, v: u16) -> SqlSerializeResult<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u32(self, v: u32) -> SqlSerializeResult<()> {
    self.serialize_u64(u64::from(v))
  }

  fn serialize_u64(self, v: u64) -> SqlSerializeResult<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_f32(self, v: f32) -> SqlSerializeResult<()> {
    self.serialize_f64(f64::from(v))
  }

  fn serialize_f64(self, v: f64) -> SqlSerializeResult<()> {
    self.output += &v.to_string();
    Ok(())
  }

  fn serialize_char(self, v: char) -> SqlSerializeResult<()> {
    self.serialize_str(&v.to_string())
  }

  fn serialize_str(self, v: &str) -> SqlSerializeResult<()> {
    self.output += v;
    Ok(())
  }

  fn serialize_bytes(self, v: &[u8]) -> SqlSerializeResult<()> {
    use serde::ser::SerializeSeq;
    let mut seq = self.serialize_seq(Some(v.len()))?;
    for byte in v {
      seq.serialize_element(byte)?;
    }
    seq.end()
  }

  fn serialize_none(self) -> SqlSerializeResult<()> {
    self.serialize_unit()
  }

  fn serialize_some<T>(self, value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_unit(self) -> SqlSerializeResult<()> {
    Ok(())
  }

  fn serialize_unit_struct(self, _name: &'static str) -> SqlSerializeResult<()> {
    self.serialize_unit()
  }

  fn serialize_unit_variant(
    self, _name: &'static str, _variant_index: u32, variant: &'static str,
  ) -> SqlSerializeResult<()> {
    self.serialize_str(variant)
  }

  fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T>(
    self, _name: &'static str, _variant_index: u32, variant: &'static str, _value: &T,
  ) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    variant.serialize(&mut *self)?;
    self.output += " = $";
    variant.serialize(&mut *self)?;
    Ok(())
  }

  fn serialize_seq(self, _len: Option<usize>) -> SqlSerializeResult<Self::SerializeSeq> {
    Ok(self)
  }

  fn serialize_tuple(self, len: usize) -> SqlSerializeResult<Self::SerializeTuple> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_struct(
    self, _name: &'static str, len: usize,
  ) -> SqlSerializeResult<Self::SerializeTupleStruct> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_variant(
    self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize,
  ) -> SqlSerializeResult<Self::SerializeTupleVariant> {
    variant.serialize(&mut *self)?;
    Ok(self)
  }

  // Maps are represented in JSON as `{ K: V, K: V, ... }`.
  fn serialize_map(self, _len: Option<usize>) -> SqlSerializeResult<Self::SerializeMap> {
    Ok(self)
  }

  fn serialize_struct(
    self, _name: &'static str, len: usize,
  ) -> SqlSerializeResult<Self::SerializeStruct> {
    self.serialize_map(Some(len))
  }

  fn serialize_struct_variant(
    self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize,
  ) -> SqlSerializeResult<Self::SerializeStructVariant> {
    variant.serialize(&mut *self)?;
    Ok(self)
  }
}

impl<'a> ser::SerializeSeq for &'a mut SqlFieldSerializer {
  type Ok = ();
  type Error = SqlSerializeError;

  fn serialize_element<T>(&mut self, value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(&mut **self)
  }

  fn end(self) -> SqlSerializeResult<()> {
    Ok(())
  }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut SqlFieldSerializer {
  type Ok = ();
  type Error = SqlSerializeError;

  fn serialize_element<T>(&mut self, value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(&mut **self)
  }

  fn end(self) -> SqlSerializeResult<()> {
    Ok(())
  }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut SqlFieldSerializer {
  type Ok = ();
  type Error = SqlSerializeError;

  fn serialize_field<T>(&mut self, value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(&mut **self)
  }

  fn end(self) -> SqlSerializeResult<()> {
    Ok(())
  }
}

impl<'a> ser::SerializeTupleVariant for &'a mut SqlFieldSerializer {
  type Ok = ();
  type Error = SqlSerializeError;

  fn serialize_field<T>(&mut self, value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    value.serialize(&mut **self)
  }

  fn end(self) -> SqlSerializeResult<()> {
    Ok(())
  }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut SqlFieldSerializer {
  type Ok = ();
  type Error = SqlSerializeError;

  fn serialize_key<T>(&mut self, key: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += " , ";
    }

    key.serialize(&mut **self)?;
    self.output += " = $";
    key.serialize(&mut **self)
  }

  fn serialize_value<T>(&mut self, value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    self.output += ":";
    value.serialize(&mut **self)
  }

  fn end(self) -> SqlSerializeResult<()> {
    Ok(())
  }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut SqlFieldSerializer {
  type Ok = ();
  type Error = SqlSerializeError;

  fn serialize_field<T>(&mut self, key: &'static str, _value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += " , ";
    }

    self.output += key;
    self.output += " = $";
    self.output += key;

    Ok(())
  }

  fn end(self) -> SqlSerializeResult<()> {
    Ok(())
  }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut SqlFieldSerializer {
  type Ok = ();
  type Error = SqlSerializeError;

  fn serialize_field<T>(&mut self, key: &'static str, _value: &T) -> SqlSerializeResult<()>
  where
    T: ?Sized + Serialize,
  {
    if !self.output.is_empty() {
      self.output += " , ";
    }

    self.output += key;
    self.output += " = $";
    self.output += key;

    Ok(())
  }

  fn end(self) -> SqlSerializeResult<()> {
    Ok(())
  }
}
