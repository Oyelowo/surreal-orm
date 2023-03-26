use std::fmt::{Display, Formatter};

use crate::sql::{Buildable, Queryable};

pub struct RawStatement(String);

impl Display for RawStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for RawStatement {
    fn build(&self) -> String {
        self.0.to_string()
    }
}

pub trait ToRawStatement
where
    Self: Sized,
{
    fn to_raw(self) -> RawStatement;
}

impl<T> ToRawStatement for T
where
    T: Queryable,
{
    fn to_raw(self) -> RawStatement {
        let query_raw =
            self.get_bindings()
                .into_iter()
                .fold(self.build(), |query_parametized, binding| {
                    query_parametized.replace(
                        binding.get_param_dollarised().as_str(),
                        binding.get_raw_value().as_str(),
                    )
                });

        RawStatement(query_raw)
    }
}
