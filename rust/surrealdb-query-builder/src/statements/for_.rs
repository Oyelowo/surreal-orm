use std::fmt::{self, Display};

use crate::{
    traits::{
        Binding, BindingsList, Buildable, Conditional, Erroneous, ErrorList, Parametric, Queryable,
        Raw, Runnable, SurrealdbModel, ToRaw,
    },
    types::{expression::Expression, CrudType, Filter, Updateables},
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

pub struct ForStart(ForData);

impl ForStart {
    pub fn where_(mut self, condition: impl Conditional) -> For {
        let condition = Filter::new(condition);
        self.0.condition = Some(condition.clone());
        self.0.bindings.extend(condition.get_bindings());
        For(self.0)
    }
}

pub fn for_(for_crud_types: impl Into<ForArgs>) -> ForStart {
    ForStart(ForData {
        crud_types: for_crud_types.into().into(),
        condition: None,
        bindings: vec![],
    })
}

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

pub struct NONE;

#[derive(Clone)]
pub enum PermissionType {
    For(For),
    Fors(Vec<For>),
    RawStatement(Raw),
    RawStatementList(Vec<Raw>),
}

impl ToRaw for PermissionType {
    fn to_raw(self) -> Raw {
        match self {
            PermissionType::For(for_one) => for_one.to_raw(),
            PermissionType::Fors(for_many) => Raw::new(
                for_many
                    .into_iter()
                    .map(|f| f.to_raw().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            PermissionType::RawStatement(r) => r,
            PermissionType::RawStatementList(raw_list) => Raw::new(
                raw_list
                    .into_iter()
                    .map(|f| f.to_raw().to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        statements::{order, select},
        traits::Operatable,
        types::Field,
    };

    use super::*;

    #[test]
    fn test_define_for_statement_state_machine() {
        let name = Field::new("name");

        let for_res = for_(CrudType::Create).where_(name.like("Oyelowo"));
        assert_eq!(
            for_res.fine_tune_params(),
            "FOR create\n\tWHERE name ~ $_param_00000001".to_string()
        );
        assert_eq!(
            for_res.to_raw().to_string(),
            "FOR create\n\tWHERE name ~ 'Oyelowo'".to_string()
        );
        // insta::assert_display_snapshot!(for_res);
        // insta::assert_debug_snapshot!(for_res.get_bindings());
    }

    #[test]
    fn test_define_for_statement_state_machine_multiple() {
        use CrudType::*;
        let name = Field::new("name");

        let for_res = for_(&[Create, Delete, Select, Update]).where_(name.is("Oyedayo"));
        assert_eq!(
            for_res.fine_tune_params(),
            "FOR create, delete, select, update\n\tWHERE name IS $_param_00000001".to_string()
        );
        assert_eq!(
            for_res.to_raw().to_string(),
            "FOR create, delete, select, update\n\tWHERE name IS 'Oyedayo'".to_string()
        );
        // insta::assert_display_snapshot!(for_res);
        // insta::assert_debug_snapshot!(for_res.get_bindings());
    }
}
