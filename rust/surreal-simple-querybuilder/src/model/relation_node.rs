use std::fmt::Display;
use std::ops::Deref;

use super::SchemaFieldType;

pub struct RelationNode<T, Y> {
  relation_name: &'static str,
  relation_type: SchemaFieldType,

  /// used when the node type is only needed for its Display impl, it is used by
  /// the Display implementation.
  node: T,

  /// used when the node type is needed for its properties, for deeper nesting,
  /// it is used by the deref implementation.
  ///
  /// This is used because relations are either: `->relation->Node` or
  /// `->relation->Node.nested_property`. In the first case the `Node` would be
  /// print out by `T::Display` impl whereas the second case needs to be added
  /// to the origin holder as the `Y::Display` impl is never called since there
  /// is direct access to the nested properties.
  nested_node: Y,
}

impl<T, Y> RelationNode<T, Y> {
  pub fn new(
    relation_name: &'static str, relation_type: SchemaFieldType, node: T, nested_node: Y,
  ) -> Self {
    Self {
      relation_name,
      relation_type,
      node,
      nested_node,
    }
  }
}

impl<T, Y> Display for RelationNode<T, Y>
where
  T: Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let joining_segment = match self.relation_type {
      SchemaFieldType::ForeignRelation => "<-",
      _ => "->",
    };

    write!(f, "{}", self.node)

    // write!(
    //   f,
    //   "{joining_segment}{}{joining_segment}{}",
    //   self.relation_name, self.node
    // )
  }
}

impl<T, Y> Deref for RelationNode<T, Y> {
  type Target = Y;

  fn deref(&self) -> &Self::Target {
    &self.nested_node
  }
}
