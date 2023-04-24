use std::fmt::{self, Display};

use crate::{Buildable, Valuex};

use super::Field;

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
    Projections(Vec<Valuex>),
}

impl Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = match self {
            ReturnType::None => "NONE".to_string(),
            ReturnType::Before => "BEFORE".to_string(),
            ReturnType::After => "AFTER".to_string(),
            ReturnType::Diff => "DIFF".to_string(),
            ReturnType::Projections(projections) => projections
                .iter()
                .map(|p| p.build())
                .collect::<Vec<_>>()
                .join(", "),
        };
        write!(f, "RETURN {return_type} ")
    }
}
impl<T: Into<Vec<Valuex>>> From<T> for ReturnType {
    fn from(value: T) -> Self {
        Self::Projections(value.into())
    }
}
// impl From<Vec<&Field>> for ReturnType {
//     fn from(value: Vec<&Field>) -> Self {
//         Self::Projections(value.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>())
//     }
// }
//
// impl From<Vec<Field>> for ReturnType {
//     fn from(value: Vec<Field>) -> Self {
//         Self::Projections(value)
//     }
// }
//
// impl<const N: usize> From<&[Field; N]> for ReturnType {
//     fn from(value: &[Field; N]) -> Self {
//         Self::Projections(value.to_vec())
//     }
// }
//
// impl<const N: usize> From<&[&Field; N]> for ReturnType {
//     fn from(value: &[&Field; N]) -> Self {
//         Self::Projections(
//             value
//                 .to_vec()
//                 .into_iter()
//                 .map(ToOwned::to_owned)
//                 .collect::<Vec<_>>(),
//         )
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

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

        let return_type = ReturnType::Projections(vec!["id".into(), "name".into()]);
        assert_eq!(return_type.to_string(), "RETURN id, name ");
    }
}
