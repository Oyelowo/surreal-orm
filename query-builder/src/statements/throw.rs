/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt;

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    StrandLike,
};

/// Creates a THROW statement builder.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::throw};
/// throw("some error message").build();
/// ```
pub fn throw(message: impl Into<StrandLike>) -> ThrowStatement {
    let message: StrandLike = message.into();
    ThrowStatement { message }
}

/// Represents the THROW statement
pub struct ThrowStatement {
    message: StrandLike,
}

impl Queryable for ThrowStatement {}

impl Erroneous for ThrowStatement {}

impl Parametric for ThrowStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Buildable for ThrowStatement {
    fn build(&self) -> String {
        format!("THROW \"{}\";", self.message.build())
    }
}

impl fmt::Display for ThrowStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Buildable;

    #[test]
    fn test_throw_stmt_build() {
        let statement = throw("some error message").build();
        assert_eq!(statement, "THROW 'some error message';");
    }
}
