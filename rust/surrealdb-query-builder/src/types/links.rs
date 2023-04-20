/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use serde::{Deserialize, Serialize};

use crate::{SurrealId, SurrealdbNode};

/// A reference to foreign node which can either be an ID or a fetched value itself or null.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Reference<V: SurrealdbNode> {
    /// the id of the foreign node. The defualt if foreign node is not fetched
    Id(SurrealId),
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
        let x = model.get_key::<SurrealId>();
        Self::Id(
            x.expect("Id not found. Make sure Id exists for this model")
                .to_owned(),
        )
    }

    /// returns the id of the foreign node if it exists
    pub fn get_id(&self) -> Option<&SurrealId> {
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
                let x = model.get_key::<SurrealId>();
                let xx = match x {
                    Some(id) => {
                        let bb = id.clone();
                        Reference::Id(bb)
                    }
                    None => Reference::Null,
                };
                Self(xx.into())
            }
        }

        impl<V: SurrealdbNode + Clone> std::convert::From<&$surrealdb_node_generics>
            for $reference_type
        {
            fn from(model: &$surrealdb_node_generics) -> Self {
                let x = model.clone().get_key::<SurrealId>();
                match x {
                    Some(x) => Self(Reference::Id(x.to_owned()).into()),
                    None => Self(Reference::Null.into()),
                }
            }
        }
    };
}

impl<V: SurrealdbNode> std::convert::From<Vec<V>> for LinkMany<V> {
    fn from(model_vec: Vec<V>) -> Self {
        let xx = model_vec
            .into_iter()
            .map(|m| {
                let x = m.get_key::<SurrealId>();
                let xx = match x {
                    Some(id) => {
                        let bb = id.clone();
                        Reference::Id(bb)
                    }
                    None => Reference::Null,
                };
                xx
            })
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

            /// Returns just the fully fetched values if fetched and available, otherwise, None
            pub fn values(&self) -> Option<Vec<&V>> {
                self.0
                    .iter()
                    .map(|m| match m {
                        Reference::FetchedValue(v) => Some(v),
                        Reference::Id(_) => None,
                        Reference::Null => None,
                    })
                    .collect::<Option<Vec<_>>>()
            }

            /// Returns just the keys of the foreign field if available, otherwise, None
            pub fn keys(&self) -> Option<Vec<&SurrealId>> {
                self.0
                    .iter()
                    .map(|m| match m {
                        Reference::FetchedValue(_) => None,
                        Reference::Id(id) => Some(id),
                        Reference::Null => None,
                    })
                    .collect::<Option<Vec<_>>>()
            }
        }
    };
}

/// ~eference to a foreign node in a simple direct one-to-many relationship
/// Returns either the foreign values if fetched, id keys of the foreign Field if not fetched,
/// empty Vec if not available
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LinkMany<V: SurrealdbNode>(Vec<Reference<V>>);

implement_deref_for_link!(LinkMany<V>; Vec<Reference<V>>);
impl_utils_for_ref_vec!(LinkMany);
implement_bidirectional_conversion!(LinkMany<V>, Vec<Reference<V>>);

/// reference to a foreign node in a one-to-many relationship via an edge
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Relate<V: SurrealdbNode>(Vec<Reference<V>>);

implement_deref_for_link!(Relate<V>; Vec<Reference<V>>);
implement_bidirectional_conversion!(Relate<V>, Vec<Reference<V>>);
impl_utils_for_ref_vec!(Relate);
