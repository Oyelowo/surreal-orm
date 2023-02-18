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
