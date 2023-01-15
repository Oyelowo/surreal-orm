use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Reference<V: SurrealdbModel> {
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
pub struct LinkOne<V: SurrealdbModel>(Reference<V>);

pub type LinkMany<V> = Vec<LinkOne<V>>;

// Use boxing to break reference cycle
pub type LinkSelf<V> = Box<LinkOne<V>>;
// pub type LinkSelf<V> = LinkOne<Box<V>>;

impl<V: SurrealdbModel> From<V> for LinkOne<V> {
    fn from(model: V) -> Self {
        let x = model.get_key();
        Self(Reference::Id(
            x.expect("Id not found. Make sure Id exists for this model"),
        ))
    }
}

impl<V: SurrealdbModel> From<&V> for LinkOne<V> {
    fn from(model: &V) -> Self {
        let x = model.get_key();
        Self(Reference::Id(
            x.expect("Id not found. Make sure Id exists for this model"),
        ))
    }
}

impl<V> LinkOne<V>
where
    V: SurrealdbModel,
{
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn from_model(model: impl SurrealdbModel) -> Self {
        let x = model.get_key();
        Self(Reference::Id(
            x.expect("Id not found. Make sure Id exists for this model"),
        ))
    }

    pub fn value(self) -> Option<V> {
        match self.0 {
            Reference::FetchedValue(v) => Some(v),
            _ => None,
        }
    }
}
