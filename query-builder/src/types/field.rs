/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::borrow::Cow;

use surrealdb::sql;

use crate::{
    Aliasable, Binding, BindingsList, Buildable, Conditional, Erroneous, ErrorList, Operatable,
    Parametric,
};

use super::Idiomx;

/// Represents a field in the database. This type wraps a `String` and
/// provides a convenient way to refer to a database fields. This can also be nested
///
/// # Examples
///
/// Creating a `Field`:
///
/// ```
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::*;
///
/// let field = Field::new("name");
///
/// assert_eq!(field.build(), "name");
/// ```
#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    bindings: BindingsList,
    graph_string: String,
    errors: ErrorList,
}

impl Field {
    /// Creates a new `Field` from a `String`.
    pub fn new(value: impl Into<String>) -> Self {
        let value: String = value.into();
        Self {
            name: value.clone(),
            graph_string: value,
            bindings: vec![],
            errors: vec![],
        }
    }

    /// Adds bindings to the field
    pub fn with_bindings(mut self, bindings: BindingsList) -> Self {
        self.bindings.extend(bindings);
        self
    }

    /// Adds errors to the field
    pub fn with_errors(mut self, errors: ErrorList) -> Self {
        self.errors.extend(errors);
        self
    }

    /// Sets field query graph. For building connection from node to node or node to edge.
    pub fn set_graph_string(&mut self, connection_string: String) -> &Self {
        self.graph_string = connection_string;
        self
    }

    /// Internal method for updating bindings
    pub fn ____________update_many_bindings<'bi>(
        &self,
        bindings: impl Into<&'bi [Binding]>,
    ) -> Self {
        let bindings: &'bi [Binding] = bindings.into();
        let updated_params = [&self.get_bindings().as_slice(), bindings].concat();
        Self {
            graph_string: self.graph_string.to_string(),
            bindings: updated_params,
            name: self.name.clone(),
            errors: self.errors.clone(),
        }
    }
}

impl Aliasable for Field {}

impl Conditional for Field {}

impl Operatable for Field {}

impl Erroneous for Field {}

impl Parametric for Field {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for Field {
    fn build(&self) -> String {
        self.graph_string.to_string()
    }
}

impl Buildable for Vec<Field> {
    fn build(&self) -> String {
        self.iter()
            .map(|f| f.build())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Buildable for Vec<&Field> {
    fn build(&self) -> String {
        self.iter()
            .map(|f| f.build())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

// impl<T: IntoIterator<Item = Field>> From<T> for Field {
//     fn from(value: T) -> Self {
//         Self::new(value.into_iter().collect())
//     }
// }

impl<const N: usize> Buildable for &[Field; N] {
    fn build(&self) -> String {
        self.to_vec()
            .iter()
            .map(|f| f.build())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl<const N: usize> Buildable for &[&Field; N] {
    fn build(&self) -> String {
        self.to_vec()
            .iter()
            .map(|f| f.build())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl<const N: usize> Parametric for &[Field; N] {
    fn get_bindings(&self) -> BindingsList {
        self.to_vec()
            .iter()
            .flat_map(|f| f.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl<const N: usize> Parametric for &[&Field; N] {
    fn get_bindings(&self) -> BindingsList {
        self.to_vec()
            .iter()
            .flat_map(|f| f.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Parametric for Vec<Field> {
    fn get_bindings(&self) -> BindingsList {
        self.iter()
            .flat_map(|f| f.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Parametric for Vec<&Field> {
    fn get_bindings(&self) -> BindingsList {
        self.iter()
            .flat_map(|f| f.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Erroneous for Vec<Field> {
    fn get_errors(&self) -> ErrorList {
        self.iter().flat_map(|f| f.get_errors()).collect::<Vec<_>>()
    }
}

impl Erroneous for Vec<&Field> {
    fn get_errors(&self) -> ErrorList {
        self.iter().flat_map(|f| f.get_errors()).collect::<Vec<_>>()
    }
}

impl Erroneous for &[Field] {
    fn get_errors(&self) -> ErrorList {
        self.iter().flat_map(|f| f.get_errors()).collect::<Vec<_>>()
    }
}

impl Erroneous for &[&Field] {
    fn get_errors(&self) -> ErrorList {
        self.iter().flat_map(|f| f.get_errors()).collect::<Vec<_>>()
    }
}

impl<const N: usize> Erroneous for &[Field; N] {
    fn get_errors(&self) -> ErrorList {
        self.iter().flat_map(|f| f.get_errors()).collect::<Vec<_>>()
    }
}

impl<const N: usize> Erroneous for &[&Field; N] {
    fn get_errors(&self) -> ErrorList {
        self.iter().flat_map(|f| f.get_errors()).collect::<Vec<_>>()
    }
}

impl From<&Field> for Idiomx {
    fn from(value: &Field) -> Self {
        Self::new(value.name.clone().into())
    }
}

impl From<Field> for sql::Idiom {
    fn from(val: Field) -> Self {
        val.name.into()
    }
}

impl<'a> From<Cow<'a, Self>> for Field {
    fn from(value: Cow<'a, Field>) -> Self {
        match value {
            Cow::Borrowed(v) => v.clone(),
            Cow::Owned(v) => v,
        }
    }
}

impl<'a> From<&'a Field> for Cow<'a, Field> {
    fn from(value: &'a Field) -> Self {
        Cow::Borrowed(value)
    }
}

impl From<Field> for Cow<'static, Field> {
    fn from(value: Field) -> Self {
        Cow::Owned(value)
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&Self> for Field {
    fn from(value: &Field) -> Self {
        value.to_owned()
    }
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<Field> for String {
    fn from(value: Field) -> Self {
        value.build()
    }
}

impl AsRef<str> for Field {
    fn as_ref(&self) -> &str {
        self.graph_string.as_str()
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{AliasName, ToRaw};

    use super::*;

    #[test]
    fn test_field() {
        let age = Field::new("age");
        let operation = age.greater_than_or_equal(18).less_than_or_equal(56);

        assert_eq!(
            operation.fine_tune_params(),
            "age >= $_param_00000001 <= $_param_00000002"
        );
        assert_eq!(operation.clone().to_raw().to_string(), "age >= 18 <= 56");
    }

    #[test]
    fn test_field_alias() {
        let age = Field::new("age");
        let age_of_human = AliasName::new("age_of_human");

        assert_eq!(age.__as__(age_of_human).build(), "age AS age_of_human");
    }

    #[test]
    fn test_field_with_operation_alias() {
        let age = Field::new("age");
        let legal_age = AliasName::new("legal_age");

        let operation = age.greater_than_or_equal(18).less_than_or_equal(56);
        let operation_aliased = operation.__as__(legal_age);

        assert_eq!(
            operation_aliased.fine_tune_params(),
            "age >= $_param_00000001 <= $_param_00000002 AS legal_age"
        );
        assert_eq!(
            operation_aliased.to_raw().to_string(),
            "age >= 18 <= 56 AS legal_age"
        );
    }
}
