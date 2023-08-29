use std::fmt::Display;

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

/// Contains metadata of initialized patch operations.
#[derive(Clone, Debug)]
pub struct PatchOpInit {
    path: String,
    op: OpType,
    value: Option<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

/// Contains metadata for patch operations.
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

/// Patchable trait
pub trait Patchable<T: Serialize>
where
    Self: std::ops::Deref<Target = Field>,
{
    /// Json Add patch operation
    /// The field acts as the path to the field to be patched. It automatically
    /// converts a field path to a valid json path to the field.
    /// So, `name` will become `/name` and `name.first` will become `/name/first`
    ///
    /// # Arguments
    ///
    /// value: The value to be patched
    ///
    /// # Examples
    /// ```rust, ignore
    /// name.patch_add("Oyelowo");
    /// ```
    fn patch_add(&self, value: impl Into<T>) -> PatchOp {
        patch_path(self.deref(), OpType::Add, value)
    }

    /// Json Remove patch operation
    /// The field acts as the path to the field to be patched. It automatically
    /// converts a field path to a valid json path to the field.
    /// So, `name` will become `/name` and `name.first` will become `/name/first`
    ///
    /// # Examples
    /// ```rust, ignore
    /// name.patch_remove();
    /// ```
    fn patch_remove(&self) -> PatchOp {
        let patch = patch(self.deref());
        PatchOp(PatchOpInit {
            op: OpType::Remove,
            ..patch
        })
    }

    /// Json Replace patch operation
    /// The field acts as the path to the field to be patched. It automatically
    /// converts a field path to a valid json path to the field.
    /// So, `name` will become `/name` and `name.first` will become `/name/first`
    ///
    /// # Arguments
    /// value: The value to be patched
    ///
    /// # Examples
    /// ```rust, ignore
    /// name.patch_replace("Oyelowo");
    /// ```
    fn patch_replace(&self, value: impl Into<T>) -> PatchOp {
        patch_path(self.deref(), OpType::Replace, value)
    }

    /// Json Change patch operation
    /// The field acts as the path to the field to be patched. It automatically
    /// converts a field path to a valid json path to the field.
    /// So, `name` will become `/name` and `name.first` will become `/name/first`
    ///
    /// # Arguments
    /// regex_match_and_replace_string: A string that matches a regex pattern and replaces it with the string
    ///
    /// # Examples
    /// ```rust, ignore
    /// // The below changes "test" to "text
    /// name.patch_change("@@ -1,4 +1,4 @@\n te\n-s\n+x\n t\n");
    /// ```
    fn patch_change(&self, regex_match_and_replace_string: &str) -> PatchOp {
        let binding = Binding::new(regex_match_and_replace_string);
        let patch = patch(self.deref());

        PatchOp(PatchOpInit {
            op: OpType::Change,
            value: Some(binding.get_param_dollarised()),
            bindings: patch.bindings.into_iter().chain(vec![binding]).collect(),
            ..patch
        })
    }

    /// Derefs to field
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

fn patch_path<T: Serialize>(field: &Field, operation_type: OpType, value: impl Into<T>) -> PatchOp {
    let (binding, errors) = derive_binding_and_errors_from_value(&value.into());
    let patch = patch(field);

    PatchOp(PatchOpInit {
        op: operation_type,
        value: Some(binding.get_param_dollarised()),
        bindings: patch.bindings.into_iter().chain(vec![binding]).collect(),
        errors,
        ..patch
    })
}

pub(crate) fn derive_binding_and_errors_from_value<T: Serialize>(
    value: &T,
) -> (Binding, Vec<String>) {
    let sql_value = sql::to_value(value);

    let (value, errors) = match sql_value {
        Ok(sql_value) => (sql_value, vec![]),
        Err(e) => (
            sql::Value::Null,
            vec![format!("Error: Unable to serialise value. \n{}", e)],
        ),
    };

    let binding = Binding::new(value);
    (binding, errors)
}

// Patch helper
fn patch(path: impl Into<Field>) -> PatchOpInit {
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
