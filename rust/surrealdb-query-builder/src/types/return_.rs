use std::fmt::{self, Display};

use super::Field;

#[derive(Debug)]
pub enum Return {
    None,
    Before,
    After,
    Diff,
    Projections(Vec<Field>),
}

impl Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = match self {
            Return::None => "NONE".to_string(),
            Return::Before => "BEFORE".to_string(),
            Return::After => "AFTER".to_string(),
            Return::Diff => "DIFF".to_string(),
            Return::Projections(projections) => projections
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
        };
        write!(f, "RETURN {return_type} ")
    }
}

impl From<Vec<&Field>> for Return {
    fn from(value: Vec<&Field>) -> Self {
        Self::Projections(value.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>())
    }
}

impl From<Vec<Field>> for Return {
    fn from(value: Vec<Field>) -> Self {
        Self::Projections(value)
    }
}

impl<const N: usize> From<&[Field; N]> for Return {
    fn from(value: &[Field; N]) -> Self {
        Self::Projections(value.to_vec())
    }
}

impl<const N: usize> From<&[&Field; N]> for Return {
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
