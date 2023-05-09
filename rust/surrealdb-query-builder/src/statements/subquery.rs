use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql;

use crate::{
    Binding, Buildable, Erroneous, ErrorList, Parametric, SurrealdbEdge, SurrealdbModel,
    SurrealdbNode, SurrealdbOrmError, Table, ToRaw, Valuex,
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
pub struct Subquery(Valuex);

impl Buildable for Subquery {
    fn build(&self) -> String {
        self.0.string.to_owned()
    }
}

impl Parametric for Subquery {
    fn get_bindings(&self) -> Vec<Binding> {
        self.0.bindings.to_owned()
    }
}

impl Erroneous for Subquery {
    fn get_errors(&self) -> ErrorList {
        // self.0.get.to_owned()
        vec![]
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

        Self(Valuex {
            string: binding.get_param_dollarised(),
            bindings: vec![binding]
            // Since we are making subqueries raw and parametizing it as a whole. Maybe, I
            // gathering the bindings from the subquery is not necessary.
            // bindings: vec![binding]
            //     .into_iter()
            //     .chain(statement.get_bindings())
            //     .collect(),
        })
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

// impl From

//
// strup    // Ifelse(IfelseStatement),
//     // Output(OutputStatement),
//     // Select(SelectStatement),
//     // Create(CreateStatement),
//     // Update(UpdateStatement),
//     // Delete(DeleteStatement),
//     // Relate(RelateStatement),
//     // Insert(InsertStatement),
//     // let subquery2 = sql::parse("SELECT * FROM user WHERE tags CONTAINS 'rust'").unwrap();
//     let subquery2 = sql::parse("SELECT * FROM user WHERE tags CONTAINS $p2").unwrap();
//     let subquery2 = subquery2.0 .0.first().unwrap();
//     let q2 = match subquery2 {
//         sql::Statement::Select(s) => Some(Subquery::Select(s.to_owned())),
//         sql::Statement::Ifelse(s) => Some(Subquery::Ifelse(s.to_owned())),
//         sql::Statement::Create(s) => Some(Subquery::Create(s.to_owned())),
//         sql::Statement::Relate(s) => Some(Subquery::Relate(s.to_owned())),
//         sql::Statement::Insert(s) => Some(Subquery::Insert(s.to_owned())),
//         sql::Statement::Update(s) => Some(Subquery::Update(s.to_owned())),
//         sql::Statement::Output(s) => Some(Subquery::Output(s.to_owned())),
//         sql::Statement::Delete(s) => Some(Subquery::Delete(s.to_owned())),
//         // sql::Statement::Relate(s) => Some(Subquery::Relate(s.to_owned())),
//         _=> None
//     };    // Ifelse(IfelseStatement),
//     // Output(OutputStatement),
//     // Select(SelectStatement),
//     // Create(CreateStatement),
//     // Update(UpdateStatement),
//     // Delete(DeleteStatement),
//     // Relate(RelateStatement),
//     // Insert(InsertStatement),
//     // let subquery2 = sql::parse("SELECT * FROM user WHERE tags CONTAINS 'rust'").unwrap();
//     let subquery2 = sql::parse("SELECT * FROM user WHERE tags CONTAINS $p2").unwrap();
//     let subquery2 = subquery2.0 .0.first().unwrap();
//     let q2 = match subquery2 {
//         sql::Statement::Select(s) => Some(Subquery::Select(s.to_owned())),
//         sql::Statement::Ifelse(s) => Some(Subquery::Ifelse(s.to_owned())),
//         sql::Statement::Create(s) => Some(Subquery::Create(s.to_owned())),
//         sql::Statement::Relate(s) => Some(Subquery::Relate(s.to_owned())),
//         sql::Statement::Insert(s) => Some(Subquery::Insert(s.to_owned())),
//         sql::Statement::Update(s) => Some(Subquery::Update(s.to_owned())),
//         sql::Statement::Output(s) => Some(Subquery::Output(s.to_owned())),
//         sql::Statement::Delete(s) => Some(Subquery::Delete(s.to_owned())),
//         // sql::Statement::Relate(s) => Some(Subquery::Relate(s.to_owned())),
//         _=> None
//     };
