/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use serde::{Deserialize, Serialize};
use surrealdb::sql;

use crate::{SurrealId, SurrealdbNode};

/// A reference to foreign node which can either be an ID or a fetched value itself or null.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Reference<V: SurrealdbNode> {
    /// the id of the foreign node. The defualt if foreign node is not fetched
    Id(sql::Thing),
    // Id(SurrealId<V>),
    /// the fetched value of the foreign node
    FetchedValue(V),
    /// null if foreign node does not exist
    Null,
}

impl<V> Reference<V>
where
    V: SurrealdbNode,
{
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn from_model(model: impl SurrealdbNode) -> Self {
        let id = model.get_id();
        Self::Id(id)
    }

    /// returns the id of the foreign node if it exists
    pub fn get_id(&self) -> Option<&sql::Thing> {
        match &self {
            Self::Id(v) => Some(v),
            _ => None,
        }
    }

    /// returns a referenced value of the foreign node if it exists and has been fetched
    pub fn value(&self) -> Option<&V> {
        match &self {
            Self::FetchedValue(v) => Some(v),
            _ => None,
        }
    }

    /// returns an owned value of the foreign node if it exists and has been fetched
    pub fn value_owned(self) -> Option<V> {
        match self {
            Self::FetchedValue(v) => Some(v),
            _ => None,
        }
    }
}

impl<V: SurrealdbNode> Default for Reference<V> {
    fn default() -> Self {
        Self::Null
    }
}

macro_rules! implement_deref_for_link {
    ($reference_ty:ty; $target:ty) => {
        impl<V: SurrealdbNode> std::ops::Deref for $reference_ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<V: SurrealdbNode> std::ops::DerefMut for $reference_ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

macro_rules! implement_bidirectional_conversion {
    ($from:ty, $to:ty) => {
        impl<V: SurrealdbNode> std::convert::From<$from> for $to {
            fn from(value: $from) -> Self {
                value.0
            }
        }

        impl<V: SurrealdbNode> std::convert::From<$to> for $from {
            fn from(value: $to) -> Self {
                Self(value)
            }
        }
    };
}

macro_rules! impl_from_model_for_ref_type {
    ($surrealdb_node_generics:ty, $reference_type:ty) => {
        impl<V: SurrealdbNode> std::convert::From<$surrealdb_node_generics> for $reference_type {
            fn from(model: $surrealdb_node_generics) -> Self {
                // let id = model.get_id::<SurrealId<$surrealdb_node_generics>>();
                let id: sql::Thing = model.get_id();
                let reference = Reference::Id(id.clone());
                Self(reference.into())
            }
        }

        impl<V: SurrealdbNode + Clone> std::convert::From<&$surrealdb_node_generics>
            for $reference_type
        {
            fn from(model: &$surrealdb_node_generics) -> Self {
                let id: sql::Thing = model.clone().get_id();
                let reference = Reference::Id(id.to_owned());
                Self(reference.into())
            }
        }
    };
}

impl<V: SurrealdbNode> std::convert::From<Vec<V>> for LinkMany<V> {
    fn from(model_vec: Vec<V>) -> Self {
        let xx = model_vec
            .into_iter()
            .map(|m| {
                let id: sql::Thing = m.get_id();
                Reference::Id(id.clone())
            })
            .collect::<Vec<Reference<V>>>();

        Self(xx)
    }
}

impl<V: SurrealdbNode> std::convert::From<Vec<sql::Thing>> for LinkMany<V> {
    fn from(model_vec: Vec<sql::Thing>) -> Self {
        let xx = model_vec
            .into_iter()
            .map(|m| Reference::Id(m.into()))
            .collect::<Vec<Reference<V>>>();

        Self(xx)
    }
}
/// A reference to a foreign node which can either be an ID or a fetched value itself or null.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LinkOne<V: SurrealdbNode>(Reference<V>);
implement_deref_for_link!(LinkOne<V>; Reference<V>);
implement_bidirectional_conversion!(LinkOne<V>, Reference<V>);
impl_from_model_for_ref_type!(V, LinkOne<V>);
// implement_from_for_reference_type!(Vec<V>, LinkMany<V>);

impl<V: SurrealdbNode> LinkOne<V> {
    /// returns nothing. Useful for satisfying types when instantiating a struct
    /// and you dont want the field be serialized
    pub fn null() -> LinkOne<V> {
        LinkOne(Reference::Null)
    }
}

/// a reference to current struct as foreign node in a one-to-one relationship which can be either an ID or a fetched value itself or null.
/// It is similar to `LinkOne` is boxed to avoid infinite recursion.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LinkSelf<V: SurrealdbNode>(Box<Reference<V>>);

impl<V: SurrealdbNode> LinkSelf<V> {
    /// returns nothing. Useful for satisfying types when instantiating a struct
    pub fn null() -> Self {
        Self(Reference::Null.into())
    }
}

// impl<V: SurrealdbNode> Default for LinkSelf<V> {}
implement_deref_for_link!(LinkSelf<V>; Box<Reference<V>>);
implement_bidirectional_conversion!(LinkSelf<V>, Box<Reference<V>>);
impl_from_model_for_ref_type!(Box<V>, LinkSelf<V>);
impl_from_model_for_ref_type!(V, LinkSelf<V>);

macro_rules! impl_utils_for_ref_vec {
    ($ref_vec:ident) => {
        impl<V: SurrealdbNode> $ref_vec<V> {
            /// Returns an empty vector
            pub fn null() -> Self {
                $ref_vec(vec![])
            }

            /// Returns the number of values that are fetched and available and not null
            pub fn values_truthy_count(&self) -> usize {
                self.0
                    .iter()
                    .filter(|m| matches!(m, Reference::FetchedValue(_)))
                    .count()
            }

            /// Returns the values that are fetched and available and not null
            pub fn values_truthy(&self) -> Vec<&V> {
                self.0
                    .iter()
                    // .filter(|m| matches!(m, Reference::FetchedValue(_)))
                    .filter_map(|m| match m {
                        Reference::FetchedValue(v) => Some(v),
                        Reference::Id(_) => None,
                        Reference::Null => None,
                    })
                    .collect::<Vec<_>>()
            }

            /// Returns just the fully fetched values if fetched and available, otherwise, None
            pub fn values(&self) -> Vec<Option<&V>> {
                self.0
                    .iter()
                    .map(|m| match m {
                        Reference::FetchedValue(v) => Some(v),
                        Reference::Id(_) => None,
                        Reference::Null => None,
                    })
                    .collect::<Vec<Option<&V>>>()
            }

            /// Returns just the keys of the foreign field. Some links may not exist
            pub fn keys(&self) -> Vec<Option<&sql::Thing>> {
                self.0
                    .iter()
                    .map(|m| match m {
                        Reference::FetchedValue(_) => None,
                        Reference::Id(id) => Some(id),
                        Reference::Null => None,
                    })
                    .collect::<Vec<Option<&sql::Thing>>>()
            }

            /// Returns only the keys that exist
            pub fn keys_truthy(&self) -> Vec<&sql::Thing> {
                self.0
                    .iter()
                    .filter_map(|m| match m {
                        Reference::FetchedValue(_) => None,
                        Reference::Id(id) => Some(id),
                        Reference::Null => None,
                    })
                    .collect::<Vec<_>>()
            }
        }
    };
}

/// Reference to a foreign node in a simple direct one-to-many relationship
/// Returns either the foreign values if fetched, id keys of the foreign Field if not fetched,
/// empty Vec if not available
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LinkMany<V: SurrealdbNode>(Vec<Reference<V>>);

// impl<V: SurrealdbNode> From<Vec<V>> for LinkMany<V> {}

implement_deref_for_link!(LinkMany<V>; Vec<Reference<V>>);
impl_utils_for_ref_vec!(LinkMany);
implement_bidirectional_conversion!(LinkMany<V>, Vec<Reference<V>>);

/// reference to a foreign node in a one-to-many relationship via an edge
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Relate<V: SurrealdbNode>(Vec<Reference<V>>);

implement_deref_for_link!(Relate<V>; Vec<Reference<V>>);
implement_bidirectional_conversion!(Relate<V>, Vec<Reference<V>>);
impl_utils_for_ref_vec!(Relate);
