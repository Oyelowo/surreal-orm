use crate::{
    count, Alias, AliasName, Aliasable, Binding, BindingsList, Buildable, Field, Function,
    Parametric, Table, ToRaw,
};

// pub trait Valuablex {
//     fn tona(self) -> Valuex;
// }
// impl<T: Into<sql::Value>> Parametric for T {
//     fn get_bindings(&self) -> BindingsList {
//         todo!()
//     }
// }
#[derive(Debug, Clone)]
pub struct Valuex {
    pub(crate) string: String,
    pub(crate) bindings: BindingsList,
}

impl Parametric for Valuex {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for Valuex {
    fn build(&self) -> String {
        self.string.to_string()
    }
}

// impl<T: Into<sql::Value>> Valuablex for T {
//     fn tona(self) -> Valuex {
//         let value: sql::Value = self.into();
//         let binding = Binding::new(value);
//
//         Valuex {
//             string: binding.get_param_dollarised(),
//             bindings: vec![binding],
//         }
//     }
// }

// impl<T: Into<sql::Values>> Mana for T {
//     fn tona(self) -> Oja {
//         let value: sql::Value = self.into();
//         let binding = Binding::new(value);
//
//         Oja {
//             string: value.to_string(),
//             bindings: vec![binding],
//         }
//     }
// }
// impl<T: Into<sql::Value>> Buildable for T {
//     fn build(&self) -> std::string::String {
//         let value: sql::Value = self.to_owned().into();
//         value.to_string()
//     }
// }
// impl Valuablex for Alias {
//     fn tona(self) -> Valuex {
//         Valuex {
//             string: self.build(),
//             bindings: self.get_bindings(),
//         }
//     }
// }
// impl Mana for std::time::Duration {
//     fn tona(self) -> String {
//         sql::Duration(self).to_string()
//     }
// }
impl From<Field> for Valuex {
    fn from(value: Field) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<Alias> for Valuex {
    fn from(value: Alias) -> Self {
        Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}
impl From<Function> for Valuex {
    fn from(value: Function) -> Self {
        Valuex {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

// impl Valuablex for Alias {
//     fn tona(self) -> Alias {
//         Valuex {
//             string: self.build(),
//             bindings: self.get_bindings(),
//         }
//     }
// }
// impl Valuablex for Field {
//     fn tona(self) -> Valuex {
//         Valuex {
//             string: self.build(),
//             bindings: self.get_bindings(),
//         }
//     }
// }
// impl Valuablex for Function {
//     fn tona(self) -> Valuex {
//         Valuex {
//             string: self.build(),
//             bindings: self.get_bindings(),
//         }
//     }
// }
pub fn val(val: impl Into<sql::Value>) -> sql::Value {
    val.into()
}

impl<T: Into<sql::Value>> From<T> for Valuex {
    fn from(value: T) -> Self {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);

        Valuex {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
        }
    }
}
#[macro_export]
macro_rules! arr {
    ($( $val:expr ),*) => {{
        vec![
            $( $crate::Valuex::from($val) ),*
        ]
    }};
}

pub use arr;
use surrealdb::sql;

#[test]
fn erer() {
    // let xx: sql::Value = sql::Idiom(vec![surrealdb::sql::Part::from(Ident::from("nana as po"))]).into();
    // let xx: sql::Value = sql::Idiom(vec![surrealdb::sql::Part::from(Ident::from(
    //     "count() AS pa",
    // ))])
    // .into();
    // let xx: sql::Value = sql::Idiom(vec![surrealdb::sql::Pjrt::Value(Ident::from(
    //     "count() AS pa",
    // ))])
    // .into();
    // xx.to_raw_string()
    // let xx: sql::Value = Ident::from("nana as po").into();
    // SELECT count() AS total, math::sum(age), gender, country FROM person GROUP BY gender, country;
    // assert_eq!(xx.as_raw_string(), "rere");
    let user = Table::new("user");
    let country = Field::new("country");
    let age = Field::new("age");
    let gender = Field::new("gender");
    let city = Field::new("city");
    let total = AliasName::new("total");
    let totall = AliasName::new("total");
    let mut mm = arr![
        1,
        2,
        3,
        count!().__as__(total),
        // math::sum!(age),
        gender,
        country,
        54,
        sql::Duration(std::time::Duration::from_secs(43))
    ];
    mm.push(34.into());
    // assert_eq!(count!().__as__(totall).tona(), "rere");
    assert_eq!(
        mm.into_iter()
            .map(|m| m.to_raw().build())
            // .map(|m| m.build())
            .collect::<Vec<_>>()
            .join(", "),
        "rere"
    );
}
