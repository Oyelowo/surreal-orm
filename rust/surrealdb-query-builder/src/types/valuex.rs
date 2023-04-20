use surrealdb::sql;

use crate::{
    count, statements::SelectStatement, Alias, AliasName, Aliasable, Binding, BindingsList,
    Buildable, Field, Function, Param, Parametric, Table, ToRaw, NONE, NULL,
};

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

impl Parametric for Vec<Valuex> {
    fn get_bindings(&self) -> BindingsList {
        self.into_iter()
            .flat_map(|m| m.get_bindings())
            .collect::<Vec<_>>()
    }
}

impl Buildable for Valuex {
    fn build(&self) -> String {
        self.string.to_string()
    }
}

impl Buildable for Vec<Valuex> {
    fn build(&self) -> String {
        self.into_iter()
            .map(|m| m.build())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

// impl AsRef<Valuex> for Field {
//     fn as_ref(&self) -> &Valuex {
//         Valuex {
//             string: self.build(),
//             bindings: self.get_bindings(),
//         }
//     }
// }

impl From<&Field> for Valuex {
    fn from(value: &Field) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<Field> for Valuex {
    fn from(value: Field) -> Self {
        Self {
            string: value.build(),
            bindings: value.get_bindings(),
        }
    }
}

impl From<Param> for Valuex {
    fn from(value: Param) -> Self {
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

impl From<NULL> for Valuex {
    fn from(_value: NULL) -> Self {
        Valuex {
            string: "NULL".to_string(),
            bindings: vec![],
        }
    }
}

impl From<NONE> for Valuex {
    fn from(_value: NONE) -> Self {
        Valuex {
            string: "NONE".to_string(),
            bindings: vec![],
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

#[cfg(test)]
mod tests {
    use surrealdb::sql;

    use crate::{functions::math, *};

    #[test]
    fn erer() {
        let country = Field::new("country");
        let age = Field::new("age");
        let gender = Field::new("gender");
        let total = AliasName::new("total");
        let mut mm = arr![
            1,
            2,
            3,
            count!().__as__(total),
            math::sum!(age),
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
                .collect::<Vec<_>>()
                .join(", "),
            "1, 2, 3, count() AS total, math::sum(age), gender, country, 54, 43s, 34"
        );
    }
}
