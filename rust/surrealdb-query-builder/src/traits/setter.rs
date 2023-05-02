use serde::Serialize;
use surrealdb::sql;

use crate::{
    statements::LetStatement, Binding, BindingsList, Buildable, Field, Param, Parametric, Valuex,
};

/// A helper struct for generating SQL update statements.
#[derive(Debug, Clone)]
pub struct Setter {
    query_string: String,
    bindings: BindingsList,
}

impl Parametric for Setter {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

/// A helper struct for generating setters used in various statements
pub fn updater(field: impl Into<Field>) -> Setter {
    let field: Field = field.into();
    Setter {
        query_string: field.build(),
        bindings: field.get_bindings(),
    }
}

impl Buildable for Setter {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}

impl std::fmt::Display for Setter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// Things that can be updated
// pub enum Updateables {
//     /// Single updater
//     Updater(Updater),
//     /// Multiple updaters
//     Updaters(Vec<Updater>),
// }
//
// impl From<Updater> for Updateables {
//     fn from(value: Updater) -> Self {
//         Self::Updater(value)
//     }
// }
//
// impl Parametric for Updateables {
//     fn get_bindings(&self) -> BindingsList {
//         match self {
//             Updateables::Updater(up) => up.get_bindings(),
//             Updateables::Updaters(ups) => ups
//                 .into_iter()
//                 .flat_map(|u| u.get_bindings())
//                 .collect::<Vec<_>>(),
//         }
//     }
// }

// enum SetArg<T: Serialize> {
//     Value(T),
//     Field(Field),
// }
struct SetArg(Valuex);

impl<T: Serialize> From<T> for SetArg {
    fn from(value: T) -> Self {
        let sql_value = sql::json(&serde_json::to_string(&value).unwrap()).unwrap();
        let binding = Binding::new(sql_value);
        Self(Valuex {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
        })
    }
}

impl From<Field> for SetArg {
    fn from(value: Field) -> Self {
        todo!()
    }
}

impl From<Param> for SetArg {
    fn from(value: Param) -> Self {
        todo!()
    }
}

impl From<LetStatement> for SetArg {
    fn from(value: LetStatement) -> Self {
        todo!()
    }
}

// impl<T: Into<Field>> From<T> for SetArg {}

// trait SetterAssignable<T: Serialize + Into<Valuex>>
trait SetterAssignable<T: Serialize>
where
    Self: std::ops::Deref<Target = Field>,
{
    fn equal(&self, value: impl Into<T>) -> Setter {
        // let value: Valuex = value.into().into();
        let operator = sql::Operator::Equal;
        let field = self.deref();

        let value: Valuex = value.into();
        let column_updater_string = format!("{field} {operator} {}", value.build());
        Setter {
            query_string: column_updater_string,
            bindings: value.get_bindings(),
        }
    }

    // fn update_field(&self, operator: sql::Operator, value: impl Into<Valuex>) -> Updater {
    //     let value: Valuex = value.into();
    //     let column_updater_string = format!("{self} {operator} {}", value.build());
    //     Self {
    //         query_string: column_updater_string,
    //         bindings: value.get_bindings(),
    //     }
    // }
}

struct Lowo(Field);

impl std::ops::Deref for Lowo {
    type Target = Field;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SetterAssignable<sql::Duration> for Lowo {}
// impl Setter<u8> for Lowo {}
fn rer() {
    let lowo = Lowo(Field::new("lowo"));
    lowo.like(34);
    lowo.equal(std::time::Duration::from_secs(1));

    // lowo.equals(45);
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
