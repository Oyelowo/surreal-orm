use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Reference<V: SurrealdbNode> {
    FetchedValue(V),
    Id(String),
    Null,
}

// impl<V: SurrealdbModel> Default for Reference<V> {
//     fn default() -> Self {
//         Self::None
//     }
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinkOne<V: SurrealdbNode>(Reference<V>);

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
pub struct LinkMany<V: SurrealdbNode>(Vec<LinkOne<V>>);
// pub type LinkMany<V> = Vec<LinkOne<V>>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Relate<V: SurrealdbNode>(Vec<LinkOne<V>>);
// pub type Relate<V> = Vec<LinkOne<V>>;
//

#[derive(Debug, Deserialize, Serialize, Clone)]
// Use boxing to break reference cycle
pub struct LinkSelf<V: SurrealdbNode>(Box<LinkOne<V>>);

impl<V: SurrealdbNode + Default> Default for LinkSelf<V> {
    fn default() -> Self {
        // Self(Default::default())
        Self(Box::new(LinkOne(Reference::Null)))
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

impl<V> LinkOne<V>
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
        Self(Reference::Id(
            x.expect("Id not found. Make sure Id exists for this model")
                .to_owned(),
        ))
    }

    pub fn id(&self) -> Option<&String> {
        match &self.0 {
            Reference::Id(v) => Some(v),
            _ => None,
        }
    }

    pub fn value_ref(&self) -> Option<&V> {
        match &self.0 {
            Reference::FetchedValue(v) => Some(v),
            _ => None,
        }
    }

    pub fn value_owned(self) -> Option<V> {
        match self.0 {
            Reference::FetchedValue(v) => Some(v),
            _ => None,
        }
    }
}
