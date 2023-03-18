use std::fmt::Display;

use surrealdb::sql;

use crate::{
    field::{Binding, Conditional},
    sql::Name,
    BindingsList, Erroneous, Operatable, Parametric,
};

pub struct Param {
    param: sql::Param,
    condition_query_string: String,
    bindings: BindingsList,
}

impl Erroneous for Param {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.condition_query_string)
    }
}

impl Conditional for Param {
    fn get_condition_query_string(&self) -> String {
        format!("{}", self.condition_query_string)
    }
}

impl Parametric for Param {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Param {
    pub fn new(param: impl Into<Name>) -> Self {
        let param: Name = param.into();
        let param = sql::Idiom::from(param);
        let param = sql::Param::from(param);
        let param_str = format!("{}", &param);

        Self {
            param: param.into(),
            condition_query_string: param_str,
            bindings: vec![].into(),
        }
    }
}

impl Operatable for Param {
    fn generate_query<T>(&self, operator: impl std::fmt::Display, value: T) -> Param
    where
        T: Into<sql::Value>,
    {
        let value: sql::Value = value.into();
        let binding = Binding::new(value);
        let condition = format!(
            "{} {} ${}",
            self.condition_query_string,
            operator,
            &binding.get_param()
        );
        let updated_bindings = self.__update_bindings(binding);

        Self {
            param: self.param.clone(),
            condition_query_string: condition,
            bindings: updated_bindings,
        }
    }
}
