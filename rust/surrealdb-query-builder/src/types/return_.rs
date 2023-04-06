use std::fmt::{self, Display};

use super::Field;

#[derive(Debug, Clone)]
pub enum ReturnType {
    None,
    Before,
    After,
    Diff,
    Projections(Vec<Field>),
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
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
        };
        write!(f, "RETURN {return_type} ")
    }
}

impl From<Vec<&Field>> for ReturnType {
    fn from(value: Vec<&Field>) -> Self {
        Self::Projections(value.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>())
    }
}

impl From<Vec<Field>> for ReturnType {
    fn from(value: Vec<Field>) -> Self {
        Self::Projections(value)
    }
}

impl<const N: usize> From<&[Field; N]> for ReturnType {
    fn from(value: &[Field; N]) -> Self {
        Self::Projections(value.to_vec())
    }
}

impl<const N: usize> From<&[&Field; N]> for ReturnType {
    fn from(value: &[&Field; N]) -> Self {
        Self::Projections(
            value
                .to_vec()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>(),
        )
    }
}
