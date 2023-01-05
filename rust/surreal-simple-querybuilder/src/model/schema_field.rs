use std::borrow::Cow;
use std::fmt::Display;

use serde::Serialize;

use crate::model::OriginHolder;
use crate::node_builder::ToNodeBuilder;

pub enum SchemaFieldType {
  Property,
  Relation,
  ForeignRelation,
}

pub struct SchemaField<const N: usize> {
  pub identifier: &'static str,
  field_type: SchemaFieldType,
  origin_holder: Option<OriginHolder<N>>,
}

impl<const N: usize> SchemaField<N> {
  pub const fn new(identifier: &'static str, field_type: SchemaFieldType) -> Self {
    Self {
      identifier,
      field_type,
      origin_holder: None,
    }
  }

  pub const fn with_origin(
    identifier: &'static str, field_type: SchemaFieldType, origin: Option<OriginHolder<N>>,
  ) -> Self {
    Self {
      identifier,
      field_type,
      origin_holder: origin,
    }
  }

  pub fn from_alias(self, alias: &'static str) -> SchemaField<{ N + 1 }> {
    let origin = match self.origin_holder {
      Some(h) => h,
      None => OriginHolder::new([""; N]),
    };

    let mut new_origin: [&'static str; N + 1] = [""; N + 1];
    new_origin[1..].clone_from_slice(&origin.segments);
    new_origin[0] = alias;

    SchemaField::<{ N + 1 }>::with_origin(
      self.identifier,
      self.field_type,
      Some(OriginHolder::new(new_origin)),
    )
  }

  /// Return the name of the field, and if the field is an edge then return the
  /// name of the edge instead.
  ///
  /// # Example
  /// ```
  /// #![allow(incomplete_features)]
  /// #![feature(generic_const_exprs)]
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// model!(Test {
  ///   normal_field,
  ///   ->edge->Test as test_edge
  /// });
  ///
  /// assert_eq!("normal_field", schema::model.normal_field.name());
  /// assert_eq!("edge", schema::model.test_edge.name());
  /// ```
  pub fn name(&self) -> &'static str {
    match self.field_type {
      SchemaFieldType::Property => self.identifier,
      _ => {
        let edge_name_index = self
          .identifier
          .chars()
          .take_while(|c| c.is_alphanumeric())
          .count();

        &self.identifier[..edge_name_index]
      }
    }
  }
}

impl<const N: usize> Display for SchemaField<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.origin_holder {
      Some(holder) => {
        write!(f, "{holder}")?;

        match &self.field_type {
          SchemaFieldType::Property => write!(f, ".")?,
          SchemaFieldType::Relation => write!(f, "->")?,
          SchemaFieldType::ForeignRelation => write!(f, "<-")?,
        };

        write!(f, "{}", self.identifier)
      }
      None => {
        // prefix depending on the field type
        match &self.field_type {
          SchemaFieldType::Property => {}
          SchemaFieldType::Relation => write!(f, "->")?,
          SchemaFieldType::ForeignRelation => write!(f, "<-")?,
        };

        write!(f, "{}", self.identifier)
      }
    }
  }
}

impl<const N: usize> ToNodeBuilder for SchemaField<N> {
  fn equals_parameterized(&self) -> String {
    // special case for the schema field as it may include dots, we replace them
    // by underscores.
    format!(
      "{self} = ${}",
      self
        .to_string()
        .replace(".", "_")
        .replace("->", "_")
        .replace("<-", "_")
    )
  }
}

impl<const N: usize> Into<Cow<'static, str>> for SchemaField<N> {
  fn into(self) -> Cow<'static, str> {
    Cow::from(self.identifier)
  }
}

impl<const N: usize> Serialize for SchemaField<N> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}
