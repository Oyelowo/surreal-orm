/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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

impl<const N: usize> From<&[CrudType; N]> for ForArgs {
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
    pub fn where_(mut self, condition: impl Conditional) -> ForPermission {
        let condition = Filter::new(condition);
        self.0.condition = Some(condition.clone());
        self.0.bindings.extend(condition.get_bindings());
        ForPermission(self.0)
    }
}

/// For statement is typically used within DEFINE TABLE and DEFINE FIELD for create
/// more granular permissions.
///
/// Examples:
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::for_permission};
/// use CrudType::*;
///
/// # let name = Field::new("name");
/// # let country = Field::new("country");
///
///  // You can create for a single crud operation
/// let statement = for_permission(Create).where_(name.like("Oyelowo"));
///  
///  // You can also create for a  list of crud operations
/// let statement = for_permission(&[Create, Delete, Select, Update]).where_(country.like("Canada"));
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn for_permission(for_crud_types: impl Into<ForArgs>) -> ForStart {
    ForStart(ForData {
        crud_types: for_crud_types.into().into(),
        condition: None,
        bindings: vec![],
    })
}

/// Builder struct for a `FOR` statement which is typeically used in DEFINE TABLE and DEFINE FIELD
/// statements for setting more granular permissions
#[derive(Clone, Debug)]
pub struct ForPermission(ForData);

impl Buildable for ForPermission {
    fn build(&self) -> String {
        let mut query = "FOR".to_string();
        if !&self.0.crud_types.is_empty() {
            query = format!(
                "{query} {}",
                &self
                    .0
                    .crud_types
                    .iter()
                    .map(|ct| { ct.to_string() })
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

impl Erroneous for ForPermission {}
impl Queryable for ForPermission {}

impl Parametric for ForPermission {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Display for ForPermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// Permission types which can be a a single For statement
/// list of `for` statements
#[derive(Clone, Debug)]
pub enum Permissions {
    /// Single `for` statement
    For(ForPermission),
    /// List of `for` statements
    Fors(Vec<ForPermission>),
    /// Single Raw statement
    RawStatement(Raw),
    /// List of Raw statements
    RawStatementList(Vec<Raw>),
}

impl ToRaw for Permissions {
    fn to_raw(self: &Permissions) -> Raw {
        match self {
            Permissions::For(for_one) => for_one.to_raw(),
            Permissions::Fors(for_many) => Raw::new(
                for_many
                    .iter()
                    .map(|f| f.to_raw().build())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            Permissions::RawStatement(r) => r.to_raw(),
            Permissions::RawStatementList(raw_list) => Raw::new(
                raw_list
                    .iter()
                    .map(|f| f.to_raw().build())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }
}

impl From<ForPermission> for Permissions {
    fn from(value: ForPermission) -> Self {
        Self::For(value)
    }
}

impl From<Vec<ForPermission>> for Permissions {
    fn from(value: Vec<ForPermission>) -> Self {
        Self::Fors(value)
    }
}

impl<const N: usize> From<&[ForPermission; N]> for Permissions {
    fn from(value: &[ForPermission; N]) -> Self {
        Self::Fors(value.to_vec())
    }
}

impl<const N: usize> From<[ForPermission; N]> for Permissions {
    fn from(value: [ForPermission; N]) -> Self {
        Self::Fors(value.to_vec())
    }
}

impl From<Raw> for Permissions {
    fn from(value: Raw) -> Self {
        Self::RawStatement(value)
    }
}

impl From<Vec<Raw>> for Permissions {
    fn from(value: Vec<Raw>) -> Self {
        Self::RawStatementList(value)
    }
}

impl From<&Vec<Raw>> for Permissions {
    fn from(value: &Vec<Raw>) -> Self {
        Self::RawStatementList(value.to_vec())
    }
}

impl<const N: usize> From<&[Raw; N]> for Permissions {
    fn from(value: &[Raw; N]) -> Self {
        Self::RawStatementList(value.to_vec())
    }
}

impl<const N: usize> From<[Raw; N]> for Permissions {
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

        let for_res = for_permission(CrudType::Create).where_(name.like("Oyelowo"));
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

        let for_res = for_permission([Create, Delete, Select, Update]).where_(name.is("Oyedayo"));
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
