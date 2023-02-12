use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Reference<V: SurrealdbNode> {
    FetchedValue(V),
    Id(SurId),
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
        let x = model.get_key();
        Self::Id(
            x.expect("Id not found. Make sure Id exists for this model")
                .to_owned(),
        )
    }

    pub fn get_id(&self) -> Option<&SurId> {
        match &self {
            Self::Id(v) => Some(v),
            _ => None,
        }
    }

    pub fn value_ref(&self) -> Option<&V> {
        match &self {
            Self::FetchedValue(v) => Some(v),
            _ => None,
        }
    }

    pub fn value(self) -> Option<V> {
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
// impl<V: SurrealdbNode> std::convert::From<V> for LinkOne<V> {
//     fn from(value: V) -> Self {
//         let reference = match value.get_key() {
//             Some(id) => Reference::Id(id.to_owned()),
//             None => Reference::Null,
//         };
//         LinkOne(reference.into())
//     }
// }
// macro_rules! implement_bidirectional_model_to_link {
//     ($from:ty, $to:ty) => {
//         impl<V: SurrealdbNode> std::convert::From<V> for $to {
//             fn from(value: V) -> Self {
//                 value.get_key()
//             }
//         }
//
//         // impl<V: SurrealdbNode> std::convert::From<$to> for V {
//         //     fn from(value: $to) -> Self {
//         //         Self(value)
//         //     }
//         // }
//     };
// }
// fn defr(x: Option<SurId>) {
//     // let x: Option<SurId> = todo!();
//     let po = match x {
//         Some(id) => Reference::id(id),
//         None => Reference::Null,
//     };
// }
// impl<V: SurrealdbNode> std::convert::From<V> for LinkOne<V> {
//     fn from(model: V) -> Self {
//         let x = model.get_key();
//         let xx = match x {
//             Some(id) => {
//                 let bb = id.clone();
//                 Reference::Id(bb)
//             }
//             None => Reference::Null,
//         };
//         Self(xx.into())
//         // Self(
//         //     Reference::Id(
//         //         x.expect("Id not found. Make sure Id exists for this model")
//         //             .to_owned(),
//         //     )
//         //     .into(),
//         // )
//     }
// }
macro_rules! impl_from_model_for_ref_type {
    ($surrealdb_node_generics:ty, $reference_type:ty) => {
        impl<V: SurrealdbNode> std::convert::From<$surrealdb_node_generics> for $reference_type {
            fn from(model: $surrealdb_node_generics) -> Self {
                let x = model.get_key();
                let xx = match x {
                    Some(id) => {
                        let bb = id.clone();
                        Reference::Id(bb)
                    }
                    None => Reference::Null,
                };
                Self(xx.into())
                // Self(
                //     Reference::Id(
                //         x.expect("Id not found. Make sure Id exists for this model")
                //             .to_owned(),
                //     )
                //     .into(),
                // )
            }
        }

        impl<V: SurrealdbNode> std::convert::From<&$surrealdb_node_generics> for $reference_type {
            fn from(model: &$surrealdb_node_generics) -> Self {
                let x = model.clone().get_key();
                match x {
                    Some(x) => Self(Reference::Id(x.to_owned()).into()),
                    None => Self(Reference::Null.into()),
                }
                // Self(Reference::Id(
                //     x.expect("Id not found. Make sure Id exists for this model"),
                // ))
            }
        }
    };
}

// impl<V: SurrealdbNode> std::convert::From<LinkMany<V>> for Vec<V> {
//     fn from(link_many: LinkMany<V>) -> Self {
//         link_many.0
//         let xx = model_vec
//             .into_iter()
//             .map(|m| {
//                 let x = m.get_key();
//                 let xx = match x {
//                     Some(id) => {
//                         let bb = id.clone();
//                         Reference::Id(bb)
//                     }
//                     None => Reference::Null,
//                 };
//                 xx
//             })
//             .collect::<Vec<Reference<V>>>();
//
//         Self(xx)
//     }
// }
impl<V: SurrealdbNode> std::convert::From<Vec<V>> for LinkMany<V> {
    fn from(model_vec: Vec<V>) -> Self {
        let xx = model_vec
            .into_iter()
            .map(|m| {
                let x = m.get_key();
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
// impl<V: SurrealdbNode> std::convert::From<Box<V>> for LinkSelf<V> {
//     fn from(model: Box<V>) -> Self {
//         let x = model.get_key();
//         Self(
//             Reference::Id(
//                 x.expect("Id not found. Make sure Id exists for this model")
//                     .to_owned(),
//             )
//             .into(),
//         )
//     }
// }
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LinkOne<V: SurrealdbNode>(Reference<V>);
implement_deref_for_link!(LinkOne<V>; Reference<V>);
implement_bidirectional_conversion!(LinkOne<V>, Reference<V>);
impl_from_model_for_ref_type!(V, LinkOne<V>);
// implement_from_for_reference_type!(Vec<V>, LinkMany<V>);

// impl<V: SurrealdbNode> From<V> for LinkOne<V> {
//     fn from(model: V) -> Self {
//         let x = model.get_key();
//         Self(Reference::Id(
//             x.expect("Id not found. Make sure Id exists for this model")
//                 .to_owned(),
//         ))
//     }
// }
//
// impl<V: SurrealdbNode> From<&V> for LinkOne<V> {
//     fn from(model: &V) -> Self {
//         let x = model.clone().get_key();
//         match x {
//             Some(x) => Self(Reference::Id(x.to_owned())),
//             None => Self(Reference::Null),
//         }
//         // Self(Reference::Id(
//         //     x.expect("Id not found. Make sure Id exists for this model"),
//         // ))
//     }
// }

impl<V: SurrealdbNode> LinkOne<V> {
    pub fn null() -> LinkOne<V> {
        LinkOne(Reference::Null)
    }
}

// impl<V: SurrealdbNode + Default> Default for LinkOne<V> {
//     fn default() -> Self {
//         // Self(Default::default())
//         Self(Reference::Null)
//     }
// }

// Use boxing to break reference cycle
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LinkSelf<V: SurrealdbNode>(Box<Reference<V>>);

impl<V: SurrealdbNode> LinkSelf<V> {
    pub fn nill() -> Self {
        Self(Reference::Null.into())
    }
}

// impl<V: SurrealdbNode> Default for LinkSelf<V> {}

implement_deref_for_link!(LinkSelf<V>; Box<Reference<V>>);
implement_bidirectional_conversion!(LinkSelf<V>, Box<Reference<V>>);
impl_from_model_for_ref_type!(Box<V>, LinkSelf<V>);
impl_from_model_for_ref_type!(V, LinkSelf<V>);

// impl<V: SurrealdbNode + Default> Default for LinkSelf<V> {
//     fn default() -> Self {
//         Self(Reference::Null.into())
//     }
// }

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LinkMany<V: SurrealdbNode>(Vec<Reference<V>>);

implement_deref_for_link!(LinkMany<V>; Vec<Reference<V>>);
// implement_bidirectional_conversion!(LinkMany<V>, Vec<Reference<V>>);

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Relate<V: SurrealdbNode>(Vec<Reference<V>>);

implement_deref_for_link!(Relate<V>; Vec<Reference<V>>);
implement_bidirectional_conversion!(Relate<V>, Vec<Reference<V>>);

impl<V: SurrealdbNode> LinkMany<V> {
    pub fn nill() -> Self {
        LinkMany(vec![])
    }

    pub fn values(&self) -> Vec<Option<&V>> {
        let xx = self
            .0
            .iter()
            .map(|m| match m {
                Reference::FetchedValue(v) => Some(v),
                Reference::Id(_) => None,
                Reference::Null => None,
            })
            .collect::<Vec<_>>();
        xx
    }

    pub fn keys(&self) -> Vec<Option<&SurId>> {
        let xx = self
            .0
            .iter()
            .map(|m| match m {
                Reference::FetchedValue(_) => None,
                Reference::Id(id) => Some(id),
                Reference::Null => None,
            })
            .collect::<Vec<_>>();
        xx
    }
}
