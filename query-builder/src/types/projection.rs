/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{Alias, BindingsList, Buildable, Field, Function, Param, Parametric, ValueLike};

/// Used to represent a list of projections to access fields of a table or
/// those of foreign tables and can even include filters.
#[derive(Debug, Clone)]
pub struct Projections(pub Vec<ValueLike>);

impl std::ops::Deref for Projections {
    type Target = Vec<ValueLike>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parametric for Projections {
    fn get_bindings(&self) -> BindingsList {
        self.0
            .iter()
            .flat_map(|m| m.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Buildable for Projections {
    fn build(&self) -> String {
        self.0
            .iter()
            .map(Buildable::build)
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl From<Vec<ValueLike>> for Projections {
    fn from(value: Vec<ValueLike>) -> Self {
        Self(value)
    }
}

impl From<Vec<&ValueLike>> for Projections {
    fn from(value: Vec<&ValueLike>) -> Self {
        Self(value.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>())
    }
}

impl From<&[ValueLike]> for Projections {
    fn from(value: &[ValueLike]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Field> for Projections {
    fn from(value: Field) -> Self {
        Self(vec![value.into()])
    }
}

impl From<Vec<Field>> for Projections {
    fn from(value: Vec<Field>) -> Self {
        Self(value.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<&Field> for Projections {
    fn from(value: &Field) -> Self {
        Self(vec![value.into()])
    }
}

impl From<&[Field]> for Projections {
    fn from(value: &[Field]) -> Self {
        Self(value.iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<Alias> for Projections {
    fn from(value: Alias) -> Self {
        Self(vec![value.into()])
    }
}

impl From<Vec<Alias>> for Projections {
    fn from(value: Vec<Alias>) -> Self {
        Self(value.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<Function> for Projections {
    fn from(value: Function) -> Self {
        Self(vec![value.into()])
    }
}

impl From<Vec<Function>> for Projections {
    fn from(value: Vec<Function>) -> Self {
        Self(value.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<&[Function]> for Projections {
    fn from(value: &[Function]) -> Self {
        Self(value.iter().cloned().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<Param> for Projections {
    fn from(value: Param) -> Self {
        Self(vec![value.into()])
    }
}

impl From<Vec<Param>> for Projections {
    fn from(value: Vec<Param>) -> Self {
        Self(value.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<&[Param]> for Projections {
    fn from(value: &[Param]) -> Self {
        Self(value.to_vec().iter().map(Into::into).collect::<Vec<_>>())
    }
}
