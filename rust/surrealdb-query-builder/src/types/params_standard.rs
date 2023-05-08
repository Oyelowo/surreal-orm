use std::ops::Deref;

use crate::{Param, SchemaGetter, SurrealdbModel, SurrealdbNode};

/// stands for surrealdb native `$value`
pub struct ValueParam(Param);

impl Deref for ValueParam {
    type Target = Param;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// creates surrealdb native `$value`
pub fn value() -> ValueParam {
    ValueParam(Param::new("value"))
}

/// stands for surrealdb native `$before`
pub struct BeforeParam(Param);

impl Deref for BeforeParam {
    type Target = Param;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// creates surrealdb native `$before`
pub fn before() -> BeforeParam {
    BeforeParam(Param::new("before"))
}

/// stands for surrealdb native `$after`
pub struct AfterParam(Param);

impl Deref for AfterParam {
    type Target = Param;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// creates surrealdb native `$after`
pub fn after() -> AfterParam {
    AfterParam(Param::new("after"))
}

/// stands for surrealdb native `$this`
pub struct ThisParam(Param);

impl Deref for ThisParam {
    type Target = Param;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// creates surrealdb native `$this`
pub fn this<T: SchemaGetter>() -> T::Schema {
    // T::schema().with_prefix("4545".into());
    let p = ThisParam(Param::new("this")).to_string();
    let xx = T::schema_prefixed(p);
    xx
}
// fn erer() {
//     this
//
// }
// pub fn this<T: SurrealdbNode>() -> ThisParam {
//     // T::schema().with_prefix("4545".into());
//     let xx = T::schema_prefixed("4545".into());
//     ThisParam(Param::new("this"))
// }
