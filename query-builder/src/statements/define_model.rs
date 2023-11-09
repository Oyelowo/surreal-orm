use crate::{
    statements::Permissions, BindingsList, Buildable, Erroneous, Parametric, Queryable, StrandLike,
};
use std::fmt::{self, Display};

/// DEFINE MODEL statement
/// The DEFINE MODEL statement allows you to define a machine learning model in the database, specifying its name, version, and permissions.
///
/// Requirements
/// You must be authenticated as a root, namespace, or database user before you can use the DEFINE MODEL statement.
/// You must select your namespace and database before you can use the DEFINE MODEL statement.
/// Statement syntax
/// DEFINE MODEL ml::@name<@version>
/// 	[ PERMISSIONS @permissions ]

/// A statement for defining a ML model.
#[derive(Clone, Debug)]
pub struct DefineModelStatement {
    model_name: String,
    version: String,
    permissions_none: Option<bool>,
    permissions_full: Option<bool>,
    permissions_for: Vec<String>,
    bindings: BindingsList,
}

/// Define a new ML model.
/// The DEFINE MODEL statement allows you to define a machine learning model in the database.
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::define_model};
///
/// # let name = Field::new("name");
/// # let age = Field::new("age");
///
/// let statement = define_model("recommendation")
///     .version("v1.2.3")
///     // Additional permission chaining accumulates
///     .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
///     .permissions(for_permission([Create, Update]).where_(name.is("Oyedayo"))) // Multiple
///     // Multiples multples
///     .permissions([
///         for_permission([Create, Delete]).where_(name.is("Oyedayo")),
///         for_permission(Update).where_(age.less_than_or_equal(130)),
///     ]);
///
/// assert!(!statement.build().is_empty());
/// ```
pub fn define_model(name: impl Into<StrandLike>) -> DefineModelStatement {
    let name: StrandLike = name.into();

    DefineModelStatement {
        model_name: name.build(),
        version: String::new(),
        permissions_none: None,
        permissions_full: None,
        permissions_for: vec![],
        bindings: name.get_bindings(),
    }
}

impl DefineModelStatement {
    /// Set the version of the model.
    pub fn version(mut self, version: impl Into<StrandLike>) -> Self {
        let version: StrandLike = version.into();
        self.version = version.build();
        self.bindings.extend(version.get_bindings());
        self
    }

    /// set no permission.
    pub fn permissions_none(mut self) -> Self {
        self.permissions_none = Some(true);
        self
    }

    /// set full permission.
    pub fn permissions_full(mut self) -> Self {
        self.permissions_full = Some(true);
        self
    }

    /// set specific permissions for specific event type inluding CREATE, UPDATE, SELECT and DELETE.
    /// Additional permission chaining accumulates
    ///  Examples:
    ///  
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, statements::define_model};
    /// use surreal_orm::CrudType::*;
    /// use surreal_orm::statements::for_permission;
    ///
    ///    # let model_name = "recommendation";
    ///    # let age = Field::new("age");
    ///    # let name = Field::new("name");
    ///    # let statement = define_model(model_name)
    ///    #    .version("v1.2.3");
    ///
    /// // You can create perimssion for a single event
    /// let statement = statement.permissions(for_permission(Select).where_(age.greater_than_or_equal(18)));
    ///
    /// // Even multiple
    /// let statement = statement.permissions(for_permission([Create, Update]).where_(name.is("Oyedayo")));
    ///
    /// // Multiples multples
    /// let statement = statement.permissions([
    ///    for_permission([Create, Delete]).where_(name.is("Oyedayo")),
    ///    for_permission(Update).where_(age.less_than_or_equal(130)),
    /// ]);
    /// ```
    pub fn permissions(mut self, fors: impl Into<Permissions>) -> Self {
        use Permissions::*;
        let fors: Permissions = fors.into();
        match fors {
            For(one) => {
                self.permissions_for.push(one.build());
                self.bindings.extend(one.get_bindings());
            }
            Fors(many) => many.iter().for_each(|f| {
                self.permissions_for.push(f.build());
                self.bindings.extend(f.get_bindings());
            }),
            RawStatement(raw) => {
                self.permissions_for.push(raw.build());
                self.bindings.extend(raw.get_bindings());
            }
            RawStatementList(raw_list) => {
                self.permissions_for.extend(
                    raw_list
                        .into_iter()
                        .map(|r| {
                            self.bindings.extend(r.get_bindings());
                            r.build()
                        })
                        .collect::<Vec<_>>(),
                );
            }
        }
        self
    }
}

impl Queryable for DefineModelStatement {}

impl Erroneous for DefineModelStatement {}

impl Parametric for DefineModelStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for DefineModelStatement {
    fn build(&self) -> String {
        let mut query = format!("DEFINE MODEL ml::{}", &self.model_name);

        if !self.version.is_empty() {
            query = format!("{query}<{version}>", version = self.version);
        }

        if let Some(true) = self.permissions_none {
            query = format!("{query} PERMISSIONS NONE");
        } else if let Some(true) = self.permissions_full {
            query = format!("{query} PERMISSIONS FULL");
        } else if !&self.permissions_for.is_empty() {
            query = format!("{query}\nPERMISSIONS\n{}", self.permissions_for.join("\n"));
        }
        query.push(';');

        query
    }
}

impl Display for DefineModelStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{statements::for_permission, CrudType::*};
    use crate::{Field, Operatable, ToRaw};

    #[test]
    fn test_define_model_statement_full() {
        let model_name = "recommendation";
        let age = Field::new("age");
        let name = Field::new("name");

        let statement = define_model(model_name)
            .version("v1.2.3")
            .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
            .permissions(for_permission([Create, Update]).where_(name.is("Oyedayo"))) //Multiple
            .permissions([
                for_permission([Create, Delete]).where_(name.is("Oyedayo")),
                for_permission(Update).where_(age.less_than_or_equal(130)),
            ]);

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE MODEL ml::recommendation<v1.2.3>\n\
                PERMISSIONS\n\
                FOR select\n\tWHERE age >= $_param_00000001\n\
                FOR create, update\n\tWHERE name IS $_param_00000002\n\
                FOR create, delete\n\tWHERE name IS $_param_00000003\n\
                FOR update\n\tWHERE age <= $_param_00000004;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE MODEL ml::recommendation<v1.2.3>\n\
                PERMISSIONS\n\
                FOR select\n\tWHERE age >= 18\n\
                FOR create, update\n\tWHERE name IS 'Oyedayo'\n\
                FOR create, delete\n\tWHERE name IS 'Oyedayo'\n\
                FOR update\n\tWHERE age <= 130;"
        );
        insta::assert_display_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 4);
    }

    #[test]
    fn test_define_model_statement_simple() {
        let model_name = "recommendation";
        let statement = define_model(model_name);

        assert_eq!(statement.build(), "DEFINE MODEL ml::recommendation;");
        insta::assert_display_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 0);
    }

    #[test]
    fn test_define_model_statement_version() {
        let model_name = "recommendation";
        let statement = define_model(model_name).version("v1.2.3");

        assert_eq!(
            statement.build(),
            "DEFINE MODEL ml::recommendation<v1.2.3>;"
        );
        insta::assert_display_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 0);
    }
}
