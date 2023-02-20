use std::collections::HashMap;

use serde::Serialize;

pub struct InsertStatement<T: Serialize> {
    table: String,
    values: Vec<T>,
}

impl<T: Serialize> InsertStatement<T> {
    pub fn new(table: String) -> Self {
        Self {
            table,
            values: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn insert_all(&mut self, values: Vec<T>) {
        self.values = values;
    }

    pub fn build(&self) -> Result<(String, Vec<(String, String)>), String> {
        if self.values.is_empty() {
            return Err(String::from("No values to insert"));
        }

        let first_value = self.values.get(0).unwrap();
        let field_names = get_field_names(first_value);
        let mut placeholders = String::new();

        for i in 1..=field_names.len() {
            if i > 1 {
                placeholders.push_str(", ");
            }
            placeholders.push('$');
            placeholders.push_str(&i.to_string());
        }

        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(&self.table);
        query.push_str(" (");
        query.push_str(&field_names.join(", "));
        query.push_str(") VALUES ");

        let mut variables = Vec::new();
        let mut values = String::new();

        for (i, value) in self.values.iter().enumerate() {
            let mut row_values = Vec::new();
            for field_name in &field_names {
                let field_value = get_field_value(value, field_name)?;
                let variable_name = format!("{}_{}", field_name, i);
                variables.push((variable_name, field_value));
                row_values.push(format!("${}", variables.len()));
            }
            if i > 0 {
                values.push_str(", ");
            }
            values.push_str("(");
            values.push_str(&row_values.join(", "));
            values.push_str(")");
        }

        query.push_str(&values);

        Ok((query, variables))
    }
}

// fn get_field_names<T>(value: &T) -> Vec<String>
// where
//     T: serde::Serialize,
// {
//     let object = serde_json::to_value(value)?
//         .as_object()
//         .ok_or("Not an object")?;
//     object.keys().map(|key| key.to_string()).collect()
// }
fn get_field_names<T>(value: &T) -> Vec<String>
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
        .unwrap()
        .as_object()
        .unwrap()
        .keys()
        .map(|key| key.to_string())
        .collect()
}

fn get_field_value<T>(value: &T, field_name: &str) -> Result<String, String>
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
        .unwrap()
        .as_object()
        .unwrap()
        .get(field_name)
        .map(|field_value| field_value.to_string())
        .ok_or(format!("Field '{}' not found in struct", field_name))
}

#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Company {
    name: String,
    founded: String,
    founders: Vec<Person>,
    tags: Vec<String>,
}
fn ere() {
    let companies = vec![
        Company {
            name: "Acme Inc.".to_string(),
            founded: "1967-05-03".to_string(),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
        },
        Company {
            name: "Apple Inc.".to_string(),
            founded: "1967-05-03".to_string(),
            founders: vec![
                Person {
                    name: "John Doe".to_string(),
                },
                Person {
                    name: "Jane Doe".to_string(),
                },
            ],
            tags: vec!["foo".to_string(), "bar".to_string()],
        },
    ];
}
// use std::collections::HashMap;
//
// struct InsertQuery {
//     table: String,
//     fields: Option<Vec<String>>,
//     values: Vec<Vec<String>>,
//     on_duplicate_key_update: Option<HashMap<String, String>>,
// }
//
// impl InsertQuery {
//     fn new(table: &str) -> InsertQuery {
//         InsertQuery {
//             table: String::from(table),
//             fields: None,
//             values: Vec::new(),
//             on_duplicate_key_update: None,
//         }
//     }
//
//     fn fields(mut self, fields: &[&str]) -> InsertQuery {
//         self.fields = Some(fields.iter().map(|f| String::from(*f)).collect());
//         self
//     }
//
//     fn values(mut self, values: &[&[&str]]) -> InsertQuery {
//         self.values = values
//             .iter()
//             .map(|v| v.iter().map(|s| String::from(*s)).collect())
//             .collect();
//         self
//     }
//
//     fn on_duplicate_key_update(mut self, updates: &[(&str, &str)]) -> InsertQuery {
//         let update_map: HashMap<String, String> = updates
//             .iter()
//             .map(|(k, v)| (String::from(*k), String::from(*v)))
//             .collect();
//         self.on_duplicate_key_update = Some(update_map);
//         self
//     }
//
//     fn build(&self) -> String {
//         let mut query = String::from("INSERT INTO ");
//         query.push_str(&self.table);
//
//         if let Some(fields) = &self.fields {
//             let fields_str = fields.join(", ");
//             query.push_str(&format!(" ({}) ", fields_str));
//         }
//
//         if !self.values.is_empty() {
//             let values_str: Vec<String> = self
//                 .values
//                 .iter()
//                 .map(|v| {
//                     let values_list = v.join(", ");
//                     format!("({})", values_list)
//                 })
//                 .collect();
//
//             query.push_str(" VALUES ");
//             query.push_str(&values_str.join(", "));
//         }
//
//         if let Some(update_map) = &self.on_duplicate_key_update {
//             let updates_str: Vec<String> = update_map
//                 .iter()
//                 .map(|(k, v)| format!("{} = {}", k, v))
//                 .collect();
//
//             query.push_str(" ON DUPLICATE KEY UPDATE ");
//             query.push_str(&updates_str.join(", "));
//         }
//
//         query.push_str(";");
//
//         query
//     }
// }
// mod xfdf {
//
//     use std::collections::HashMap;
//
//     use serde::{Deserialize, Serialize};
//
//     #[derive(Serialize)]
//     #[serde(rename_all = "snake_case")]
//     enum Value<'a> {
//         Str(&'a str),
//         Struct(HashMap<&'a str, Value<'a>>),
//         Array(Vec<Value<'a>>),
//     }
//
//     impl<'a> Value<'a> {
//         fn from_serde_value(v: &'a serde_json::Value) -> Self {
//             match v {
//                 serde_json::Value::String(s) => Value::Str(s),
//                 serde_json::Value::Array(a) => {
//                     let v: Vec<Value<'a>> = a.iter().map(Value::from_serde_value).collect();
//                     Value::Array(v)
//                 }
//                 serde_json::Value::Object(o) => {
//                     let v: HashMap<&'a str, Value<'a>> = o
//                         .iter()
//                         .map(|(k, v)| (k.as_str(), Value::from_serde_value(v)))
//                         .collect();
//                     Value::Struct(v)
//                 }
//                 _ => unreachable!(),
//             }
//         }
//
//         fn to_sql_value(&self) -> String {
//             match self {
//                 Value::Str(s) => s.to_string(),
//                 Value::Struct(fields) => {
//                     let fields = fields
//                         .iter()
//                         .map(|(k, v)| format!("{}: {}", k, v.to_sql_value()))
//                         .collect::<Vec<String>>()
//                         .join(", ");
//                     format!("{{{}}}", fields)
//                 }
//                 Value::Array(values) => {
//                     let values = values
//                         .iter()
//                         .map(Value::to_sql_value)
//                         .collect::<Vec<String>>()
//                         .join(", ");
//                     format!("[{}]", values)
//                 }
//             }
//         }
//     }
//
//     pub struct InsertQuery<'a> {
//         table: &'a str,
//         fields: Vec<&'a str>,
//         values: Vec<Value<'a>>,
//         on_duplicate_key_update: Vec<(&'a str, &'a str)>,
//     }
//
//     impl<'a> InsertQuery<'a> {
//         pub fn new(table: &'a str) -> Self {
//             Self {
//                 table,
//                 fields: Vec::new(),
//                 values: Vec::new(),
//                 on_duplicate_key_update: Vec::new(),
//             }
//         }
//
//         pub fn fields(&mut self, fields: &'a [&'a str]) -> &mut Self {
//             self.fields = fields.to_vec();
//             self
//         }
//
//         // pub fn values(&mut self, values: &'a [serde_json::Value]) -> &mut Self {
//         //     self.values = values.iter().map(Value::from_serde_value).collect();
//         //     self
//         // }
//         pub fn values<T: Serialize>(&mut self, values: &'a [T]) -> &mut Self {
//             self.values = values
//                 .iter()
//                 .map(|v| Value::from_serde_value(&serde_json::to_value(v).unwrap()))
//                 .collect();
//             self
//         }
//
//         pub fn on_duplicate_key_update(&mut self, fields: &'a [(&'a str, &'a str)]) -> &mut Self {
//             self.on_duplicate_key_update = fields.to_vec();
//             self
//         }
//
//         pub fn build(&self) -> String {
//             let fields = self.fields.join(", ");
//             let values = self
//                 .values
//                 .iter()
//                 .map(Value::to_sql_value)
//                 .collect::<Vec<String>>()
//                 .join(", ");
//             let mut sql = format!(
//                 "INSERT INTO {} ({}) VALUES ({})",
//                 self.table, fields, values
//             );
//
//             if !self.on_duplicate_key_update.is_empty() {
//                 let update_fields = self
//                     .on_duplicate_key_update
//                     .iter()
//                     .map(|(k, v)| format!("{} = {}", k, v))
//                     .collect::<Vec<String>>()
//                     .join(", ");
//                 sql.push_str(&format!(" ON DUPLICATE KEY UPDATE {}", update_fields));
//             }
//
//             sql
//         }
//     }
//
//     #[derive(Serialize, Deserialize, Debug)]
//     struct Founder {
//         person: String,
//     }
//
//     #[derive(Serialize, Deserialize, Debug)]
//     struct Company {
//         name: String,
//         founded: String,
//         founders: Vec<Founder>,
//         tags: Vec<String>,
//     }
//
//     #[derive(Serialize, Deserialize)]
//     struct Person {
//         name: String,
//         age: u8,
//     }
//
//     #[test]
//     fn test_surrealdb_insert() {
//         // Create a new `Person` instance
//         let person = Person {
//             name: "Alice".to_owned(),
//             age: 25,
//         };
//
//         let person = Person {
//             name: "Alice".to_owned(),
//             age: 25,
//         };
//
//         let mut insert_query = InsertQuery::new("person");
//         insert_query.fields(&["name", "age"]);
//         insert_query.values(&[Value::from(person)]);
//         // Create an `InsertQuery` builder and set the table name, fields, and values
//         let mut insert_query = InsertQuery::new("person");
//         insert_query.fields(&["name", "age"]);
//         insert_query.values(&[person]);
//
//         // Build the SQL query string
//         let sql = insert_query.build();
//         // let query = InsertQuery::new("company").values(&Company {
//         //     name: "SurrealDB".to_string(),
//         //     founded: "2021-09-10".to_string(),
//         //     founders: vec![
//         //         Founder {
//         //             person: "tobie".to_string(),
//         //         },
//         //         Founder {
//         //             person: "jaime".to_string(),
//         //         },
//         //     ],
//         //     tags: vec!["big data".to_string(), "database".to_string()],
//         // });
//         // assert_eq!(query, "INSERT INTO company {\"name\":\"SurrealDB\",\"founded\":\"2021-09-10\",\"founders\":[{\"person\":\"tobie\"},{\"person\":\"jaime\"}],\"tags\":[\"big data\",\"database\"]};");
//     }
// }
// ///
// ///
// /// Third
// mod xxxxx {
//
//     use std::collections::HashMap;
//
//     use serde::Serialize;
//
//     #[derive(Serialize)]
//     #[serde(rename_all = "snake_case")]
//     enum Value<'a> {
//         Str(&'a str),
//         Struct(HashMap<&'a str, Value<'a>>),
//         Array(Vec<Value<'a>>),
//     }
//
//     impl<'a> Value<'a> {
//         fn from_serde_value(v: &'a serde_json::Value) -> Self {
//             match v {
//                 serde_json::Value::String(s) => Value::Str(s),
//                 serde_json::Value::Array(a) => {
//                     let v: Vec<Value<'a>> = a.iter().map(Value::from_serde_value).collect();
//                     Value::Array(v)
//                 }
//                 serde_json::Value::Object(o) => {
//                     let v: HashMap<&'a str, Value<'a>> = o
//                         .iter()
//                         .map(|(k, v)| (k.as_str(), Value::from_serde_value(v)))
//                         .collect();
//                     Value::Struct(v)
//                 }
//                 _ => unreachable!(),
//             }
//         }
//
//         fn to_sql_value(&self) -> String {
//             match self {
//                 Value::Str(s) => s.to_string(),
//                 Value::Struct(fields) => {
//                     let fields = fields
//                         .iter()
//                         .map(|(k, v)| format!("{}: {}", k, v.to_sql_value()))
//                         .collect::<Vec<String>>()
//                         .join(", ");
//                     format!("{{{}}}", fields)
//                 }
//                 Value::Array(values) => {
//                     let values = values
//                         .iter()
//                         .map(Value::to_sql_value)
//                         .collect::<Vec<String>>()
//                         .join(", ");
//                     format!("[{}]", values)
//                 }
//             }
//         }
//     }
//
//     pub struct InsertQuery<'a> {
//         table: &'a str,
//         fields: Vec<&'a str>,
//         values: Vec<Vec<Value<'a>>>,
//         on_duplicate_key_update: Vec<(&'a str, &'a str)>,
//     }
//
//     impl<'a> InsertQuery<'a> {
//         pub fn new(table: &'a str) -> Self {
//             Self {
//                 table,
//                 fields: Vec::new(),
//                 values: Vec::new(),
//                 on_duplicate_key_update: Vec::new(),
//             }
//         }
//
//         pub fn fields(&mut self, fields: &'a [&'a str]) -> &mut Self {
//             self.fields = fields.to_vec();
//             self
//         }
//
//         pub fn values(&mut self, values: &'a [Vec<serde_json::Value>]) -> &mut Self {
//             self.values = values
//                 .iter()
//                 .map(|row| row.iter().map(Value::from_serde_value).collect())
//                 .collect();
//             self
//         }
//
//         pub fn on_duplicate_key_update(&mut self, fields: &'a [(&'a str, &'a str)]) -> &mut Self {
//             self.on_duplicate_key_update = fields.to_vec();
//             self
//         }
//
//         pub fn build(&self) -> String {
//             let fields = self.fields.join(", ");
//             let values = self
//                 .values
//                 .iter()
//                 .map(|row| {
//                     let row_values = row
//                         .iter()
//                         .map(Value::to_sql_value)
//                         .collect::<Vec<String>>()
//                         .join(", ");
//                     format!("({})", row_values)
//                 })
//                 .collect::<Vec<String>>()
//                 .join(", ");
//             let on_duplicate_key_update = if !self.on_duplicate_key_update.is_empty() {
//                 let fields = self
//                     .on_duplicate_key_update
//                     .iter()
//                     .map(|(k, v)| format!("{} = {}", k, v))
//                     .collect::<Vec<String>>()
//                     .join(", ");
//                 format!("ON DUPLICATE KEY UPDATE {}", fields)
//             } else {
//                 String::new()
//             };
//             format!(
//                 "INSERT INTO {} ({}) VALUES {} {};",
//                 self.table, fields, values, on_duplicate_key_update
//             )
//         }
//     }
