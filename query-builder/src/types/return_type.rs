/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{Buildable, Projections, ValueLike};
use std::fmt::{self, Display};

/// Return type
#[derive(Debug, Clone)]
pub enum ReturnType {
    /// Return nothing
    None,
    /// Return previous state
    Before,
    /// Return current state after change. This is the default
    After,
    /// Return the diff
    Diff,
    /// Return the listed fields/projection
    Projections(Projections),
}

impl From<Vec<ValueLike>> for ReturnType {
    fn from(value_like: Vec<ValueLike>) -> Self {
        ReturnType::Projections(Projections(value_like))
    }
}

impl Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = match self {
            ReturnType::None => "NONE".to_string(),
            ReturnType::Before => "BEFORE".to_string(),
            ReturnType::After => "AFTER".to_string(),
            ReturnType::Diff => "DIFF".to_string(),
            ReturnType::Projections(projections) => projections.build(),
        };
        write!(f, "RETURN {return_type} ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_return_type() {
        let return_type = ReturnType::None;
        assert_eq!(return_type.to_string(), "RETURN NONE ");

        let return_type = ReturnType::Before;
        assert_eq!(return_type.to_string(), "RETURN BEFORE ");

        let return_type = ReturnType::After;
        assert_eq!(return_type.to_string(), "RETURN AFTER ");

        let return_type = ReturnType::Diff;
        assert_eq!(return_type.to_string(), "RETURN DIFF ");

        let id = Field::new("id");
        let name = Field::new("name");
        let return_type = ReturnType::Projections(vec![id, name].into());
        assert_eq!(return_type.to_string(), "RETURN id, name ");
    }
}
