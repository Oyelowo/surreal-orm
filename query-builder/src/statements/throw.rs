/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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

impl Erroneous for ThrowStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.message.get_errors()
    }
}

impl Parametric for ThrowStatement {
    fn get_bindings(&self) -> BindingsList {
        self.message.get_bindings()
    }
}

impl Buildable for ThrowStatement {
    fn build(&self) -> String {
        format!("THROW {};", self.message.build())
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
    use crate::{traits::Buildable, ToRaw};

    #[test]
    fn test_throw_stmt_build() {
        let statement = throw("some error message");
        assert_eq!(statement.fine_tune_params(), "THROW $_param_00000001;");
        assert_eq!(statement.to_raw().build(), "THROW 'some error message';");
    }
}
