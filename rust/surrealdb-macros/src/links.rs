use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Reference<V: SurrealdbNode> {
    FetchedValue(V),
    Id(String),
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

    pub fn id(&self) -> Option<&String> {
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
    ($ident:ident; $target:ty) => {
        impl<V: SurrealdbNode> std::ops::Deref for $ident<V> {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<V: SurrealdbNode> std::ops::DerefMut for $ident<V> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinkOne<V: SurrealdbNode>(Reference<V>);
implement_deref_for_link!(LinkOne; Reference<V>);

// impl<V: SurrealdbNode> Deref for LinkOne<V> {
//     type Target = Reference<V>;
//
//     fn deref(&self) -> &Self::Target {
//         todo!()
//     }
// }

impl<V: SurrealdbNode> From<LinkOne<V>> for Reference<V> {
    fn from(value: LinkOne<V>) -> Self {
        todo!()
    }
}

impl<V: SurrealdbNode> From<Reference<V>> for LinkOne<V> {
    fn from(value: Reference<V>) -> Self {
        todo!()
    }
}

impl<V: SurrealdbNode> LinkOne<V> {
    pub fn null() -> LinkOne<V> {
        LinkOne(Reference::Null)
    }
}

impl<V: SurrealdbNode + Default> Default for LinkOne<V> {
    fn default() -> Self {
        // Self(Default::default())
        Self(Reference::Null)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LinkMany<V: SurrealdbNode>(Vec<Reference<V>>);
implement_deref_for_link!(LinkMany; Vec<Reference<V>>);

// impl<V: SurrealdbNode> Deref for LinkMany<V> {
//     type Target = Vec<Reference<V>>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Relate<V: SurrealdbNode>(Vec<Reference<V>>);
implement_deref_for_link!(Relate; Vec<Reference<V>>);

impl<V: SurrealdbNode> From<Relate<V>> for Vec<Reference<V>> {
    fn from(value: Relate<V>) -> Self {
        value.0
    }
}

impl<V: SurrealdbNode> From<Vec<Reference<V>>> for Relate<V> {
    fn from(value: Vec<Reference<V>>) -> Self {
        Self(value)
    }
}
// pub type Relate<V> = Vec<LinkOne<V>>;
//

#[derive(Debug, Deserialize, Serialize, Clone)]
// Use boxing to break reference cycle
pub struct LinkSelf<V: SurrealdbNode>(Box<Reference<V>>);

implement_deref_for_link!(LinkSelf; Box<Reference<V>>);
// impl<V: SurrealdbNode> Deref for LinkSelf<V> {
//     type Target = Box<Reference<V>>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl<V: SurrealdbNode> DerefMut for LinkSelf<V> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

impl<V: SurrealdbNode + Default> Default for LinkSelf<V> {
    fn default() -> Self {
        Self(Reference::Null.into())
    }
}
// pub type LinkSelf<V> = Box<LinkOne<V>>;

impl<V: SurrealdbNode> From<V> for LinkOne<V> {
    fn from(model: V) -> Self {
        let x = model.get_key();
        Self(Reference::Id(
            x.expect("Id not found. Make sure Id exists for this model")
                .to_owned(),
        ))
    }
}

impl<V: SurrealdbNode> From<&V> for LinkOne<V> {
    fn from(model: &V) -> Self {
        let x = model.clone().get_key();
        match x {
            Some(x) => Self(Reference::Id(x.to_owned())),
            None => Self(Reference::Null),
        }
        // Self(Reference::Id(
        //     x.expect("Id not found. Make sure Id exists for this model"),
        // ))
    }
}
