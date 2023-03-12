use std::fmt::{self, Display};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql;

use crate::{db_field::Binding, query_select::SelectStatement, BindingsList, DbFilter, Parametric};

#[derive(Clone)]
enum Expression {
    SelectStatement(SelectStatement),
    // Value(T),
    Value(sql::Value),
}

//     I: Into<Expression
// impl<I, T> From<I> for ExpressionContent
//
//     I: Into<Expression >,
//     //
// {
//     fn from(value: I) -> Self {
//         // let expression: Expression  = self.into();
//         // ExpressionContent(format!("{expression}"))
//         todo!()
//     }
//     // fn into(self) -> ExpressionContent {
// }
// }

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            Expression::SelectStatement(s) => &format!("({s})"),
            // Expression::SelectStatement(s) => s.get_bindings().first().unwrap().get_raw(),
            Expression::Value(v) => {
                let bindings = self.get_bindings();
                assert_eq!(bindings.len(), 1);
                &format!("{}", self.get_bindings().first().expect("Param must have been generated for value. This is a bug. Please report here: ").get_param())
            }
        };
        write!(f, "{}", x)
    }
}

impl<T: Serialize + DeserializeOwned> Parametric for Expression {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Expression::SelectStatement(s) => s
                .get_bindings()
                .into_iter()
                // query must have already been built and bound
                .map(|b| b.with_raw(format!("({s})")))
                .collect::<_>(),
            Expression::Value(v) => {
                let sql_value = sql::json(&serde_json::to_string(&v).unwrap()).unwrap();
                vec![Binding::new(sql_value)]
            }
        }
    }
}

impl<T: Serialize + DeserializeOwned> From<SelectStatement> for Expression {
    fn from(value: SelectStatement) -> Self {
        Self::SelectStatement(value)
    }
}

// impl<T: Into<sql::Value>> From  for Expression {
impl<T: Serialize + DeserializeOwned> From for Expression {
    fn from(value: T) -> Self {
        Self::Value(value.into())
    }
}

fn if_(cond: bool) -> IfStatement {
    todo!()
}

fn test() {
    if_(true)
        .then(32)
        .else_if(false)
        .then(54)
        .else_if(true)
        .then(900)
        .else_(45);

    // if_then(true, i32)
    //     .else_if_then(true, 45)
    //     .else_if_then(true, 45)
    //     .else_(56);
}

pub struct ThenExpression {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ThenExpression {
    fn else_if(&mut self, cond: bool) -> ElseIfStatement {
        todo!()
    }

    fn else_(&mut self, expression: i32) -> String {
        todo!()
    }
}

pub struct ElseIfStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl ElseIfStatement {
    pub fn then(mut self, expression: i32) -> ThenExpression {
        todo!()
    }
}

struct ExpressionContent(String);

impl ExpressionContent {
    fn empty() -> Self {
        Self("".into())
    }
}

struct FlowStatementData {
    if_data: Flow,
    else_if_data: Vec<Flow>,
    else_data: ExpressionContent,
}
// enum FlowStatementData {
//     If(Flow),
//     ElseIfs(Vec<Flow>),
//     Else(ExpressionContent),
//     End,
// }

impl FlowStatementData {
    fn update_if(mut self, condition: DbFilter) -> Self {
        self.if_data.condition = condition;
        self
    }
}

struct Flow {
    condition: DbFilter,
    expression: ExpressionContent,
}

pub struct ElseStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl Parametric for IfStatement {
    fn get_bindings(&self) -> crate::BindingsList {
        todo!()
    }
}

pub struct IfStatement {
    flow_data: FlowStatementData,
    bindings: BindingsList,
}

impl IfStatement {
    pub fn new(condition: impl Into<DbFilter>) -> Self {
        let condition: DbFilter = condition.into();
        let bindings = condition.get_bindings();

        let flow_data = FlowStatementData {
            if_data: Flow {
                condition: condition.into(),
                expression: ExpressionContent::empty(),
            },
            else_if_data: Default::default(),
            else_data: ExpressionContent::empty(),
        };

        Self {
            flow_data,
            bindings,
        }
    }

    pub fn then(mut self, expression: impl Into<Expression>) -> ThenExpression {
        // self.flow_data.
        // let xx = FlowStatementData::If(Flow { condition: (), expression: () })
        //     let condition: DbFilter = condition.into();
        //     self.condition = format!("{}", condition);
        //     let then_expression: Expression  = then_expression.into();
        //     let param = match &then_expression {
        //         Expression::SelectStatement(s) => format!("({s})"),
        //         Expression::Value(v) => self
        //             .get_bindings()
        //             .first()
        //             .expect("Must have one binding")
        //             .get_raw()
        //             .to_string(),
        //     };
        //     // self.then_expression = then_expression.to_string();
        //     self.bindings.extend(condition.get_bindings());
        //     self.bindings.extend(then_expression.get_bindings());
        // self
        self.flow_data.if_data.expression = expression.into();
        ThenExpression {
            flow_data: self.flow_data,
            bindings: todo!(),
        }
    }

    // pub fn if_then<T: Serialize + DeserializeOwned>(
    //     mut self,
    //     condition: impl Into<DbFilter>,
    //     then_expression: impl Into<Expression >,
    // ) -> Self {
    //     let condition: DbFilter = condition.into();
    //     self.condition = format!("{}", condition);
    //     let then_expression: Expression  = then_expression.into();
    //     let param = match &then_expression {
    //         Expression::SelectStatement(s) => format!("({s})"),
    //         Expression::Value(v) => self
    //             .get_bindings()
    //             .first()
    //             .expect("Must have one binding")
    //             .get_raw()
    //             .to_string(),
    //     };
    //     // self.then_expression = then_expression.to_string();
    //     self.bindings.extend(condition.get_bindings());
    //     self.bindings.extend(then_expression.get_bindings());
    //     self
    // }
    //
    // pub fn else_if_then<T: Serialize + DeserializeOwned>(
    //     self,
    //     condition: impl Into<DbFilter>,
    //     then_expression: impl Into<Expression >,
    // ) -> Self {
    //     // Self {
    //     //     condition: condition.into().to_string(),
    //     //     then_expression: then_expression.to_string(),
    //     //     ..self
    //     // }
    //     todo!()
    // }
    //
    // pub fn else_<E>(mut self, expression: E) -> Self
    //
    //     E: ToString,
    // {
    //     self.else_expression = Some(expression.to_string());
    //     self
    // }
}

// impl fmt::Display for IfStatement {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "IF {} THEN\n\t{}", self.condition, self.then_expression)?;
//         for i in 0..self.else_if_conditions.len() {
//             write!(
//                 f,
//                 "\nELSE IF {} THEN\n\t{}",
//                 self.else_if_conditions[i], self.else_if_expressions[i]
//             )?;
//         }
//         if let Some(else_expr) = &self.else_expression {
//             write!(f, "\nELSE\n\t{}", else_expr)?;
//         }
//         write!(f, "\nEND")
//     }
// }
fn main() {
    // let statement = IfElseStatement::new()
    //     .if_then("$scope = 'admin'", "SELECT * FROM account")
    //     .else_if("$scope = 'user'", "SELECT * FROM $auth.account")
    //     .else_expr("[]")
    //     .build()
    //     .unwrap();

    // println!("{}", statement);
}
