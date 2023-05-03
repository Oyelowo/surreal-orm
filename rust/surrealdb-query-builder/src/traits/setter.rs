use std::ops::Deref;

use serde::Serialize;
use surrealdb::sql;

use crate::{
    statements::LetStatement, Binding, BindingsList, Buildable, Conditional, Erroneous, ErrorList,
    Field, Operation, Param, Parametric, Valuex,
};

/// A helper struct for generating SQL update statements.
#[derive(Debug, Clone)]
pub struct Setter {
    query_string: String,
    bindings: BindingsList,
    errors: ErrorList,
}

// impl Parametric for Setter {
//     fn get_bindings(&self) -> BindingsList {
//         self.bindings.to_vec()
//     }
// }
//
// impl Buildable for Setter {
//     fn build(&self) -> String {
//         self.query_string.to_string()
//     }
// }

// impl Erroneous for Setter {
//     fn get_errors(&self) -> ErrorList {
//         self.errors.to_vec()
//     }
// }

impl std::fmt::Display for Setter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

struct SetArg {
    string: String,
    bindings: BindingsList,
    errors: ErrorList,
}

impl Buildable for SetArg {
    fn build(&self) -> String {
        self.string.to_string()
    }
}

impl Parametric for SetArg {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Erroneous for SetArg {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_vec()
    }
}

impl<T: Serialize> From<T> for SetArg {
    fn from(value: T) -> Self {
        let sql_value = sql::json(&serde_json::to_string(&value).unwrap()).unwrap();
        let binding = Binding::new(sql_value);

        Self {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        }
    }
}

impl From<Field> for SetArg {
    fn from(value: Field) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<Param> for SetArg {
    fn from(value: Param) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl From<LetStatement> for SetArg {
    fn from(value: LetStatement) -> Self {
        Self {
            string: value.get_param().build(),
            bindings: value.get_bindings(),
            errors: value.get_errors(),
        }
    }
}

impl<T: Into<Setter>> From<&T> for Setter {
    fn from(value: &T) -> Self {
        let setter: Setter = value.into();
        setter
    }
}

// impl Conditional for Setter {}
impl<T: Into<Setter>> Parametric for T {
    fn get_bindings(&self) -> BindingsList {
        let setter: Setter = self.into();
        setter.bindings.to_vec()
    }
}

impl<T> Buildable for T
where
    T: Into<Setter>,
{
    fn build(&self) -> String {
        let setter: Setter = self.into();
        setter.query_string.to_string()
    }
}
impl<T: Into<Setter>> Erroneous for T {
    fn get_errors(&self) -> ErrorList {
        let setter: Setter = self.into();
        setter.errors.to_vec()
    }
}
impl<T: Into<Setter>> Conditional for T {}

pub trait SetterAssignable<T: Serialize>
where
    Self: std::ops::Deref<Target = Field>,
{
    fn equal(&self, value: impl Into<T>) -> Setter {
        let operator = sql::Operator::Equal;
        let field = self.deref();
        let set_arg: SetArg = value.into().into();

        let column_updater_string = format!("{field} {operator} {}", set_arg.build());
        Setter {
            query_string: column_updater_string,
            bindings: set_arg.get_bindings(),
            errors: set_arg.get_errors(),
        }
    }

    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

type Test<T> = T;
struct Dayo<T>(Test<T>);

// struct Setter;
type Mana = Dayo<super::Setter>;
type Lowa = i32;

mod field_module {
    use super::SetterAssignable;
    use super::*;
    // use crate::Field;
    use surrealdb::sql;

    struct Setter;
    type Lowa = Dayo<super::Mana>;
    pub struct Lowo(pub(super) crate::Field);

    impl std::ops::DerefMut for Lowo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl std::ops::Deref for Lowo {
        type Target = Field;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl SetterAssignable<sql::Duration> for Lowo {}
}
// impl Setter<u8> for Lowo {}
fn rer() {
    let lowo = field_module::Lowo(Field::new("lowo"));
    // lowo.like(34);
    lowo.equal(std::time::Duration::from_secs(1));
    // lowo.equal(Field::new("lowo"));
    // lowo.equal(lowo.deref());

    // lowo.equal(45);
    // lowo.equals(LinkOne::from(Weapon::default()));
    // lowo.equals("dfdf");
    // lowo.equal(15);
    // lowo
    // lowo.equals(LinkOne::from(SpaceShip::default()));
    // lowo.equals(LinkOne::from(Weapon::default()));
}

impl Setter {
    // pub fn equal(&self, value: impl Into<sql::Value>) -> Self {
    //     let value: sql::Value = value.into();
    //     self.update_field(Operator::Equal, value)
    // }
    // // pub fn increment_by(&self, value: impl Into<sql::Number>) -> Self {
    //     let value: sql::Number = value.into();
    //     self.update_field(Operator::Inc, value)
    // }
    // pub fn append(&self, value: impl Into<sql::Value>) -> Self {
    //     self.update_field(Operator::Inc, value)
    // }
    // pub fn decrement_by(&self, value: impl Into<sql::Number>) -> Self {
    //     let value: sql::Number = value.into();
    //     self.update_field(Operator::Dec, value)
    // }
    // pub fn remove(&self, value: impl Into<sql::Value>) -> Self {
    //     self.update_field(Operator::Dec, value)
    // }
    // pub fn plus_equal(&self, value: impl Into<sql::Value>) -> Self {
    //     self.update_field(Operator::Inc, value)
    // }
    //
    // pub fn minus_equal(&self, value: impl Into<sql::Value>) -> Self {
    //     self.update_field(Operator::Dec, value)
    // }
    //
    // fn update_field(&self, operator: sql::Operator, value: impl Into<Valuex>) -> Updater {
    //     let value: Valuex = value.into();
    //     let column_updater_string = format!("{self} {operator} {}", value.build());
    //     Self {
    //         query_string: column_updater_string,
    //         bindings: value.get_bindings(),
    //     }
    // }
}
