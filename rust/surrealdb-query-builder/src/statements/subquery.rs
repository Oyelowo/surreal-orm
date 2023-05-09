use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    Binding, Buildable, Erroneous, ErrorList, Parametric, SurrealdbEdge, SurrealdbModel,
    SurrealdbNode, SurrealdbOrmError, Table, ToRaw, Valuex, BindingsList,
};

use super::{
    CreateStatement, DeleteStatement, IfStatement, InsertStatement, RelateStatement,
    SelectStatement, UpdateStatement,
};

// #[allow(missing_docs)]
// #[derive(Debug, Clone)]
// pub enum Subquery<M, N, E>
// where
//     M: SurrealdbModel + Serialize + DeserializeOwned,
//     N: SurrealdbNode + Serialize + DeserializeOwned,
//     E: SurrealdbEdge + Serialize + DeserializeOwned,
// {
//     Value(Valuex),
//     Ifelse(IfStatement),
//     // Output(OutputStatement),  // TODO. This is a Return statement
//     Select(SelectStatement),
//     Create(CreateStatement<N>),
//     Update(UpdateStatement<M>),
//     Delete(DeleteStatement<M>),
//     Relate(RelateStatement<E>),
//     Insert(InsertStatement<N>),
// }

#[derive(Debug, Clone)]
pub struct Subquery{
    query_string: String,
    bindings: BindingsList,
    errors: ErrorList
}

impl Buildable for Subquery {
    fn build(&self) -> String {
        self.query_string.to_owned()
    }
}

impl Parametric for Subquery {
    fn get_bindings(&self) -> Vec<Binding> {
        self.bindings.to_owned()
    }
}

impl Erroneous for Subquery {
    fn get_errors(&self) -> ErrorList {
        self.errors.to_owned()
    }
}

fn statement_str_to_subquery(statement: &str) -> Result<sql::Subquery, SurrealdbOrmError> {
    let query = sql::parse(statement).unwrap();
    let parsed_statement = query.0 .0.first().unwrap();
    let subquery = match parsed_statement {
        sql::Statement::Select(s) => sql::Subquery::Select(s.to_owned()),
        sql::Statement::Ifelse(s) => sql::Subquery::Ifelse(s.to_owned()),
        sql::Statement::Create(s) => sql::Subquery::Create(s.to_owned()),
        sql::Statement::Relate(s) => sql::Subquery::Relate(s.to_owned()),
        sql::Statement::Insert(s) => sql::Subquery::Insert(s.to_owned()),
        sql::Statement::Update(s) => sql::Subquery::Update(s.to_owned()),
        // sql::Statement::Value(s) => Subquery::Value(s.to_owned()),
        sql::Statement::Delete(s) => sql::Subquery::Delete(s.to_owned()),
        // sql::Statement::Relate(s) => Some(Subquery::Relate(s.to_owned())),
        // _ => panic!("Invalid subquery"),
        _ => return Err(SurrealdbOrmError::InvalidSubquery(statement.to_string())),
    };
    Ok(subquery)
}

impl From<SelectStatement> for Subquery {
    fn from(statement: SelectStatement) -> Self {
        let subquery = statement_str_to_subquery(&statement.to_raw().build()).unwrap();
        let binding = Binding::new(subquery);

        Self{
            query_string: binding.get_param_dollarised(),
            bindings: vec![binding],
                // Since we are making subqueries raw and parametizing it as a whole. Maybe, I
                // gathering the bindings from the subquery is not necessary.
                // bindings: vec![binding]
                //     .into_iter()
                //     .chain(statement.get_bindings())
                //     .collect(),
                errors: statement.get_errors(),
        }
    }
}

// impl From<Table> for Subquery {
//     fn from(value: Table) -> Self {
//         Self(Valuex {
//             string: value.to_string(),
//             bindings: vec![],
//         })
//     }
// }
//
// impl From<sql::Thing> for Subquery {
//     fn from(value: sql::Thing) -> Self {
//         let binding = Binding::new(value);
//         Self(Valuex {
//             string: binding.get_param_dollarised(),
//             bindings: vec![binding],
//         })
//     }
// }

