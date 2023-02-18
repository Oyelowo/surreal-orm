use std::collections::HashMap;

struct InsertQuery {
    table: String,
    fields: Option<Vec<String>>,
    values: Vec<Vec<String>>,
    on_duplicate_key_update: Option<HashMap<String, String>>,
}

impl InsertQuery {
    fn new(table: &str) -> InsertQuery {
        InsertQuery {
            table: String::from(table),
            fields: None,
            values: Vec::new(),
            on_duplicate_key_update: None,
        }
    }

    fn fields(mut self, fields: &[&str]) -> InsertQuery {
        self.fields = Some(fields.iter().map(|f| String::from(*f)).collect());
        self
    }

    fn values(mut self, values: &[&[&str]]) -> InsertQuery {
        self.values = values
            .iter()
            .map(|v| v.iter().map(|s| String::from(*s)).collect())
            .collect();
        self
    }

    fn on_duplicate_key_update(mut self, updates: &[(&str, &str)]) -> InsertQuery {
        let update_map: HashMap<String, String> = updates
            .iter()
            .map(|(k, v)| (String::from(*k), String::from(*v)))
            .collect();
        self.on_duplicate_key_update = Some(update_map);
        self
    }

    fn build(&self) -> String {
        let mut query = String::from("INSERT INTO ");
        query.push_str(&self.table);

        if let Some(fields) = &self.fields {
            let fields_str = fields.join(", ");
            query.push_str(&format!(" ({}) ", fields_str));
        }

        if !self.values.is_empty() {
            let values_str: Vec<String> = self
                .values
                .iter()
                .map(|v| {
                    let values_list = v.join(", ");
                    format!("({})", values_list)
                })
                .collect();

            query.push_str(" VALUES ");
            query.push_str(&values_str.join(", "));
        }

        if let Some(update_map) = &self.on_duplicate_key_update {
            let updates_str: Vec<String> = update_map
                .iter()
                .map(|(k, v)| format!("{} = {}", k, v))
                .collect();

            query.push_str(" ON DUPLICATE KEY UPDATE ");
            query.push_str(&updates_str.join(", "));
        }

        query.push_str(";");

        query
    }
}
mod xfdf {

    use std::collections::HashMap;

    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename_all = "snake_case")]
    enum Value<'a> {
        Str(&'a str),
        Struct(HashMap<&'a str, Value<'a>>),
        Array(Vec<Value<'a>>),
    }

    impl<'a> Value<'a> {
        fn from_serde_value(v: &'a serde_json::Value) -> Self {
            match v {
                serde_json::Value::String(s) => Value::Str(s),
                serde_json::Value::Array(a) => {
                    let v: Vec<Value<'a>> = a.iter().map(Value::from_serde_value).collect();
                    Value::Array(v)
                }
                serde_json::Value::Object(o) => {
                    let v: HashMap<&'a str, Value<'a>> = o
                        .iter()
                        .map(|(k, v)| (k.as_str(), Value::from_serde_value(v)))
                        .collect();
                    Value::Struct(v)
                }
                _ => unreachable!(),
            }
        }

        fn to_sql_value(&self) -> String {
            match self {
                Value::Str(s) => s.to_string(),
                Value::Struct(fields) => {
                    let fields = fields
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v.to_sql_value()))
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{{{}}}", fields)
                }
                Value::Array(values) => {
                    let values = values
                        .iter()
                        .map(Value::to_sql_value)
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("[{}]", values)
                }
            }
        }
    }

    pub struct InsertQuery<'a> {
        table: &'a str,
        fields: Vec<&'a str>,
        values: Vec<Value<'a>>,
        on_duplicate_key_update: Vec<(&'a str, &'a str)>,
    }

    impl<'a> InsertQuery<'a> {
        pub fn new(table: &'a str) -> Self {
            Self {
                table,
                fields: Vec::new(),
                values: Vec::new(),
                on_duplicate_key_update: Vec::new(),
            }
        }

        pub fn fields(&mut self, fields: &'a [&'a str]) -> &mut Self {
            self.fields = fields.to_vec();
            self
        }

        pub fn values(&mut self, values: &'a [serde_json::Value]) -> &mut Self {
            self.values = values.iter().map(Value::from_serde_value).collect();
            self
        }

        pub fn on_duplicate_key_update(&mut self, fields: &'a [(&'a str, &'a str)]) -> &mut Self {
            self.on_duplicate_key_update = fields.to_vec();
            self
        }

        pub fn build(&self) -> String {
            let fields = self.fields.join(", ");
            let values = self
                .values
                .iter()
                .map(Value::to_sql_value)
                .collect::<Vec<String>>()
                .join(", ");
            let mut sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                self.table, fields, values
            );

            if !self.on_duplicate_key_update.is_empty() {
                let fields = self
                    .on_duplicate_key_update
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ");

                sql.push_str(" ON DUPLICATE KEY UPDATE ");
                sql.push_str(&fields);
            }

            sql
        }
    }
}
mod xxxxx {

    use std::collections::HashMap;

    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename_all = "snake_case")]
    enum Value<'a> {
        Str(&'a str),
        Struct(HashMap<&'a str, Value<'a>>),
        Array(Vec<Value<'a>>),
    }

    impl<'a> Value<'a> {
        fn from_serde_value(v: &'a serde_json::Value) -> Self {
            match v {
                serde_json::Value::String(s) => Value::Str(s),
                serde_json::Value::Array(a) => {
                    let v: Vec<Value<'a>> = a.iter().map(Value::from_serde_value).collect();
                    Value::Array(v)
                }
                serde_json::Value::Object(o) => {
                    let v: HashMap<&'a str, Value<'a>> = o
                        .iter()
                        .map(|(k, v)| (k.as_str(), Value::from_serde_value(v)))
                        .collect();
                    Value::Struct(v)
                }
                _ => unreachable!(),
            }
        }

        fn to_sql_value(&self) -> String {
            match self {
                Value::Str(s) => s.to_string(),
                Value::Struct(fields) => {
                    let fields = fields
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v.to_sql_value()))
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{{{}}}", fields)
                }
                Value::Array(values) => {
                    let values = values
                        .iter()
                        .map(Value::to_sql_value)
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("[{}]", values)
                }
            }
        }
    }

    pub struct InsertQuery<'a> {
        table: &'a str,
        fields: Vec<&'a str>,
        values: Vec<Vec<Value<'a>>>,
        on_duplicate_key_update: Vec<(&'a str, &'a str)>,
    }

    impl<'a> InsertQuery<'a> {
        pub fn new(table: &'a str) -> Self {
            Self {
                table,
                fields: Vec::new(),
                values: Vec::new(),
                on_duplicate_key_update: Vec::new(),
            }
        }

        pub fn fields(&mut self, fields: &'a [&'a str]) -> &mut Self {
            self.fields = fields.to_vec();
            self
        }

        pub fn values(&mut self, values: &'a [Vec<serde_json::Value>]) -> &mut Self {
            self.values = values
                .iter()
                .map(|row| row.iter().map(Value::from_serde_value).collect())
                .collect();
            self
        }

        pub fn on_duplicate_key_update(&mut self, fields: &'a [(&'a str, &'a str)]) -> &mut Self {
            self.on_duplicate_key_update = fields.to_vec();
            self
        }

        pub fn build(&self) -> String {
            let fields = self.fields.join(", ");
            let values = self
                .values
                .iter()
                .map(|row| {
                    let row_values = row
                        .iter()
                        .map(Value::to_sql_value)
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("({})", row_values)
                })
                .collect::<Vec<String>>()
                .join(", ");
            let on_duplicate_key_update = if !self.on_duplicate_key_update.is_empty() {
                let fields = self
                    .on_duplicate_key_update
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("ON DUPLICATE KEY UPDATE {}", fields)
            } else {
                String::new()
            };
            format!(
                "INSERT INTO {} ({}) VALUES {} {};",
                self.table, fields, values, on_duplicate_key_update
            )
        }
    }
}
