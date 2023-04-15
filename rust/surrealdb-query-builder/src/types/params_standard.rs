use std::ops::Deref;

use crate::Param;

pub struct ValueParam(Param);

impl Deref for ValueParam {
    type Target = Param;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn value() -> ValueParam {
    ValueParam(Param::new("value"))
}

pub struct BeforeParam(Param);

impl Deref for BeforeParam {
    type Target = Param;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn before() -> BeforeParam {
    BeforeParam(Param::new("before"))
}

pub struct AfterParam(Param);

impl Deref for AfterParam {
    type Target = Param;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn after() -> AfterParam {
    AfterParam(Param::new("after"))
}
