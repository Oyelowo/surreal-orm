/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::{self, Display};

use crate::{
    traits::{BindingsList, Buildable, Conditional, Erroneous, Parametric, Queryable, Raw, ToRaw},
    types::{CrudType, Filter},
};

#[derive(Clone, Debug)]
struct ForData {
    crud_types: Vec<CrudType>,
    condition: Option<Filter>,
    bindings: BindingsList,
}

#[derive(Clone, Debug)]
pub enum ForArgs {
    ForOption(CrudType),
    ForOptions(Vec<CrudType>),
}
impl From<CrudType> for ForArgs {
    fn from(value: CrudType) -> Self {
        Self::ForOption(value)
    }
}

impl From<Vec<CrudType>> for ForArgs {
    fn from(value: Vec<CrudType>) -> Self {
        Self::ForOptions(value)
    }
}
impl From<ForArgs> for Vec<CrudType> {
    fn from(value: ForArgs) -> Self {
        match value {
            ForArgs::ForOption(one) => vec![one],
            ForArgs::ForOptions(many) => many,
        }
    }
}

impl<'a, const N: usize> From<&[CrudType; N]> for ForArgs {
    fn from(value: &[CrudType; N]) -> Self {
        Self::ForOptions(value.to_vec())
    }
}

impl<const N: usize> From<[CrudType; N]> for ForArgs {
    fn from(value: [CrudType; N]) -> Self {
        Self::ForOptions(value.to_vec())
    }
}

pub struct ForStart(ForData);

impl ForStart {
    pub fn where_(mut self, condition: impl Conditional) -> For {
        let condition = Filter::new(condition);
        self.0.condition = Some(condition.clone());
        self.0.bindings.extend(condition.get_bindings());
        For(self.0)
    }
}

/// For statement is typically used within DEFINE TABLE and DEFINE FIELD for create
/// more granular permissions.
///
/// Examples:
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::for_};
/// use CrudType::*;
///
/// # let name = Field::new("name");
/// # let country = Field::new("country");
///  // You can create for a single crud operation
/// let statement = for_(Create).where_(name.like("Oyelowo"));
///  
///  // You can also create for a  list of crud operations
/// let statement = for_(&[Create, Delete, Select, Update]).where_(country.like("Canada"));
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn for_(for_crud_types: impl Into<ForArgs>) -> ForStart {
    ForStart(ForData {
        crud_types: for_crud_types.into().into(),
        condition: None,
        bindings: vec![],
    })
}

/// Builder struct for a `FOR` statement which is typeically used in DEFINE TABLE and DEFINE FIELD
/// statements for setting more granular permissions
#[derive(Clone, Debug)]
pub struct For(ForData);

impl Buildable for For {
    fn build(&self) -> String {
        let mut query = format!("FOR");
        if !&self.0.crud_types.is_empty() {
            query = format!(
                "{query} {}",
                &self
                    .0
                    .crud_types
                    .iter()
                    .map(|ct| {
                        let ct = ct.to_string();
                        ct
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        if let Some(cond) = &self.0.condition {
            query = format!("{query}\n\tWHERE {cond}");
        }
        query
    }
}

impl Erroneous for For {}
impl Queryable for For {}

impl Parametric for For {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Display for For {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// Permission types which can be a a single For statement
/// list of `for` statements
#[derive(Clone, Debug)]
pub enum PermissionType {
    /// Single `for` statement
    For(For),
    /// List of `for` statements
    Fors(Vec<For>),
    /// Single Raw statement
    RawStatement(Raw),
    /// List of Raw statements
    RawStatementList(Vec<Raw>),
}

impl ToRaw for PermissionType {
    fn to_raw(self: &PermissionType) -> Raw {
        match self {
            PermissionType::For(for_one) => for_one.to_raw(),
            PermissionType::Fors(for_many) => Raw::new(
                for_many
                    .into_iter()
                    .map(|f| f.to_raw().build())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            PermissionType::RawStatement(r) => r.to_raw(),
            PermissionType::RawStatementList(raw_list) => Raw::new(
                raw_list
                    .into_iter()
                    .map(|f| f.to_raw().build())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        }
    }
}

impl From<For> for PermissionType {
    fn from(value: For) -> Self {
        Self::For(value)
    }
}

impl From<Vec<For>> for PermissionType {
    fn from(value: Vec<For>) -> Self {
        Self::Fors(value)
    }
}

impl<'a, const N: usize> From<&[For; N]> for PermissionType {
    fn from(value: &[For; N]) -> Self {
        Self::Fors(value.to_vec())
    }
}

impl<const N: usize> From<[For; N]> for PermissionType {
    fn from(value: [For; N]) -> Self {
        Self::Fors(value.to_vec())
    }
}

impl From<Raw> for PermissionType {
    fn from(value: Raw) -> Self {
        Self::RawStatement(value)
    }
}

impl From<Vec<Raw>> for PermissionType {
    fn from(value: Vec<Raw>) -> Self {
        Self::RawStatementList(value)
    }
}

impl From<&Vec<Raw>> for PermissionType {
    fn from(value: &Vec<Raw>) -> Self {
        Self::RawStatementList(value.to_vec())
    }
}

impl<'a, const N: usize> From<&[Raw; N]> for PermissionType {
    fn from(value: &[Raw; N]) -> Self {
        Self::RawStatementList(value.to_vec())
    }
}

impl<const N: usize> From<[Raw; N]> for PermissionType {
    fn from(value: [Raw; N]) -> Self {
        Self::RawStatementList(value.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Field, Operatable};

    #[test]
    fn test_define_for_statement_state_machine() {
        let name = Field::new("name");

        let for_res = for_(CrudType::Create).where_(name.like("Oyelowo"));
        assert_eq!(
            for_res.fine_tune_params(),
            "FOR create\n\tWHERE name ~ $_param_00000001"
        );
        assert_eq!(
            for_res.to_raw().build(),
            "FOR create\n\tWHERE name ~ 'Oyelowo'"
        );
    }

    #[test]
    fn test_define_for_statement_state_machine_multiple() {
        use CrudType::*;
        let name = Field::new("name");

        let for_res = for_(&[Create, Delete, Select, Update]).where_(name.is("Oyedayo"));
        assert_eq!(
            for_res.fine_tune_params(),
            "FOR create, delete, select, update\n\tWHERE name IS $_param_00000001"
        );

        assert_eq!(
            for_res.to_raw().build(),
            "FOR create, delete, select, update\n\tWHERE name IS 'Oyedayo'"
        );
    }
}
