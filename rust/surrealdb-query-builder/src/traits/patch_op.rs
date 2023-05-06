use std::{fmt::Display, ops::Deref};

use serde::Serialize;
use surrealdb::sql;

use crate::{Binding, BindingsList, Buildable, Erroneous, ErrorList, Field, Parametric};

#[derive(Clone, Debug)]
enum OpType {
    /// Adds values along the path using JSON patch operation
    Add,
    /// Removes values along the path using JSON patch operation
    Remove,
    /// Replaces values along the path using JSON patch operation
    Replace,
    /// Moves values along the path using JSON patch operation
    Change,
}
impl Display for OpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            OpType::Add => "add",
            OpType::Remove => "remove",
            OpType::Replace => "replace",
            OpType::Change => "change",
        };
        write!(f, "{}", op)
    }
}

#[derive(Clone, Debug)]
pub struct PatchOpInit {
    path: String,
    op: OpType,
    value: Option<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

///
pub struct PatchOp(PatchOpInit);

impl std::ops::Deref for PatchOp {
    type Target = PatchOpInit;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Buildable for PatchOp {
    fn build(&self) -> String {
        let mut patch = vec![];
        let path = self.path.to_string();
        let op = self.op.to_string();
        let value = self.value.clone();
        let value = match value {
            Some(value) => format!(", value: {}", value),
            None => "".to_string(),
        };
        patch.push(format!("{{ op: '{}', path: {}{} }}", op, path, value));
        patch.join(", ")
    }
}

impl From<PatchOp> for Vec<PatchOp> {
    fn from(patch_op: PatchOp) -> Self {
        vec![patch_op]
    }
}

impl Parametric for PatchOp {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Erroneous for PatchOp {
    fn get_errors(&self) -> ErrorList {
        self.0.errors.to_vec()
    }
}

pub trait Patchable<T: Serialize>
where
    Self: std::ops::Deref<Target = Field>,
{
    fn patch_add(&self, value: impl Into<T>) -> PatchOp {
        patch_path(self.deref(), OpType::Add, value)
    }

    fn patch_remove(&self) -> PatchOp {
        let patch = patch(self.deref());
        PatchOp(PatchOpInit {
            op: OpType::Remove,
            ..patch
        })
    }

    fn patch_replace(&self, value: impl Into<T>) -> PatchOp {
        patch_path(self.deref(), OpType::Replace, value)
    }

    fn patch_change(&self, value: &str) -> PatchOp {
        // patch_path(self.deref(), OpType::Change, value)
        // let sql_value = sql::json(&serde_json::to_string(&value.into()).unwrap()).unwrap();
        let binding = Binding::new(value);
        let field = self.deref();
        let patch = patch(field);

        PatchOp(PatchOpInit {
            op: OpType::Change,
            value: Some(binding.get_param_dollarised()),
            bindings: patch.bindings.into_iter().chain(vec![binding]).collect(),
            ..patch
        })
    }

    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

fn patch_path<T: Serialize>(field: &Field, operation_type: OpType, value: impl Into<T>) -> PatchOp {
    let sql_value = sql::json(&serde_json::to_string(&value.into()).unwrap()).unwrap();
    let binding = Binding::new(sql_value);
    // let field = field.deref();
    let patch = patch(field);

    PatchOp(PatchOpInit {
        op: operation_type,
        value: Some(binding.get_param_dollarised()),
        bindings: patch.bindings.into_iter().chain(vec![binding]).collect(),
        ..patch
    })
}

/// Json patch operation
/// # Arguments
/// * `path` - The path to the field to be patched. Use the same field operation
/// you use in the library for accessing top level or nested fields. It automatically
/// converts that to a valid json path to the field.
///
/// # Examples
/// ```
/// # use surrealdb_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::{patch}};
/// let ref name = Field::new("name");
/// let name_first = Field::new("name.first");
///
/// let patch_op = patch(name).add("Oyelowo");
/// let patch_op = patch(name_first).change("Oyelowo");
/// let patch_op = patch(name).replace("Oyelowo");
/// let patch_op = patch(name).remove();
/// ```
pub fn patch(path: impl Into<Field>) -> PatchOpInit {
    let path: Field = path.into();
    let path = path.build();
    let path = path.split('.').collect::<Vec<&str>>();
    // Check if any of the item in the array contains invalid identifier
    // i.e not start with aplhabet, contains only alphanumeric and underscore
    // if any of the item is invalid, return error
    // Must be e.g name, name.first, name.first.second, so that we can easily replace `.` with `/`
    let bad_path = path.iter().filter(|item| {
        item.starts_with(|c: char| !c.is_alphabetic() && c != '_')
            || item
                .chars()
                .any(|c: char| !(c.is_alphanumeric() || c == '_'))
    });

    let mut errors = vec![];
    if bad_path.count() > 0 {
        errors.push("The path you have provided is invalid. Make sure that there are no clauses or conditions included. Valid path include e.g name, name(E).first, name(E).first(E).second, etc.".to_string());
    }

    let path = format!("/{}", path.join("/"));
    let path_binding = Binding::new(sql::Value::from(path));

    PatchOpInit {
        path: path_binding.get_param_dollarised(),
        op: OpType::Add,
        value: None,
        bindings: vec![path_binding],
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn can_build_patch_operation() {
        let email = Field::new("_email");

        let patch_op = PatchOp(patch(email));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(patch_op.to_raw().build(), "{ op: 'add', path: '/_email' }");
    }

    #[test]
    fn gathers_errors_when_invalid_path_is_provided() {
        let email = Field::new("email[WHERE id = 1]");

        let patch_op = PatchOp(patch(email));

        assert_eq!(patch_op.get_errors().len(), 1);
        assert_eq!(
            patch_op.get_errors().first().unwrap(),
            "The path you have provided is invalid. \
            Make sure that there are no clauses or conditions included. Valid path include \
            e.g name, name(E).first, name(E).first(E).second, etc."
        );
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/email[WHERE id = 1]' }"
        );
    }

    #[test]
    fn gathers_error_when_clauses_uses() {
        get_invalid_paths(Field::new("name[WHERE id = 1]"));
        get_invalid_paths(Field::new("name[WHERE id = 1].first"));
        get_invalid_paths(Field::new("name[0]"));
        get_invalid_paths(Field::new("name[1]"));
        get_invalid_paths(Field::new("name[$]"));
        get_invalid_paths(Field::new("name[*]"));
        get_invalid_paths(Field::new("name->"));
        get_invalid_paths(Field::new("name->writes"));
        get_invalid_paths(Field::new("->writes"));
        get_invalid_paths(Field::new("->"));
        get_invalid_paths(Field::new("->->"));
        get_invalid_paths(Field::new("-"));
        get_invalid_paths(Field::new("_-"));
        get_invalid_paths(Field::new("-something"));
        get_invalid_paths(Field::new("name->writes->book"));
        get_invalid_paths(Field::new("->writes->book"));
        get_invalid_paths(Field::new("user:oye->write->blog:mars"));
        get_invalid_paths(Field::new(
            "->knows->person->(knows WHERE influencer = true)",
        ));
        get_invalid_paths(Field::new("5book"));
        get_invalid_paths(Field::new("-book_"));
        get_invalid_paths(Field::new("*book_"));
        get_invalid_paths(Field::new("$book_"));
        get_invalid_paths(Field::new("%book_"));
        get_invalid_paths(Field::new("&book_"));
        get_invalid_paths(Field::new("#book_"));
        get_invalid_paths(Field::new("@book_"));
        get_invalid_paths(Field::new("(book_"));
        get_invalid_paths(Field::new(")book_"));
        get_invalid_paths(Field::new("book*"));
        get_invalid_paths(Field::new("bo$ok"));
    }

    fn get_invalid_paths(field: Field) {
        let patch_op = PatchOp(patch(field));

        assert_eq!(patch_op.get_errors().len(), 1);
        assert_eq!(
            patch_op.get_errors().first().unwrap(),
            "The path you have provided is invalid. \
            Make sure that there are no clauses or conditions included. Valid path include \
            e.g name, name(E).first, name(E).first(E).second, etc."
        );
    }

    #[test]
    fn can_build_add_operation() {
        let name = Field::new("_name.first");

        let patch_op = PatchOp(patch(name));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/_name/first' }"
        );
    }

    #[test]
    fn can_build_change_operation() {
        let name = Field::new("name.first");

        let patch_op = PatchOp(patch(name));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/name/first' }"
        );
    }

    #[test]
    fn can_build_remove_operation() {
        let name = Field::new("name.first");

        let patch_op = PatchOp(patch(name));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/name/first' }"
        );
    }

    #[test]
    fn can_build_replace_operation() {
        let name = Field::new("name.first.title");

        let patch_op = PatchOp(patch(name));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/name/first/title' }"
        );
    }
}
