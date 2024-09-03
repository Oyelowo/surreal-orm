/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt;

use crate::traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable};

/// Creates a BREAK statement.
///
/// Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::break_};
/// break_();
/// ```
pub fn break_() -> BreakStatement {
    BreakStatement
}

/// Represents the BREAK statement
pub struct BreakStatement;

impl Queryable for BreakStatement {}

impl Erroneous for BreakStatement {}

impl Parametric for BreakStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Buildable for BreakStatement {
    fn build(&self) -> String {
        "BREAK;".to_string()
    }
}

impl fmt::Display for BreakStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Buildable;

    #[test]
    fn test_break_stmt_build() {
        let statement = break_().build();
        assert_eq!(statement, "BREAK;");
    }
}
