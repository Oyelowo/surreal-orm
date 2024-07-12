/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{Model, Node, SurrealId};
use serde::{Deserialize, Serialize};
use surrealdb::sql;

/// A reference to foreign node which can either be an ID or a fetched value itself or null.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Reference<V: Model> {
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
    V: Node,
{
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn from_model(model: impl Model) -> Self {
        let id: sql::Thing = model.get_id_as_thing();
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

impl<V: Node> Default for Reference<V> {
    fn default() -> Self {
        Self::Null
    }
}

macro_rules! implement_deref_for_link {
    ($reference_ty:ty; $target:ty) => {
        impl<V: Node> ::std::ops::Deref for $reference_ty {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<V: Node> ::std::ops::DerefMut for $reference_ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

macro_rules! implement_bidirectional_conversion {
    ($from:ty, $to:ty) => {
        impl<V: Node> std::convert::From<$from> for $to {
            fn from(value: $from) -> Self {
                value.0
            }
        }

        impl<V: Node> std::convert::From<$to> for $from {
            fn from(value: $to) -> Self {
                Self(value)
            }
        }
    };
}

macro_rules! impl_from_model_for_ref_type {
    ($surreal_node_generics:ty, $reference_type:ty) => {
        impl<V: Node> std::convert::From<$surreal_node_generics> for $reference_type {
            fn from(model: $surreal_node_generics) -> Self {
                // let id = model.get_id::<SurrealId<$surreal_node_generics>>();
                let id: sql::Thing = model.get_id_as_thing();
                let reference = Reference::Id(id.clone());
                Self(reference.into())
            }
        }

        impl<V: Node + Clone> std::convert::From<&$surreal_node_generics> for $reference_type {
            fn from(model: &$surreal_node_generics) -> Self {
                let id: sql::Thing = model.clone().get_id_as_thing();
                let reference = Reference::Id(id.to_owned());
                Self(reference.into())
            }
        }
    };
}

// The original derive default adds addition default bounds to the 
// generic type which is not wanted here as we can generate a 
// default without needing to know the wrapped type default.
// Out default is either null or empty vec
// With this, user does not have to implement default for the wrapped type
// even if they use the serde default attribute
macro_rules! implement_custom_default_for_link {
    ($reference_ty:ty) => {
        impl<V: Node> Default for $reference_ty {
            fn default() -> Self {
                Self::null()
            }
        }
    };
}
impl<V: Node> std::convert::From<Vec<V>> for LinkMany<V> {
    fn from(model_vec: Vec<V>) -> Self {
        let xx = model_vec
            .into_iter()
            .map(|m| {
                let id: sql::Thing = m.get_id_as_thing();
                Reference::Id(id.clone())
            })
            .collect::<Vec<Reference<V>>>();

        Self(xx)
    }
}

impl<V: Node> std::convert::From<Vec<sql::Thing>> for LinkMany<V> {
    fn from(model_vec: Vec<sql::Thing>) -> Self {
        let xx = model_vec
            .into_iter()
            .map(|m| Reference::Id(m))
            .collect::<Vec<Reference<V>>>();

        Self(xx)
    }
}
/// A reference to a foreign node which can either be an ID or a fetched value itself or null.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinkOne<V: Node>(Reference<V>);

implement_deref_for_link!(LinkOne<V>; Reference<V>);
implement_bidirectional_conversion!(LinkOne<V>, Reference<V>);
impl_from_model_for_ref_type!(V, LinkOne<V>);
// implement_from_for_reference_type!(Vec<V>, LinkMany<V>);
implement_custom_default_for_link!(LinkOne<V>);

impl<V: Node> From<LinkOne<V>> for Option<sql::Thing> {
    fn from(link: LinkOne<V>) -> Self {
        match link.0 {
            Reference::Id(id) => Some(id),
            _ => None,
        }
    }
}

impl<T, Id> From<SurrealId<T, Id>> for LinkOne<T>
where
    T: Node,
    Id: Into<sql::Id>,
{
    fn from(id: SurrealId<T, Id>) -> Self {
        let reference = Reference::Id(id.into());
        Self(reference)
    }
}

impl<V: Node> LinkOne<V> {
    /// returns nothing. Useful for satisfying types when instantiating a struct
    /// and you dont want the field be serialized
    pub fn null() -> LinkOne<V> {
        LinkOne(Reference::Null)
    }
}

/// a reference to current struct as foreign node in a one-to-one relationship which can be either an ID or a fetched value itself or null.
/// It is similar to `LinkOne` is boxed to avoid infinite recursion.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinkSelf<V: Node>(Box<Reference<V>>);

impl<V: Node> From<LinkSelf<V>> for Option<sql::Thing> {
    fn from(link: LinkSelf<V>) -> Self {
        match link.0.as_ref() {
            Reference::Id(id) => Some(id.clone()),
            _ => None,
        }
    }
}

impl<V: Node> LinkSelf<V> {
    /// returns nothing. Useful for satisfying types when instantiating a struct
    pub fn null() -> Self {
        Self(Reference::Null.into())
    }
}

impl<T, Id> From<SurrealId<T, Id>> for LinkSelf<T>
where
    T: Node,
    Id: Into<sql::Id>,
{
    fn from(id: SurrealId<T, Id>) -> Self {
        let reference = Reference::Id(id.into());
        Self(reference.into())
    }
}

implement_custom_default_for_link!(LinkSelf<V>);
implement_deref_for_link!(LinkSelf<V>; Box<Reference<V>>);
implement_bidirectional_conversion!(LinkSelf<V>, Box<Reference<V>>);
impl_from_model_for_ref_type!(Box<V>, LinkSelf<V>);
impl_from_model_for_ref_type!(V, LinkSelf<V>);

macro_rules! impl_utils_for_ref_vec {
    ($ref_vec:ident) => {
        impl<V: Node> $ref_vec<V> {
            /// Returns an empty vector
            pub fn null() -> Self {
                $ref_vec(::std::vec![])
            }

            /// Returns the number of values that are fetched and available and not null
            pub fn values_truthy_count(&self) -> usize {
                self.0
                    .iter()
                    .filter(|m| matches!(m, Reference::FetchedValue(_)))
                    .count()
            }

            /// Returns the values that are fetched and available and not null
            pub fn values_truthy(&self) -> ::std::vec::Vec<&V> {
                self.0
                    .iter()
                    // .filter(|m| matches!(m, Reference::FetchedValue(_)))
                    .filter_map(|m| match m {
                        Reference::FetchedValue(v) => Some(v),
                        Reference::Id(_) => None,
                        Reference::Null => None,
                    })
                    .collect::<::std::vec::Vec<_>>()
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
                    .collect::<::std::vec::Vec<::std::option::Option<&V>>>()
            }

            /// Returns just the keys of the foreign field. Some links may not exist
            pub fn keys(&self) -> ::std::vec::Vec<::std::option::Option<sql::Thing>> {
                self.0
                    .iter()
                    .map(|m| match m {
                        Reference::FetchedValue(fetched_value) => {
                            Some(fetched_value.get_id_as_thing())
                        }
                        Reference::Id(id) => Some(id.to_owned()),
                        Reference::Null => None,
                    })
                    .collect::<::std::vec::Vec<::std::option::Option<sql::Thing>>>()
            }

            /// Returns only the keys that are non-None ids.
            /// It does not check if the ids actually exist in the
            /// foreign table.
            pub fn keys_truthy(&self) -> ::std::vec::Vec<sql::Thing> {
                self.0
                    .iter()
                    .filter_map(|m| match m {
                        Reference::FetchedValue(fetched_value) => {
                            Some(fetched_value.get_id_as_thing())
                        }
                        Reference::Id(id) => Some(id.to_owned()),
                        Reference::Null => None,
                    })
                    .collect::<::std::vec::Vec<_>>()
            }

            /// Returns only the keys that exist if foreign links are fetched
            /// and available.
            pub fn keys_checked(&self) -> ::std::vec::Vec<sql::Thing> {
                self.0
                    .iter()
                    .filter_map(|m| match m {
                        Reference::FetchedValue(fetched_value) => {
                            Some(fetched_value.get_id_as_thing())
                        }
                        Reference::Id(_id) => None,
                        Reference::Null => None,
                    })
                    .collect::<::std::vec::Vec<_>>()
            }
        }
    };
}

/// Reference to a foreign node in a simple direct one-to-many relationship
/// Returns either the foreign values if fetched, id keys of the foreign Field if not fetched,
/// empty Vec if not available
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinkMany<V: Node>(Vec<Reference<V>>);

impl<V: Node> IntoIterator for LinkMany<V> {
    type Item = Reference<V>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<V: Node> From<LinkMany<V>> for Vec<Option<sql::Thing>> {
    fn from(link: LinkMany<V>) -> Self {
        link.0
            .into_iter()
            .map(|r| match r {
                Reference::Id(id) => Some(id),
                _ => None,
            })
            .collect::<Vec<Option<sql::Thing>>>()
    }
}

// impl<V: Node> From<Vec<V>> for LinkMany<V> {}

implement_custom_default_for_link!(LinkMany<V>);
implement_deref_for_link!(LinkMany<V>; Vec<Reference<V>>);
impl_utils_for_ref_vec!(LinkMany);
implement_bidirectional_conversion!(LinkMany<V>, Vec<Reference<V>>);

/// reference to a foreign node in a many-to-many relationship via an edge
/// This is not stored in the database and is merely a readonly field
/// derived from the edge referencing the foreign table.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Relate<V: Node>(Vec<Reference<V>>);

impl<V: Node> Default for Relate<V> {
    fn default() -> Self {
        Self::null()
    }
}


implement_deref_for_link!(Relate<V>; Vec<Reference<V>>);
implement_bidirectional_conversion!(Relate<V>, Vec<Reference<V>>);
impl_utils_for_ref_vec!(Relate);
