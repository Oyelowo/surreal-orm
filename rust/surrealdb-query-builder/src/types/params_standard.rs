use std::ops::Deref;

use crate::Param;

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
