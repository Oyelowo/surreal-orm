use std::{fmt::Display, marker::PhantomData, time::Duration};

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql::{self, Operator};

use crate::{
    db_field::Binding,
    query_insert::{Buildable, Runnable},
    query_select::SelectStatement,
    value_type_wrappers::SurrealId,
    BindingsList, DbField, Parametric, SurrealdbEdge,
};

// RELATE @from -> @table -> @with
// 	[ CONTENT @value
// 	  | SET @field = @value ...
// 	]
// 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
// 	[ TIMEOUT @duration ]
// 	[ PARALLEL ]
// ;

// Student::with(None|id|Query).writes.(Book::id|None|Query);
trait Relationable {}

enum Relatables<T>
where
    T: Serialize + DeserializeOwned,
{
    None,
    SurrealId(SurrealId),
    SelectStatement(SelectStatement<T>),
}

fn relate<T>(relatables: impl Into<Relatables<T>>)
where
    T: Serialize + DeserializeOwned,
{
}

#[derive(Debug)]
pub enum ReturnType {
    None,
    Before,
    After,
    Diff,
    Projections(Vec<&'static str>),
}

pub struct RelateStatement<T>
where
    T: Serialize + DeserializeOwned,
{
    from: Option<String>,
    table: Option<String>,
    with: Option<String>,
    relation: String,
    content_param: Option<String>,
    set: Option<Vec<(String, String)>>,
    return_type: Option<ReturnType>,
    timeout: Option<Duration>,
    parallel: bool,
    bindings: BindingsList,
    __return_type: PhantomData<T>,
}

impl<T> RelateStatement<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new() -> Self {
        RelateStatement {
            from: None,
            relation: "".into(),
            table: None,
            with: None,
            content_param: None,
            set: None,
            return_type: None,
            timeout: None,
            parallel: false,
            __return_type: PhantomData,
            bindings: vec![],
        }
    }

    pub fn relations(mut self, connection: impl Parametric + Display) -> Self {
        self.relation = connection.to_string();
        self.bindings.extend(connection.get_bindings());
        self
    }

    pub fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    pub fn with(mut self, with: &str) -> Self {
        self.with = Some(with.to_string());
        self
    }

    pub fn content(mut self, content: impl SurrealdbEdge + Serialize) -> Self {
        let xx = sql::json(&serde_json::to_string(&content).unwrap()).unwrap();
        let x = Binding::new(xx);
        self.content_param = Some(x.get_param().to_owned());
        self.bindings.push(x);
        self
    }

    pub fn set(mut self, field: &str, value: &str) -> Self {
        let set_vec = match self.set {
            Some(mut v) => {
                v.push((field.to_string(), value.to_string()));
                v
            }
            None => vec![(field.to_string(), value.to_string())],
        };
        self.set = Some(set_vec);
        self
    }

    pub fn return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }

    pub fn build(&self) -> String {
        let mut query = String::new();

        if !&self.relation.is_empty() {
            query += &format!("RELATE {} -> ", self.relation);
        }

        if let Some(content) = &self.content_param {
            query += &format!("CONTENT ${} ", content);
        }

        if let Some(set) = &self.set {
            query += "SET ";
            let set_vec = set
                .iter()
                .map(|(field, value)| format!("{} = {}", field, value))
                .collect::<Vec<String>>()
                .join(", ");
            query += &set_vec;
            query += " ";
        }

        if let Some(return_type) = &self.return_type {
            query += "RETURN ";
            match return_type {
                ReturnType::None => query += "NONE ",
                ReturnType::Before => query += "BEFORE ",
                ReturnType::After => query += "AFTER ",
                ReturnType::Diff => query += "DIFF ",
                ReturnType::Projections(projections) => {
                    let projections = projections
                        .iter()
                        .map(|p| format!("{}", p))
                        .collect::<Vec<String>>()
                        .join(", ");
                    query += &projections;
                    query += " ";
                }
            }
        }

        if let Some(timeout) = &self.timeout {
            query += &format!("TIMEOUT {} ", timeout.as_millis());
        }

        if self.parallel {
            query += "PARALLEL ";
        }

        query += ";";

        query
    }
}

impl<T> std::fmt::Display for RelateStatement<T>
where
    T: Serialize + DeserializeOwned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T> Parametric for RelateStatement<T>
where
    T: Serialize + DeserializeOwned,
{
    fn get_bindings(&self) -> crate::BindingsList {
        todo!()
    }
}

impl<T> Buildable for RelateStatement<T>
where
    T: Serialize + DeserializeOwned,
{
    fn build(&self) -> String {
        format!("{self}")
    }
}

impl<T: Serialize + DeserializeOwned> Runnable<T> for RelateStatement<T> {}

#[test]
fn test_query_builder() {
    let query = RelateStatement::<i32>::new()
        .from("from")
        .table("table")
        .with("with")
        // .content("content")
        .set("field1", "value1")
        .set("field2", "value2")
        .return_type(ReturnType::Projections(vec!["projection1", "projection2"]))
        .timeout(Duration::from_secs(30))
        .parallel()
        .build();

    assert_eq!(
        query,
        "RELATE from -> table -> with CONTENT content SET field1 = value1, field2 = value2 RETURN projection1, projection2 TIMEOUT 30000 PARALLEL ;"
    );
}
