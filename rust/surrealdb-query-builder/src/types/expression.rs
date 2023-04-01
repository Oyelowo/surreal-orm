#[derive(Clone)]
pub enum Expression {
    SelectStatement(SelectStatement),
    Value(sql::Value),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expression = match self {
            Expression::SelectStatement(s) => format!("({s})"),
            // Expression::SelectStatement(s) => s.get_bindings().first().unwrap().get_raw(),
            Expression::Value(v) => {
                let bindings = self.get_bindings();
                assert_eq!(bindings.len(), 1);
                format!("{}", self.get_bindings().first().expect("Param must have been generated for value. This is a bug. Please report here: ").get_param())
            }
        };
        write!(f, "{}", expression)
    }
}

impl Parametric for Expression {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Expression::SelectStatement(s) => s
                .get_bindings()
                .into_iter()
                // query must have already been built and bound
                .map(|b| b.with_raw(format!("({s})")))
                .collect::<_>(),
            Expression::Value(sql_value) => {
                // let sql_value = sql::json(&serde_json::to_string(&v).unwrap()).unwrap();
                let sql_value: sql::Value = sql_value.to_owned();
                vec![Binding::new(sql_value.clone()).with_raw(sql_value.to_raw_string())]
            }
        }
    }
}

impl From<SelectStatement> for Expression {
    fn from(value: SelectStatement) -> Self {
        Self::SelectStatement(value)
    }
}

impl<T: Into<sql::Value>> From<T> for Expression {
    fn from(value: T) -> Self {
        Self::Value(value.into())
    }
}
